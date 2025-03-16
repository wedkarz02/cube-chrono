use std::sync::Arc;

use super::{account_services, jwt_services};
use crate::services::utils::password_utils::{hash_password, verify_password};
use crate::{
    error::{AppError, AuthError},
    models::account::{Account, Role},
    routes::auth::AuthPayload,
    AppState,
};
use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse, Extension};
use mongodb::results::DeleteResult;

pub async fn register(
    state: &Arc<AppState>,
    auth_payload: AuthPayload,
    roles: &[Role],
) -> Result<Account, AppError> {
    if account_services::find_by_username(state, &auth_payload.username)
        .await?
        .is_some()
    {
        return Err(AuthError::UsernameAlreadyTaken.into());
    }

    let new_account = Account::new(
        &auth_payload.username,
        &hash_password(&auth_payload.password),
        roles,
    );

    let account_id = new_account.id;
    account_services::insert(state, new_account).await?;
    account_services::find_by_id(state, account_id)
        .await?
        .ok_or(anyhow::Error::msg("New account not inserted").into())
}

pub async fn login(
    state: &Arc<AppState>,
    auth_payload: AuthPayload,
) -> Result<(String, String), AppError> {
    let account = account_services::find_by_username(state, &auth_payload.username)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify_password(&account.hashed_password, &auth_payload.password) {
        return Err(AuthError::InvalidCredentials.into());
    }

    let (access_token, refresh_token) = jwt_services::generate_pair(
        account.id,
        &state
            .env
            .jwt_access_secret,
        &state
            .env
            .jwt_refresh_secret,
    )?;

    let refresh_out_token = refresh_token
        .token
        .clone();
    jwt_services::insert_refresh(state, refresh_token).await?;

    Ok((access_token, refresh_out_token))
}

pub async fn refresh(state: &Arc<AppState>, refresh_token: &str) -> Result<String, AppError> {
    if jwt_services::find_refresh_by_token(state, refresh_token)
        .await?
        .is_none()
    {
        return Err(AuthError::TokenInvalid.into());
    }

    let claims = jwt_services::decode_token(
        refresh_token,
        &state
            .env
            .jwt_refresh_secret,
    )?;

    let access_token = jwt_services::generate_token(
        claims.sub,
        chrono::Utc::now()
            .checked_add_signed(jwt_services::ACCESS_TOKEN_EXPIRATION)
            .ok_or(anyhow::Error::msg("Failed to create access token"))?
            .timestamp(),
        &state
            .env
            .jwt_access_secret,
    )?;

    Ok(access_token)
}

pub async fn logout(state: &Arc<AppState>, refresh_token: &str) -> Result<String, AppError> {
    let deleted_count = jwt_services::delete_refresh_by_token(state, refresh_token)
        .await?
        .deleted_count;

    if deleted_count != 0 {
        Ok("Logged out".to_string())
    } else {
        Err(AuthError::Unauthorized.into())
    }
}

pub async fn revoke_all_refresh_tokens(
    state: &Arc<AppState>,
    account: Account,
    password: &str,
) -> Result<DeleteResult, AppError> {
    if !verify_password(&account.hashed_password, password) {
        return Err(AuthError::InvalidCredentials.into());
    }

    let deleted_result = jwt_services::delete_many_refresh_by_account_id(state, account.id).await?;
    Ok(deleted_result)
}

pub async fn auth_guard(
    Extension(state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let access_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| {
            v.to_str()
                .ok()
        })
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AuthError::Unauthorized)?;

    let claims = jwt_services::decode_token(
        access_token,
        &state
            .env
            .jwt_access_secret,
    )?;

    let account = account_services::find_by_id(&state, claims.sub)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    req.extensions_mut()
        .insert(account);
    Ok(next
        .run(req)
        .await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::{mock, predicate::eq};
    use mongodb::bson::Uuid;

    #[derive(Debug, Clone)]
    pub struct MockDeleteResult {
        pub deleted_count: u64,
    }

    impl From<mongodb::results::DeleteResult> for MockDeleteResult {
        fn from(result: mongodb::results::DeleteResult) -> Self {
            MockDeleteResult {
                deleted_count: result.deleted_count,
            }
        }
    }

    #[async_trait]
    pub trait AuthService: Send + Sync {
        async fn register(
            &self,
            auth_payload: AuthPayload,
            roles: &[Role],
        ) -> Result<Account, AppError>;
        async fn login(&self, auth_payload: AuthPayload) -> Result<(String, String), AppError>;
        async fn refresh(&self, refresh_token: &str) -> Result<String, AppError>;
        async fn logout(&self, refresh_token: &str) -> Result<String, AppError>;
        async fn revoke_all_refresh_tokens(
            &self,
            account: Account,
            password: &str,
        ) -> Result<MockDeleteResult, AppError>;
    }

    mock! {
        pub AuthRepo {}

        #[async_trait]
        impl AuthService for AuthRepo {
            async fn register(&self, auth_payload: AuthPayload, roles: &[Role]) -> Result<Account, AppError>;
            async fn login(&self, auth_payload: AuthPayload) -> Result<(String, String), AppError>;
            async fn refresh(&self, refresh_token: &str) -> Result<String, AppError>;
            async fn logout(&self, refresh_token: &str) -> Result<String, AppError>;
            async fn revoke_all_refresh_tokens(&self, account: Account, password: &str) -> Result<MockDeleteResult, AppError>;
        }
    }

    #[tokio::test]
    async fn test_register() {
        let mut mock_repo = MockAuthRepo::new();
        let auth_payload = AuthPayload {
            username: "new_user".to_string(),
            password: "secure_password".to_string(),
        };
        let roles = vec![Role::User];

        let new_account = Account {
            id: Uuid::new(),
            username: auth_payload
                .username
                .clone(),
            hashed_password: "hashed_password".to_string(),
            roles: roles.clone(),
        };

        mock_repo
            .expect_register()
            .with(eq(auth_payload.clone()), eq(roles.clone()))
            .returning(move |_, _| Ok(new_account.clone()));

        let result = mock_repo
            .register(auth_payload, &roles)
            .await
            .unwrap();
        assert_eq!(result.username, "new_user");
    }

    #[tokio::test]
    async fn test_login() {
        let mut mock_repo = MockAuthRepo::new();
        let auth_payload = AuthPayload {
            username: "test_user".to_string(),
            password: "correct_password".to_string(),
        };

        let tokens = (
            "access_token_example".to_string(),
            "refresh_token_example".to_string(),
        );

        mock_repo
            .expect_login()
            .with(eq(auth_payload.clone()))
            .returning(move |_| Ok(tokens.clone()));

        let result = mock_repo
            .login(auth_payload)
            .await
            .unwrap();
        assert_eq!(result.0, "access_token_example");
        assert_eq!(result.1, "refresh_token_example");
    }

    #[tokio::test]
    async fn test_refresh() {
        let mut mock_repo = MockAuthRepo::new();
        let refresh_token = "valid_refresh_token";

        mock_repo
            .expect_refresh()
            .with(eq(refresh_token.to_string()))
            .returning(move |_| Ok("new_access_token".to_string()));

        let result = mock_repo
            .refresh(refresh_token)
            .await
            .unwrap();
        assert_eq!(result, "new_access_token");
    }

    #[tokio::test]
    async fn test_logout() {
        let mut mock_repo = MockAuthRepo::new();
        let refresh_token = "valid_refresh_token";

        mock_repo
            .expect_logout()
            .with(eq(refresh_token.to_string()))
            .returning(move |_| Ok("Logged out".to_string()));

        let result = mock_repo
            .logout(refresh_token)
            .await
            .unwrap();
        assert_eq!(result, "Logged out");
    }

    #[tokio::test]
    async fn test_revoke_all_refresh_tokens() {
        let mut mock_repo = MockAuthRepo::new();
        let account = Account {
            id: Uuid::new(),
            username: "test_user".to_string(),
            hashed_password: "hashed_password".to_string(),
            roles: vec![Role::User],
        };
        let password = "correct_password";

        let delete_result = MockDeleteResult { deleted_count: 3 };

        mock_repo
            .expect_revoke_all_refresh_tokens()
            .with(eq(account.clone()), eq(password.to_string()))
            .returning(move |_, _| Ok(delete_result.clone()));

        let result = mock_repo
            .revoke_all_refresh_tokens(account, password)
            .await
            .unwrap();
        assert_eq!(result.deleted_count, 3);
    }
}

use std::sync::Arc;

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::{doc, Uuid};
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AuthError};
use crate::models::refresh_token::RefreshToken;
use crate::AppState;

use super::{get_collection, Collections};

pub const REFRESH_TOKEN_EXPIRATION: chrono::TimeDelta = chrono::Duration::days(30);
pub const ACCESS_TOKEN_EXPIRATION: chrono::TimeDelta = chrono::Duration::minutes(15);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
}

pub fn generate_token(sub: Uuid, exp: i64, secret: &str) -> Result<String, AppError> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims { sub, exp },
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| match *err.kind() {
        ErrorKind::ExpiredSignature => AuthError::TokenExpired,
        _ => AuthError::TokenInvalid,
    })?;

    Ok(token_data.claims)
}

pub fn generate_pair(
    sub: Uuid,
    access_secret: &str,
    refresh_secret: &str,
) -> Result<(String, RefreshToken), AppError> {
    let access_token = generate_token(
        sub,
        chrono::Utc::now()
            .checked_add_signed(ACCESS_TOKEN_EXPIRATION)
            .ok_or(anyhow::Error::msg("Failed to create access token"))?
            .timestamp(),
        access_secret,
    )?;

    let refresh_expiration_timestamp = chrono::Utc::now()
        .checked_add_signed(REFRESH_TOKEN_EXPIRATION)
        .ok_or(anyhow::Error::msg("Failed to create refresh token"))?
        .timestamp();

    let refresh_token = RefreshToken::new(
        sub,
        refresh_expiration_timestamp,
        &generate_token(sub, refresh_expiration_timestamp, refresh_secret)?,
    );

    Ok((access_token, refresh_token))
}

pub async fn insert_refresh(
    state: &Arc<AppState>,
    token: RefreshToken,
) -> Result<InsertOneResult, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);
    let result = refresh_tokens
        .insert_one(token)
        .await?;

    Ok(result)
}

#[allow(unused)]
pub async fn find_refresh_by_id(
    state: &Arc<AppState>,
    id: Uuid,
) -> Result<Option<RefreshToken>, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);
    let result = refresh_tokens
        .find_one(doc! { "_id": id })
        .await?;

    Ok(result)
}

#[allow(unused)]
pub async fn find_refresh_by_user_id(
    state: &Arc<AppState>,
    user_id: Uuid,
) -> Result<Option<RefreshToken>, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);
    let result = refresh_tokens
        .find_one(doc! { "user_id": user_id })
        .await?;

    Ok(result)
}

pub async fn find_refresh_by_token(
    state: &Arc<AppState>,
    token: &str,
) -> Result<Option<RefreshToken>, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);
    let result = refresh_tokens
        .find_one(doc! { "token": token })
        .await?;

    Ok(result)
}

#[allow(unused)]
pub async fn delete_refresh_by_id(
    state: &Arc<AppState>,
    id: Uuid,
) -> Result<DeleteResult, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);
    let result = refresh_tokens
        .delete_one(doc! { "_id": id })
        .await?;

    Ok(result)
}

pub async fn delete_many_refresh_by_account_id(
    state: &Arc<AppState>,
    id: Uuid,
) -> Result<DeleteResult, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);
    let result = refresh_tokens
        .delete_many(doc! { "account_id": id })
        .await?;

    Ok(result)
}

pub async fn delete_refresh_by_token(
    state: &Arc<AppState>,
    token: &str,
) -> Result<DeleteResult, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);
    let result = refresh_tokens
        .delete_one(doc! { "token": token })
        .await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::{mock, predicate::eq};
    use mongodb::bson::{doc, Bson, Uuid};
    use mongodb::results::{DeleteResult, InsertOneResult};

    #[derive(Debug, Clone)]
    pub struct MockInsertOneResult {
        #[allow(unused)]
        pub inserted_id: Bson,
    }

    impl From<InsertOneResult> for MockInsertOneResult {
        fn from(result: InsertOneResult) -> Self {
            MockInsertOneResult {
                inserted_id: result.inserted_id,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct MockDeleteResult {
        pub deleted_count: u64,
    }

    impl From<DeleteResult> for MockDeleteResult {
        fn from(result: DeleteResult) -> Self {
            MockDeleteResult {
                deleted_count: result.deleted_count,
            }
        }
    }

    #[async_trait]
    pub trait JwtRepository: Send + Sync {
        async fn insert_refresh(
            &self,
            token: RefreshToken,
        ) -> Result<MockInsertOneResult, AppError>;
        async fn find_refresh_by_token(
            &self,
            token: &str,
        ) -> Result<Option<RefreshToken>, AppError>;
        async fn delete_refresh_by_token(&self, token: &str) -> Result<MockDeleteResult, AppError>;
        async fn delete_many_refresh_by_account_id(
            &self,
            account_id: Uuid,
        ) -> Result<MockDeleteResult, AppError>;
    }

    mock! {
        pub JwtRepo {}

        #[async_trait]
        impl JwtRepository for JwtRepo {
            async fn insert_refresh(
                &self,
                token: RefreshToken,
            ) -> Result<MockInsertOneResult, AppError>;
            async fn find_refresh_by_token(
                &self,
                token: &str,
            ) -> Result<Option<RefreshToken>, AppError>;
            async fn delete_refresh_by_token(
                &self,
                token: &str,
            ) -> Result<MockDeleteResult, AppError>;
            async fn delete_many_refresh_by_account_id(
                &self,
                account_id: Uuid,
            ) -> Result<MockDeleteResult, AppError>;
        }
    }

    #[tokio::test]
    async fn test_generate_token() {
        let sub = Uuid::new();
        let secret = "test_secret";

        let result = generate_token(sub, 1234567890, secret);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_decode_token() {
        let sub = Uuid::new();
        let secret = "test_secret";
        let exp = chrono::Utc::now()
            .checked_add_signed(ACCESS_TOKEN_EXPIRATION)
            .unwrap()
            .timestamp();
        let token = generate_token(sub, exp, secret).unwrap();

        let result = decode_token(&token, secret);
        assert!(result.is_ok());
        assert_eq!(
            sub,
            result
                .unwrap()
                .sub
        );
    }

    #[tokio::test]
    async fn test_insert_refresh() {
        let mut mock_repo = MockJwtRepo::new();
        let refresh_token = RefreshToken {
            id: Uuid::new(),
            account_id: Uuid::new(),
            token: "sample_refresh_token".to_string(),
            expiry_timestamp: chrono::Utc::now().timestamp(),
        };

        let insert_result = MockInsertOneResult {
            inserted_id: Bson::Document(doc! { "_id": refresh_token.id }),
        };

        mock_repo
            .expect_insert_refresh()
            .with(eq(refresh_token.clone()))
            .returning(move |_| Ok(insert_result.clone()));

        let result = mock_repo
            .insert_refresh(refresh_token)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_refresh_by_token() {
        let mut mock_repo = MockJwtRepo::new();
        let token = "sample_refresh_token";

        let refresh_token = RefreshToken {
            id: Uuid::new(),
            account_id: Uuid::new(),
            token: token.to_string(),
            expiry_timestamp: chrono::Utc::now().timestamp(),
        };

        mock_repo
            .expect_find_refresh_by_token()
            .with(eq(token.to_string()))
            .returning(move |_| Ok(Some(refresh_token.clone())));

        let result = mock_repo
            .find_refresh_by_token(token)
            .await
            .unwrap();
        assert!(result.is_some());
        assert_eq!(
            result
                .unwrap()
                .token,
            token
        );
    }

    #[tokio::test]
    async fn test_delete_refresh_by_token() {
        let mut mock_repo = MockJwtRepo::new();
        let token = "sample_refresh_token";

        let delete_result = MockDeleteResult { deleted_count: 1 };

        mock_repo
            .expect_delete_refresh_by_token()
            .with(eq(token.to_string()))
            .returning(move |_| Ok(delete_result.clone()));

        let result = mock_repo
            .delete_refresh_by_token(token)
            .await
            .unwrap();
        assert_eq!(result.deleted_count, 1);
    }

    #[tokio::test]
    async fn test_delete_many_refresh_by_account_id() {
        let mut mock_repo = MockJwtRepo::new();
        let account_id = Uuid::new();

        let delete_result = MockDeleteResult { deleted_count: 1 };

        mock_repo
            .expect_delete_many_refresh_by_account_id()
            .with(eq(account_id))
            .returning(move |_| Ok(delete_result.clone()));

        let result = mock_repo
            .delete_many_refresh_by_account_id(account_id)
            .await
            .unwrap();
        assert_eq!(result.deleted_count, 1);
    }
}

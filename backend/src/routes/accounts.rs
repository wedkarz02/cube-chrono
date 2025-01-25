use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, put},
    Extension, Router,
};
use axum_extra::json;
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::{AppError, AuthError},
    models::account::{Account, AccountDto, Role},
    services::{
        self,
        utils::password_utils::{hash_password, verify_password},
        validation_services::{self, ValidatedJson, ValidatedPath},
    },
    AppState,
};

use super::PathId;

pub async fn read_logged(
    Extension(account): Extension<Account>,
) -> Result<impl IntoResponse, AppError> {
    let acc_dto = AccountDto::from(account);
    Ok((
        StatusCode::OK,
        json!({
            "message": "Account details found",
            "payload": {
                "logged_account": acc_dto
            }
        }),
    ))
}

#[derive(Deserialize, Validate)]
pub struct ChangeUsernamePayload {
    #[validate(length(min = 4, max = 32, message = "length must be in range (4..=32)"))]
    #[validate(custom(function = "validation_services::ascii_string"))]
    username: String,
}

async fn change_username(
    Extension(state): Extension<Arc<AppState>>,
    Extension(account): Extension<Account>,
    ValidatedJson(payload): ValidatedJson<ChangeUsernamePayload>,
) -> Result<impl IntoResponse, AppError> {
    if (services::account_services::find_by_username(&state, &payload.username).await?).is_some() {
        return Err(AuthError::UsernameAlreadyTaken.into());
    }

    let new_account = Account {
        id: account.id,
        username: payload.username,
        hashed_password: account.hashed_password,
        roles: account.roles,
    };

    let update_res = services::account_services::update(&state, new_account).await?;
    Ok((
        StatusCode::OK,
        json!({
            "message": "Username updated",
            "payload": {
                "modified_count": update_res.modified_count
            }
        }),
    ))
}

#[derive(Deserialize, Validate)]
pub struct ChangePasswordPayload {
    #[validate(custom(function = "validation_services::strong_password"))]
    new_password: String,
    old_password: String,
}

async fn change_password(
    Extension(state): Extension<Arc<AppState>>,
    Extension(account): Extension<Account>,
    ValidatedJson(payload): ValidatedJson<ChangePasswordPayload>,
) -> Result<impl IntoResponse, AppError> {
    if !verify_password(&account.hashed_password, &payload.old_password) {
        return Err(AuthError::InvalidCredentials.into());
    }

    let new_account = Account {
        id: account.id,
        username: account
            .username
            .clone(),
        hashed_password: hash_password(&payload.new_password),
        roles: account
            .roles
            .to_owned(),
    };

    let update_res = services::account_services::update(&state, new_account).await?;
    let revoked_count =
        services::auth_services::revoke_all_refresh_tokens(&state, account, &payload.new_password)
            .await?
            .deleted_count;

    Ok((
        StatusCode::OK,
        json!({
            "message": "Password updated, all sessions revoked",
            "paylaod": {
                "modified_count": update_res.modified_count,
                "revoked_sessions": format!("Successfully revoked all ({}) sessions", revoked_count)
            }
        }),
    ))
}

async fn delete_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Extension(account): Extension<Account>,
    ValidatedPath(path): ValidatedPath<PathId>,
) -> Result<impl IntoResponse, AppError> {
    if !account.has_role(Role::Admin) {
        return Err(AuthError::Forbidden.into());
    }

    let del_count = services::account_services::delete_by_id(&state, path.id)
        .await?
        .deleted_count;

    Ok((
        StatusCode::OK,
        json!({
            "message": if del_count > 0 { "Account deleted" } else { "Nothing to delete" },
            "payload": {
                "deleted_count": del_count
            }
        }),
    ))
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
        .route("/logged", get(read_logged))
        .route("/logged/change-username", put(change_username))
        .route("/logged/change-password", put(change_password))
        .route("/{id}", delete(delete_by_id))
        .layer(axum::middleware::from_fn(
            services::auth_services::auth_guard,
        ));

    Router::new()
        .merge(protected_routes)
        .layer(Extension(state))
}

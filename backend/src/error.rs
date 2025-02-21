use axum::{extract::rejection, http::StatusCode, response::IntoResponse};
use axum_extra::json;

macro_rules! impl_internal_from {
    ($($err_type:ty),* $(,)?) => {
        $(
            impl From<$err_type> for AppError {
                fn from(err: $err_type) -> Self {
                    AppError::Internal(anyhow::Error::new(err))
                }
            }
        )*
    };
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("Json rejection error: {0}")]
    JsonRejection(#[from] rejection::JsonRejection),
    #[error("Query rejection error: {0}")]
    QueryRejection(#[from] rejection::QueryRejection),
    #[error("Path rejection error: {0}")]
    PathRejection(#[from] rejection::PathRejection),
    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),
    #[error("Not implemented")]
    NotImplemented,
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        if let AppError::Internal(_) = self {
            tracing::error!("{}", self.to_string());
        }

        let (status, body) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Validation(vali_error) => (
                StatusCode::BAD_REQUEST,
                vali_error
                    .to_string()
                    .replace("\n", "; "),
            ),
            AppError::JsonRejection(json_error) => {
                (StatusCode::BAD_REQUEST, json_error.to_string())
            }
            AppError::QueryRejection(query_error) => {
                (StatusCode::BAD_REQUEST, query_error.to_string())
            }
            AppError::PathRejection(path_error) => {
                (StatusCode::BAD_REQUEST, path_error.to_string())
            }
            AppError::Auth(auth_error) => (auth_error.status_code(), auth_error.to_string()),
            AppError::NotImplemented => (StatusCode::NOT_IMPLEMENTED, self.to_string()),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error: Something went wrong".to_owned(),
            ),
        };
        (status, json!({ "error": body })).into_response()
    }
}

impl_internal_from!(mongodb::error::Error, jsonwebtoken::errors::Error);

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Token is invalid")]
    TokenInvalid,
    #[error("Token has expired")]
    TokenExpired,
    #[error("Username already taken")]
    UsernameAlreadyTaken,
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::Forbidden => StatusCode::FORBIDDEN,
            AuthError::TokenInvalid => StatusCode::UNAUTHORIZED,
            AuthError::TokenExpired => StatusCode::UNAUTHORIZED,
            AuthError::UsernameAlreadyTaken => StatusCode::CONFLICT,
        }
    }
}

use axum::{http::StatusCode, response::IntoResponse};
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
    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),
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
            AppError::Auth(auth_error) => (auth_error.status_code(), auth_error.to_string()),
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
    #[error("User already exists")]
    UserAlreadyExists,
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::Forbidden => StatusCode::FORBIDDEN,
            AuthError::UserAlreadyExists => StatusCode::CONFLICT,
        }
    }
}

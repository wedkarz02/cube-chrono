use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::{doc, Uuid};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AuthError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_token(sub: Uuid, exp: usize, secret: &str) -> Result<String, AppError> {
    let claims = Claims {
        sub,
        iat: chrono::Utc::now().timestamp() as usize,
        exp,
    };

    Ok(jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::Forbidden)?;

    Ok(token_data.claims)
}

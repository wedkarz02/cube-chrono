use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::{doc, Uuid};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AuthError};

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
    .map_err(|_| AuthError::TokenInvalid)?;

    Ok(token_data.claims)
}

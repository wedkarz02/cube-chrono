use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::{doc, Uuid};
use serde::{Deserialize, Serialize};

use crate::error::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_token(sub: Uuid, exp: usize, secret: &str) -> String {
    let claims = Claims {
        sub,
        iat: chrono::Utc::now().timestamp() as usize,
        exp,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
    let token_data = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::Forbidden)?;

    Ok(token_data.claims)
}

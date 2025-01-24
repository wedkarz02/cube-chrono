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

pub async fn delete_many_refresh_by_user_id(
    state: &Arc<AppState>,
    id: Uuid,
) -> Result<DeleteResult, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);
    let result = refresh_tokens
        .delete_many(doc! { "user_id": id })
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

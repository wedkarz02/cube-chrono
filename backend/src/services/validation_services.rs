use std::ops::RangeInclusive;

use axum::{
    extract::{FromRequest, FromRequestParts, Path, Query, Request},
    http::request::Parts,
    Json,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationError};

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedQuery(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPath<T>(pub T);

impl<T> FromRequestParts<()> for ValidatedPath<T>
where
    T: DeserializeOwned + Validate + Send,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &()) -> Result<Self, Self::Rejection> {
        let Path(value) = Path::<T>::from_request_parts(parts, &()).await?;
        value.validate()?;
        Ok(ValidatedPath(value))
    }
}

pub fn ascii_string(value: &str) -> Result<(), ValidationError> {
    if !value.is_ascii() {
        return Err(ValidationError::new("invalid")
            .with_message("must only contain ASCII characters".into()));
    }
    Ok(())
}

enum PasswordRules {
    Length(RangeInclusive<usize>),
    CapitalLetter,
    Digit,
    SpecialChar,
    Ascii,
}

impl PasswordRules {
    fn validate(&self, value: &str) -> bool {
        match self {
            PasswordRules::Length(range) => range.contains(&value.len()),
            PasswordRules::CapitalLetter => value
                .chars()
                .any(|c| c.is_uppercase()),
            PasswordRules::Digit => value
                .chars()
                .any(|c| c.is_ascii_digit()),
            PasswordRules::SpecialChar => value
                .chars()
                .any(|c| !c.is_alphanumeric()),
            PasswordRules::Ascii => value.is_ascii(),
        }
    }

    fn msg(&self) -> String {
        match self {
            PasswordRules::Length(range) => format!(
                "length must be in range ({}..={})",
                range.start(),
                range.end()
            ),
            PasswordRules::CapitalLetter => "must include at least one capital letter".to_string(),
            PasswordRules::Digit => "must include at least one digit".to_string(),
            PasswordRules::SpecialChar => "must include at least one special character".to_string(),
            PasswordRules::Ascii => "must only contain ASCII characters".to_string(),
        }
    }
}

pub fn strong_password(value: &str) -> Result<(), ValidationError> {
    let rules = vec![
        PasswordRules::Length(8..=256),
        PasswordRules::CapitalLetter,
        PasswordRules::Digit,
        PasswordRules::Ascii,
        PasswordRules::SpecialChar,
    ];

    for rule in rules {
        if !rule.validate(value) {
            return Err(ValidationError::new("invalid").with_message(
                rule.msg()
                    .into(),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Clone, Validate, Serialize, Deserialize)]
    pub struct TestPayload {
        #[validate(custom(function = "ascii_string"))]
        pub username: String,
        #[validate(custom(function = "strong_password"))]
        pub password: String,
    }

    #[tokio::test]
    async fn test_validated_json_success() {
        let valid_payload = TestPayload {
            username: "validuser".to_string(),
            password: "StrongP@ssw0rd".to_string(),
        };

        let body = serde_json::to_string(&valid_payload).unwrap();
        let req = Request::builder()
            .uri("/test")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let validated_json = ValidatedJson::<TestPayload>::from_request(req, &())
            .await
            .expect("Failed to validate JSON");

        assert_eq!(
            validated_json
                .0
                .username,
            "validuser"
        );
        assert_eq!(
            validated_json
                .0
                .password,
            "StrongP@ssw0rd"
        );
    }

    #[tokio::test]
    async fn test_validated_json_failure() {
        let invalid_payload = TestPayload {
            username: "invalid_无效的_user".to_string(),
            password: "weakpass".to_string(),
        };

        let body = serde_json::to_string(&invalid_payload).unwrap();
        let req = Request::builder()
            .uri("/test")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let result = ValidatedJson::<TestPayload>::from_request(req, &()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validated_query_success() {
        let query = "username=validuser&password=StrongP@ssw0rd";
        let req = Request::builder()
            .uri(format!("/test?{}", query))
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::empty())
            .unwrap();

        let validated_query = ValidatedQuery::<TestPayload>::from_request(req, &())
            .await
            .expect("Failed to validate query parameters");

        assert_eq!(
            validated_query
                .0
                .username,
            "validuser"
        );
        assert_eq!(
            validated_query
                .0
                .password,
            "StrongP@ssw0rd"
        );
    }

    #[tokio::test]
    async fn test_ascii_string_valid() {
        let valid_input = "valid_ascii";
        let result = ascii_string(valid_input);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ascii_string_invalid() {
        let invalid_input = "invalid_无效的_string";
        let result = ascii_string(invalid_input);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_strong_password_valid() {
        let valid_password = "ValidP@ssw0rd";
        let result = strong_password(valid_password);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_strong_password_invalid_length() {
        let invalid_password = "short";
        let result = strong_password(invalid_password);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_strong_password_invalid_no_digit() {
        let invalid_password = "NoDigitPassword";
        let result = strong_password(invalid_password);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_strong_password_invalid_no_special_char() {
        let invalid_password = "NoSpecialChar123";
        let result = strong_password(invalid_password);
        assert!(result.is_err());
    }
}

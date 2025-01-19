use std::ops::RangeInclusive;

use axum::{
    extract::{FromRequest, Request},
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

pub fn ascii_string(value: &str) -> Result<(), ValidationError> {
    if !value.is_ascii() {
        return Err(ValidationError::new("contains non-ASCII characters"));
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
            PasswordRules::Digit => "password must include at least one digit".to_string(),
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
        // Check for Ascii-only before special characters
        // (avoid goofy non-Ascii special characters just to be safe)
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

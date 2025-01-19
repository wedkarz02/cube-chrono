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
    MinLength,
    MaxLength,
    CapitalLetter,
    Digit,
    SpecialChar,
    Ascii,
}

impl PasswordRules {
    fn validate(&self, value: &str) -> bool {
        match self {
            PasswordRules::MinLength => self.min_length(value),
            PasswordRules::MaxLength => self.max_length(value),
            PasswordRules::CapitalLetter => self.capital_letter(value),
            PasswordRules::Digit => self.digit(value),
            PasswordRules::SpecialChar => self.special_char(value),
            PasswordRules::Ascii => self.ascii(value),
        }
    }

    fn msg(&self) -> &'static str {
        match self {
            PasswordRules::MinLength => "password must be at least 8 characters long",
            PasswordRules::MaxLength => "password must be at most 256 characters long",
            PasswordRules::CapitalLetter => "password must include at least one capital letter",
            PasswordRules::Digit => "password must include at least one digit",
            PasswordRules::SpecialChar => "password must include at least one special character",
            PasswordRules::Ascii => "password must only contain ASCII characters",
        }
    }

    fn min_length(&self, value: &str) -> bool {
        value.len() >= 8
    }

    fn max_length(&self, value: &str) -> bool {
        value.len() <= 256
    }

    fn capital_letter(&self, value: &str) -> bool {
        value
            .chars()
            .any(|c| c.is_uppercase())
    }

    fn digit(&self, value: &str) -> bool {
        value
            .chars()
            .any(|c| c.is_ascii_digit())
    }

    fn special_char(&self, value: &str) -> bool {
        value
            .chars()
            .any(|c| !c.is_alphanumeric())
    }

    // NOTE (wedkarz): A bit of reudundancy with ascii_string validator existing already
    //                 but this reduces complexity a bit
    fn ascii(&self, value: &str) -> bool {
        value.is_ascii()
    }
}

pub fn strong_password(value: &str) -> Result<(), ValidationError> {
    let rules = vec![
        // Check for Ascii-only first
        PasswordRules::Ascii,
        PasswordRules::MinLength,
        PasswordRules::MaxLength,
        PasswordRules::CapitalLetter,
        PasswordRules::Digit,
        PasswordRules::SpecialChar,
    ];

    for rule in rules {
        if !rule.validate(value) {
            return Err(ValidationError::new(rule.msg()));
        }
    }

    Ok(())
}

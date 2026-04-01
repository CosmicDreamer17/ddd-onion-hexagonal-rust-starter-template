use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct UserId(String);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Reconstitute from a persisted value (e.g. database row).
    pub fn from_persisted(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.is_empty() || !value.contains('@') {
            return Err(DomainError::InvalidEmail(value));
        }
        Ok(Self(value))
    }

    /// Reconstitute from a persisted value (e.g. database row).
    pub fn from_persisted(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.is_empty() {
            return Err(DomainError::ValidationError(
                "Username cannot be empty".to_string(),
            ));
        }
        Ok(Self(value))
    }

    /// Reconstitute from a persisted value (e.g. database row).
    pub fn from_persisted(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types_export() {
        // This test exists to trigger ts-rs export
    }

    #[test]
    fn test_email_validation_rejects_empty() {
        let result = Email::new(String::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_email_validation_rejects_missing_at() {
        let result = Email::new("not-an-email".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_email_validation_accepts_valid() {
        let result = Email::new("user@example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "user@example.com");
    }

    #[test]
    fn test_username_validation_rejects_empty() {
        let result = Username::new(String::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_username_validation_accepts_valid() {
        let result = Username::new("alice".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "alice");
    }
}

use serde::{Serialize, Deserialize};
use thiserror::Error;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct UserId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Email(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Username(pub String);

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
    #[error("Database error: {0}")]
    DatabaseError(String),
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
}

//! Shared error types used across validation modules in the Ursus library.
//! Centralising error definitions here keeps error handling consistent
//! and avoids duplication between modules.

use std::fmt;

/// General validation error that can be returned by any validation module.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValidationError {
    /// The input value is empty or missing.
    EmptyInput,
    /// The input value has an invalid format.
    InvalidFormat,
    /// The input value is too short.
    TooShort,
    /// The input value is too long.
    TooLong,
    /// The value does not match the expected pattern or reference.
    Mismatch,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyInput => write!(f, "input is empty"),
            ValidationError::InvalidFormat => write!(f, "invalid format"),
            ValidationError::TooShort => write!(f, "value is too short"),
            ValidationError::TooLong => write!(f, "value is too long"),
            ValidationError::Mismatch => write!(f, "values do not match"),
        }
    }
}

impl std::error::Error for ValidationError {}

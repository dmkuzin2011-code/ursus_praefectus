//! # Phone number validation module
//!
//! Provides synchronous validation for international phone numbers.
//! Accepts an optional leading `+` followed by digits and common separators
//! (spaces, dashes, parentheses, dots).

use crate::Ursus;

/// Errors that can occur during phone number validation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PhoneError {
    /// The input string is empty.
    EmptyInput,
    /// The phone number contains invalid characters.
    InvalidCharacters,
    /// The phone number has too few digits.
    TooShort,
    /// The phone number has too many digits.
    TooLong,
}

impl std::fmt::Display for PhoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoneError::EmptyInput => write!(f, "phone number is empty"),
            PhoneError::InvalidCharacters => write!(f, "phone number contains invalid characters"),
            PhoneError::TooShort => write!(f, "phone number is too short (minimum 7 digits)"),
            PhoneError::TooLong => write!(f, "phone number is too long (maximum 15 digits)"),
        }
    }
}

impl std::error::Error for PhoneError {}

/// Minimum and maximum number of digits for a valid phone number.
const MIN_PHONE_DIGITS: usize = 7;
const MAX_PHONE_DIGITS: usize = 15;

/// Characters allowed besides digits in a phone number.
const ALLOWED_SEPARATORS: &[char] = &[' ', '-', '(', ')', '.', '+'];

impl Ursus {
#[cfg(feature = "phone")]
pub async fn validate_phone(phone: &str) -> Result<(), PhoneError> {
    if phone.is_empty() {
        return Err(PhoneError::EmptyInput);
    }

    let mut digit_count = 0;

    for (i, c) in phone.chars().enumerate() {
        if c.is_ascii_digit() {
            digit_count += 1;
        } else if c == '+' && i == 0 {
        } else if ALLOWED_SEPARATORS.contains(&c) {
        } else {
            return Err(PhoneError::InvalidCharacters);
        }
    }

    if digit_count < MIN_PHONE_DIGITS {
        return Err(PhoneError::TooShort);
    }

    if digit_count > MAX_PHONE_DIGITS {
        return Err(PhoneError::TooLong);
    }

    Ok(())
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_phones() {
        assert!(Ursus::validate_phone("+1 234 567 8900").await.is_ok());
        assert!(Ursus::validate_phone("+44-20-7946-0958").await.is_ok());
        assert!(Ursus::validate_phone("555-1234").await.is_ok());
        assert!(Ursus::validate_phone("+7 (495) 123-45-67").await.is_ok());
        assert!(Ursus::validate_phone("1.800.555.0199").await.is_ok());
        assert!(Ursus::validate_phone("+1234567").await.is_ok()); // exactly 7 digits
    }

    #[tokio::test]
    async fn test_invalid_phones() {
        assert!(Ursus::validate_phone("").await.is_err());
        assert!(Ursus::validate_phone("12345").await.is_err()); // too short
        assert!(Ursus::validate_phone("abc").await.is_err());
        assert!(Ursus::validate_phone("+abc-def").await.is_err());
        assert!(Ursus::validate_phone("1234567890123456").await.is_err()); // 16 digits, too long
    }

    #[test]
    fn test_phone_error_display() {
        assert_eq!(PhoneError::EmptyInput.to_string(), "phone number is empty");
        assert_eq!(
            PhoneError::TooShort.to_string(),
            "phone number is too short (minimum 7 digits)"
        );
    }
}
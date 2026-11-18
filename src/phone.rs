//! # Phone number validation module
//!
//! Provides synchronous validation for international phone numbers.
//! Accepts an optional leading `+` followed by digits and common separators
//! (spaces, dashes, parentheses, dots).

use crate::Ursus;
use crate::specific_errors::ValidationError;

/// Minimum and maximum number of digits for a valid phone number.
const MIN_PHONE_DIGITS: usize = 7;
const MAX_PHONE_DIGITS: usize = 15;

/// Characters allowed besides digits in a phone number.
const ALLOWED_SEPARATORS: &[char] = &[' ', '-', '(', ')', '.', '+'];

impl Ursus {
#[cfg(feature = "phone")]
pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if phone.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    let mut digit_count = 0;

    for (i, c) in phone.chars().enumerate() {
        if c.is_ascii_digit() {
            digit_count += 1;
        } else if c == '+' && i == 0 {
        } else if ALLOWED_SEPARATORS.contains(&c) {
        } else {
            return Err(ValidationError::InvalidFormat);
        }
    }

    if digit_count < MIN_PHONE_DIGITS {
        return Err(ValidationError::TooShort);
    }

    if digit_count > MAX_PHONE_DIGITS {
        return Err(ValidationError::TooLong);
    }

    Ok(())
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phones() {
        assert!(Ursus::validate_phone("+1 234 567 8900").is_ok());
        assert!(Ursus::validate_phone("+44-20-7946-0958").is_ok());
        assert!(Ursus::validate_phone("555-1234").is_ok());
        assert!(Ursus::validate_phone("+7 (495) 123-45-67").is_ok());
        assert!(Ursus::validate_phone("1.800.555.0199").is_ok());
        assert!(Ursus::validate_phone("+1234567").is_ok()); // exactly 7 digits
    }

    #[test]
    fn test_invalid_phones() {
        assert!(Ursus::validate_phone("").is_err());
        assert!(Ursus::validate_phone("12345").is_err()); // too short
        assert!(Ursus::validate_phone("abc").is_err());
        assert!(Ursus::validate_phone("+abc-def").is_err());
        assert!(Ursus::validate_phone("1234567890123456").is_err()); // 16 digits, too long
    }

}
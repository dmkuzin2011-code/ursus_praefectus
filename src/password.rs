use crate::Ursus;
use crate::specific_errors::ValidationError;

impl Ursus {
/// Checks that password length is at least `size`.
pub fn validate_min_len(password: &str, size: usize) -> Result<bool, ValidationError> {
    if password.chars().count() < size {
        Err(ValidationError::TooShort)
    } else {
        Ok(true)
    }
}

/// Checks that password length is at most `size`.
pub fn validate_max_len(password: &str, size: usize) -> Result<bool, ValidationError> {
    if password.chars().count() > size {
        Err(ValidationError::TooLong)
    } else {
        Ok(true)
    }
}

/// Compares password with expected value **case-sensitive**.
pub fn compare_case_sensitive(password: &str, correct: &str) -> Result<bool, ValidationError> {
    if password == correct {
        Ok(true)
    } else {
        Err(ValidationError::Mismatch)
    }
}

/// Compares password with expected value **case-insensitive**.
pub fn compare_case_insensitive(password: &str, correct: &str) -> Result<bool, ValidationError> {
    if password.to_lowercase() == correct.to_lowercase() {
        Ok(true)
    } else {
        Err(ValidationError::Mismatch)
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_min_len() {
        assert!(Ursus::validate_min_len("a", 5).is_err());
    }

    #[test]
    fn test_validate_max_len() {
        assert!(Ursus::validate_max_len("aaaaaa", 5).is_err());
    }

    #[test]
    fn test_compare_case_sensitive() {
        assert!(Ursus::compare_case_sensitive("a", "a").is_ok());
        assert!(Ursus::compare_case_sensitive("A", "a").is_err());
    }

    #[test]
    fn test_compare_case_insensitive() {
        assert!(Ursus::compare_case_insensitive("A", "a").is_ok());
    }
}
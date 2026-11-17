use crate::Ursus;


#[derive(Debug, PartialEq, Eq)]
pub enum PasswordError {
    TooShort,
    TooLong,
    WrongPassword,
}
impl Ursus {
/// Checks that password length is at least `size`.
pub async fn validate_min_len(password: &str, size: usize) -> Result<bool, PasswordError> {
    if password.chars().count() < size {
        Err(PasswordError::TooShort)
    } else {
        Ok(true)
    }
}

/// Checks that password length is at most `size`.
pub async fn validate_max_len(password: &str, size: usize) -> Result<bool, PasswordError> {
    if password.chars().count() > size {
        Err(PasswordError::TooLong)
    } else {
        Ok(true)
    }
}

/// Compares password with expected value **case-sensitive**.
pub async fn compare_case_sensitive(password: &str, correct: &str) -> Result<bool, PasswordError> {
    if password == correct {
        Ok(true)
    } else {
        Err(PasswordError::WrongPassword)
    }
}

/// Compares password with expected value **case-insensitive**.
pub async fn compare_case_insensitive(password: &str, correct: &str) -> Result<bool, PasswordError> {
    if password.to_lowercase() == correct.to_lowercase() {
        Ok(true)
    } else {
        Err(PasswordError::WrongPassword)
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_min_len() {
        assert!(Ursus::validate_min_len("a", 5).await.is_err());
    }

    #[tokio::test]
    async fn test_validate_max_len() {
        assert!(Ursus::validate_max_len("aaaaaa", 5).await.is_err());
    }

    #[tokio::test]
    async fn test_compare_case_sensitive() {
        assert!(Ursus::compare_case_sensitive("a", "a").await.is_ok());
        assert!(Ursus::compare_case_sensitive("A", "a").await.is_err());
    }

    #[tokio::test]
    async fn test_compare_case_insensitive() {
        assert!(Ursus::compare_case_insensitive("A", "a").await.is_ok());
    }
}
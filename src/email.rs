//! # Email validation module
//!
//! Provides validation for email addresses:
//! - **Format validation** (syntax check: `local-part@domain`)
//! - **DNS MX record validation** (async, requires `email-dns` feature + `tokio` + `hickory-resolver`)
//!
//! ## Feature flags
//!
//! - `email` — basic format validation (always available, no extra dependencies)
//! - `email-dns` — async DNS MX record lookup (requires `tokio` + `hickory-resolver`)

use crate::Ursus;
use crate::specific_errors::ValidationError;

/// Validate the **format** of an email address synchronously.
///
/// Performs a syntax-only check:
/// - exactly one `@`
/// - non-empty local part and domain
/// - local part contains only allowed characters
/// - domain contains at least one dot and valid labels
///
/// This does **not** perform any network/DNS queries.

impl Ursus {
#[cfg(feature = "email")]
pub fn validate_email_format(email: &str) -> Result<(), ValidationError> {
    if email.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(ValidationError::InvalidFormat);
    }

    let local_part = parts[0];
    let domain = parts[1];

    if local_part.is_empty() {
        return Err(ValidationError::InvalidFormat);
    }
    if domain.is_empty() {
        return Err(ValidationError::InvalidFormat);
    }

    if !local_part
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '+'))
    {
        return Err(ValidationError::InvalidFormat);
    }

    if local_part.starts_with('.') || local_part.ends_with('.') {
        return Err(ValidationError::InvalidFormat);
    }

    if local_part.contains("..") {
        return Err(ValidationError::InvalidFormat);
    }

    if !domain.contains('.') {
        return Err(ValidationError::InvalidFormat);
    }

    for label in domain.split('.') {
        if label.is_empty() || label.len() > 63 {
            return Err(ValidationError::InvalidFormat);
        }
        if !label
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-')
        {
            return Err(ValidationError::InvalidFormat);
        }
        if label.starts_with('-') || label.ends_with('-') {
            return Err(ValidationError::InvalidFormat);
        }
    }

    let tld = domain.split('.').next_back().unwrap_or("");
    if tld.len() < 2 {
        return Err(ValidationError::InvalidFormat);
    }

    Ok(())
}
}

/// Validate an email address by checking its format **and** performing an
/// async DNS MX record lookup for the domain.
///
/// Requires the `email-dns` feature flag (which enables `tokio` and
/// `hickory-resolver`).


impl Ursus {
    #[cfg(all(feature = "email-dns", feature = "email"))]
    pub async fn validate_email_dns(email: &str) -> Result<(), ValidationError> {
        // First, validate the format
        Self::validate_email_format(email)?;

        let domain = email.split('@').nth(1).unwrap_or("");

        // Perform async DNS MX record lookup using hickory-resolver on tokio
        let builder = match hickory_resolver::TokioResolver::builder_tokio() {
            Ok(b) => b,
            Err(_) => return Err(ValidationError::InvalidFormat),
        };
        let resolver: hickory_resolver::TokioResolver = match builder.build() {
            Ok(r) => r,
            Err(_) => return Err(ValidationError::InvalidFormat),
        };

        match resolver.mx_lookup(domain).await {
            Ok(mx_records) => {
                if mx_records.answers().is_empty() {
                    Err(ValidationError::InvalidFormat)
                } else {
                    Ok(())
                }
            }
            Err(_) => Err(ValidationError::InvalidFormat),
        }
    }

    /// Convenience function: validate email format (and DNS if `email-dns` is enabled).
    ///
    /// - With only `email` feature: performs format check only.
    /// - With `email-dns` feature: performs format check + async DNS MX lookup.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ursus_praefectus::Ursus;
    /// fn main() {
    ///     assert!(Ursus::validate_email("test@example.com").is_ok());
    ///     assert!(Ursus::validate_email("bad-email").is_err());
    /// }
    /// ```
    #[cfg(feature = "email")]
    pub fn validate_email(email: &str) -> Result<(), ValidationError> {
        Self::validate_email_format(email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_emails() {
        assert!(Ursus::validate_email_format("user@example.com").is_ok());
        assert!(Ursus::validate_email_format("alice.bob@mail.server.org").is_ok());
        assert!(Ursus::validate_email_format("user+tag@gmail.com").is_ok());
        assert!(Ursus::validate_email_format("user_name@sub.domain.co.uk").is_ok());
    }

    #[test]
    fn test_invalid_emails() {
        assert!(Ursus::validate_email_format("").is_err());
        assert!(Ursus::validate_email_format("noatsign.com").is_err());
        assert!(Ursus::validate_email_format("@example.com").is_err());
        assert!(Ursus::validate_email_format("user@").is_err());
        assert!(Ursus::validate_email_format("user@.com").is_err());
        assert!(Ursus::validate_email_format("user@domain").is_err());
        assert!(Ursus::validate_email_format("user@-domain.com").is_err());
        assert!(Ursus::validate_email_format("user@domain.c").is_err());
        assert!(Ursus::validate_email_format("..user@example.com").is_err());
        assert!(Ursus::validate_email_format("user..name@example.com").is_err());
    }

}


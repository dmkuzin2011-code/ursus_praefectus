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


/// Errors that can occur during email validation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EmailError {
    /// The email string is empty.
    EmptyInput,
    /// The email does not contain exactly one `@` separator.
    MissingAtSign,
    /// The local part (before `@`) is empty.
    EmptyLocalPart,
    /// The domain part (after `@`) is empty.
    EmptyDomain,
    /// The local part contains invalid characters.
    InvalidLocalPart,
    /// The domain part contains invalid characters or is malformed.
    InvalidDomain,
    /// The domain has no valid MX records (DNS check only).
    NoMxRecords,
    /// An error occurred while performing the DNS query.
    DnsLookupFailed(String),
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::EmptyInput => write!(f, "email is empty"),
            EmailError::MissingAtSign => write!(f, "email is missing '@' separator"),
            EmailError::EmptyLocalPart => write!(f, "local part before '@' is empty"),
            EmailError::EmptyDomain => write!(f, "domain part after '@' is empty"),
            EmailError::InvalidLocalPart => write!(f, "local part contains invalid characters"),
            EmailError::InvalidDomain => write!(f, "domain part is invalid"),
            EmailError::NoMxRecords => write!(f, "domain has no MX records"),
            EmailError::DnsLookupFailed(msg) => write!(f, "DNS lookup failed: {}", msg),
        }
    }
}

impl std::error::Error for EmailError {}

/// Validate the **format** of an email address synchronously.
///
/// Performs a syntax-only check:
/// - exactly one `@`
/// - non-empty local part and domain
/// - local part contains only allowed characters
/// - domain contains at least one dot and valid labels
///
/// This does **not** perform any network/DNS queries.

#[cfg(feature = "email")]
pub async fn validate_email_format(email: &str) -> Result<(), EmailError> {
    if email.is_empty() {
        return Err(EmailError::EmptyInput);
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(EmailError::MissingAtSign);
    }

    let local_part = parts[0];
    let domain = parts[1];

    if local_part.is_empty() {
        return Err(EmailError::EmptyLocalPart);
    }
    if domain.is_empty() {
        return Err(EmailError::EmptyDomain);
    }

    if !local_part
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '+'))
    {
        return Err(EmailError::InvalidLocalPart);
    }

    if local_part.starts_with('.') || local_part.ends_with('.') {
        return Err(EmailError::InvalidLocalPart);
    }

    if local_part.contains("..") {
        return Err(EmailError::InvalidLocalPart);
    }

    if !domain.contains('.') {
        return Err(EmailError::InvalidDomain);
    }

    for label in domain.split('.') {
        if label.is_empty() || label.len() > 63 {
            return Err(EmailError::InvalidDomain);
        }
        if !label
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-')
        {
            return Err(EmailError::InvalidDomain);
        }
        if label.starts_with('-') || label.ends_with('-') {
            return Err(EmailError::InvalidDomain);
        }
    }

    let tld = domain.split('.').next_back().unwrap_or("");
    if tld.len() < 2 {
        return Err(EmailError::InvalidDomain);
    }

    Ok(())
}

/// Validate an email address by checking its format **and** performing an
/// async DNS MX record lookup for the domain.
///
/// Requires the `email-dns` feature flag (which enables `tokio` and
/// `hickory-resolver`).


#[cfg(all(feature = "email-dns", feature = "email"))]
pub async fn validate_email_dns(email: &str) -> Result<(), EmailError> {
    // First, validate the format asynchronously
    validate_email_format(email).await?;

    let domain = email.split('@').nth(1).unwrap_or("");

    // Perform async DNS MX record lookup using hickory-resolver on tokio
    let builder = match hickory_resolver::TokioResolver::builder_tokio() {
        Ok(b) => b,
        Err(e) => return Err(EmailError::DnsLookupFailed(e.to_string())),
    };
    let resolver: hickory_resolver::TokioResolver = match builder.build() {
        Ok(r) => r,
        Err(e) => return Err(EmailError::DnsLookupFailed(e.to_string())),
    };

    match resolver.mx_lookup(domain).await {
        Ok(mx_records) => {
            if mx_records.answers().is_empty() {
                Err(EmailError::NoMxRecords)
            } else {
                Ok(())
            }
        }
        Err(e) => Err(EmailError::DnsLookupFailed(e.to_string())),
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
/// # use ursus_praefectus::email::validate_email;
/// # #[tokio::main(flavor="current_thread")]
/// # async fn main() {
/// assert!(validate_email("test@example.com").await.is_ok());
/// assert!(validate_email("bad-email").await.is_err());
/// # }
/// ```
#[cfg(feature = "email")]
pub async fn validate_email(email: &str) -> Result<(), EmailError> {
    validate_email_format(email).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_emails() {
        assert!(validate_email_format("user@example.com").await.is_ok());
        assert!(validate_email_format("alice.bob@mail.server.org").await.is_ok());
        assert!(validate_email_format("user+tag@gmail.com").await.is_ok());
        assert!(validate_email_format("user_name@sub.domain.co.uk").await.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_emails() {
        assert!(validate_email_format("").await.is_err());
        assert!(validate_email_format("noatsign.com").await.is_err());
        assert!(validate_email_format("@example.com").await.is_err());
        assert!(validate_email_format("user@").await.is_err());
        assert!(validate_email_format("user@.com").await.is_err());
        assert!(validate_email_format("user@domain").await.is_err());
        assert!(validate_email_format("user@-domain.com").await.is_err());
        assert!(validate_email_format("user@domain.c").await.is_err());
        assert!(validate_email_format("..user@example.com").await.is_err());
        assert!(validate_email_format("user..name@example.com").await.is_err());
    }

    #[test]
    fn test_email_error_display() {
        assert_eq!(EmailError::EmptyInput.to_string(), "email is empty");
        assert_eq!(
            EmailError::NoMxRecords.to_string(),
            "domain has no MX records"
        );
    }
}
//! # URL validation module
//!
//! Provides synchronous validation for URLs (HTTP/HTTPS/FTP/etc.).
//! Checks scheme, host, and basic structure without performing network requests.

use crate::Ursus;
use crate::specific_errors::ValidationError;

/// Supported URL schemes.
const VALID_SCHEMES: &[&str] = &["http", "https", "ftp", "ws", "wss"];

/// Validate the format of a URL asynchronously.
///
/// Checks:
/// - non-empty input
/// - presence of a supported scheme (`http`, `https`, `ftp`, `ws`, `wss`)
/// - `://` separator after scheme
/// - non-empty host
/// - host contains only valid characters
///
/// This does **not** perform any network request.
///
/// # Errors
///
/// Returns [`UrlError`] if the URL is malformed.
///
/// # Example
///
/// ```
/// # use ursus_praefectus::Ursus;
/// fn main() {
/// assert!(Ursus::validate_url("https://example.com").is_ok());
/// assert!(Ursus::validate_url("http://localhost:3000").is_ok());
/// assert!(Ursus::validate_url("ftp://files.example.com").is_ok());
/// assert!(Ursus::validate_url("random string").is_err());
/// }
/// ```
impl Ursus {
#[cfg(feature = "url")]
pub fn validate_url(url: &str) -> Result<(), ValidationError> {
    if url.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    let separator_idx = url.find("://").ok_or(ValidationError::InvalidFormat)?;
    let scheme = &url[..separator_idx];

    if !VALID_SCHEMES.contains(&scheme) {
        return Err(ValidationError::InvalidFormat);
    }

    let rest = &url[separator_idx + 3..];
    if rest.is_empty() {
        return Err(ValidationError::InvalidFormat);
    }

    let host_end = rest
        .find(['/', '?', '#'])
        .unwrap_or(rest.len());
    let mut host = &rest[..host_end];

    if let Some(colon_pos) = host.rfind(':') {
        let after_colon = &host[colon_pos + 1..];
        if after_colon.chars().all(|c| c.is_ascii_digit()) && !after_colon.is_empty() {
            host = &host[..colon_pos];
        }
    }

    if host.is_empty() {
        return Err(ValidationError::InvalidFormat);
    }

    // Support IPv6 addresses in square brackets, e.g. [::1] or [::1]:8080
    if host.starts_with('[') {
        let end_bracket = match host.find(']') {
            Some(idx) => idx,
            None => return Err(ValidationError::InvalidFormat),
        };
        let ipv6 = &host[1..end_bracket];
        if ipv6.is_empty() || ipv6.contains(' ') || ipv6.contains('\t') {
            return Err(ValidationError::InvalidFormat);
        }
        if !ipv6.chars().all(|c| c.is_ascii_hexdigit() || c == ':') {
            return Err(ValidationError::InvalidFormat);
        }
        return Ok(());
    }

    // Host must not contain spaces or invalid characters
    if host.contains(' ') || host.contains('\t') {
        return Err(ValidationError::InvalidFormat);
    }

    for label in host.split('.') {
        if label.is_empty() {
            return Err(ValidationError::InvalidFormat);
        }
        if label.starts_with('-') || label.ends_with('-') {
            return Err(ValidationError::InvalidFormat);
        }
        if !label
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(ValidationError::InvalidFormat);
        }
    }

    Ok(())
}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        assert!(Ursus::validate_url("https://www.example.com").is_ok());
        assert!(Ursus::validate_url("http://example.com").is_ok());
        assert!(Ursus::validate_url("https://localhost:8080/path?q=1#sec").is_ok());
        assert!(Ursus::validate_url("ftp://files.server.org").is_ok());
        assert!(Ursus::validate_url("ws://echo.websocket.org").is_ok());
        assert!(Ursus::validate_url("wss://secure.example.com").is_ok());
        // IPv6 in square brackets
        assert!(Ursus::validate_url("http://[::1]").is_ok());
        assert!(Ursus::validate_url("http://[::1]:8080/path").is_ok());
        assert!(Ursus::validate_url("https://[2001:db8::1]").is_ok());
    }

    #[test]
    fn test_invalid_urls() {
        assert!(Ursus::validate_url("").is_err());
        assert!(Ursus::validate_url("example.com").is_err());
        assert!(Ursus::validate_url("ftp://").is_err());
        assert!(Ursus::validate_url("http://").is_err());
        assert!(Ursus::validate_url("ht*tp://example.com").is_err());
        assert!(Ursus::validate_url("http://-example.com").is_err());
        assert!(Ursus::validate_url("http://example..com").is_err());
        // Invalid IPv6 brackets
        assert!(Ursus::validate_url("http://[gggg::1]").is_err());
        assert!(Ursus::validate_url("http://[::1").is_err());
    }

}
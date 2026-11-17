//! # URL validation module
//!
//! Provides synchronous validation for URLs (HTTP/HTTPS/FTP/etc.).
//! Checks scheme, host, and basic structure without performing network requests.

use crate::Ursus;


/// Errors that can occur during URL validation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UrlError {
    /// The input string is empty.
    EmptyInput,
    /// The URL has no scheme (e.g. `https://`).
    MissingScheme,
    /// The scheme is not supported (only http, https, ftp, ws, wss allowed).
    UnsupportedScheme,
    /// The host part is missing or empty.
    MissingHost,
    /// The host contains invalid characters.
    InvalidHost,
    /// The URL is malformed in some other way.
    InvalidFormat(String),
}

impl std::fmt::Display for UrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlError::EmptyInput => write!(f, "URL is empty"),
            UrlError::MissingScheme => write!(f, "URL is missing a scheme"),
            UrlError::UnsupportedScheme => write!(f, "unsupported URL scheme"),
            UrlError::MissingHost => write!(f, "URL is missing a host"),
            UrlError::InvalidHost => write!(f, "URL has an invalid host"),
            UrlError::InvalidFormat(msg) => write!(f, "invalid URL format: {}", msg),
        }
    }
}

impl std::error::Error for UrlError {}

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
/// # use ursus_praefectus::url::validate_url;
/// # #[tokio::main(flavor="current_thread")]
/// # async fn main() {
/// assert!(validate_url("https://example.com").await.is_ok());
/// assert!(validate_url("http://localhost:3000").await.is_ok());
/// assert!(validate_url("ftp://files.example.com").await.is_ok());
/// assert!(validate_url("random string").await.is_err());
/// # }
/// ```
/// 
impl  Ursus {
#[cfg(feature = "url")]
pub async fn validate_url(url: &str) -> Result<(), UrlError> {
    if url.is_empty() {
        return Err(UrlError::EmptyInput);
    }

    let separator_idx = url.find("://").ok_or(UrlError::MissingScheme)?;
    let scheme = &url[..separator_idx];

    if !VALID_SCHEMES.contains(&scheme) {
        return Err(UrlError::UnsupportedScheme);
    }

    let rest = &url[separator_idx + 3..];
    if rest.is_empty() {
        return Err(UrlError::MissingHost);
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
        return Err(UrlError::MissingHost);
    }

    // Host must not contain spaces or invalid characters
    if host.contains(' ') || host.contains('\t') {
        return Err(UrlError::InvalidHost);
    }

    for label in host.split('.') {
        if label.is_empty() {
            return Err(UrlError::InvalidHost);
        }
        if label.starts_with('-') || label.ends_with('-') {
            return Err(UrlError::InvalidHost);
        }
        if !label
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(UrlError::InvalidHost);
        }
    }

    Ok(())
}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_urls() {
        assert!(Ursus::validate_url("https://www.example.com").await.is_ok());
        assert!(Ursus::validate_url("http://example.com").await.is_ok());
        assert!(Ursus::validate_url("https://localhost:8080/path?q=1#sec").await.is_ok());
        assert!(Ursus::validate_url("ftp://files.server.org").await.is_ok());
        assert!(Ursus::validate_url("ws://echo.websocket.org").await.is_ok());
        assert!(Ursus::validate_url("wss://secure.example.com").await.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_urls() {
        assert!(Ursus::validate_url("").await.is_err());
        assert!(Ursus::validate_url("example.com").await.is_err());
        assert!(Ursus::validate_url("ftp://").await.is_err());
        assert!(Ursus::validate_url("http://").await.is_err());
        assert!(Ursus::validate_url("ht*tp://example.com").await.is_err());
        assert!(Ursus::validate_url("http://-example.com").await.is_err());
        assert!(Ursus::validate_url("http://example..com").await.is_err());
    }

    #[test]
    fn test_url_error_display() {
        assert_eq!(UrlError::EmptyInput.to_string(), "URL is empty");
        assert_eq!(
            UrlError::MissingHost.to_string(),
            "URL is missing a host"
        );
    }
}
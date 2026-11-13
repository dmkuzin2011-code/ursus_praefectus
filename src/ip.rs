//! # IP address validation module

/// Errors that can occur during IP address validation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IpError {
    /// The input string is empty.
    EmptyInput,
    /// The string is not a valid IPv4 address.
    InvalidIpv4,
    /// The string is not a valid IPv6 address.
    InvalidIpv6,
    /// The string is neither a valid IPv4 nor IPv6 address.
    InvalidIp,
}

impl std::fmt::Display for IpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpError::EmptyInput => write!(f, "IP address is empty"),
            IpError::InvalidIpv4 => write!(f, "invalid IPv4 address"),
            IpError::InvalidIpv6 => write!(f, "invalid IPv6 address"),
            IpError::InvalidIp => write!(f, "invalid IP address (not IPv4 or IPv6)"),
        }
    }
}

impl std::error::Error for IpError {}

/// Validate an IPv4 address (dotted-decimal notation, e.g. `192.168.1.1`).

#[cfg(feature = "ip")]
pub async fn validate_ipv4(ip: &str) -> Result<(), IpError> {
    if ip.is_empty() {
        return Err(IpError::EmptyInput);
    }

    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return Err(IpError::InvalidIpv4);
    }

    for part in &parts {
        if part.is_empty() || part.len() > 3 {
            return Err(IpError::InvalidIpv4);
        }
        if part.len() > 1 && part.starts_with('0') {
            return Err(IpError::InvalidIpv4);
        }
        let value: u32 = match part.parse() {
            Ok(v) => v,
            Err(_) => return Err(IpError::InvalidIpv4),
        };
        if value > 255 {
            return Err(IpError::InvalidIpv4);
        }
    }

    Ok(())
}

/// Validate an IPv6 address (colon-hex notation, e.g. `2001:db8::1`).

#[cfg(feature = "ip")]
pub async fn validate_ipv6(ip: &str) -> Result<(), IpError> {
    if ip.is_empty() {
        return Err(IpError::EmptyInput);
    }

    let colon_count = ip.chars().filter(|&c| c == ':').count();
    if colon_count > 7 {
        return Err(IpError::InvalidIpv6);
    }

    let has_compression = ip.contains("::");

    let groups: Vec<&str> = if has_compression {
        let parts: Vec<&str> = ip.split("::").collect();
        if parts.len() != 2 {
            return Err(IpError::InvalidIpv6);
        }
        let left: Vec<&str> = if parts[0].is_empty() {
            Vec::new()
        } else {
            parts[0].split(':').collect()
        };
        let right: Vec<&str> = if parts[1].is_empty() {
            Vec::new()
        } else {
            parts[1].split(':').collect()
        };
        let total = left.len() + right.len();
        if total > 7 {
            return Err(IpError::InvalidIpv6);
        }
        let mut all = left;
        all.extend(std::iter::repeat("0").take(8 - total));
        all.extend(right);
        all
    } else {
        ip.split(':').collect()
    };

    if groups.len() != 8 {
        return Err(IpError::InvalidIpv6);
    }

    for group in &groups {
        if group.is_empty() {
            return Err(IpError::InvalidIpv6);
        }
        if group.len() > 4 {
            return Err(IpError::InvalidIpv6);
        }
        if !group.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(IpError::InvalidIpv6);
        }
    }

    Ok(())
}

/// Validate an IP address — tries IPv4 first, then IPv6.

#[cfg(feature = "ip")]
pub async fn validate_ip(ip: &str) -> Result<(), IpError> {
    if ip.is_empty() {
        return Err(IpError::EmptyInput);
    }

    if ip.contains(':') {
        validate_ipv6(ip).await.map_err(|_| IpError::InvalidIp)
    } else {
        validate_ipv4(ip).await.map_err(|_| IpError::InvalidIp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_ipv4() {
        assert!(validate_ipv4("192.168.1.1").await.is_ok());
        assert!(validate_ipv4("0.0.0.0").await.is_ok());
        assert!(validate_ipv4("255.255.255.255").await.is_ok());
        assert!(validate_ipv4("1.2.3.4").await.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_ipv4() {
        assert!(validate_ipv4("").await.is_err());
        assert!(validate_ipv4("256.1.1.1").await.is_err());
        assert!(validate_ipv4("1.2.3").await.is_err());
        assert!(validate_ipv4("1.2.3.4.5").await.is_err());
        assert!(validate_ipv4("abc.def.ghi.jkl").await.is_err());
        assert!(validate_ipv4("01.2.3.4").await.is_err());
    }

    #[tokio::test]
    async fn test_valid_ipv6() {
        assert!(validate_ipv6("2001:db8::1").await.is_ok());
        assert!(validate_ipv6("::1").await.is_ok());
        assert!(validate_ipv6("::").await.is_ok());
        assert!(validate_ipv6("fe80::1").await.is_ok());
        assert!(validate_ipv6("2001:0db8:0000:0000:0000:0000:0000:0001").await.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_ipv6() {
        assert!(validate_ipv6("").await.is_err());
        assert!(validate_ipv6("not-ipv6").await.is_err());
        assert!(validate_ipv6("2001:db8:::1").await.is_err());
        assert!(validate_ipv6("gggg::1").await.is_err());
    }

    #[tokio::test]
    async fn test_validate_ip() {
        assert!(validate_ip("10.0.0.1").await.is_ok());
        assert!(validate_ip("::1").await.is_ok());
        assert!(validate_ip("invalid").await.is_err());
        assert!(validate_ip("").await.is_err());
    }

    #[test]
    fn test_ip_error_display() {
        assert_eq!(IpError::EmptyInput.to_string(), "IP address is empty");
        assert_eq!(IpError::InvalidIpv4.to_string(), "invalid IPv4 address");
        assert_eq!(IpError::InvalidIpv6.to_string(), "invalid IPv6 address");
        assert_eq!(
            IpError::InvalidIp.to_string(),
            "invalid IP address (not IPv4 or IPv6)"
        );
    }
}

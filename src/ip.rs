//! # IP address validation module

use crate::Ursus;
use crate::specific_errors::ValidationError;

/// Validate an IPv4 address (dotted-decimal notation, e.g. `192.168.1.1`).

impl Ursus {
#[cfg(feature = "ip")]
pub fn validate_ipv4(ip: &str) -> Result<(), ValidationError> {
    if ip.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return Err(ValidationError::InvalidFormat);
    }

    for part in &parts {
        if part.is_empty() || part.len() > 3 {
            return Err(ValidationError::InvalidFormat);
        }
        if part.len() > 1 && part.starts_with('0') {
            return Err(ValidationError::InvalidFormat);
        }
        let value: u32 = match part.parse() {
            Ok(v) => v,
            Err(_) => return Err(ValidationError::InvalidFormat),
        };
        if value > 255 {
            return Err(ValidationError::InvalidFormat);
        }
    }

    Ok(())
}

/// Validate an IPv6 address (colon-hex notation, e.g. `2001:db8::1`).

#[cfg(feature = "ip")]
pub fn validate_ipv6(ip: &str) -> Result<(), ValidationError> {
    if ip.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    let colon_count = ip.chars().filter(|&c| c == ':').count();
    if colon_count > 7 {
        return Err(ValidationError::InvalidFormat);
    }

    let has_compression = ip.contains("::");

    let groups: Vec<&str> = if has_compression {
        let parts: Vec<&str> = ip.split("::").collect();
        if parts.len() != 2 {
            return Err(ValidationError::InvalidFormat);
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
            return Err(ValidationError::InvalidFormat);
        }
        let mut all = left;
        all.extend(std::iter::repeat_n("0", 8 - total));
        all.extend(right);
        all
    } else {
        ip.split(':').collect()
    };

    if groups.len() != 8 {
        return Err(ValidationError::InvalidFormat);
    }

    for group in &groups {
        if group.is_empty() {
            return Err(ValidationError::InvalidFormat);
        }
        if group.len() > 4 {
            return Err(ValidationError::InvalidFormat);
        }
        if !group.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ValidationError::InvalidFormat);
        }
    }

    Ok(())
}

/// Validate an IP address — tries IPv4 first, then IPv6.

#[cfg(feature = "ip")]
pub fn validate_ip(ip: &str) -> Result<(), ValidationError> {
    if ip.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    if ip.contains(':') {
        Ursus::validate_ipv6(ip).map_err(|_| ValidationError::InvalidFormat)
    } else {
        Ursus::validate_ipv4(ip).map_err(|_| ValidationError::InvalidFormat)
    }
}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ipv4() {
        assert!(Ursus::validate_ipv4("192.168.1.1").is_ok());
        assert!(Ursus::validate_ipv4("0.0.0.0").is_ok());
        assert!(Ursus::validate_ipv4("255.255.255.255").is_ok());
        assert!(Ursus::validate_ipv4("1.2.3.4").is_ok());
    }

    #[test]
    fn test_invalid_ipv4() {
        assert!(Ursus::validate_ipv4("").is_err());
        assert!(Ursus::validate_ipv4("256.1.1.1").is_err());
        assert!(Ursus::validate_ipv4("1.2.3").is_err());
        assert!(Ursus::validate_ipv4("1.2.3.4.5").is_err());
        assert!(Ursus::validate_ipv4("abc.def.ghi.jkl").is_err());
        assert!(Ursus::validate_ipv4("01.2.3.4").is_err());
    }

    #[test]
    fn test_valid_ipv6() {
        assert!(Ursus::validate_ipv6("2001:db8::1").is_ok());
        assert!(Ursus::validate_ipv6("::1").is_ok());
        assert!(Ursus::validate_ipv6("::").is_ok());
        assert!(Ursus::validate_ipv6("fe80::1").is_ok());
        assert!(Ursus::validate_ipv6("2001:0db8:0000:0000:0000:0000:0000:0001").is_ok());
    }

    #[test]
    fn test_invalid_ipv6() {
        assert!(Ursus::validate_ipv6("").is_err());
        assert!(Ursus::validate_ipv6("not-ipv6").is_err());
        assert!(Ursus::validate_ipv6("2001:db8:::1").is_err());
        assert!(Ursus::validate_ipv6("gggg::1").is_err());
    }

    #[test]
    fn test_validate_ip() {
        assert!(Ursus::validate_ip("10.0.0.1").is_ok());
        assert!(Ursus::validate_ip("::1").is_ok());
        assert!(Ursus::validate_ip("invalid").is_err());
        assert!(Ursus::validate_ip("").is_err());
    }

}

# Ursus — Documentation

**Ursus** is a lightweight Rust library for data validation and sanitization.

It provides validation for:
- Email addresses (with optional async DNS MX lookup via tokio)
- URLs
- IPv4/IPv6 addresses
- Phone numbers
- Passwords
- Occurrence counting in collections
- Collection sanitization

---

## Installation

Add Ursus to your Cargo.toml:

`	oml
[dependencies]
ursus_praefectus = "0.1.1"
`

Or from crates.io (when published).

---

## Feature Flags

Ursus uses feature flags to let you opt into only the modules you need:

| Feature | Description |
|---------|-------------|
| email | Email format validation (no network) |
| email-dns | Email format + async DNS MX lookup (requires 	okio + hickory-resolver) |
| url | URL format validation |
| ip | IPv4/IPv6 address validation |
| phone | Phone number validation |
| clear | Collection sanitization — remove unwanted values from a Vec |
| default | Enables email, url, ip, phone, clear |

Example — enable only email and clear:

`	oml
[dependencies]
ursus_praefectus = { version = "0.1.1", default-features = false, features = ["email", "clear"] }
`

---

## API Reference

### Email module (ursus::email)

| Function | Description |
|----------|-------------|
| Ursus::validate_email(email) | Format-only validation |
| Ursus::validate_email_format(email) | Syntax-only check (no network) |
| Ursus::validate_email_dns(email) | Format + async DNS MX lookup (requires email-dns) |

**Errors:** EmailError — EmptyInput, MissingAtSign, EmptyLocalPart, EmptyDomain, InvalidLocalPart, InvalidDomain, NoMxRecords, DnsLookupFailed

---

### URL module (ursus::url)

| Function | Description |
|----------|-------------|
| Ursus::validate_url(url) | Validates scheme, host, and basic URL structure |

**Supported schemes:** http, https, tp, ws, wss

**Errors:** UrlError — EmptyInput, MissingScheme, UnsupportedScheme, MissingHost, InvalidHost, InvalidFormat

---

### IP module (ursus::ip)

| Function | Description |
|----------|-------------|
| Ursus::validate_ipv4(ip) | Validates IPv4 dotted-decimal address |
| Ursus::validate_ipv6(ip) | Validates IPv6 colon-hex address (supports :: compression) |
| Ursus::validate_ip(ip) | Tries IPv4, then IPv6 |

**Errors:** IpError — EmptyInput, InvalidIpv4, InvalidIpv6, InvalidIp

---

### Phone module (ursus::phone)

| Function | Description |
|----------|-------------|
| Ursus::validate_phone(phone) | Validates international phone number (7–15 digits, optional + prefix) |

**Errors:** PhoneError — EmptyInput, InvalidCharacters, TooShort, TooLong

---

### Password module (ursus::password)

| Function | Description |
|----------|-------------|
| Ursus::validate_min_len(password, size) | Checks minimum length |
| Ursus::validate_max_len(password, size) | Checks maximum length |
| Ursus::compare_case_sensitive(password, correct) | Case-sensitive comparison |
| Ursus::compare_case_insensitive(password, correct) | Case-insensitive comparison |

**Errors:** PasswordError — TooShort, TooLong, WrongPassword

---

### Counts module (ursus::counts)

| Function | Description |
|----------|-------------|
| Ursus::count_occurrences(collection, target) | Counts occurrences of 	arget in collection |

**Errors:** Count — EmptyError

---

### Clear module (ursus::clear)

| Function | Description |
|----------|-------------|
| Ursus::clear_collection(collection, delete_values) | Removes specified values from a mutable Vec in-place |

**Errors:** ClearedError — EmptyInput

#### Example

`ust
use ursus_praefectus::Ursus;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut collection = vec!["apple", "banana", "cherry", "banana"];
    let delete_values = vec!["banana"];

    Ursus::clear_collection(&mut collection, &delete_values).await.unwrap();
    assert_eq!(collection, vec!["apple", "cherry"]);
}
`

---

## Running Tests

`ash
cargo test
`

Run with all features:

`ash
cargo test --all-features
`

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| 	okio | Async runtime for DNS resolution and async functions |
| hickory-resolver | Async DNS resolver (MX record lookup, optional via email-dns) |

---

## License

Distributed under the Apache-2.0 License. See LICENSE for details.

**Ursus** — Simple, modular data validation for Rust.


<div align="center">
    <img src="assets/ursus.png" align="center">
</div>

# Ursus — Data Validation Library

**Ursus** is a lightweight Rust library for data validation. It provides validation for email addresses (with optional async DNS MX lookup via tokio), URLs, IPv4/IPv6 addresses, phone numbers, passwords, and occurrence counting in collections.

---

## 📦 Installation

Add Ursus to your `Cargo.toml`:

```toml
[dependencies]
ursus = { path = "../Ursus" }
```

Or from crates.io (when published):

```toml
[dependencies]
ursus = "0.2.0"
```

---

## 🎛 Feature Flags

Ursus uses feature flags to let you opt into only the modules you need:

| Feature | Description |
|---------|-------------|
| `email` | Email format validation (no network) |
| `email-dns` | Email format + async DNS MX lookup (requires `tokio` + `hickory-resolver`) |
| `url` | URL format validation |
| `ip` | IPv4/IPv6 address validation |
| `phone` | Phone number validation |
| `default` | Enables `email`, `url`, `ip`, `phone` |

Example: enable only email with DNS:

```toml
[dependencies]
ursus = { version = "0.2.0", default-features = false, features = ["email-dns"] }
```

---

## 🚀 Quick Start

```rust
use ursus::email::validate_email;
use ursus::url::validate_url;
use ursus::ip::validate_ip;
use ursus::phone::validate_phone;
use ursus::password::{validate_min_len, compare_case_sensitive, PasswordError};
use ursus::counts::{count_occurrences, Count};

#[tokio::main]
async fn main() {
    // Email format validation
    assert!(validate_email("user@example.com").await.is_ok());
    assert!(validate_email("not-an-email").await.is_err());

    // URL validation
    assert!(validate_url("https://www.example.com").await.is_ok());
    assert!(validate_url("not-a-url").await.is_err());

    // IP validation
    assert!(validate_ip("192.168.1.1").await.is_ok());
    assert!(validate_ip("2001:db8::1").await.is_ok());
    assert!(validate_ip("invalid").await.is_err());

    // Phone validation
    assert!(validate_phone("+1 234 567 8900").await.is_ok());
    assert!(validate_phone("12345").await.is_err());

    // Password validation
    assert!(validate_min_len("mypassword", 8).await.is_ok());
    assert!(compare_case_sensitive("secret", "secret").await.is_ok());

    // Count occurrences
    let collection = vec!["a", "b", "a", "c", "a"];
    assert_eq!(count_occurrences(collection, "a").await, Ok(3));
}
```

---

## 🔐 Async Email Validation with DNS (tokio)

Email validation with DNS MX record lookup requires the `email-dns` feature and uses **tokio** for async DNS resolution:

```rust
use ursus::email::validate_email_dns;

#[tokio::main]
async fn main() {
    match validate_email_dns("user@gmail.com").await {
        Ok(()) => println!("Email is valid (format + DNS MX)"),
        Err(e) => println!("Validation failed: {}", e),
    }
}
```

This performs:
1. **Format check** — validates the email syntax locally
2. **DNS MX lookup** — asynchronously queries DNS for MX records of the domain using `hickory-resolver` on tokio

---

## 📚 API Reference

### Email module (`ursus::email`)

| Function | Description |
|----------|-------------|
| `validate_email(email)` | Format-only validation |
| `validate_email_format(email)` | Syntax-only check (no network) |
| `validate_email_dns(email)` | Format + async DNS MX lookup (requires `email-dns`) |

**Errors:** `EmailError` — `EmptyInput`, `MissingAtSign`, `EmptyLocalPart`, `EmptyDomain`, `InvalidLocalPart`, `InvalidDomain`, `NoMxRecords`, `DnsLookupFailed`

### URL module (`ursus::url`)

| Function | Description |
|----------|-------------|
| `validate_url(url)` | Validates scheme, host, and basic URL structure |

**Supported schemes:** `http`, `https`, `ftp`, `ws`, `wss`

**Errors:** `UrlError` — `EmptyInput`, `MissingScheme`, `UnsupportedScheme`, `MissingHost`, `InvalidHost`, `InvalidFormat`

### IP module (`ursus::ip`)

| Function | Description |
|----------|-------------|
| `validate_ipv4(ip)` | Validates IPv4 dotted-decimal address |
| `validate_ipv6(ip)` | Validates IPv6 colon-hex address (supports `::` compression) |
| `validate_ip(ip)` | Tries IPv4, then IPv6 |

**Errors:** `IpError` — `EmptyInput`, `InvalidIpv4`, `InvalidIpv6`, `InvalidIp`

### Phone module (`ursus::phone`)

| Function | Description |
|----------|-------------|
| `validate_phone(phone)` | Validates international phone number (7–15 digits, optional `+` prefix) |

**Errors:** `PhoneError` — `EmptyInput`, `InvalidCharacters`, `TooShort`, `TooLong`

### Password module (`ursus::password`)

| Function | Description |
|----------|-------------|
| `validate_min_len(password, size)` | Checks minimum length |
| `validate_max_len(password, size)` | Checks maximum length |
| `compare_case_sensitive(password, correct)` | Case-sensitive comparison |
| `compare_case_insensitive(password, correct)` | Case-insensitive comparison |

**Errors:** `PasswordError` — `TooShort`, `TooLong`, `WrongPassword`

### Counts module (`ursus::counts`)

| Function | Description |
|----------|-------------|
| `count_occurrences(collection, target)` | Counts occurrences of `target` in collection |

**Errors:** `Count` — `EmptyError`

---

## 🛠 Running Tests

```bash
cargo test
```

Run with all features:

```bash
cargo test --all-features
```

---

## 🛡 Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime for DNS resolution |
| `hickory-resolver` | Async DNS resolver (MX record lookup) |

---

## 📄 License

Distributed under the Apache-2.0 License. See `LICENSE` for details.

**Ursus** — Simple, modular data validation for Rust.


<div align="center">
    <img src="assets/Ursus.png" align="center">
</div>

# Ursus — Data Validation Library

**Ursus** is a lightweight Rust library for data validation and sanitization. It provides validation for email addresses (with optional async DNS MX lookup via tokio), URLs, IPv4/IPv6 addresses, phone numbers, passwords, occurrence counting in collections, and collection sanitization.

---

## 📦 Installation

Add Ursus to your `Cargo.toml`:

Or from crates.io (when published):


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
| `clear` | Collection sanitization — remove unwanted values from a `Vec` |
| `default` | Enables `email`, `url`, `ip`, `phone`, `clear` |

---


## 🔐 Async Email Validation with DNS (tokio)

Email validation with DNS MX record lookup requires the `email-dns` feature and uses **tokio** for async DNS resolution:

It performs:
1. **Format check** — validates the email syntax locally
2. **DNS MX lookup** — asynchronously queries DNS for MX records of the domain using `hickory-resolver` on tokio

---

## 📚 API Reference

### Email module (`ursus::email`)

| Function | Description |
|----------|-------------|
| `Ursus::validate_email(email)` | Format-only validation |
| `Ursus::validate_email_format(email)` | Syntax-only check (no network) |
| `Ursus::validate_email_dns(email)` | Format + async DNS MX lookup (requires `email-dns`) |

**Errors:** `EmailError` — `EmptyInput`, `MissingAtSign`, `EmptyLocalPart`, `EmptyDomain`, `InvalidLocalPart`, `InvalidDomain`, `NoMxRecords`, `DnsLookupFailed`

### URL module (`ursus::url`)

| Function | Description |
|----------|-------------|
| `Ursus::validate_url(url)` | Validates scheme, host, and basic URL structure |

**Supported schemes:** `http`, `https`, `ftp`, `ws`, `wss`

**Errors:** `UrlError` — `EmptyInput`, `MissingScheme`, `UnsupportedScheme`, `MissingHost`, `InvalidHost`, `InvalidFormat`

### IP module (`ursus::ip`)

| Function | Description |
|----------|-------------|
| `Ursus::validate_ipv4(ip)` | Validates IPv4 dotted-decimal address |
| `Ursus::validate_ipv6(ip)` | Validates IPv6 colon-hex address (supports `::` compression) |
| `Ursus::validate_ip(ip)` | Tries IPv4, then IPv6 |

**Errors:** `IpError` — `EmptyInput`, `InvalidIpv4`, `InvalidIpv6`, `InvalidIp`

### Phone module (`ursus::phone`)

| Function | Description |
|----------|-------------|
| `Ursus::validate_phone(phone)` | Validates international phone number (7–15 digits, optional `+` prefix) |

**Errors:** `PhoneError` — `EmptyInput`, `InvalidCharacters`, `TooShort`, `TooLong`

### Password module (`ursus::password`)

| Function | Description |
|----------|-------------|
| `Ursus::validate_min_len(password, size)` | Checks minimum length |
| `Ursus::validate_max_len(password, size)` | Checks maximum length |
| `Ursus::compare_case_sensitive(password, correct)` | Case-sensitive comparison |
| `Ursus::compare_case_insensitive(password, correct)` | Case-insensitive comparison |

**Errors:** `PasswordError` — `TooShort`, `TooLong`, `WrongPassword`

### Counts module (`ursus::counts`)

| Function | Description |
|----------|-------------|
| `Ursus::count_occurrences(collection, target)` | Counts occurrences of `target` in collection |

**Errors:** `Count` — `EmptyError`

### Clear module (`ursus::clear`)

| Function | Description |
|----------|-------------|
| `Ursus::clear_collection(collection, delete_values)` | Removes specified values from a mutable `Vec` in-place |

**Errors:** `ClearedError` — `EmptyInput`

---

Full documentation is available in [`DOCUMENTATION.md`](DOCUMENTATION.md).

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

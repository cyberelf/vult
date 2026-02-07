# Security Testing Guide

This document describes security testing practices for Vult.

## Overview

Vult's security depends on:
1. **Cryptography**: Proper use of AES-256-GCM and Argon2id
2. **Key Management**: Secure PIN handling and per-key encryption
3. **Memory Safety**: Zeroizing sensitive data
4. **Dependency Security**: Auditing third-party crates

## Testing Layers

### 1. Unit Tests

Located in: `src/**/*.rs` (inline `#[cfg(test)]` modules)

**Coverage:**
- Encryption/decryption roundtrips
- Key derivation determinism
- PIN validation
- Authentication flows
- Key CRUD operations

**Run:**
```bash
cargo test --lib
```

### 2. Property-Based Tests

Located in: `src/crypto.rs` (proptest suite)

**Tests:**
- `prop_encrypt_decrypt_roundtrip`: Ensures any data can be encrypted and decrypted
- `prop_per_key_encryption_roundtrip`: Verifies per-key encryption isolation
- `prop_same_plaintext_different_nonces`: Ensures unique ciphertexts
- `prop_wrong_key_fails_decryption`: Verifies authentication
- `prop_key_derivation_deterministic`: Ensures consistent key generation
- `prop_different_salts_different_keys`: Verifies salt effectiveness
- `prop_different_key_names_different_keys`: Verifies per-key isolation

**Run:**
```bash
cargo test crypto::proptests --release
```

**Note:** These tests run 256 iterations by default. For thorough testing:
```bash
PROPTEST_CASES=10000 cargo test crypto::proptests --release
```

### 3. Integration Tests

Located in: `tests/`

**Test Files:**
- `integration_test.rs`: Library integration tests
- `auto_lock_integration_test.rs`: GUI auto-lock tests
- `cli_integration_test.rs`: CLI end-to-end tests

**Run:**
```bash
# All integration tests
cargo test --test '*'

# Specific test file
cargo test --test integration_test
```

### 4. Static Analysis

#### Clippy (Linting)

```bash
# Run with deny warnings
cargo clippy --lib --features gui -- -D warnings

# Check specific lints
cargo clippy --lib -- -W clippy::pedantic -W clippy::nursery
```

Configuration in `clippy.toml`.

#### cargo-audit (Dependency Vulnerabilities)

```bash
# Install
cargo install cargo-audit

# Run
cargo audit

# Check for specific advisory
cargo audit --deny RUSTSEC-2021-0001
```

Updates advisory database from https://github.com/rustsec/advisory-db

#### cargo-deny (License and Security)

```bash
# Install
cargo install cargo-deny

# Run all checks
cargo deny check

# Specific checks
cargo deny check advisories  # Security vulnerabilities
cargo deny check licenses    # License compliance
cargo deny check bans        # Banned crates
cargo deny check sources     # Registry verification
```

Configuration in `deny.toml`.

#### cargo-geiger (Unsafe Code)

```bash
# Install
cargo install cargo-geiger

# Scan for unsafe code
cargo geiger --all-features

# Detailed report
cargo geiger --all-features --output-format GitHubMarkdown > unsafe-report.md
```

**Current Status:** Vult uses minimal `unsafe` code, primarily in dependencies.

### 5. Memory Safety Testing

#### MIRI (Undefined Behavior Detection)

```bash
# Install
rustup +nightly component add miri

# Run on specific test
cargo +nightly miri test --lib crypto::tests::test_zeroization

# Run on all tests (slow)
cargo +nightly miri test --lib
```

**Limitations:**
- Cannot run code that interacts with the OS (networking, filesystem in some modes)
- Much slower than normal test execution

### 6. Fuzz Testing (Optional)

**Setup:**
```bash
cargo install cargo-fuzz
cargo fuzz init
```

**Targets to Fuzz:**
1. PIN input validation
2. Encryption/decryption
3. Key derivation
4. Database queries (SQL injection)
5. CLI argument parsing

**Example fuzz target (conceptual):**
```rust
// fuzz/fuzz_targets/pin_validation.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use vult::core::validate_pin;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = validate_pin(s);
    }
});
```

**Run:**
```bash
cargo fuzz run pin_validation -- -max_total_time=60
```

## Security Checklist

Before each release:

- [ ] Run full test suite: `cargo test --all-features`
- [ ] Run property tests with high iteration count: `PROPTEST_CASES=10000 cargo test crypto::proptests`
- [ ] Run cargo-audit: `cargo audit`
- [ ] Run cargo-deny: `cargo deny check`
- [ ] Run clippy with strict lints: `cargo clippy -- -D warnings`
- [ ] Review any new `unsafe` blocks
- [ ] Check for hardcoded secrets or test credentials
- [ ] Verify zeroization of sensitive data
- [ ] Test PIN requirements (minimum length, charset)
- [ ] Verify encryption uses unique nonces
- [ ] Test session timeout behavior
- [ ] Review dependency updates for security advisories

## Continuous Integration

Security checks automated in `.github/workflows/ci.yml`:

```yaml
- name: Run cargo-audit
  run: cargo audit

- name: Run clippy
  run: cargo clippy --lib --features gui -- -D warnings

- name: Run tests
  run: cargo test --lib
```

## Sensitive Data Handling

### Verifying Zeroization

Use MIRI or manual inspection:

```rust
#[test]
fn test_pin_is_zeroized() {
    use zeroize::Zeroize;
    
    let mut pin = String::from("123456");
    let ptr = pin.as_ptr();
    
    // Use PIN...
    
    pin.zeroize();
    
    // Verify memory is cleared (unsafe inspection)
    // In practice, rely on zeroize crate's correctness
}
```

**Current Status:** 
- PINs use `String` (not auto-zeroized)
- Vault keys use custom `VaultKey` type with `Zeroize` implementation
- Consider migrating PIN storage to use `zeroize::Zeroizing<String>`

### Memory Dump Analysis (Advanced)

For paranoid verification:

```bash
# Run process, get PID
cargo run --bin vult-gui &
PID=$!

# Dump memory (requires root)
sudo gcore $PID

# Search for sensitive data (should not find plaintext)
strings core.$PID | grep "my-secret-key"
```

## Known Limitations

1. **No Hardware Security Module (HSM)**: Uses software-based encryption
2. **Memory Dumps**: Decrypted keys exist in RAM while unlocked
3. **Swap**: Sensitive data might be swapped to disk by OS
4. **Compiler Optimizations**: May not fully eliminate sensitive data from memory

## Reporting Vulnerabilities

See `.well-known/security.txt` for reporting procedures.

## References

- [OWASP Cryptographic Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [The Rustonomicon (Unsafe Rust)](https://doc.rust-lang.org/nomicon/)
- [cargo-audit Documentation](https://github.com/rustsec/rustsec/tree/main/cargo-audit)

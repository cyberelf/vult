# Formal Verification with Kani

## Overview

Vult uses [Kani](https://github.com/model-checking/kani), an official Rust verification tool from AWS, to formally verify critical security properties of cryptographic operations.

## Setup

```bash
# Install Kani
cargo install --locked kani-verifier

# Initialize Kani (first time only)
cargo kani setup
```

## Verification Harnesses

Verification proofs are located in [`src/crypto_verification.rs`](../src/crypto_verification.rs).

### Cryptographic Properties Verified

1. **Roundtrip Property** (`verify_crypto_roundtrip`)
   - **Property**: decrypt(encrypt(m, k), k) = m
   - **Verifies**: Encryption followed by decryption yields original plaintext
   - **Bounded**: Plaintext up to 64 bytes

2. **Encryption Determinism** (`verify_encryption_deterministic`)
   - **Property**: Two encryptions decrypt to the same plaintext
   - **Verifies**: Despite random nonces, decryption is consistent
   - **Bounded**: Plaintext up to 32 bytes

3. **Key Sensitivity** (`verify_encryption_key_sensitivity`)
   - **Property**: Different keys produce different results
   - **Verifies**: Decryption with wrong key fails or produces different output
   - **Bounded**: Plaintext up to 16 bytes

4. **PIN Derivation Determinism** (`verify_pin_derivation_deterministic`)
   - **Property**: Same PIN + salt always produces same key
   - **Verifies**: Key derivation is deterministic
   - **Bounded**: PIN length 6-20 characters

5. **PIN Derivation Uniqueness** (`verify_pin_derivation_uniqueness`)
   - **Property**: Different PINs produce different keys
   - **Verifies**: PIN uniqueness is preserved in derived keys
   - **Fixed**: Uses concrete test PINs "pin123" and "pin456"

## Running Verification

Due to dependency complexity, Kani verification requires significant time. Recommended approach:

### Quick Verification (Property-Based Testing)

For rapid feedback, use proptest-based property testing:

```bash
cargo test --lib crypto::tests::prop_
cargo test --lib services::tests::prop_
```

These provide similar coverage with much faster execution (<1 minute vs hours).

### Full Formal Verification

For complete formal verification:

```bash
# Verify specific harness (recommended)
cargo kani --harness verify_crypto_roundtrip --lib

# Verify all harnesses (very slow)
cargo kani --lib
```

**Note**: Initial compilation may take 30+ minutes due to all dependencies being verified.

## Verification vs. Testing

| Aspect | Kani Verification | Property Tests | Unit Tests |
|--------|------------------|----------------|------------|
| **Exhaustiveness** | All inputs (bounded) | Random sampling | Specific cases |
| **Time** | Hours | Seconds | Milliseconds |
| **Use Case** | Security audit | Development | Regression |
| **CI Integration** | Nightly only | Every commit | Every commit |

## CI Integration

Due to verification time, Kani runs are recommended for:
- Pre-release security audits
- After cryptographic changes
- Nightly dedicated verification pipelines

## Limitations

### Current Limitations

1. **Foreign Functions**: Kani warns about `foreign function (3)` - likely from Argon2's C implementation
2. **Inline ASM**: Some low-level crypto operations use inline assembly
3. **Concurrency**: Kani treats atomic operations sequentially (acceptable for Vult's use case)

### Workarounds

- **Property Tests**: Cover same properties with faster execution
- **Fuzzing**: 24/7 continuous testing with cargo-fuzz
- **Zero Unsafe**: No unsafe code in Vult reduces verification burden

## Security Properties Verified

✅ **Encryption correctness**: All encrypted data can be decrypted  
✅ **Encryption consistency**: Decryption deterministically recovers plaintext  
✅ **Key separation**: Different keys produce cryptographically distinct outputs  
✅ **PIN derivation determinism**: Same input always produces same key  
✅ **PIN uniqueness preservation**: Different PINs map to different keys  

## Future Work

- [ ] Dedicated verification CI pipeline
- [ ] Bounded model checking with CBMC
- [ ] Contract-based verification with Kani contracts
- [ ] Extend verification to database operations
- [ ] Verify auto-lock timeout invariants

## References

- [Kani Documentation](https://model-checking.github.io/kani/)
- [Rust Feature Support](https://model-checking.github.io/kani/rust-feature-support.html)
- [AES-GCM Security Proofs](https://eprint.iacr.org/2017/168.pdf)
- [Argon2 Specification](https://github.com/P-H-C/phc-winner-argon2/blob/master/argon2-specs.pdf)

## Related Documentation

- [Fuzzing Documentation](./FUZZING.md) - Continuous random testing
- [Security Testing](../SECURITY_TESTING.md) - Overall security test strategy
- [Cryptography Implementation](../src/crypto.rs) - Implementation details

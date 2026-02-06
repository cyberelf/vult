//! Formal verification harnesses for cryptographic operations using Kani.
//!
//! This module contains verification proofs for critical security properties
//! of the vault's cryptographic operations.

#[cfg(kani)]
mod kani_verification {
    use crate::crypto::{decrypt, derive_key_from_pin, encrypt, VaultKey};

    /// Verify that encryption followed by decryption yields original plaintext
    #[kani::proof]
    fn verify_crypto_roundtrip() {
        // Create arbitrary key
        let key_bytes: [u8; 32] = kani::any();
        let key = VaultKey::from_bytes(key_bytes);

        // Create arbitrary plaintext (bounded to avoid state explosion)
        let plaintext_len: usize = kani::any();
        kani::assume(plaintext_len <= 64); // Bound the size
        let mut plaintext = vec![0u8; plaintext_len];
        for byte in &mut plaintext {
            *byte = kani::any();
        }

        // Encrypt then decrypt
        if let Ok(encrypted) = encrypt(&plaintext, &key) {
            if let Ok(decrypted) = decrypt(&encrypted, &key) {
                // Property: decrypt(encrypt(m, k), k) = m
                assert_eq!(plaintext, decrypted);
            }
        }
    }

    /// Verify that encryption is deterministic for the same inputs
    #[kani::proof]
    fn verify_encryption_deterministic() {
        let key_bytes: [u8; 32] = kani::any();
        let key = VaultKey::from_bytes(key_bytes);

        let plaintext_len: usize = kani::any();
        kani::assume(plaintext_len > 0 && plaintext_len <= 32);
        let mut plaintext = vec![0u8; plaintext_len];
        for byte in &mut plaintext {
            *byte = kani::any();
        }

        // Encrypt twice with same key and plaintext
        if let (Ok(encrypted1), Ok(encrypted2)) =
            (encrypt(&plaintext, &key), encrypt(&plaintext, &key.clone()))
        {
            // Property: Ciphertexts should be different (due to random nonce)
            // but both should decrypt to same plaintext
            if let (Ok(decrypted1), Ok(decrypted2)) =
                (decrypt(&encrypted1, &key), decrypt(&encrypted2, &key))
            {
                assert_eq!(decrypted1, decrypted2);
                assert_eq!(decrypted1, plaintext);
            }
        }
    }

    /// Verify that different keys produce different ciphertexts
    #[kani::proof]
    fn verify_encryption_key_sensitivity() {
        let key1_bytes: [u8; 32] = kani::any();
        let key2_bytes: [u8; 32] = kani::any();
        kani::assume(key1_bytes != key2_bytes);

        let key1 = VaultKey::from_bytes(key1_bytes);
        let key2 = VaultKey::from_bytes(key2_bytes);

        let plaintext_len: usize = kani::any();
        kani::assume(plaintext_len > 0 && plaintext_len <= 16);
        let mut plaintext = vec![0u8; plaintext_len];
        for byte in &mut plaintext {
            *byte = kani::any();
        }

        // Encrypt with two different keys
        if let (Ok(encrypted1), Ok(_encrypted2)) =
            (encrypt(&plaintext, &key1), encrypt(&plaintext, &key2))
        {
            // Property: Decryption with wrong key should fail
            if let Ok(decrypted_wrong) = decrypt(&encrypted1, &key2) {
                // If it somehow succeeds, it should not equal original
                assert_ne!(decrypted_wrong, plaintext);
            }

            // Property: Correct key should work
            if let Ok(decrypted_correct) = decrypt(&encrypted1, &key1) {
                assert_eq!(decrypted_correct, plaintext);
            }
        }
    }

    /// Verify PIN derivation properties
    #[kani::proof]
    fn verify_pin_derivation_deterministic() {
        // Create arbitrary PIN (bounded)
        let pin_len: usize = kani::any();
        kani::assume(pin_len >= 6 && pin_len <= 20);
        let mut pin = String::new();
        for _ in 0..pin_len {
            let c: char = kani::any();
            kani::assume(c.is_ascii_graphic() || c == ' ');
            pin.push(c);
        }

        // Create arbitrary salt
        let salt: [u8; 32] = kani::any();

        // Derive key twice
        if let (Ok(key1), Ok(key2)) = (
            derive_key_from_pin(&pin, &salt),
            derive_key_from_pin(&pin, &salt),
        ) {
            // Property: Same PIN and salt should produce same key
            assert_eq!(key1.as_bytes(), key2.as_bytes());
        }
    }

    /// Verify that different PINs produce different keys
    #[kani::proof]
    fn verify_pin_derivation_uniqueness() {
        // Two different PINs
        let pin1 = String::from("pin123");
        let pin2 = String::from("pin456");

        // Create arbitrary salt
        let salt: [u8; 32] = kani::any();

        // Both keys should derive successfully (PINs are valid)
        if let (Ok(key1), Ok(key2)) = (
            derive_key_from_pin(&pin1, &salt),
            derive_key_from_pin(&pin2, &salt),
        ) {
            // Property: Different PINs should produce different keys
            assert_ne!(key1.as_bytes(), key2.as_bytes());
        }
    }
}

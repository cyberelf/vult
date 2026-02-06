//! Cryptographic operations for secure storage.
//!
//! This module provides:
//! - Key derivation using Argon2id
//! - Authenticated encryption using AES-256-GCM
//! - Per-key encryption (unique key per API key)
//! - Secure random generation
//!
//! # Algorithms
//!
//! - **Key Derivation**: Argon2id with 64MB memory, 3 iterations, 4 lanes
//! - **Encryption**: AES-256-GCM with 12-byte nonce, 16-byte auth tag
//!
//! # Security Properties
//!
//! - All keys are zeroized when dropped
//! - Each encryption uses a random nonce
//! - Ciphertexts are authenticated (tamper detection)
//! - Memory-hard key derivation resists GPU attacks
//!
//! # Example
//!
//! ```rust,ignore
//! use vult::crypto::{derive_key_from_pin, encrypt, decrypt, generate_salt};
//!
//! // Derive a key from PIN
//! let salt = generate_salt();
//! let key = derive_key_from_pin("my-pin", &salt)?;
//!
//! // Encrypt data
//! let encrypted = encrypt(b"secret data", &key)?;
//!
//! // Decrypt data
//! let decrypted = decrypt(&encrypted, &key)?;
//! assert_eq!(decrypted, b"secret data");
//! ```

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Algorithm, Argon2, Params, Version,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Encryption key derived from PIN
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct VaultKey([u8; 32]);

impl VaultKey {
    /// Create a new vault key from raw bytes (should only be used after derivation)
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get the key bytes for use with AES-GCM
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Encrypted data with nonce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

/// Errors related to cryptographic operations
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Key derivation failed: {0}")]
    KeyDerivation(String),

    #[error("Encryption failed: {0}")]
    Encryption(String),

    #[error("Decryption failed: {0}")]
    Decryption(String),

    #[error("Invalid key length")]
    InvalidKeyLength,

    #[error("Invalid nonce length")]
    InvalidNonceLength,
}

/// Result type for crypto operations
pub type Result<T> = std::result::Result<T, CryptoError>;

/// Derives a 256-bit encryption key from a PIN using Argon2id.
///
/// # Parameters
/// - `pin`: The user's PIN (must be at least 6 characters)
/// - `salt`: The salt to use for key derivation
///
/// # Security Parameters
/// - Memory: 64 MiB
/// - Iterations: 3
/// - Parallelism: 4
/// - Output length: 256 bits
pub fn derive_key_from_pin(pin: &str, salt: &[u8; 32]) -> Result<VaultKey> {
    // Invariant: PIN must meet minimum length requirement
    if pin.len() < 6 {
        return Err(CryptoError::KeyDerivation(
            "PIN must be at least 6 characters".to_string(),
        ));
    }

    // Invariant: Salt must be exactly 32 bytes
    debug_assert_eq!(salt.len(), 32, "Salt must be exactly 32 bytes");

    // Argon2id parameters for high security
    let params = Params::new(65536, 3, 4, Some(32))
        .map_err(|e| CryptoError::KeyDerivation(e.to_string()))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, argon2::Version::V0x13, params);

    // Generate password hash using the salt directly
    let salt_string = SaltString::encode_b64(salt.as_slice())
        .map_err(|e| CryptoError::KeyDerivation(e.to_string()))?;

    let password_hash = argon2
        .hash_password(pin.as_bytes(), &salt_string)
        .map_err(|e| CryptoError::KeyDerivation(e.to_string()))?;

    // Extract the raw hash bytes directly from the PasswordHash
    // The hash output contains the raw bytes we need for the encryption key
    let hash_output = password_hash.hash.unwrap();
    let hash_bytes = hash_output.as_bytes();

    // Invariant: Argon2 output must be at least 32 bytes
    if hash_bytes.len() < 32 {
        return Err(CryptoError::InvalidKeyLength);
    }
    debug_assert!(
        hash_bytes.len() >= 32,
        "Argon2 output must be at least 32 bytes"
    );

    // Take first 32 bytes as our key
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&hash_bytes[..32]);

    // Invariant: Derived key must not be all zeros
    debug_assert!(
        !key_array.iter().all(|&b| b == 0),
        "Derived key must not be all zeros"
    );

    Ok(VaultKey(key_array))
}

/// Generates a random salt for key derivation.
pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Encrypts plaintext using AES-256-GCM.
///
/// # Parameters
/// - `plaintext`: The data to encrypt
/// - `key`: The vault encryption key
///
/// # Returns
/// Encrypted data with nonce for later decryption
pub fn encrypt(plaintext: &[u8], key: &VaultKey) -> Result<EncryptedData> {
    // Invariant: Key must be exactly 32 bytes
    debug_assert_eq!(key.as_bytes().len(), 32, "VaultKey must be 32 bytes");

    let cipher = Aes256Gcm::new(key.as_bytes().into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Invariant: Generated nonce must be exactly 12 bytes
    debug_assert_eq!(nonce.len(), 12, "AES-GCM nonce must be 12 bytes");

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;

    // Invariant: Ciphertext should be larger than plaintext (includes auth tag)
    debug_assert!(
        ciphertext.len() >= plaintext.len(),
        "Ciphertext should include authentication tag"
    );

    Ok(EncryptedData {
        ciphertext,
        nonce: nonce.to_vec(),
    })
}

/// Decrypts ciphertext using AES-256-GCM.
///
/// # Parameters
/// - `encrypted`: The encrypted data with nonce
/// - `key`: The vault encryption key
///
/// # Returns
/// The decrypted plaintext
pub fn decrypt(encrypted: &EncryptedData, key: &VaultKey) -> Result<Vec<u8>> {
    // Invariant: Key must be exactly 32 bytes
    debug_assert_eq!(key.as_bytes().len(), 32, "VaultKey must be 32 bytes");

    // Invariant: Nonce must be exactly 12 bytes for AES-GCM
    if encrypted.nonce.len() != 12 {
        return Err(CryptoError::InvalidNonceLength);
    }

    // Invariant: Ciphertext must not be empty
    debug_assert!(
        !encrypted.ciphertext.is_empty(),
        "Ciphertext cannot be empty"
    );

    let mut nonce_array = [0u8; 12];
    nonce_array.copy_from_slice(&encrypted.nonce);
    let nonce = Nonce::from(nonce_array);

    let cipher = Aes256Gcm::new(key.as_bytes().into());

    let plaintext = cipher
        .decrypt(&nonce, encrypted.ciphertext.as_ref())
        .map_err(|e| CryptoError::Decryption(e.to_string()))?;

    Ok(plaintext)
}

/// Securely generates a random vault key for biometric authentication.
pub fn generate_vault_key() -> VaultKey {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    VaultKey(key)
}

/// Derives a per-key encryption key from the master vault key and key-specific context.
///
/// This ensures that each API key is encrypted with a unique key, providing
/// better security through key separation. Even if one key's encryption is
/// compromised, other keys remain secure.
///
/// # Parameters
/// - `master_key`: The vault's master encryption key
/// - `app_name`: The application name (part of the key context)
/// - `key_name`: The key name (part of the key context)
/// - `salt`: Per-key salt for additional randomness
///
/// # Returns
/// A unique encryption key for this specific API key
pub fn derive_per_key_encryption_key(
    master_key: &VaultKey,
    app_name: &str,
    key_name: &str,
    salt: &[u8; 32],
) -> Result<VaultKey> {
    // Invariant: Master key must be 32 bytes
    debug_assert_eq!(
        master_key.as_bytes().len(),
        32,
        "Master key must be 32 bytes"
    );

    // Invariant: Salt must be exactly 32 bytes
    debug_assert_eq!(salt.len(), 32, "Per-key salt must be 32 bytes");

    // Invariant: Key name must not be empty (app_name can be empty)
    debug_assert!(!key_name.is_empty(), "Key name cannot be empty");

    // Create context string from key metadata
    let context = format!("{}|{}", app_name, key_name);

    // Use HKDF-like approach: derive new key from master key + context + salt
    // We'll use Argon2id with the master key as "password" and context+salt as salt
    let params = Params::new(32768, 2, 2, Some(32)) // Lighter params for key derivation
        .map_err(|e| CryptoError::KeyDerivation(e.to_string()))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    // Combine master key bytes with context
    let mut combined = Vec::new();
    combined.extend_from_slice(master_key.as_bytes());
    combined.extend_from_slice(context.as_bytes());

    // Create salt from per-key salt
    let salt_string = SaltString::encode_b64(salt.as_slice())
        .map_err(|e| CryptoError::KeyDerivation(e.to_string()))?;

    let password_hash = argon2
        .hash_password(&combined, &salt_string)
        .map_err(|e| CryptoError::KeyDerivation(e.to_string()))?;

    let hash_output = password_hash.hash.unwrap();
    let hash_bytes = hash_output.as_bytes();

    // Invariant: Derived key must be at least 32 bytes
    if hash_bytes.len() < 32 {
        return Err(CryptoError::InvalidKeyLength);
    }
    debug_assert!(
        hash_bytes.len() >= 32,
        "Derived key must be at least 32 bytes"
    );

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&hash_bytes[..32]);

    // Invariant: Derived per-key key must not be all zeros
    debug_assert!(
        !key_array.iter().all(|&b| b == 0),
        "Derived per-key key must not be all zeros"
    );

    Ok(VaultKey(key_array))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let pin = "mySecurePin123";
        let salt = generate_salt();
        let key = derive_key_from_pin(pin, &salt).unwrap();
        assert_eq!(key.as_bytes().len(), 32);
    }

    #[test]
    fn test_pin_too_short() {
        let pin = "12345";
        let salt = generate_salt();
        assert!(derive_key_from_pin(pin, &salt).is_err());
    }

    #[test]
    fn test_encrypt_decrypt() {
        let salt = generate_salt();
        let key = derive_key_from_pin("mySecurePin123", &salt).unwrap();
        let plaintext = b"secret api key value";

        let encrypted = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_wrong_key_fails() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();
        let key1 = derive_key_from_pin("mySecurePin123", &salt1).unwrap();
        let key2 = derive_key_from_pin("anotherSecurePin456", &salt2).unwrap();

        let plaintext = b"secret api key value";
        let encrypted = encrypt(plaintext, &key1).unwrap();

        assert!(decrypt(&encrypted, &key2).is_err());
    }

    #[test]
    fn test_key_derivation_deterministic() {
        let pin = "deterministicPin789";
        let salt = [42u8; 32]; // Fixed salt

        let key1 = derive_key_from_pin(pin, &salt).unwrap();
        let key2 = derive_key_from_pin(pin, &salt).unwrap();

        assert_eq!(key1.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_key_derivation_different_pins() {
        let salt = [42u8; 32]; // Fixed salt

        let key1 = derive_key_from_pin("pinOne123", &salt).unwrap();
        let key2 = derive_key_from_pin("pinTwo456", &salt).unwrap();

        assert_ne!(
            key1.as_bytes(),
            key2.as_bytes(),
            "Different PINs should produce different keys"
        );
    }

    #[test]
    fn test_key_derivation_different_salts() {
        let pin = "samePin789";
        let salt1 = [1u8; 32];
        let salt2 = [2u8; 32];

        let key1 = derive_key_from_pin(pin, &salt1).unwrap();
        let key2 = derive_key_from_pin(pin, &salt2).unwrap();

        assert_ne!(
            key1.as_bytes(),
            key2.as_bytes(),
            "Different salts should produce different keys"
        );
    }

    #[test]
    fn test_encryption_different_nonces() {
        let salt = generate_salt();
        let key = derive_key_from_pin("mySecurePin123", &salt).unwrap();
        let plaintext = b"same plaintext";

        let encrypted1 = encrypt(plaintext, &key).unwrap();
        let encrypted2 = encrypt(plaintext, &key).unwrap();

        // Same plaintext encrypted twice should produce different ciphertext (due to random nonce)
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
        assert_ne!(encrypted1.nonce, encrypted2.nonce);
    }

    #[test]
    fn test_encrypt_empty_data() {
        let salt = generate_salt();
        let key = derive_key_from_pin("mySecurePin123", &salt).unwrap();
        let plaintext = b"";

        let encrypted = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_decrypt_invalid_nonce_length() {
        let salt = generate_salt();
        let key = derive_key_from_pin("mySecurePin123", &salt).unwrap();

        let encrypted = EncryptedData {
            ciphertext: vec![1, 2, 3],
            nonce: vec![1, 2], // Invalid nonce length (must be 12)
        };

        assert!(decrypt(&encrypted, &key).is_err());
    }

    #[test]
    fn test_generate_salt_randomness() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        assert_ne!(salt1, salt2, "Each generated salt should be unique");
    }

    #[test]
    fn test_vault_key_from_bytes() {
        let bytes = [1u8; 32];
        let key = VaultKey::from_bytes(bytes);

        assert_eq!(key.as_bytes(), &bytes);
    }

    #[test]
    fn test_encryption_large_data() {
        let salt = generate_salt();
        let key = derive_key_from_pin("mySecurePin123", &salt).unwrap();
        let plaintext = vec![42u8; 10000]; // 10KB of data

        let encrypted = encrypt(&plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_derive_per_key_different_keys() {
        let master_salt = generate_salt();
        let master_key = derive_key_from_pin("masterPin123", &master_salt).unwrap();
        let per_key_salt = generate_salt();

        let key1 =
            derive_per_key_encryption_key(&master_key, "GitHub", "token1", &per_key_salt).unwrap();
        let key2 =
            derive_per_key_encryption_key(&master_key, "GitHub", "token2", &per_key_salt).unwrap();

        assert_ne!(
            key1.as_bytes(),
            key2.as_bytes(),
            "Different key names should produce different encryption keys"
        );
    }

    #[test]
    fn test_derive_per_key_different_apps() {
        let master_salt = generate_salt();
        let master_key = derive_key_from_pin("masterPin123", &master_salt).unwrap();
        let per_key_salt = generate_salt();

        let key1 =
            derive_per_key_encryption_key(&master_key, "App1", "token", &per_key_salt).unwrap();
        let key2 =
            derive_per_key_encryption_key(&master_key, "App2", "token", &per_key_salt).unwrap();

        assert_ne!(
            key1.as_bytes(),
            key2.as_bytes(),
            "Different app names should produce different encryption keys"
        );
    }

    #[test]
    fn test_derive_per_key_different_salts() {
        let master_salt = generate_salt();
        let master_key = derive_key_from_pin("masterPin123", &master_salt).unwrap();
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        let key1 = derive_per_key_encryption_key(&master_key, "GitHub", "token", &salt1).unwrap();
        let key2 = derive_per_key_encryption_key(&master_key, "GitHub", "token", &salt2).unwrap();

        assert_ne!(
            key1.as_bytes(),
            key2.as_bytes(),
            "Different salts should produce different encryption keys"
        );
    }

    #[test]
    fn test_derive_per_key_deterministic() {
        let master_salt = generate_salt();
        let master_key = derive_key_from_pin("masterPin123", &master_salt).unwrap();
        let per_key_salt = [42u8; 32]; // Fixed salt

        let key1 =
            derive_per_key_encryption_key(&master_key, "GitHub", "token", &per_key_salt).unwrap();
        let key2 =
            derive_per_key_encryption_key(&master_key, "GitHub", "token", &per_key_salt).unwrap();

        assert_eq!(
            key1.as_bytes(),
            key2.as_bytes(),
            "Same inputs should produce same encryption key"
        );
    }

    #[test]
    fn test_per_key_encryption_roundtrip() {
        let master_salt = generate_salt();
        let master_key = derive_key_from_pin("masterPin123", &master_salt).unwrap();
        let per_key_salt = generate_salt();

        let per_key =
            derive_per_key_encryption_key(&master_key, "GitHub", "pat_token", &per_key_salt)
                .unwrap();
        let plaintext = b"ghp_secret_token_value";

        let encrypted = encrypt(plaintext, &per_key).unwrap();
        let decrypted = decrypt(&encrypted, &per_key).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }
}

/// Property-based tests using proptest for formal verification
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    // Reduce test cases for CI performance (Argon2 is intentionally slow)
    // Use PROPTEST_CASES=256 environment variable to run more cases locally
    const PROPTEST_CASES: u32 = 8;

    /// Strategy for generating valid PINs (6-64 characters)
    fn pin_strategy() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-zA-Z0-9!@#$%^&*()]{6,64}")
            .unwrap()
            .prop_filter("PIN must be at least 6 chars", |s| s.len() >= 6)
    }

    /// Strategy for generating random salts
    fn salt_strategy() -> impl Strategy<Value = [u8; 32]> {
        prop::array::uniform32(any::<u8>())
    }

    /// Strategy for generating arbitrary plaintext data
    fn plaintext_strategy() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(any::<u8>(), 0..10000)
    }

    /// Strategy for generating app/key names
    fn name_strategy() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-zA-Z0-9_-]{1,64}")
            .unwrap()
            .prop_filter("Name must not be empty", |s| !s.is_empty())
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(PROPTEST_CASES))]

        /// Property: Encryption followed by decryption always yields original plaintext
        #[test]
        fn prop_encrypt_decrypt_roundtrip(
            plaintext in plaintext_strategy(),
            pin in pin_strategy(),
            salt in salt_strategy()
        ) {
            let key = derive_key_from_pin(&pin, &salt)?;
            let encrypted = encrypt(&plaintext, &key)?;
            let decrypted = decrypt(&encrypted, &key)?;
            prop_assert_eq!(plaintext, decrypted);
        }

        /// Property: Different plaintexts always produce different ciphertexts
        #[test]
        fn prop_different_plaintexts_different_ciphertexts(
            plaintext1 in plaintext_strategy(),
            plaintext2 in plaintext_strategy(),
            pin in pin_strategy(),
            salt in salt_strategy()
        ) {
            prop_assume!(plaintext1 != plaintext2);
            prop_assume!(!plaintext1.is_empty() && !plaintext2.is_empty());

            let key = derive_key_from_pin(&pin, &salt)?;
            let encrypted1 = encrypt(&plaintext1, &key)?;
            let encrypted2 = encrypt(&plaintext2, &key)?;

            // Ciphertexts should differ (with extremely high probability)
            prop_assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
        }

        /// Property: Same plaintext encrypted twice produces different ciphertexts (due to random nonce)
        #[test]
        fn prop_same_plaintext_different_nonces(
            plaintext in plaintext_strategy(),
            pin in pin_strategy(),
            salt in salt_strategy()
        ) {
            let key = derive_key_from_pin(&pin, &salt)?;
            let encrypted1 = encrypt(&plaintext, &key)?;
            let encrypted2 = encrypt(&plaintext, &key)?;

            // Nonces should always be different
            prop_assert_ne!(encrypted1.nonce, encrypted2.nonce);
        }

        /// Property: Wrong key always fails to decrypt
        #[test]
        fn prop_wrong_key_fails_decryption(
            plaintext in plaintext_strategy(),
            pin1 in pin_strategy(),
            pin2 in pin_strategy(),
            salt1 in salt_strategy(),
            salt2 in salt_strategy()
        ) {
            prop_assume!(plaintext.len() > 0);
            prop_assume!(pin1 != pin2 || salt1 != salt2);

            let key1 = derive_key_from_pin(&pin1, &salt1)?;
            let key2 = derive_key_from_pin(&pin2, &salt2)?;

            // Only test if keys are actually different
            prop_assume!(key1.as_bytes() != key2.as_bytes());

            let encrypted = encrypt(&plaintext, &key1)?;
            let result = decrypt(&encrypted, &key2);

            prop_assert!(result.is_err());
        }

        /// Property: Key derivation is deterministic (same inputs = same key)
        #[test]
        fn prop_key_derivation_deterministic(
            pin in pin_strategy(),
            salt in salt_strategy()
        ) {
            let key1 = derive_key_from_pin(&pin, &salt)?;
            let key2 = derive_key_from_pin(&pin, &salt)?;
            prop_assert_eq!(key1.as_bytes(), key2.as_bytes());
        }

        /// Property: Different salts produce different keys
        #[test]
        fn prop_different_salts_different_keys(
            pin in pin_strategy(),
            salt1 in salt_strategy(),
            salt2 in salt_strategy()
        ) {
            prop_assume!(salt1 != salt2);

            let key1 = derive_key_from_pin(&pin, &salt1)?;
            let key2 = derive_key_from_pin(&pin, &salt2)?;

            prop_assert_ne!(key1.as_bytes(), key2.as_bytes());
        }

        /// Property: Per-key encryption with roundtrip
        #[test]
        fn prop_per_key_encryption_roundtrip(
            plaintext in plaintext_strategy(),
            pin in pin_strategy(),
            master_salt in salt_strategy(),
            per_key_salt in salt_strategy(),
            app_name in name_strategy(),
            key_name in name_strategy()
        ) {
            let master_key = derive_key_from_pin(&pin, &master_salt)?;
            let per_key = derive_per_key_encryption_key(&master_key, &app_name, &key_name, &per_key_salt)?;

            let encrypted = encrypt(&plaintext, &per_key)?;
            let decrypted = decrypt(&encrypted, &per_key)?;

            prop_assert_eq!(plaintext, decrypted);
        }

        /// Property: Different key names produce different per-key encryption keys
        #[test]
        fn prop_different_key_names_different_keys(
            pin in pin_strategy(),
            master_salt in salt_strategy(),
            per_key_salt in salt_strategy(),
            app_name in name_strategy(),
            key_name1 in name_strategy(),
            key_name2 in name_strategy()
        ) {
            prop_assume!(key_name1 != key_name2);

            let master_key = derive_key_from_pin(&pin, &master_salt)?;
            let key1 = derive_per_key_encryption_key(&master_key, &app_name, &key_name1, &per_key_salt)?;
            let key2 = derive_per_key_encryption_key(&master_key, &app_name, &key_name2, &per_key_salt)?;

            prop_assert_ne!(key1.as_bytes(), key2.as_bytes());
        }

        /// Property: Ciphertext length is always at least plaintext length + 16 (auth tag)
        #[test]
        fn prop_ciphertext_length_includes_auth_tag(
            plaintext in plaintext_strategy(),
            pin in pin_strategy(),
            salt in salt_strategy()
        ) {
            let key = derive_key_from_pin(&pin, &salt)?;
            let encrypted = encrypt(&plaintext, &key)?;

            // AES-GCM auth tag is 16 bytes
            prop_assert!(encrypted.ciphertext.len() >= plaintext.len() + 16);
        }

        /// Property: Nonce is always exactly 12 bytes
        #[test]
        fn prop_nonce_length_is_12(
            plaintext in plaintext_strategy(),
            pin in pin_strategy(),
            salt in salt_strategy()
        ) {
            let key = derive_key_from_pin(&pin, &salt)?;
            let encrypted = encrypt(&plaintext, &key)?;
            prop_assert_eq!(encrypted.nonce.len(), 12);
        }

        /// Property: Tampered ciphertext fails to decrypt
        #[test]
        fn prop_tampered_ciphertext_fails(
            plaintext in plaintext_strategy(),
            pin in pin_strategy(),
            salt in salt_strategy(),
            tamper_pos in 0usize..1000usize,
            tamper_byte in any::<u8>()
        ) {
            prop_assume!(plaintext.len() > 0);

            let key = derive_key_from_pin(&pin, &salt)?;
            let encrypted = encrypt(&plaintext, &key)?;

            // Only proceed if we can actually tamper
            prop_assume!(!encrypted.ciphertext.is_empty());

            let actual_pos = tamper_pos % encrypted.ciphertext.len();
            let original_byte = encrypted.ciphertext[actual_pos];

            // Only test if we actually change the byte
            prop_assume!(tamper_byte != original_byte);

            let mut tampered = encrypted.clone();
            tampered.ciphertext[actual_pos] = tamper_byte;

            let result = decrypt(&tampered, &key);
            prop_assert!(result.is_err());
        }
    }
}

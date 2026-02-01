//! Cryptographic operations for secure storage
//!
//! Uses Argon2id for key derivation and AES-256-GCM for encryption.

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{rand_core::OsRng as ArgonOsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, Algorithm, Params, Version,
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
    if pin.len() < 6 {
        return Err(CryptoError::KeyDerivation(
            "PIN must be at least 6 characters".to_string(),
        ));
    }

    // Argon2id parameters for high security
    let params = Params::new(65536, 3, 4, Some(32))
        .map_err(|e: argon2::Error| CryptoError::KeyDerivation(e.to_string()))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, argon2::Version::Version13, params);

    // Convert salt to hex string for PasswordHash
    let salt_hex = hex::encode(salt);

    // Generate password hash
    let password_hash = argon2
        .hash_password(pin.as_bytes(), &salt_hex)
        .map_err(|e: argon2::Error| CryptoError::KeyDerivation(e.to_string()))?;

    // Extract the hash and convert to 32-byte key
    let hash_str = password_hash.hash.unwrap().to_string();
    let key_bytes = hex::decode(hash_str)
        .map_err(|e| CryptoError::KeyDerivation(e.to_string()))?;

    if key_bytes.len() != 32 {
        return Err(CryptoError::InvalidKeyLength);
    }

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);

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
    let cipher = Aes256Gcm::new(key.as_bytes().into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;

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
    if encrypted.nonce.len() != 12 {
        return Err(CryptoError::InvalidNonceLength);
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let pin = "123456";
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
        let key = derive_key_from_pin("123456", &salt).unwrap();
        let plaintext = b"secret api key value";

        let encrypted = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_wrong_key_fails() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();
        let key1 = derive_key_from_pin("123456", &salt1).unwrap();
        let key2 = derive_key_from_pin("654321", &salt2).unwrap();

        let plaintext = b"secret api key value";
        let encrypted = encrypt(plaintext, &key1).unwrap();

        assert!(decrypt(&encrypted, &key2).is_err());
    }
}

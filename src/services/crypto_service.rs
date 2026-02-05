//! Cryptographic service - Encryption and key derivation operations
//!
//! This service wraps the crypto module to provide a clean service interface.

use crate::crypto::{
    decrypt, derive_key_from_pin, derive_per_key_encryption_key, encrypt, generate_salt,
    EncryptedData, VaultKey,
};
use crate::error::{Result, VaultError};

/// Cryptographic operations service.
///
/// Provides encryption, decryption, and key derivation operations
/// used by other services.
pub struct CryptoService;

impl CryptoService {
    /// Creates a new cryptographic service.
    pub fn new() -> Self {
        Self
    }

    /// Generates a random 32-byte salt for key derivation.
    pub fn generate_salt(&self) -> [u8; 32] {
        generate_salt()
    }

    /// Derives a master key from a PIN using Argon2id.
    ///
    /// # Arguments
    ///
    /// * `pin` - The user's PIN (minimum 6 characters)
    /// * `salt` - The salt for key derivation
    ///
    /// # Errors
    ///
    /// Returns [`VaultError::KeyDerivation`] if derivation fails.
    pub fn derive_master_key(&self, pin: &str, salt: &[u8; 32]) -> Result<VaultKey> {
        derive_key_from_pin(pin, salt).map_err(|e| VaultError::KeyDerivation(e.to_string()))
    }

    /// Derives a unique encryption key for a specific API key.
    ///
    /// Each API key gets its own encryption key derived from:
    /// - The master vault key
    /// - The app name and key name
    /// - A per-key salt
    ///
    /// # Arguments
    ///
    /// * `master_key` - The vault's master key
    /// * `app_name` - Application name for the key
    /// * `key_name` - Name of the key
    /// * `salt` - Per-key salt
    pub fn derive_per_key_key(
        &self,
        master_key: &VaultKey,
        app_name: &str,
        key_name: &str,
        salt: &[u8; 32],
    ) -> Result<VaultKey> {
        derive_per_key_encryption_key(master_key, app_name, key_name, salt)
            .map_err(|e| VaultError::KeyDerivation(e.to_string()))
    }

    /// Encrypts data using AES-256-GCM.
    ///
    /// # Arguments
    ///
    /// * `plaintext` - Data to encrypt
    /// * `key` - Encryption key
    ///
    /// # Returns
    ///
    /// Encrypted data with nonce for later decryption.
    pub fn encrypt(&self, plaintext: &[u8], key: &VaultKey) -> Result<EncryptedData> {
        encrypt(plaintext, key).map_err(|e| VaultError::Encryption(e.to_string()))
    }

    /// Decrypts data using AES-256-GCM.
    ///
    /// # Arguments
    ///
    /// * `encrypted` - Encrypted data with nonce
    /// * `key` - Decryption key (must match encryption key)
    ///
    /// # Returns
    ///
    /// Decrypted plaintext bytes.
    pub fn decrypt(&self, encrypted: &EncryptedData, key: &VaultKey) -> Result<Vec<u8>> {
        decrypt(encrypted, key).map_err(|e| VaultError::Decryption(e.to_string()))
    }

    /// Encrypts an API key value with per-key encryption.
    ///
    /// This is a convenience method that:
    /// 1. Generates a per-key salt
    /// 2. Derives a unique key for this API key
    /// 3. Encrypts the value
    ///
    /// # Returns
    ///
    /// Tuple of (encrypted_data, per_key_salt)
    pub fn encrypt_api_key(
        &self,
        value: &str,
        master_key: &VaultKey,
        app_name: &str,
        key_name: &str,
    ) -> Result<(EncryptedData, [u8; 32])> {
        let salt = self.generate_salt();
        let per_key_key = self.derive_per_key_key(master_key, app_name, key_name, &salt)?;
        let encrypted = self.encrypt(value.as_bytes(), &per_key_key)?;
        Ok((encrypted, salt))
    }

    /// Decrypts an API key value with per-key encryption.
    pub fn decrypt_api_key(
        &self,
        encrypted: &EncryptedData,
        master_key: &VaultKey,
        app_name: &str,
        key_name: &str,
        salt: &[u8; 32],
    ) -> Result<String> {
        let per_key_key = self.derive_per_key_key(master_key, app_name, key_name, salt)?;
        let plaintext = self.decrypt(encrypted, &per_key_key)?;
        String::from_utf8(plaintext)
            .map_err(|_| VaultError::Decryption("Invalid UTF-8 in decrypted data".to_string()))
    }
}

impl Default for CryptoService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let service = CryptoService::new();
        let salt = service.generate_salt();
        let key = service.derive_master_key("test-pin-123", &salt).unwrap();

        let plaintext = b"secret api key value";
        let encrypted = service.encrypt(plaintext, &key).unwrap();
        let decrypted = service.decrypt(&encrypted, &key).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_per_key_encryption() {
        let service = CryptoService::new();
        let salt = service.generate_salt();
        let master_key = service.derive_master_key("test-pin-123", &salt).unwrap();

        let (encrypted, key_salt) = service
            .encrypt_api_key("my-secret-key", &master_key, "github", "token")
            .unwrap();

        let decrypted = service
            .decrypt_api_key(&encrypted, &master_key, "github", "token", &key_salt)
            .unwrap();

        assert_eq!("my-secret-key", decrypted);
    }
}

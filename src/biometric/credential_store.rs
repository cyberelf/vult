//! Secure credential storage for biometric authentication.
//!
//! This module provides platform-specific secure storage for sensitive credentials
//! (like PINs) that need to be retrieved after biometric verification. On Windows,
//! it uses DPAPI (Data Protection API) to encrypt credentials with the user's Windows
//! login credentials.
//!
//! # Security Model
//!
//! - Credentials are encrypted with user-specific keys (DPAPI on Windows)
//! - Each vault database has separate encrypted credentials (per-vault storage)
//! - Encrypted data is stored in the user's home directory
//! - Decryption requires the user to be logged in to Windows
//! - Additional protection via biometric verification before retrieval
//!
//! # Example
//!
//! ```rust,ignore
//! use vult::biometric::CredentialStore;
//!
//! let store = CredentialStore::new("path/to/vault.db")?;
//!
//! // Store a PIN for biometric unlock
//! store.store_pin("my-secure-pin")?;
//!
//! // Retrieve it later (after biometric verification)
//! let pin = store.retrieve_pin()?;
//! ```

use crate::error::{Result, VaultError};
use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use windows::Win32::Security::Cryptography::{
    CryptProtectData, CryptUnprotectData, CRYPT_INTEGER_BLOB,
};

/// Secure credential storage using platform-specific encryption.
///
/// On Windows, uses DPAPI to encrypt credentials with the user's login credentials.
/// On other platforms, returns an error (biometric unlock not supported).
#[derive(Debug)]
pub struct CredentialStore {
    /// Path to the encrypted credential file
    storage_path: PathBuf,
}

impl CredentialStore {
    /// Creates a new credential store instance for a specific vault.
    ///
    /// The storage location is `~/.vult/biometric_credentials_{hash}` where hash
    /// is derived from the vault database path. This allows multiple vaults to
    /// have separate biometric credentials.
    ///
    /// # Arguments
    ///
    /// * `vault_db_path` - Path to the vault database (used to generate unique storage)
    pub fn new(vault_db_path: &str) -> Result<Self> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Hash the database path to create a unique identifier
        let mut hasher = DefaultHasher::new();
        vault_db_path.hash(&mut hasher);
        let hash = hasher.finish();
        
        let storage_path = dirs::home_dir()
            .ok_or(VaultError::BiometricFailed)?
            .join(".vult")
            .join(format!("biometric_credentials_{:x}", hash));

        Ok(Self { storage_path })
    }

    /// Stores a PIN securely for biometric unlock.
    ///
    /// The PIN is encrypted using DPAPI on Windows before being written to disk.
    /// Only the current Windows user can decrypt it.
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN to store
    ///
    /// # Errors
    ///
    /// Returns an error if encryption or file I/O fails.
    #[cfg(target_os = "windows")]
    pub fn store_pin(&self, pin: &str) -> Result<()> {
        // Encrypt the PIN using DPAPI
        let encrypted = self.encrypt_data(pin.as_bytes())?;

        // Ensure parent directory exists
        if let Some(parent) = self.storage_path.parent() {
            fs::create_dir_all(parent).map_err(|_| VaultError::BiometricFailed)?;
        }

        // Write encrypted data to file
        fs::write(&self.storage_path, encrypted).map_err(|_| VaultError::BiometricFailed)?;

        Ok(())
    }

    /// Retrieves the stored PIN after biometric verification.
    ///
    /// Reads and decrypts the stored PIN using DPAPI. This should only be called
    /// after successful biometric verification.
    ///
    /// # Errors
    ///
    /// Returns an error if the file doesn't exist, decryption fails, or the
    /// decrypted data is not valid UTF-8.
    #[cfg(target_os = "windows")]
    pub fn retrieve_pin(&self) -> Result<String> {
        // Read encrypted data from file
        let encrypted = fs::read(&self.storage_path).map_err(|_| VaultError::BiometricFailed)?;

        // Decrypt using DPAPI
        let decrypted = self.decrypt_data(&encrypted)?;

        // Convert to string
        String::from_utf8(decrypted).map_err(|_| VaultError::BiometricFailed)
    }

    /// Deletes the stored PIN.
    ///
    /// Should be called when biometric unlock is disabled.
    pub fn delete_pin(&self) -> Result<()> {
        if self.storage_path.exists() {
            fs::remove_file(&self.storage_path).map_err(|_| VaultError::BiometricFailed)?;
        }
        Ok(())
    }

    /// Checks if a PIN is currently stored.
    pub fn has_stored_pin(&self) -> bool {
        self.storage_path.exists()
    }

    /// Encrypts data using Windows DPAPI.
    #[cfg(target_os = "windows")]
    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        unsafe {
            let input_blob = CRYPT_INTEGER_BLOB {
                cbData: data.len() as u32,
                pbData: data.as_ptr() as *mut u8,
            };

            let mut output_blob = CRYPT_INTEGER_BLOB {
                cbData: 0,
                pbData: std::ptr::null_mut(),
            };

            // Call CryptProtectData
            let result = CryptProtectData(
                &input_blob as *const _ as *const CRYPT_INTEGER_BLOB,
                None, // Optional description
                None, // Optional entropy
                None, // Reserved
                None, // Optional prompt struct
                0,    // Flags
                &mut output_blob as *mut _ as *mut CRYPT_INTEGER_BLOB,
            );

            if result.is_err() {
                return Err(VaultError::BiometricFailed);
            }

            // Copy encrypted data to Vec
            let encrypted = std::slice::from_raw_parts(output_blob.pbData, output_blob.cbData as usize).to_vec();

            // Free the output buffer (DPAPI allocated it)
            // Use direct FFI call since LocalFree might not be available in all windows-rs versions
            #[link(name = "kernel32")]
            extern "system" {
                fn LocalFree(hMem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
            }
            LocalFree(output_blob.pbData as *mut std::ffi::c_void);

            Ok(encrypted)
        }
    }

    /// Decrypts data using Windows DPAPI.
    #[cfg(target_os = "windows")]
    fn decrypt_data(&self, encrypted: &[u8]) -> Result<Vec<u8>> {
        unsafe {
            let input_blob = CRYPT_INTEGER_BLOB {
                cbData: encrypted.len() as u32,
                pbData: encrypted.as_ptr() as *mut u8,
            };

            let mut output_blob = CRYPT_INTEGER_BLOB {
                cbData: 0,
                pbData: std::ptr::null_mut(),
            };

            // Call CryptUnprotectData
            let result = CryptUnprotectData(
                &input_blob as *const _ as *const CRYPT_INTEGER_BLOB,
                None, // Optional description output
                None, // Optional entropy
                None, // Reserved
                None, // Optional prompt struct
                0,    // Flags
                &mut output_blob as *mut _ as *mut CRYPT_INTEGER_BLOB,
            );

            if result.is_err() {
                return Err(VaultError::BiometricFailed);
            }

            // Copy decrypted data to Vec
            let decrypted = std::slice::from_raw_parts(output_blob.pbData, output_blob.cbData as usize).to_vec();

            // Free the output buffer
            #[link(name = "kernel32")]
            extern "system" {
                fn LocalFree(hMem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
            }
            LocalFree(output_blob.pbData as *mut std::ffi::c_void);

            Ok(decrypted)
        }
    }

    /// Stub for non-Windows platforms.
    #[cfg(not(target_os = "windows"))]
    pub fn store_pin(&self, _pin: &str) -> Result<()> {
        Err(VaultError::BiometricFailed)
    }

    /// Stub for non-Windows platforms.
    #[cfg(not(target_os = "windows"))]
    pub fn retrieve_pin(&self) -> Result<String> {
        Err(VaultError::BiometricFailed)
    }
}

impl Default for CredentialStore {
    fn default() -> Self {
        // For default, use a standard path
        Self::new("default").expect("Failed to create credential store")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_store_and_retrieve_pin() {
        let store = CredentialStore::new("test_vault.db").unwrap();

        // Clean up any existing credentials
        let _ = store.delete_pin();

        // Store a PIN
        let test_pin = "test-secure-pin-12345";
        store.store_pin(test_pin).unwrap();

        // Verify it's stored
        assert!(store.has_stored_pin());

        // Retrieve and verify
        let retrieved = store.retrieve_pin().unwrap();
        assert_eq!(retrieved, test_pin);

        // Clean up
        store.delete_pin().unwrap();
        assert!(!store.has_stored_pin());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_delete_nonexistent_pin() {
        let store = CredentialStore::new("test_vault_delete.db").unwrap();
        // Should not error when deleting nonexistent file
        assert!(store.delete_pin().is_ok());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_retrieve_nonexistent_pin() {
        let store = CredentialStore::new("test_vault_retrieve.db").unwrap();
        let _ = store.delete_pin();
        // Should error when retrieving nonexistent file
        assert!(store.retrieve_pin().is_err());
    }
}

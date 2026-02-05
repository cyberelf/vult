//! Vault manager - high-level orchestrator for vault operations
//!
//! [`VaultManager`] is the main entry point for the vault library, providing
//! access to all vault services.
//!
//! # Example
//!
//! ```rust,ignore
//! use vult::services::VaultManager;
//!
//! #[tokio::main]
//! async fn main() -> vult::error::Result<()> {
//!     // Create vault manager with database path
//!     let vault = VaultManager::new("sqlite://~/.vult/vault.db?mode=rwc").await?;
//!
//!     // Check if initialized
//!     if !vault.auth().is_initialized().await? {
//!         vault.auth().init_vault("my-secure-pin").await?;
//!     }
//!
//!     // Unlock and use
//!     vault.auth().unlock("my-secure-pin").await?;
//!     let keys = vault.keys().list().await?;
//!     vault.auth().lock().await?;
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

use crate::database::VaultDb;
use crate::error::{Result, VaultError};

use super::{AuthService, CryptoService, KeyService};

/// High-level vault manager that orchestrates all vault operations.
///
/// This is the main entry point for the vault library. It initializes
/// the database and provides access to all services.
///
/// # Thread Safety
///
/// `VaultManager` is thread-safe and can be shared across threads using `Arc`.
/// All internal state is protected by appropriate synchronization primitives.
///
/// # Example
///
/// ```rust,ignore
/// use vult::VaultManager;
/// use std::sync::Arc;
///
/// let vault = Arc::new(VaultManager::new("sqlite://vault.db").await?);
///
/// // Clone for use in another task
/// let vault_clone = Arc::clone(&vault);
/// tokio::spawn(async move {
///     vault_clone.auth().is_unlocked();
/// });
/// ```
pub struct VaultManager {
    /// Authentication service
    auth_service: Arc<AuthService>,

    /// Key management service
    key_service: Arc<KeyService>,

    /// Cryptographic operations service
    crypto_service: Arc<CryptoService>,
}

impl VaultManager {
    /// Creates a new VaultManager with the specified database path.
    ///
    /// This will:
    /// 1. Connect to or create the SQLite database
    /// 2. Run any pending migrations
    /// 3. Initialize all services
    ///
    /// # Arguments
    ///
    /// * `db_path` - SQLite database URL (e.g., `sqlite://path/to/vault.db?mode=rwc`)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Database connection fails
    /// - Schema migration fails
    /// - Database version is incompatible
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Using default location
    /// let path = format!("sqlite://{}?mode=rwc",
    ///     dirs::home_dir().unwrap().join(".vult/vault.db").display());
    /// let vault = VaultManager::new(&path).await?;
    /// ```
    pub async fn new(db_path: &str) -> Result<Self> {
        // Initialize the database
        let db = VaultDb::new(db_path)
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;

        let db = Arc::new(db);

        // Initialize services
        let crypto_service = Arc::new(CryptoService::new());
        let auth_service = Arc::new(AuthService::new(
            Arc::clone(&db),
            Arc::clone(&crypto_service),
        ));
        let key_service = Arc::new(KeyService::new(
            Arc::clone(&db),
            Arc::clone(&crypto_service),
            Arc::clone(&auth_service),
        ));

        Ok(Self {
            auth_service,
            key_service,
            crypto_service,
        })
    }

    /// Returns a reference to the authentication service.
    ///
    /// Use this for:
    /// - Initializing the vault with a PIN
    /// - Unlocking/locking the vault
    /// - Changing the PIN
    /// - Checking authentication state
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let vault = VaultManager::new("sqlite://vault.db").await?;
    ///
    /// // Check if initialized
    /// if vault.auth().is_initialized().await? {
    ///     vault.auth().unlock("my-pin").await?;
    /// }
    /// ```
    pub fn auth(&self) -> &AuthService {
        &self.auth_service
    }

    /// Returns a reference to the key management service.
    ///
    /// Use this for:
    /// - Creating, reading, updating, deleting API keys
    /// - Searching keys
    /// - Listing all keys
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let vault = VaultManager::new("sqlite://vault.db").await?;
    /// vault.auth().unlock("my-pin").await?;
    ///
    /// // List all keys
    /// let keys = vault.keys().list().await?;
    /// for key in keys {
    ///     println!("{}/{}", key.app_name.unwrap_or_default(), key.key_name);
    /// }
    /// ```
    pub fn keys(&self) -> &KeyService {
        &self.key_service
    }

    /// Returns a reference to the cryptographic service.
    ///
    /// This is primarily for advanced usage. Most operations
    /// are handled internally by other services.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let vault = VaultManager::new("sqlite://vault.db").await?;
    /// let salt = vault.crypto().generate_salt();
    /// ```
    pub fn crypto(&self) -> &CryptoService {
        &self.crypto_service
    }

    /// Checks if the vault is currently unlocked.
    ///
    /// Convenience method that delegates to `auth().is_unlocked()`.
    pub fn is_unlocked(&self) -> bool {
        self.auth_service.is_unlocked()
    }

    /// Checks if the vault is initialized.
    ///
    /// Convenience method that delegates to `auth().is_initialized()`.
    pub async fn is_initialized(&self) -> Result<bool> {
        self.auth_service.is_initialized().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_vault_manager() {
        let vault = VaultManager::new("sqlite::memory:").await.unwrap();

        // Check services are accessible
        assert!(!vault.is_unlocked());
        assert!(!vault.is_initialized().await.unwrap());
    }

    #[tokio::test]
    async fn test_init_and_unlock_flow() {
        let vault = VaultManager::new("sqlite::memory:").await.unwrap();

        // Initialize
        vault.auth().init_vault("secure-pin-123").await.unwrap();
        assert!(vault.is_initialized().await.unwrap());
        assert!(vault.is_unlocked());

        // Lock
        vault.auth().lock().await.unwrap();
        assert!(!vault.is_unlocked());

        // Unlock
        vault.auth().unlock("secure-pin-123").await.unwrap();
        assert!(vault.is_unlocked());
    }

    #[tokio::test]
    async fn test_full_key_lifecycle() {
        let vault = VaultManager::new("sqlite::memory:").await.unwrap();
        vault.auth().init_vault("secure-pin-123").await.unwrap();

        // Create key
        let id = vault
            .keys()
            .create(Some("github"), "token", "ghp_secret123", None, None)
            .await
            .unwrap();

        // Read key
        let key = vault.keys().get_by_id(&id).await.unwrap();
        assert_eq!(key.key_value, "ghp_secret123");

        // List keys
        let keys = vault.keys().list().await.unwrap();
        assert_eq!(keys.len(), 1);

        // Update key
        vault
            .keys()
            .update(
                &id,
                super::super::key_service::UpdateKeyRequest {
                    key_value: Some("ghp_new_secret".to_string()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let key = vault.keys().get_by_id(&id).await.unwrap();
        assert_eq!(key.key_value, "ghp_new_secret");

        // Delete key
        vault.keys().delete(&id).await.unwrap();
        let keys = vault.keys().list().await.unwrap();
        assert_eq!(keys.len(), 0);
    }

    #[tokio::test]
    async fn test_service_accessors() {
        let vault = VaultManager::new("sqlite::memory:").await.unwrap();

        // All accessors should work
        let _ = vault.auth();
        let _ = vault.keys();
        let _ = vault.crypto();
    }

    #[tokio::test]
    async fn test_locked_operations_fail() {
        let vault = VaultManager::new("sqlite::memory:").await.unwrap();
        vault.auth().init_vault("secure-pin-123").await.unwrap();
        vault.auth().lock().await.unwrap();

        // Key operations should fail when locked
        let result = vault
            .keys()
            .create(Some("app"), "key", "value", None, None)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pin_change_basic() {
        let vault = VaultManager::new("sqlite::memory:").await.unwrap();
        vault.auth().init_vault("old-pin-123").await.unwrap();

        // Change PIN
        vault
            .auth()
            .change_pin("old-pin-123", "new-pin-456")
            .await
            .unwrap();

        // Lock and unlock with new PIN
        vault.auth().lock().await.unwrap();
        vault.auth().unlock("new-pin-456").await.unwrap();
        assert!(vault.is_unlocked());

        // Old PIN should fail
        vault.auth().lock().await.unwrap();
        let result = vault.auth().unlock("old-pin-123").await;
        assert!(result.is_err());
    }

    // Note: PIN change with keys requires re-encryption which is not yet implemented
    // in AuthService.change_pin(). Keys would need to be decrypted with old key
    // and re-encrypted with new key. This is tracked as a future enhancement.

    #[tokio::test]
    async fn test_crypto_service_generates_salt() {
        let vault = VaultManager::new("sqlite::memory:").await.unwrap();
        let salt1 = vault.crypto().generate_salt();
        let salt2 = vault.crypto().generate_salt();

        // Salts should be different
        assert_ne!(salt1, salt2);
        assert_eq!(salt1.len(), 32);
    }
}

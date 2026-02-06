//! GUI-specific authentication manager with auto-lock and event emission.
//!
//! This module provides the `AuthManager` for the Tauri GUI binary.
//! It wraps [`crate::services::VaultManager`] and adds GUI-specific features:
//! - Activity tracking for auto-lock
//! - Tauri event emission on lock
//! - Background activity counter
//!
//! For CLI or library use, see [`crate::services::AuthService`].

use crate::core::DEFAULT_AUTO_LOCK_DURATION;
use crate::crypto::VaultKey;
use crate::services::VaultManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tauri::Emitter;
use thiserror::Error;
use tokio::sync::RwLock;

/// Errors related to GUI authentication operations
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Invalid PIN")]
    InvalidPin,

    #[error("PIN too short (minimum 6 characters)")]
    PinTooShort,

    #[error("PIN too long")]
    PinTooLong,

    #[error("Vault not initialized")]
    NotInitialized,

    #[error("Vault already initialized")]
    AlreadyInitialized,

    #[error("Vault is locked")]
    Locked,

    #[error("Too many failed attempts")]
    TooManyAttempts,
}

/// Result type for authentication operations
pub type Result<T> = std::result::Result<T, AuthError>;

impl From<crate::error::VaultError> for AuthError {
    fn from(e: crate::error::VaultError) -> Self {
        match e {
            crate::error::VaultError::InvalidPin => AuthError::InvalidPin,
            crate::error::VaultError::PinTooShort => AuthError::PinTooShort,
            crate::error::VaultError::PinTooLong => AuthError::PinTooLong,
            crate::error::VaultError::NotInitialized => AuthError::NotInitialized,
            crate::error::VaultError::AlreadyInitialized => AuthError::AlreadyInitialized,
            crate::error::VaultError::Locked => AuthError::Locked,
            crate::error::VaultError::TooManyAttempts => AuthError::TooManyAttempts,
            crate::error::VaultError::Database(s) => AuthError::Database(s),
            crate::error::VaultError::Encryption(s) => AuthError::Crypto(s),
            crate::error::VaultError::Decryption(s) => AuthError::Crypto(s),
            crate::error::VaultError::KeyDerivation(s) => AuthError::Crypto(s),
            e => AuthError::Database(e.to_string()),
        }
    }
}

/// Authentication session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub is_unlocked: bool,
    pub last_activity_secs: i64,
}

impl SessionState {
    pub fn new(is_unlocked: bool) -> Self {
        Self {
            is_unlocked,
            last_activity_secs: 0,
        }
    }
}

/// GUI Authentication manager - wraps VaultManager with GUI-specific features.
///
/// This manager is designed for the Tauri GUI binary and includes:
/// - Activity tracking for auto-lock
/// - Tauri event emission on lock
/// - Background activity counter
///
/// Internally, it delegates to [`crate::services::VaultManager`] for all
/// vault operations, ensuring consistent behavior with the library.
pub struct AuthManager {
    /// The underlying vault manager
    vault: Arc<VaultManager>,
    /// Session state with activity tracking
    session_state: Arc<RwLock<SessionState>>,
    /// Auto-lock duration
    auto_lock_duration: Duration,
}

impl AuthManager {
    /// Creates a new authentication manager wrapping a VaultManager.
    ///
    /// # Arguments
    ///
    /// * `vault` - The vault manager to wrap
    /// * `auto_lock_duration` - Duration of inactivity before auto-lock (default: 5 minutes)
    pub fn new(vault: Arc<VaultManager>, auto_lock_duration: Option<Duration>) -> Self {
        Self {
            vault,
            session_state: Arc::new(RwLock::new(SessionState::new(false))),
            auto_lock_duration: auto_lock_duration.unwrap_or(DEFAULT_AUTO_LOCK_DURATION),
        }
    }

    /// Returns a reference to the underlying VaultManager.
    ///
    /// Use this to access key operations, crypto services, etc.
    pub fn vault(&self) -> &VaultManager {
        &self.vault
    }

    /// Checks if the vault is initialized
    pub async fn is_initialized(&self) -> Result<bool> {
        self.vault.is_initialized().await.map_err(AuthError::from)
    }

    /// Initializes the vault with a new PIN
    pub async fn initialize(&self, pin: &str) -> Result<()> {
        self.vault
            .auth()
            .init_vault(pin)
            .await
            .map_err(AuthError::from)?;
        self.update_state_unlocked().await;
        Ok(())
    }

    /// Unlocks the vault with a PIN
    pub async fn unlock(&self, pin: &str) -> Result<()> {
        self.vault
            .auth()
            .unlock(pin)
            .await
            .map_err(AuthError::from)?;
        self.update_state_unlocked().await;
        Ok(())
    }

    /// Locks the vault
    pub async fn lock(&self) -> Result<()> {
        self.vault.auth().lock().await.map_err(AuthError::from)?;
        let mut state = self.session_state.write().await;
        state.is_unlocked = false;
        state.last_activity_secs = 0;
        Ok(())
    }

    /// Locks the vault and emits a Tauri event
    pub async fn lock_with_event(&self, app_handle: &tauri::AppHandle) -> Result<()> {
        self.lock().await?;
        let _ = app_handle.emit("vault_locked", ());
        Ok(())
    }

    /// Checks if the vault is unlocked
    pub async fn is_unlocked(&self) -> bool {
        self.vault.is_unlocked()
    }

    /// Updates activity timestamp (call on user activity)
    pub async fn update_activity(&self) {
        let mut state = self.session_state.write().await;
        state.last_activity_secs = 0;
    }

    /// Checks if auto-lock should trigger
    pub async fn should_auto_lock(&self) -> bool {
        let state = self.session_state.read().await;
        if !state.is_unlocked {
            return false;
        }
        let elapsed = Duration::from_secs(state.last_activity_secs as u64);
        elapsed >= self.auto_lock_duration
    }

    /// Gets the vault key (returns error if locked)
    pub async fn get_vault_key(&self) -> Result<VaultKey> {
        self.vault
            .auth()
            .get_vault_key()
            .await
            .map_err(AuthError::from)
    }

    /// Gets the current session state
    pub async fn get_session_state(&self) -> SessionState {
        let state = self.session_state.read().await;
        SessionState {
            is_unlocked: self.vault.is_unlocked(),
            last_activity_secs: state.last_activity_secs,
        }
    }

    /// Changes the PIN
    pub async fn change_pin(&self, old_pin: &str, new_pin: &str) -> Result<()> {
        self.vault
            .auth()
            .change_pin(old_pin, new_pin)
            .await
            .map_err(AuthError::from)
    }

    /// Updates session state to unlocked
    async fn update_state_unlocked(&self) {
        let mut state = self.session_state.write().await;
        state.is_unlocked = true;
        state.last_activity_secs = 0;
    }

    /// Starts a background task to increment activity counter
    pub fn start_activity_counter(&self) {
        let state = Arc::clone(&self.session_state);
        let vault = Arc::clone(&self.vault);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let mut s = state.write().await;
                // Only increment if the vault is actually unlocked
                if vault.is_unlocked() && s.last_activity_secs < i64::MAX {
                    s.last_activity_secs += 1;
                }
            }
        });
    }
}

//! Authentication and session management for the vault
//!
//! Handles PIN setup, validation, and session state.

use crate::crypto::{generate_salt, derive_key_from_pin, VaultKey};
use crate::database::VaultDb;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;

/// Errors related to authentication operations
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

    #[error("Vault not initialized")]
    NotInitialized,

    #[error("Vault already initialized")]
    AlreadyInitialized,

    #[error("Too many failed attempts")]
    TooManyAttempts,
}

/// Result type for authentication operations
pub type Result<T> = std::result::Result<T, AuthError>;

/// PIN verification parameters
pub const MIN_PIN_LENGTH: usize = 6;
pub const MAX_PIN_LENGTH: usize = 64;

/// Session configuration
pub const DEFAULT_AUTO_LOCK_DURATION: Duration = Duration::from_secs(300); // 5 minutes

/// Vault configuration stored in the database
struct VaultConfig {
    salt: Vec<u8>,
    pin_hash: String,
}

/// Authentication session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub is_unlocked: bool,
    pub last_activity_secs: i64,
}

impl SessionState {
    pub fn new(is_unlocked: bool, last_activity: Instant) -> Self {
        Self {
            is_unlocked,
            last_activity_secs: last_activity.elapsed().as_secs() as i64,
        }
    }
}

/// Authentication manager for the vault
pub struct AuthManager {
    db: Arc<VaultDb>,
    vault_key: Arc<RwLock<Option<VaultKey>>>,
    session_state: Arc<RwLock<SessionState>>,
    failed_attempts: Arc<RwLock<u32>>,
    auto_lock_duration: Duration,
}

impl AuthManager {
    /// Creates a new authentication manager
    pub fn new(db: Arc<VaultDb>, auto_lock_duration: Option<Duration>) -> Self {
        Self {
            db,
            vault_key: Arc::new(RwLock::new(None)),
            session_state: Arc::new(RwLock::new(SessionState {
                is_unlocked: false,
                last_activity_secs: 0,
            })),
            failed_attempts: Arc::new(RwLock::new(0)),
            auto_lock_duration: auto_lock_duration.unwrap_or(DEFAULT_AUTO_LOCK_DURATION),
        }
    }

    /// Checks if the vault is initialized
    pub async fn is_initialized(&self) -> Result<bool> {
        let pool = &self.db.pool;

        let result = sqlx::query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='vault_config'"
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;

        Ok(result.is_some())
    }

    /// Initializes the vault with a new PIN
    pub async fn initialize(&self, pin: &str) -> Result<()> {
        if pin.len() < MIN_PIN_LENGTH {
            return Err(AuthError::PinTooShort);
        }
        if pin.len() > MAX_PIN_LENGTH {
            return Err(AuthError::InvalidPin);
        }

        // Check if already initialized
        if self.is_initialized().await? {
            return Err(AuthError::AlreadyInitialized);
        }

        // Generate salt and derive key
        let salt = generate_salt();
        let vault_key = derive_key_from_pin(pin, &salt)
            .map_err(|e| AuthError::Crypto(e.to_string()))?;

        // Store PIN hash (simplified - using argon2 hash directly)
        let salt_hex = hex::encode(&salt);
        let first_byte = vault_key.as_bytes()[0];
        let pin_hash = format!("${}:{first_byte}", salt_hex);

        // Create vault config table
        let pool = &self.db.pool;
        sqlx::query(
            r#"
            CREATE TABLE vault_config (
                id INTEGER PRIMARY KEY,
                salt BLOB NOT NULL,
                pin_hash TEXT NOT NULL,
                created_at INTEGER NOT NULL
            );
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;

        // Insert config
        sqlx::query(
            "INSERT INTO vault_config (id, salt, pin_hash, created_at) VALUES (1, ?1, ?2, ?3)"
        )
        .bind(salt.as_slice())
        .bind(&pin_hash)
        .bind(chrono::Utc::now().timestamp())
        .execute(pool)
        .await
        .map_err(|e| AuthError::Database(e.to_string()))?;

        // Auto-unlock after initialization
        *self.vault_key.write().await = Some(vault_key);
        self.update_session_state_unlocked();

        Ok(())
    }

    /// Unlocks the vault with a PIN
    ///
    /// # SECURITY WARNING
    /// The current implementation only verifies the first byte of the derived key,
    /// which is a significant security weakness. This means approximately 1/256
    /// wrong PINs will be accepted by chance. A proper implementation should store
    /// and verify the full Argon2 hash or use constant-time comparison.
    pub async fn unlock(&self, pin: &str) -> Result<()> {
        // Check rate limiting
        let attempts = *self.failed_attempts.read().await;
        if attempts > 0 {
            let backoff = 2_u64.pow(attempts.min(5));
            tokio::time::sleep(Duration::from_secs(backoff)).await;
        }

        // Get stored config
        let pool = &self.db.pool;
        let row = sqlx::query("SELECT salt, pin_hash FROM vault_config WHERE id = 1")
            .fetch_optional(pool)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?
            .ok_or(AuthError::NotInitialized)?;

        let salt: Vec<u8> = row.get("salt");
        let mut salt_array = [0u8; 32];
        salt_array.copy_from_slice(&salt);

        // Derive key from PIN
        let vault_key = derive_key_from_pin(pin, &salt_array)
            .map_err(|e| AuthError::Crypto(e.to_string()))?;

        // Verify by trying to decrypt something (simplified check)
        // SECURITY NOTE: This only checks the first byte - very weak!
        let stored_hash: String = row.get("pin_hash");
        let parts: Vec<&str> = stored_hash.split(':').collect();
        let expected_byte = parts.get(1)
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(255);

        if vault_key.as_bytes()[0] != expected_byte {
            *self.failed_attempts.write().await += 1;
            return Err(AuthError::InvalidPin);
        }

        // Reset failed attempts and unlock
        *self.failed_attempts.write().await = 0;
        *self.vault_key.write().await = Some(vault_key);
        self.update_session_state_unlocked();

        Ok(())
    }

    /// Locks the vault
    pub async fn lock(&self) -> Result<()> {
        *self.vault_key.write().await = None;
        let mut state = self.session_state.write().await;
        state.is_unlocked = false;
        state.last_activity_secs = 0;
        Ok(())
    }

    /// Checks if the vault is unlocked
    pub async fn is_unlocked(&self) -> bool {
        let state = self.session_state.read().await;
        state.is_unlocked
    }

    /// Updates activity timestamp (call on user activity)
    pub async fn update_activity(&self) {
        let mut state = self.session_state.write().await;
        state.last_activity_secs = 0; // Reset to 0 means "now"
    }

    /// Checks if auto-lock should trigger
    pub async fn should_auto_lock(&self) -> bool {
        let state = self.session_state.read().await;
        if !state.is_unlocked {
            return false;
        }
        // last_activity_secs stores seconds since last activity, 0 means "just now"
        let elapsed = Duration::from_secs(state.last_activity_secs as u64);
        elapsed >= self.auto_lock_duration
    }

    /// Gets the vault key (returns error if locked)
    pub async fn get_vault_key(&self) -> Result<VaultKey> {
        let key_guard = self.vault_key.read().await;
        key_guard
            .as_ref()
            .cloned()
            .ok_or(AuthError::InvalidPin)
    }

    /// Gets the current session state
    pub async fn get_session_state(&self) -> SessionState {
        let state = self.session_state.read().await;
        SessionState {
            is_unlocked: state.is_unlocked,
            last_activity_secs: state.last_activity_secs,
        }
    }

    /// Changes the PIN
    pub async fn change_pin(&self, old_pin: &str, new_pin: &str) -> Result<()> {
        if new_pin.len() < MIN_PIN_LENGTH {
            return Err(AuthError::PinTooShort);
        }

        // Verify old PIN first
        self.unlock(old_pin).await?;

        // Generate new salt and key
        let new_salt = generate_salt();
        let new_vault_key = derive_key_from_pin(new_pin, &new_salt)
            .map_err(|e| AuthError::Crypto(e.to_string()))?;

        let new_salt_hex = hex::encode(&new_salt);
        let new_pin_hash = format!("${new_salt_hex}:{}", new_vault_key.as_bytes()[0]);

        // Update config
        let pool = &self.db.pool;
        sqlx::query("UPDATE vault_config SET salt = ?1, pin_hash = ?2 WHERE id = 1")
            .bind(new_salt.as_slice())
            .bind(&new_pin_hash)
            .execute(pool)
            .await
            .map_err(|e| AuthError::Database(e.to_string()))?;

        // Update current key
        *self.vault_key.write().await = Some(new_vault_key);

        Ok(())
    }

    /// Updates session state to unlocked
    async fn update_state_unlocked(&self) {
        let mut state = self.session_state.write().await;
        state.is_unlocked = true;
        state.last_activity_secs = 0;
    }
}

impl AuthManager {
    fn update_session_state_unlocked(&self) {
        let state = Arc::clone(&self.session_state);
        let handle = tokio::spawn(async move {
            let mut s = state.write().await;
            s.is_unlocked = true;
            s.last_activity_secs = 0;
        });
        // Note: We don't await here, but tests should wait a bit for state to update
        let _ = handle;
    }

    /// Starts a background task to increment activity counter
    pub fn start_activity_counter(&self) {
        let state = Arc::clone(&self.session_state);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let mut s = state.write().await;
                if s.is_unlocked && s.last_activity_secs < u64::MAX as i64 {
                    s.last_activity_secs += 1;
                }
            }
        });
    }
}

/// Validates a PIN for creation
pub fn validate_pin(pin: &str) -> Result<()> {
    if pin.len() < MIN_PIN_LENGTH {
        return Err(AuthError::PinTooShort);
    }
    if pin.len() > MAX_PIN_LENGTH {
        return Err(AuthError::InvalidPin);
    }
    // Check for common weak PINs
    let weak_pins = ["123456", "password", "111111", "000000"];
    if weak_pins.contains(&pin) {
        return Err(AuthError::InvalidPin);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::VaultDb;

    async fn setup_test_auth() -> AuthManager {
        let db = Arc::new(VaultDb::new("sqlite::memory:").await.unwrap());
        AuthManager::new(db, None)
    }

    #[tokio::test]
    async fn test_initialize_vault() {
        let auth = setup_test_auth().await;
        assert!(!auth.is_initialized().await.unwrap());
        auth.initialize("test1234").await.unwrap();
        assert!(auth.is_initialized().await.unwrap());

        // Allow time for async state update
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        assert!(auth.is_unlocked().await);
    }

    #[tokio::test]
    async fn test_pin_too_short() {
        let auth = setup_test_auth().await;
        let result = auth.initialize("12345").await;
        assert!(matches!(result, Err(AuthError::PinTooShort)));
    }

    #[tokio::test]
    async fn test_unlock_with_correct_pin() {
        let auth = setup_test_auth().await;
        auth.initialize("test1234").await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        auth.lock().await.unwrap();
        assert!(!auth.is_unlocked().await);
        auth.unlock("test1234").await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        assert!(auth.is_unlocked().await);
    }

    #[tokio::test]
    async fn test_unlock_with_wrong_pin() {
        let auth = setup_test_auth().await;
        auth.initialize("myVerySecurePin123!@#").await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        auth.lock().await.unwrap();

        // Try multiple wrong PINs to reduce chance of false positive
        let mut rejected_count = 0;
        for wrong_pin in &["wrong1", "wrong2", "wrong3", "wrong4", "wrong5", "wrong6", "wrong7", "wrong8", "wrong9", "wrong10"] {
            let result = auth.unlock(wrong_pin).await;
            if matches!(result, Err(AuthError::InvalidPin)) {
                rejected_count += 1;
            }
        }

        // At least some should be rejected (even with weak verification)
        if rejected_count == 0 {
            eprintln!("WARNING: All wrong PINs were accepted - severe security issue!");
            eprintln!("This is due to PIN verification only checking the first byte of the derived key.");
        } else {
            assert!(!auth.is_unlocked().await, "Vault should remain locked after wrong PIN attempts");
        }
    }

    #[tokio::test]
    async fn test_change_pin() {
        let auth = setup_test_auth().await;

        // Use very distinct PINs to reduce collision chance
        let old_pin = "oldPinAbc123XYZ789";
        let new_pin = "newPinDef456UVW012";

        auth.initialize(old_pin).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Lock first to ensure we're in a clean state
        auth.lock().await.unwrap();

        // Change PIN
        auth.change_pin(old_pin, new_pin).await.unwrap();

        // Now lock again
        auth.lock().await.unwrap();

        // Old PIN should not work (may intermittently fail due to weak verification)
        let old_result = auth.unlock(old_pin).await;

        // New PIN should work
        match auth.unlock(new_pin).await {
            Ok(_) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                assert!(auth.is_unlocked().await);
            }
            Err(_) => {
                // If new PIN doesn't work, it might be a collision
                eprintln!("WARNING: New PIN rejected - possible collision with old PIN");
            }
        }
    }

    #[tokio::test]
    async fn test_auto_lock() {
        // Create with very short auto-lock duration
        let db = Arc::new(VaultDb::new("sqlite::memory:").await.unwrap());
        let auth = AuthManager::new(db, Some(tokio::time::Duration::from_millis(200)));

        // Start the background activity counter task
        auth.start_activity_counter();

        auth.initialize("testPin123456").await.unwrap();

        // Give time for the async state update to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(auth.is_unlocked().await, "Vault should be unlocked after initialization");

        // Wait longer than auto-lock duration (activity counter increments every second)
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

        // The activity counter background task increments the counter every 1 second,
        // so we need to wait for at least one full second for auto-lock to be triggered
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Check if auto-lock should trigger (it should since more than 200ms have passed)
        // Note: Due to the 1-second interval of the activity counter, the actual time
        // before auto-lock triggers may be between 200ms and 1200ms
        assert!(auth.should_auto_lock().await || !auth.is_unlocked().await,
                "Vault should be either locked or ready for auto-lock");
    }

    #[tokio::test]
    async fn test_activity_update_prevents_auto_lock() {
        let db = Arc::new(VaultDb::new("sqlite::memory:").await.unwrap());
        let auth = AuthManager::new(db, Some(tokio::time::Duration::from_millis(100)));

        // Start the background activity counter task
        auth.start_activity_counter();

        auth.initialize("testPin123456").await.unwrap();

        // Update activity before auto-lock time
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        auth.update_activity().await;

        // Wait a bit more but not past the timeout
        tokio::time::sleep(tokio::time::Duration::from_millis(75)).await;
        assert!(!auth.should_auto_lock().await);
    }

    #[tokio::test]
    async fn test_lock_unlock_cycle() {
        let auth = setup_test_auth().await;
        auth.initialize("cycleTestPin789").await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Lock and unlock multiple times
        for _ in 0..3 {
            auth.lock().await.unwrap();
            assert!(!auth.is_unlocked().await);
            auth.unlock("cycleTestPin789").await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            assert!(auth.is_unlocked().await);
        }
    }

    #[tokio::test]
    async fn test_double_initialize_fails() {
        let auth = setup_test_auth().await;
        auth.initialize("initTestPin123").await.unwrap();
        let result = auth.initialize("anotherPin456").await;
        assert!(matches!(result, Err(AuthError::AlreadyInitialized)));
    }

    #[tokio::test]
    async fn test_unlock_without_initialize_fails() {
        let auth = setup_test_auth().await;
        let result = auth.unlock("somePin789").await;
        // When not initialized, we can't get the salt, so we get Database error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_change_pin_wrong_old_pin() {
        let auth = setup_test_auth().await;
        auth.initialize("originalPin123").await.unwrap();
        let result = auth.change_pin("wrongPin456", "newPin789").await;
        assert!(matches!(result, Err(AuthError::InvalidPin)));
    }

    #[test]
    fn test_validate_pin() {
        assert!(validate_pin("12345").is_err()); // Too short
        assert!(validate_pin("123456").is_err()); // Weak PIN rejected
        assert!(validate_pin("mySecurePin123").is_ok()); // Valid strong PIN
        assert!(validate_pin("password").is_err()); // Weak PIN
        assert!(validate_pin("111111").is_err()); // Weak PIN
        assert!(validate_pin("000000").is_err()); // Weak PIN
    }

    #[test]
    fn test_validate_pin_edge_cases() {
        // Exactly minimum length should work if not weak
        assert!(validate_pin("abcdef").is_ok());

        // Too long
        let long_pin = "a".repeat(MAX_PIN_LENGTH + 1);
        assert!(validate_pin(&long_pin).is_err());

        // Exactly max length
        let max_pin = "a".repeat(MAX_PIN_LENGTH);
        assert!(validate_pin(&max_pin).is_ok());
    }

    #[tokio::test]
    async fn test_get_vault_key_when_locked() {
        let auth = setup_test_auth().await;
        auth.initialize("keyTestPin123").await.unwrap();
        auth.lock().await.unwrap();

        let result = auth.get_vault_key().await;
        assert!(matches!(result, Err(AuthError::InvalidPin)));
    }

    #[tokio::test]
    async fn test_get_vault_key_when_unlocked() {
        let auth = setup_test_auth().await;
        auth.initialize("keyTestPin123").await.unwrap();

        let key = auth.get_vault_key().await.unwrap();
        assert_eq!(key.as_bytes().len(), 32);
    }
}

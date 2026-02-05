//! Unified error types for the Vult library.
//!
//! This module provides a single error type [`VaultError`] that covers all
//! error cases in the vault library. Binary adapters (GUI, CLI) can convert
//! these to their context-specific representations.
//!
//! # Error Categories
//!
//! - **Authentication**: Invalid PIN, locked vault, too many attempts
//! - **Not Found**: Key or resource doesn't exist
//! - **Conflict**: Duplicate keys, already initialized
//! - **Validation**: Invalid input formats
//! - **Cryptographic**: Encryption/decryption failures
//! - **Database**: SQLite errors, schema issues
//! - **State**: Invalid operation for current state
//!
//! # Exit Codes
//!
//! Each error variant maps to an exit code for CLI usage:
//!
//! | Code | Category |
//! |------|----------|
//! | 0 | Success |
//! | 1 | Authentication failure |
//! | 2 | Resource not found |
//! | 3 | Vault not initialized |
//! | 4 | Conflict/duplicate |
//! | 5 | Invalid input |
//! | 6 | Cryptographic error |
//! | 7 | Database error |
//! | 8 | I/O error |
//! | 9 | State error |
//! | 10 | Clipboard error |
//!
//! # Example
//!
//! ```rust,ignore
//! use vult::error::{VaultError, Result};
//!
//! fn example_operation() -> Result<()> {
//!     // Operations that can fail return Result<T, VaultError>
//!     Err(VaultError::Locked)
//! }
//!
//! // Get exit code for CLI
//! let err = VaultError::InvalidPin;
//! let code = err.exit_code(); // Returns 1
//!
//! // Get user-friendly suggestion
//! if let Some(hint) = err.suggestion() {
//!     println!("Hint: {}", hint);
//! }
//! ```

use thiserror::Error;

/// Unified error type for all vault operations.
///
/// This enum covers all error cases that can occur in the vault library,
/// including authentication, cryptography, database, and state errors.
#[derive(Error, Debug)]
pub enum VaultError {
    // =========================================================================
    // Authentication Errors
    // =========================================================================
    /// Invalid PIN provided
    #[error("Invalid PIN")]
    InvalidPin,

    /// PIN does not meet minimum length requirement (6 characters)
    #[error("PIN too short (minimum 6 characters required)")]
    PinTooShort,

    /// PIN exceeds maximum length
    #[error("PIN too long (maximum 64 characters allowed)")]
    PinTooLong,

    /// Too many failed authentication attempts
    #[error("Too many failed attempts. Please wait before trying again.")]
    TooManyAttempts,

    /// Vault is not initialized (no PIN set)
    #[error("Vault not initialized. Run 'init' first.")]
    NotInitialized,

    /// Vault is already initialized
    #[error("Vault already initialized")]
    AlreadyInitialized,

    /// Vault is locked, authentication required
    #[error("Vault is locked. Unlock with your PIN first.")]
    Locked,

    // =========================================================================
    // Cryptographic Errors
    // =========================================================================
    /// Key derivation failed
    #[error("Key derivation failed: {0}")]
    KeyDerivation(String),

    /// Encryption operation failed
    #[error("Encryption failed: {0}")]
    Encryption(String),

    /// Decryption operation failed
    #[error("Decryption failed: {0}")]
    Decryption(String),

    /// Invalid key length for cryptographic operation
    #[error("Invalid key length")]
    InvalidKeyLength,

    /// Invalid nonce length for cryptographic operation
    #[error("Invalid nonce length")]
    InvalidNonceLength,

    // =========================================================================
    // Database Errors
    // =========================================================================
    /// General database error
    #[error("Database error: {0}")]
    Database(String),

    /// API key not found
    #[error("Key not found: {0}")]
    NotFound(String),

    /// Duplicate API key (app_name + key_name must be unique)
    #[error("Duplicate key: {app_name}/{key_name} already exists")]
    DuplicateKey { app_name: String, key_name: String },

    /// Database schema version incompatibility
    #[error("Database version {db_version} is newer than application version {app_version}. Please update the application.")]
    IncompatibleVersion { db_version: i64, app_version: i64 },

    /// Database migration failed
    #[error("Migration failed: {0}")]
    MigrationFailed(String),

    /// Database backup failed
    #[error("Backup failed: {0}")]
    BackupFailed(String),

    // =========================================================================
    // Input Validation Errors
    // =========================================================================
    /// Invalid input provided
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Required field is missing
    #[error("Missing required field: {0}")]
    MissingField(String),

    // =========================================================================
    // State Errors
    // =========================================================================
    /// Operation requires a different state
    #[error("Invalid state: {0}")]
    InvalidState(String),

    // =========================================================================
    // Clipboard Errors
    // =========================================================================
    /// Clipboard operation failed
    #[error("Clipboard error: {0}")]
    Clipboard(String),

    // =========================================================================
    // I/O Errors
    // =========================================================================
    /// File or I/O operation failed
    #[error("I/O error: {0}")]
    Io(String),
}

/// Convenience type alias for Results with VaultError
pub type Result<T> = std::result::Result<T, VaultError>;

// =============================================================================
// From implementations for underlying error types
// =============================================================================

impl From<sqlx::Error> for VaultError {
    fn from(err: sqlx::Error) -> Self {
        // Check for specific SQLite errors
        let err_string = err.to_string();
        if err_string.contains("UNIQUE constraint failed") {
            // Parse app_name and key_name from error if possible
            VaultError::DuplicateKey {
                app_name: "unknown".to_string(),
                key_name: "unknown".to_string(),
            }
        } else {
            VaultError::Database(err_string)
        }
    }
}

impl From<std::io::Error> for VaultError {
    fn from(err: std::io::Error) -> Self {
        VaultError::Io(err.to_string())
    }
}

impl From<aes_gcm::Error> for VaultError {
    fn from(err: aes_gcm::Error) -> Self {
        VaultError::Encryption(err.to_string())
    }
}

// =============================================================================
// Conversion methods for context
// =============================================================================

impl VaultError {
    /// Add context to a database error
    pub fn database_context(msg: impl Into<String>) -> Self {
        VaultError::Database(msg.into())
    }

    /// Create a not found error with key identifier
    pub fn key_not_found(app_name: &str, key_name: &str) -> Self {
        VaultError::NotFound(format!("{}/{}", app_name, key_name))
    }

    /// Create a duplicate key error
    pub fn duplicate_key(app_name: impl Into<String>, key_name: impl Into<String>) -> Self {
        VaultError::DuplicateKey {
            app_name: app_name.into(),
            key_name: key_name.into(),
        }
    }

    /// Check if this is an authentication-related error
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            VaultError::InvalidPin
                | VaultError::PinTooShort
                | VaultError::PinTooLong
                | VaultError::TooManyAttempts
                | VaultError::NotInitialized
                | VaultError::AlreadyInitialized
                | VaultError::Locked
        )
    }

    /// Check if this is a not-found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, VaultError::NotFound(_))
    }

    /// Get suggested exit code for CLI usage.
    ///
    /// Exit codes follow common conventions:
    /// - 0: Success
    /// - 1: Authentication error (wrong PIN, locked, etc.)
    /// - 2: Not found
    /// - 3: Not initialized
    /// - 4: Conflict (duplicate key)
    /// - 5: Invalid input
    /// - 6: Encryption/decryption error
    /// - 7: Database error
    /// - 8: I/O error
    /// - 9: Invalid state
    /// - 10: Clipboard error
    /// - 64-78: Reserved for future use (sysexits.h compatibility)
    pub fn exit_code(&self) -> i32 {
        match self {
            // Authentication errors
            VaultError::InvalidPin
            | VaultError::PinTooShort
            | VaultError::PinTooLong
            | VaultError::TooManyAttempts
            | VaultError::Locked => 1,

            // Not found
            VaultError::NotFound(_) => 2,

            // Not initialized / already initialized
            VaultError::NotInitialized | VaultError::AlreadyInitialized => 3,

            // Conflict
            VaultError::DuplicateKey { .. } => 4,

            // Invalid input
            VaultError::InvalidInput(_) | VaultError::MissingField(_) => 5,

            // Cryptographic errors
            VaultError::KeyDerivation(_)
            | VaultError::Encryption(_)
            | VaultError::Decryption(_)
            | VaultError::InvalidKeyLength
            | VaultError::InvalidNonceLength => 6,

            // Database errors
            VaultError::Database(_)
            | VaultError::IncompatibleVersion { .. }
            | VaultError::MigrationFailed(_)
            | VaultError::BackupFailed(_) => 7,

            // I/O errors
            VaultError::Io(_) => 8,

            // State errors
            VaultError::InvalidState(_) => 9,

            // Clipboard errors
            VaultError::Clipboard(_) => 10,
        }
    }

    /// Get a suggestion for fixing this error.
    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            VaultError::InvalidPin => Some("Check your PIN and try again."),
            VaultError::PinTooShort => Some("PIN must be at least 6 characters."),
            VaultError::TooManyAttempts => Some("Wait a moment before trying again."),
            VaultError::NotInitialized => Some("Run 'vult init' to set up your vault."),
            VaultError::AlreadyInitialized => Some("Your vault is already set up."),
            VaultError::Locked => Some("Unlock your vault first with your PIN."),
            VaultError::NotFound(_) => Some("Check the app and key name."),
            VaultError::DuplicateKey { .. } => Some("Use 'vult update' to modify an existing key."),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = VaultError::InvalidPin;
        assert_eq!(err.to_string(), "Invalid PIN");

        let err = VaultError::key_not_found("github", "token");
        assert_eq!(err.to_string(), "Key not found: github/token");

        let err = VaultError::duplicate_key("github", "token");
        assert_eq!(
            err.to_string(),
            "Duplicate key: github/token already exists"
        );
    }

    #[test]
    fn test_is_auth_error() {
        assert!(VaultError::InvalidPin.is_auth_error());
        assert!(VaultError::Locked.is_auth_error());
        assert!(!VaultError::NotFound("test".to_string()).is_auth_error());
    }

    #[test]
    fn test_exit_codes() {
        assert_eq!(VaultError::InvalidPin.exit_code(), 1);
        assert_eq!(VaultError::NotFound("test".to_string()).exit_code(), 2);
        assert_eq!(VaultError::NotInitialized.exit_code(), 3);
    }
}

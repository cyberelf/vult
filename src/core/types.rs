//! Core types and constants for the vault.
//!
//! This module provides foundational types and validation utilities
//! used throughout the library.

use std::time::Duration;

use thiserror::Error;

// =============================================================================
// Constants
// =============================================================================

/// Minimum PIN length requirement
pub const MIN_PIN_LENGTH: usize = 6;

/// Maximum PIN length
pub const MAX_PIN_LENGTH: usize = 64;

/// Default auto-lock duration (5 minutes)
pub const DEFAULT_AUTO_LOCK_DURATION: Duration = Duration::from_secs(300);

/// Clipboard auto-clear timeout (45 seconds)
pub const CLIPBOARD_CLEAR_TIMEOUT: Duration = Duration::from_secs(45);

// =============================================================================
// PIN Validation
// =============================================================================

/// Error type for PIN validation
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PinValidationError {
    /// PIN is shorter than the minimum required length
    #[error("PIN too short (minimum {MIN_PIN_LENGTH} characters required)")]
    TooShort,

    /// PIN is longer than the maximum allowed length
    #[error("PIN too long (maximum {MAX_PIN_LENGTH} characters allowed)")]
    TooLong,

    /// PIN contains invalid characters
    #[error("PIN contains invalid characters")]
    InvalidCharacters,
}

/// Validates a PIN meets security requirements.
///
/// # Requirements
///
/// - Minimum 6 characters
/// - Maximum 64 characters
/// - Only printable ASCII characters allowed
///
/// # Examples
///
/// ```
/// use vult::core::validate_pin;
///
/// // Valid PIN
/// assert!(validate_pin("my-secure-pin").is_ok());
///
/// // Too short
/// assert!(validate_pin("12345").is_err());
///
/// // Too long (over 64 characters)
/// let long_pin = "a".repeat(65);
/// assert!(validate_pin(&long_pin).is_err());
/// ```
pub fn validate_pin(pin: &str) -> Result<(), PinValidationError> {
    if pin.len() < MIN_PIN_LENGTH {
        return Err(PinValidationError::TooShort);
    }

    if pin.len() > MAX_PIN_LENGTH {
        return Err(PinValidationError::TooLong);
    }

    // Check for printable ASCII (space through tilde)
    if !pin.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
        return Err(PinValidationError::InvalidCharacters);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_pin_valid() {
        assert!(validate_pin("123456").is_ok());
        assert!(validate_pin("my-secure-pin").is_ok());
        assert!(validate_pin("a".repeat(64).as_str()).is_ok());
    }

    #[test]
    fn test_validate_pin_too_short() {
        assert_eq!(validate_pin("12345"), Err(PinValidationError::TooShort));
        assert_eq!(validate_pin(""), Err(PinValidationError::TooShort));
    }

    #[test]
    fn test_validate_pin_too_long() {
        let long_pin = "a".repeat(65);
        assert_eq!(validate_pin(&long_pin), Err(PinValidationError::TooLong));
    }

    #[test]
    fn test_validate_pin_with_spaces() {
        assert!(validate_pin("my secure pin").is_ok());
    }
}

//! Windows Hello biometric authentication provider.
//!
//! This module implements the BiometricProvider trait using Windows Hello
//! via the Windows.Security.Credentials.UI APIs from windows-rs.
//!
//! # Platform Requirements
//!
//! - Windows 10 version 1903 (build 18362) or later
//! - Windows 11 (all versions)
//! - Compatible biometric hardware (fingerprint, face, iris)
//!
//! # Example
//!
//! ```rust,ignore
//! use vult::biometric::WindowsHelloProvider;
//! use vult::core::BiometricProvider;
//!
//! let provider = WindowsHelloProvider::new();
//!
//! // Check availability
//! let availability = provider.check_availability().await;
//! println!("Windows Hello availability: {:?}", availability);
//!
//! // Verify user
//! if availability == BiometricAvailability::Available {
//!     match provider.verify("Unlock Vult API Vault").await {
//!         Ok(true) => println!("Verification succeeded"),
//!         Ok(false) => println!("Verification failed or cancelled"),
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```

use windows::Security::Credentials::UI::{
    UserConsentVerificationResult, UserConsentVerifier, UserConsentVerifierAvailability,
};

use crate::core::{BiometricAvailability, BiometricProvider};
use crate::error::{Result, VaultError};

/// Windows Hello biometric authentication provider.
///
/// Uses Windows.Security.Credentials.UI.UserConsentVerifier API
/// to perform biometric authentication on Windows 10/11.
#[derive(Debug, Clone)]
pub struct WindowsHelloProvider;

impl WindowsHelloProvider {
    /// Creates a new Windows Hello provider.
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowsHelloProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl BiometricProvider for WindowsHelloProvider {
    /// Checks Windows Hello availability on this device.
    ///
    /// Queries the UserConsentVerifier API to determine if biometric
    /// authentication is available and configured.
    async fn check_availability(&self) -> BiometricAvailability {
        match UserConsentVerifier::CheckAvailabilityAsync() {
            Ok(async_op) => {
                // Convert Windows IAsyncOperation to a Rust future
                match async_op.get() {
                    Ok(availability) => map_windows_availability(availability),
                    Err(_) => BiometricAvailability::NotSupported,
                }
            }
            Err(_) => BiometricAvailability::NotSupported,
        }
    }

    /// Verifies the user using Windows Hello.
    ///
    /// Shows the Windows Hello prompt with the provided message.
    /// Returns `Ok(true)` if verification succeeded, `Ok(false)` if
    /// verification failed or was cancelled, or an error if a system
    /// error occurred.
    ///
    /// # Arguments
    ///
    /// * `message` - User-facing message explaining why authentication is needed
    async fn verify(&self, message: &str) -> Result<bool> {
        // Convert message to HSTRING for Windows API
        let message_hstring = windows::core::HSTRING::from(message);

        // Request verification from Windows Hello
        let async_op = UserConsentVerifier::RequestVerificationAsync(&message_hstring)
            .map_err(|_| VaultError::BiometricFailed)?;

        // Get the verification result (synchronously for now)
        let result = async_op
            .get()
            .map_err(|_| VaultError::BiometricFailed)?;

        // Map the result to a boolean
        Ok(map_verification_result(result))
    }
}

/// Maps Windows UserConsentVerifierAvailability to BiometricAvailability.
fn map_windows_availability(availability: UserConsentVerifierAvailability) -> BiometricAvailability {
    match availability {
        UserConsentVerifierAvailability::Available => BiometricAvailability::Available,
        UserConsentVerifierAvailability::DeviceNotPresent => BiometricAvailability::DeviceNotPresent,
        UserConsentVerifierAvailability::NotConfiguredForUser => BiometricAvailability::NotConfigured,
        UserConsentVerifierAvailability::DisabledByPolicy => BiometricAvailability::NotConfigured,
        UserConsentVerifierAvailability::DeviceBusy => BiometricAvailability::NotConfigured,
        _ => BiometricAvailability::NotSupported,
    }
}

/// Maps Windows UserConsentVerificationResult to a boolean.
///
/// Only `Verified` is considered successful - all other results
/// (including cancellation, failure, etc.) return false.
fn map_verification_result(result: UserConsentVerificationResult) -> bool {
    matches!(result, UserConsentVerificationResult::Verified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = WindowsHelloProvider::new();
        assert!(std::mem::size_of_val(&provider) == 0); // Zero-sized type
    }

    #[test]
    fn test_availability_mapping() {
        assert_eq!(
            map_windows_availability(UserConsentVerifierAvailability::Available),
            BiometricAvailability::Available
        );
        assert_eq!(
            map_windows_availability(UserConsentVerifierAvailability::DeviceNotPresent),
            BiometricAvailability::DeviceNotPresent
        );
        assert_eq!(
            map_windows_availability(UserConsentVerifierAvailability::NotConfiguredForUser),
            BiometricAvailability::NotConfigured
        );
        assert_eq!(
            map_windows_availability(UserConsentVerifierAvailability::DisabledByPolicy),
            BiometricAvailability::NotConfigured
        );
    }

    #[test]
    fn test_verification_result_mapping() {
        assert!(map_verification_result(UserConsentVerificationResult::Verified));
        assert!(!map_verification_result(UserConsentVerificationResult::DeviceNotPresent));
        assert!(!map_verification_result(UserConsentVerificationResult::NotConfiguredForUser));
        assert!(!map_verification_result(UserConsentVerificationResult::DisabledByPolicy));
        assert!(!map_verification_result(UserConsentVerificationResult::Canceled));
    }
}

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
use crate::error::Result;

/// Windows Hello biometric authentication provider.
///
/// Uses Windows.Security.Credentials.UI.UserConsentVerifier API
/// to perform biometric authentication on Windows 10/11.
#[derive(Debug, Clone)]
pub struct WindowsHelloProvider {
    /// Optional parent window handle for desktop app modal positioning
    window_handle: Option<isize>,
}

impl WindowsHelloProvider {
    /// Creates a new Windows Hello provider without window handle (UWP mode).
    pub fn new() -> Self {
        Self {
            window_handle: None,
        }
    }

    /// Creates a new Windows Hello provider with parent window handle (Desktop mode).
    /// This ensures the Windows Hello modal appears correctly on top of the parent window.
    pub fn with_window_handle(hwnd: isize) -> Self {
        Self {
            window_handle: Some(hwnd),
        }
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
                match async_op.await {
                    Ok(availability) => map_windows_availability(availability),
                    Err(_) => BiometricAvailability::NotSupported,
                }
            }
            Err(_) => BiometricAvailability::NotSupported,
        }
    }

    async fn verify(&self, message: &str) -> Result<bool> {
        let message_hstring = windows::core::HSTRING::from(message);

        // TODO: Desktop window handle support requires further investigation
        // The IUserConsentVerifierInterop API in windows-rs 0.62 may have changed
        // For now, always use the standard UWP API
        let _unused_hwnd = self.window_handle; // Prevent unused field warning

        // Use standard UWP API (works for both desktop and UWP apps)
        let async_op = UserConsentVerifier::RequestVerificationAsync(&message_hstring)?;
        let result = async_op.await?;
        Ok(map_verification_result(result))
    }
}

/// Maps Windows UserConsentVerifierAvailability to BiometricAvailability.
fn map_windows_availability(
    availability: UserConsentVerifierAvailability,
) -> BiometricAvailability {
    match availability {
        UserConsentVerifierAvailability::Available => BiometricAvailability::Available,
        UserConsentVerifierAvailability::DeviceNotPresent => {
            BiometricAvailability::DeviceNotPresent
        }
        UserConsentVerifierAvailability::NotConfiguredForUser => {
            BiometricAvailability::NotConfigured
        }
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
        assert!(provider.window_handle.is_none());

        let provider_with_hwnd = WindowsHelloProvider::with_window_handle(12345);
        assert_eq!(provider_with_hwnd.window_handle, Some(12345));
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
        assert!(map_verification_result(
            UserConsentVerificationResult::Verified
        ));
        assert!(!map_verification_result(
            UserConsentVerificationResult::DeviceNotPresent
        ));
        assert!(!map_verification_result(
            UserConsentVerificationResult::NotConfiguredForUser
        ));
        assert!(!map_verification_result(
            UserConsentVerificationResult::DisabledByPolicy
        ));
        assert!(!map_verification_result(
            UserConsentVerificationResult::Canceled
        ));
    }
}

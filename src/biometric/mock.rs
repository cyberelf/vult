//! Mock biometric provider for testing
//!
//! This module provides a mock implementation of BiometricProvider
//! for use in tests, allowing controlled simulation of biometric
//! authentication scenarios without accessing real hardware.

use crate::core::{BiometricAvailability, BiometricProvider};
use crate::error::Result;
use std::sync::{Arc, Mutex};

/// Mock biometric provider for testing.
///
/// Allows controlling availability and verification results
/// to test different authentication scenarios.
#[derive(Debug, Clone)]
pub struct MockBiometricProvider {
    state: Arc<Mutex<MockState>>,
}

#[derive(Debug, Clone)]
struct MockState {
    /// The availability status to return
    availability: BiometricAvailability,
    /// Whether verification should succeed
    should_verify: bool,
    /// Number of times verify was called
    verify_call_count: usize,
}

impl MockBiometricProvider {
    /// Creates a new mock provider with default settings
    /// (available, verification succeeds).
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MockState {
                availability: BiometricAvailability::Available,
                should_verify: true,
                verify_call_count: 0,
            })),
        }
    }

    /// Creates a mock provider that returns the given availability status.
    pub fn with_availability(availability: BiometricAvailability) -> Self {
        Self {
            state: Arc::new(Mutex::new(MockState {
                availability,
                should_verify: true,
                verify_call_count: 0,
            })),
        }
    }

    /// Sets the availability status to return.
    pub fn set_availability(&self, availability: BiometricAvailability) {
        if let Ok(mut state) = self.state.lock() {
            state.availability = availability;
        }
    }

    /// Sets whether verification should succeed.
    pub fn set_should_verify(&self, should_verify: bool) {
        if let Ok(mut state) = self.state.lock() {
            state.should_verify = should_verify;
        }
    }

    /// Gets the number of times verify was called.
    pub fn verify_call_count(&self) -> usize {
        self.state.lock().map(|s| s.verify_call_count).unwrap_or(0)
    }

    /// Resets the verify call count.
    pub fn reset_call_count(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.verify_call_count = 0;
        }
    }
}

impl Default for MockBiometricProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl BiometricProvider for MockBiometricProvider {
    async fn check_availability(&self) -> BiometricAvailability {
        self.state
            .lock()
            .map(|s| s.availability.clone())
            .unwrap_or(BiometricAvailability::NotSupported)
    }

    async fn verify(&self, _message: &str) -> Result<bool> {
        if let Ok(mut state) = self.state.lock() {
            state.verify_call_count += 1;
            Ok(state.should_verify)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider_default() {
        let provider = MockBiometricProvider::new();
        assert_eq!(
            provider.check_availability().await,
            BiometricAvailability::Available
        );
        assert!(provider.verify("test").await.unwrap());
    }

    #[tokio::test]
    async fn test_mock_provider_availability() {
        let provider = MockBiometricProvider::with_availability(BiometricAvailability::NotConfigured);
        assert_eq!(
            provider.check_availability().await,
            BiometricAvailability::NotConfigured
        );

        provider.set_availability(BiometricAvailability::DeviceNotPresent);
        assert_eq!(
            provider.check_availability().await,
            BiometricAvailability::DeviceNotPresent
        );
    }

    #[tokio::test]
    async fn test_mock_provider_verification() {
        let provider = MockBiometricProvider::new();
        
        // Should succeed by default
        assert!(provider.verify("test").await.unwrap());
        assert_eq!(provider.verify_call_count(), 1);

        // Set to fail
        provider.set_should_verify(false);
        assert!(!provider.verify("test").await.unwrap());
        assert_eq!(provider.verify_call_count(), 2);

        // Reset count
        provider.reset_call_count();
        assert_eq!(provider.verify_call_count(), 0);
    }
}

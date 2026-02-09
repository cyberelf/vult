// Only run these tests when the windows-biometric feature is enabled
#![cfg(feature = "windows-biometric")]

//! Integration tests for biometric authentication.
//!
//! Tests biometric availability detection and unlock flows
//! with fallback to PIN authentication.

use std::path::PathBuf;
use tempfile::TempDir;
use vult::core::{BiometricAvailability, BiometricProvider};
use vult::services::VaultManager;

#[cfg(test)]
use vult::biometric::MockBiometricProvider;

/// Helper to create a test vault
fn setup_test_vault() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_vault.db");
    (temp_dir, db_path)
}

#[tokio::test]
async fn test_biometric_availability_detection() {
    let (_temp_dir, db_path) = setup_test_vault();
    let mock_provider = MockBiometricProvider::new();

    // Test different availability states
    let test_cases = vec![
        BiometricAvailability::Available,
        BiometricAvailability::NotConfigured,
        BiometricAvailability::DeviceNotPresent,
        BiometricAvailability::NotSupported,
    ];

    for expected_availability in test_cases {
        mock_provider.set_availability(expected_availability.clone());

        // Check availability via the mock provider
        let availability = mock_provider.check_availability().await;
        assert_eq!(
            availability, expected_availability,
            "Availability mismatch for {:?}",
            expected_availability
        );
    }
}

#[tokio::test]
async fn test_biometric_unlock_success() {
    let (_temp_dir, db_path) = setup_test_vault();
    let mock_provider = MockBiometricProvider::new();

    // Initialize vault
    let mut manager = VaultManager::new(db_path.to_str().unwrap())
        .await
        .unwrap();
    manager.auth().init_vault("test123456").await.unwrap();

    // Configure mock to succeed
    mock_provider.set_availability(BiometricAvailability::Available);
    mock_provider.set_should_verify(true);

    // Simulate biometric verification
    let result: Result<bool, _> = mock_provider.verify("Unlock Vult").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);

    // Verify the provider was called
    assert_eq!(mock_provider.verify_call_count(), 1);
}

#[tokio::test]
async fn test_biometric_unlock_failure() {
    let (_temp_dir, db_path) = setup_test_vault();
    let mock_provider = MockBiometricProvider::new();

    // Initialize vault
    let mut manager = VaultManager::new(db_path.to_str().unwrap())
        .await
        .unwrap();
    manager.auth().init_vault("test123456").await.unwrap();

    // Configure mock to fail verification
    mock_provider.set_availability(BiometricAvailability::Available);
    mock_provider.set_should_verify(false);

    // Simulate biometric verification - should return false (not verified)
    let result: Result<bool, _> = mock_provider.verify("Unlock Vult").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);

    // Verify the provider was called
    assert_eq!(mock_provider.verify_call_count(), 1);
}

#[tokio::test]
async fn test_biometric_fallback_to_pin() {
    let (_temp_dir, db_path) = setup_test_vault();
    let mock_provider = MockBiometricProvider::new();

    // Initialize vault
    let mut manager = VaultManager::new(db_path.to_str().unwrap())
        .await
        .unwrap();
    let pin = "test123456";
    manager.auth().init_vault(pin).await.unwrap();
    manager.auth().lock().await.unwrap();

    // Configure mock to fail
    mock_provider.set_availability(BiometricAvailability::Available);
    mock_provider.set_should_verify(false);

    // Try biometric verification - should return false
    let biometric_result: Result<bool, _> = mock_provider.verify("Unlock Vult").await;
    assert!(biometric_result.is_ok());
    assert_eq!(biometric_result.unwrap(), false);

    // Now use PIN as fallback - should succeed
    let pin_result = manager.auth().unlock(pin).await;
    assert!(
        pin_result.is_ok(),
        "PIN unlock should succeed after biometric failure"
    );
    assert!(manager.is_unlocked(), "Vault should be unlocked with PIN");
}

#[tokio::test]
async fn test_biometric_not_available_fallback() {
    let (_temp_dir, db_path) = setup_test_vault();
    let mock_provider = MockBiometricProvider::new();

    // Initialize vault
    let mut manager = VaultManager::new(db_path.to_str().unwrap())
        .await
        .unwrap();
    let pin = "test123456";
    manager.auth().init_vault(pin).await.unwrap();
    manager.auth().lock().await.unwrap();

    // Configure mock to be unavailable
    mock_provider.set_availability(BiometricAvailability::NotConfigured);

    // Check availability - should not be available
    let availability = mock_provider.check_availability().await;
    assert_eq!(availability, BiometricAvailability::NotConfigured);

    // Use PIN instead
    let pin_result = manager.auth().unlock(pin).await;
    assert!(
        pin_result.is_ok(),
        "PIN unlock should work when biometric not available"
    );
    assert!(manager.is_unlocked(), "Vault should be unlocked with PIN");
}

#[tokio::test]
async fn test_multiple_biometric_attempts() {
    let (_temp_dir, db_path) = setup_test_vault();
    let mock_provider = MockBiometricProvider::new();

    // Initialize vault
    let mut manager = VaultManager::new(db_path.to_str().unwrap())
        .await
        .unwrap();
    manager.auth().init_vault("test123456").await.unwrap();

    // Configure mock
    mock_provider.set_availability(BiometricAvailability::Available);

    // First attempt - fail
    mock_provider.set_should_verify(false);
    let result1: Result<bool, _> = mock_provider.verify("Unlock Vult").await;
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), false);
    assert_eq!(mock_provider.verify_call_count(), 1);

    // Second attempt - succeed
    mock_provider.set_should_verify(true);
    let result2: Result<bool, _> = mock_provider.verify("Unlock Vult").await;
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), true);
    assert_eq!(mock_provider.verify_call_count(), 2);
}

#[tokio::test]
async fn test_mock_provider_call_counting() {
    let mock_provider = MockBiometricProvider::new();
    
    mock_provider.set_availability(BiometricAvailability::Available);
    mock_provider.set_should_verify(true);

    // Multiple calls
    for i in 1..=5 {
        let _result: Result<bool, _> = mock_provider.verify("test").await;
        assert_eq!(mock_provider.verify_call_count(), i);
    }

    // Reset
    mock_provider.reset_call_count();
    assert_eq!(mock_provider.verify_call_count(), 0);
}

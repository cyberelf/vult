//! Integration tests for biometric availability detection
//!
//! These tests verify that:
//! 1. Feature flag properly gates biometric functionality
//! 2. VaultManager correctly initializes biometric provider
//! 3. AuthService correctly returns availability status

#[cfg(feature = "windows-biometric")]
mod with_biometric_feature {
    use std::sync::Arc;
    use tempfile::TempDir;
    use vult::biometric::WindowsHelloProvider;
    use vult::core::{BiometricAvailability, BiometricProvider};
    use vult::services::VaultManager;

    /// Helper to create a temporary vault database
    async fn create_test_vault() -> (TempDir, Arc<VaultManager>) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

        let vault = Arc::new(VaultManager::new(&db_url).await.unwrap());
        (temp_dir, vault)
    }

    #[tokio::test]
    async fn test_windows_hello_provider_exists() {
        // Verify WindowsHelloProvider can be instantiated
        let provider = WindowsHelloProvider::new();
        let availability = provider.check_availability().await;

        // Should return one of the valid availability states
        match availability {
            BiometricAvailability::Available
            | BiometricAvailability::NotConfigured
            | BiometricAvailability::DeviceNotPresent
            | BiometricAvailability::NotSupported => {
                // Valid result
                println!("Windows Hello availability: {:?}", availability);
            }
        }
    }

    #[tokio::test]
    async fn test_vault_manager_has_biometric_provider() {
        let (_temp_dir, vault) = create_test_vault().await;

        // VaultManager should have been initialized with a biometric provider
        // when windows-biometric feature is enabled
        let availability = vault.auth().check_biometric_availability().await;

        // Should NOT return NotSupported when feature is enabled
        // (unless Windows Hello is truly not available on this system)
        println!("Biometric availability from VaultManager: {:?}", availability);

        // Test that the availability is deterministic
        let availability2 = vault.auth().check_biometric_availability().await;
        assert_eq!(
            format!("{:?}", availability),
            format!("{:?}", availability2),
            "Availability check should be deterministic"
        );
    }

    #[tokio::test]
    async fn test_mock_provider_integration() {
        let (_temp_dir, vault) = create_test_vault().await;

        // The vault should have a biometric provider (real WindowsHelloProvider)
        // Let's verify it returns a sensible result
        let result = vault.auth().check_biometric_availability().await;

        // Log the result for debugging
        eprintln!("Biometric availability result: {:?}", result);

        // The result should be one of the valid enum variants
        match result {
            BiometricAvailability::Available => {
                println!("✓ Windows Hello is available and configured on this system");
            }
            BiometricAvailability::NotConfigured => {
                println!("⚠ Windows Hello hardware exists but is not configured");
            }
            BiometricAvailability::DeviceNotPresent => {
                println!("⚠ No biometric hardware detected");
            }
            BiometricAvailability::NotSupported => {
                panic!("Feature flag is enabled but provider returned NotSupported - this indicates the provider wasn't initialized correctly");
            }
        }
    }

    #[tokio::test]
    async fn test_feature_flag_is_active() {
        // This test only compiles when windows-biometric feature is enabled
        // If this test runs, we know the feature flag is active

        println!("✓ windows-biometric feature flag is ACTIVE");
        
        // Verify we can create a WindowsHelloProvider
        let _provider = WindowsHelloProvider::new();
        println!("✓ WindowsHelloProvider can be instantiated");

        // Verify VaultManager initializes with provider
        let (_temp_dir, vault) = create_test_vault().await;
        let availability = vault.auth().check_biometric_availability().await;
        
        // Log what we got
        println!("Availability from vault: {:?}", availability);
        
        // If feature is enabled, we should NEVER get NotSupported
        // (we might get other states if hardware isn't present, but provider should be set)
        assert_ne!(
            format!("{:?}", availability),
            format!("{:?}", BiometricAvailability::NotSupported),
            "With windows-biometric feature enabled, VaultManager should have a provider"
        );
    }
}

#[cfg(not(feature = "windows-biometric"))]
mod without_biometric_feature {
    use std::sync::Arc;
    use tempfile::TempDir;
    use vult::core::BiometricAvailability;
    use vult::services::VaultManager;

    async fn create_test_vault() -> (TempDir, Arc<VaultManager>) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

        let vault = Arc::new(VaultManager::new(&db_url).await.unwrap());
        (temp_dir, vault)
    }

    #[tokio::test]
    async fn test_no_biometric_when_feature_disabled() {
        let (_temp_dir, vault) = create_test_vault().await;

        // Without the feature flag, should always return NotSupported
        let availability = vault.auth().check_biometric_availability().await;

        assert_eq!(
            format!("{:?}", availability),
            format!("{:?}", BiometricAvailability::NotSupported),
            "Without windows-biometric feature, should return NotSupported"
        );
    }
}

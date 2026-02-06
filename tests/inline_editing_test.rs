//! Inline Editing Tests
//!
//! These tests specifically verify the inline editing functionality,
//! particularly the re-encryption behavior when app_name or key_name changes.

use std::path::PathBuf;
use std::sync::Arc;

use vult::services::{key_service::KeyService, key_service::UpdateKeyRequest};
use vult::database::VaultDb;
use vult::crypto_service::CryptoService;
use vult::auth_service::AuthService;

/// Helper to create test services with a temporary database
async fn setup_test_services(test_name: &str) -> (KeyService, AuthService) {
    let db_path = get_test_db_path(test_name);
    let db = Arc::new(VaultDb::new(&db_path.to_string_lossy()).await.unwrap());
    let crypto = Arc::new(CryptoService::new());
    let auth = Arc::new(AuthService::new(Arc::clone(&db), Arc::clone(&crypto)));

    // Initialize vault
    auth.init_vault("test-pin-123").await.unwrap();

    let key_service = KeyService::new(Arc::clone(&db), Arc::clone(&crypto), Arc::clone(&auth));

    // Clean up the database file after test
    tokio::spawn(async move {
        let _ = std::fs::remove_file(db_path);
    });

    (key_service, auth)
}

/// Helper to get a test database path
fn get_test_db_path(test_name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("vult_test_{}.db", test_name));
    // Remove existing test database if it exists
    let _ = std::fs::remove_file(&path);
    path
}

/// Test re-encryption when app_name changes during inline editing
#[tokio::test]
async fn test_inline_edit_app_name_change() {
    let (service, _auth) = setup_test_services("app_name_change").await;

    // Create a key with app_name "github"
    let id = service
        .create(Some("github"), "token", "ghp_secret123", None, None)
        .await
        .unwrap();

    // Verify the key exists
    let original_key = service.get("github", "token").await.unwrap();
    assert_eq!(original_key.app_name, Some("github".to_string()));
    assert_eq!(original_key.key_name, "token");
    assert_eq!(original_key.key_value, "ghp_secret123");

    // Update app_name to "gitlab" (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(Some("gitlab".to_string())),
        ..Default::default()
    };

    service.update(&id, update_request).await.unwrap();

    // Verify the key can now be accessed with new app_name
    let updated_key = service.get("gitlab", "token").await.unwrap();
    assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old app_name no longer works
    let result = service.get("github", "token").await;
    assert!(result.is_err());
}

/// Test re-encryption when key_name changes during inline editing
#[tokio::test]
async fn test_inline_edit_key_name_change() {
    let (service, _auth) = setup_test_services("key_name_change").await;

    // Create a key with key_name "token"
    let id = service
        .create(Some("github"), "token", "ghp_secret123", None, None)
        .await
        .unwrap();

    // Verify the key exists
    let original_key = service.get("github", "token").await.unwrap();
    assert_eq!(original_key.app_name, Some("github".to_string()));
    assert_eq!(original_key.key_name, "token");
    assert_eq!(original_key.key_value, "ghp_secret123");

    // Update key_name to "personal-token" (simulating inline edit)
    let update_request = UpdateKeyRequest {
        key_name: Some("personal-token".to_string()),
        ..Default::default()
    };

    service.update(&id, update_request).await.unwrap();

    // Verify the key can now be accessed with new key_name
    let updated_key = service.get("github", "personal-token").await.unwrap();
    assert_eq!(updated_key.app_name, Some("github".to_string()));
    assert_eq!(updated_key.key_name, "personal-token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old key_name no longer works
    let result = service.get("github", "token").await;
    assert!(result.is_err());
}

/// Test re-encryption when both app_name and key_name change during inline editing
#[tokio::test]
async fn test_inline_edit_both_name_changes() {
    let (service, _auth) = setup_test_services("both_name_change").await;

    // Create a key with app_name "github" and key_name "token"
    let id = service
        .create(Some("github"), "token", "ghp_secret123", None, None)
        .await
        .unwrap();

    // Verify the key exists
    let original_key = service.get("github", "token").await.unwrap();
    assert_eq!(original_key.app_name, Some("github".to_string()));
    assert_eq!(original_key.key_name, "token");
    assert_eq!(original_key.key_value, "ghp_secret123");

    // Update both app_name and key_name (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(Some("gitlab".to_string())),
        key_name: Some("access-token".to_string()),
        ..Default::default()
    };

    service.update(&id, update_request).await.unwrap();

    // Verify the key can now be accessed with new names
    let updated_key = service.get("gitlab", "access-token").await.unwrap();
    assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
    assert_eq!(updated_key.key_name, "access-token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old names no longer work
    let result1 = service.get("github", "token").await;
    assert!(result1.is_err());

    let result2 = service.get("github", "access-token").await;
    assert!(result2.is_err());

    let result3 = service.get("gitlab", "token").await;
    assert!(result3.is_err());
}

/// Test re-encryption when app_name changes to None
#[tokio::test]
async fn test_inline_edit_app_name_to_none() {
    let (service, _auth) = setup_test_services("app_name_to_none").await;

    // Create a key with app_name "github"
    let id = service
        .create(Some("github"), "token", "ghp_secret123", None, None)
        .await
        .unwrap();

    // Update app_name to None (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(None),
        ..Default::default()
    };

    service.update(&id, update_request).await.unwrap();

    // Verify the key can be accessed with no app_name
    let updated_key = service.get("", "token").await.unwrap();
    assert!(updated_key.app_name.is_none());
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old app_name no longer works
    let result = service.get("github", "token").await;
    assert!(result.is_err());
}

/// Test re-encryption when app_name changes from None to Some
#[tokio::test]
async fn test_inline_edit_from_no_app_name() {
    let (service, _auth) = setup_test_services("no_app_name").await;

    // Create a key with no app_name
    let id = service
        .create(None, "token", "ghp_secret123", None, None)
        .await
        .unwrap();

    // Update app_name to "gitlab" (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(Some("gitlab".to_string())),
        ..Default::default()
    };

    service.update(&id, update_request).await.unwrap();

    // Verify the key can be accessed with new app_name
    let updated_key = service.get("gitlab", "token").await.unwrap();
    assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old access method no longer works
    let result = service.get("", "token").await;
    assert!(result.is_err());
}

/// Test re-encryption with value change and app_name change
#[tokio::test]
async fn test_inline_edit_value_and_app_name_change() {
    let (service, _auth) = setup_test_services("value_app_change").await;

    // Create a key with app_name "github" and value "old_value"
    let id = service
        .create(Some("github"), "token", "old_value", None, None)
        .await
        .unwrap();

    // Update both app_name and value (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(Some("gitlab".to_string())),
        key_value: Some("new_value".to_string()),
        ..Default::default()
    };

    service.update(&id, update_request).await.unwrap();

    // Verify the key can be accessed with new app_name and has new value
    let updated_key = service.get("gitlab", "token").await.unwrap();
    assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "new_value");

    // Verify old app_name no longer works
    let result = service.get("github", "token").await;
    assert!(result.is_err());
}

/// Test error handling when updating non-existent key
#[tokio::test]
async fn test_inline_edit_non_existent_key() {
    let (service, _auth) = setup_test_services("non_existent").await;

    // Try to update a non-existent key
    let update_request = UpdateKeyRequest {
        key_name: Some("new-name".to_string()),
        ..Default::default()
    };

    let result = service.update("non-existent-id", update_request).await;
    assert!(result.is_err());
}

/// Test partial update - only description (no re-encryption needed)
#[tokio::test]
async fn test_inline_edit_metadata_only() {
    let (service, _auth) = setup_test_services("metadata_only").await;

    // Create a key with description
    let id = service
        .create(Some("github"), "token", "ghp_secret123", None, Some("Old description"))
        .await
        .unwrap();

    // Update only description (should not trigger re-encryption)
    let update_request = UpdateKeyRequest {
        description: Some(Some("New description".to_string())),
        ..Default::default()
    };

    service.update(&id, update_request).await.unwrap();

    // Verify only description changed
    let updated_key = service.get("github", "token").await.unwrap();
    assert_eq!(updated_key.app_name, Some("github".to_string()));
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "ghp_secret123");
    assert_eq!(updated_key.description, Some("New description".to_string()));
}
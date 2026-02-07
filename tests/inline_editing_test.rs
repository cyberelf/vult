//! Inline Editing Tests
//!
//! These tests specifically verify the inline editing functionality,
//! particularly the re-encryption behavior when app_name or key_name changes.

use std::path::PathBuf;

use vult::services::{VaultManager, key_service::UpdateKeyRequest};

/// Helper to create test vault with a temporary database
async fn setup_test_vault(test_name: &str) -> (VaultManager, PathBuf) {
    let db_path = get_test_db_path(test_name);
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

    let vault = VaultManager::new(&db_url)
        .await
        .expect("Failed to create vault");

    // Initialize vault
    vault.auth().init_vault("test-pin-123")
        .await
        .expect("Failed to init vault");

    // Unlock vault for testing
    vault.auth().unlock("test-pin-123")
        .await
        .expect("Failed to unlock vault");

    (vault, db_path)
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
    let (vault, db_path) = setup_test_vault("app_name_change").await;

    // Create a key with app_name "github"
    let id = vault.keys()
        .create(Some("github"), "token", "ghp_secret123", None, None)
        .await
        .expect("Failed to create key");

    // Verify the key exists
    let original_key = vault.keys().get("github", "token").await.expect("Key not found");
    assert_eq!(original_key.app_name, Some("github".to_string()));
    assert_eq!(original_key.key_name, "token");
    assert_eq!(original_key.key_value, "ghp_secret123");

    // Update app_name to "gitlab" (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(Some("gitlab".to_string())),
        ..Default::default()
    };

    vault.keys().update(&id, update_request).await.expect("Failed to update key");

    // Verify the key can now be accessed with new app_name
    let updated_key = vault.keys().get("gitlab", "token").await.expect("Key not found");
    assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old app_name no longer works
    let result = vault.keys().get("github", "token").await;
    assert!(result.is_err());

    // Cleanup
    let _ = std::fs::remove_file(db_path);
}

/// Test re-encryption when key_name changes during inline editing
#[tokio::test]
async fn test_inline_edit_key_name_change() {
    let (vault, db_path) = setup_test_vault("key_name_change").await;

    // Create a key with key_name "token"
    let id = vault.keys()
        .create(Some("github"), "token", "ghp_secret123", None, None)
        .await
        .expect("Failed to create key");

    // Verify the key exists
    let original_key = vault.keys().get("github", "token").await.expect("Key not found");
    assert_eq!(original_key.app_name, Some("github".to_string()));
    assert_eq!(original_key.key_name, "token");
    assert_eq!(original_key.key_value, "ghp_secret123");

    // Update key_name to "personal-token" (simulating inline edit)
    let update_request = UpdateKeyRequest {
        key_name: Some("personal-token".to_string()),
        ..Default::default()
    };

    vault.keys().update(&id, update_request).await.expect("Failed to update key");

    // Verify the key can now be accessed with new key_name
    let updated_key = vault.keys().get("github", "personal-token").await.expect("Key not found");
    assert_eq!(updated_key.app_name, Some("github".to_string()));
    assert_eq!(updated_key.key_name, "personal-token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old key_name no longer works
    let result = vault.keys().get("github", "token").await;
    assert!(result.is_err());

    // Cleanup
    let _ = std::fs::remove_file(db_path);
}

/// Test re-encryption when both app_name and key_name change during inline editing
#[tokio::test]
async fn test_inline_edit_both_name_changes() {
    let (vault, db_path) = setup_test_vault("both_name_change").await;

    // Create a key with app_name "github" and key_name "token"
    let id = vault.keys()
        .create(Some("github"), "token", "ghp_secret123", None, None)
        .await
        .expect("Failed to create key");

    // Verify the key exists
    let original_key = vault.keys().get("github", "token").await.expect("Key not found");
    assert_eq!(original_key.app_name, Some("github".to_string()));
    assert_eq!(original_key.key_name, "token");
    assert_eq!(original_key.key_value, "ghp_secret123");

    // Update both app_name and key_name (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(Some("gitlab".to_string())),
        key_name: Some("access-token".to_string()),
        ..Default::default()
    };

    vault.keys().update(&id, update_request).await.expect("Failed to update key");

    // Verify the key can now be accessed with new names
    let updated_key = vault.keys().get("gitlab", "access-token").await.expect("Key not found");
    assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
    assert_eq!(updated_key.key_name, "access-token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old names no longer work
    let result1 = vault.keys().get("github", "token").await;
    assert!(result1.is_err());

    let result2 = vault.keys().get("github", "access-token").await;
    assert!(result2.is_err());

    let result3 = vault.keys().get("gitlab", "token").await;
    assert!(result3.is_err());

    // Cleanup
    let _ = std::fs::remove_file(db_path);
}

/// Test re-encryption when app_name changes to None
#[tokio::test]
async fn test_inline_edit_app_name_to_none() {
    let (vault, db_path) = setup_test_vault("app_name_to_none").await;

    // Create a key with app_name "github"
    let id = vault.keys()
        .create(Some("github"), "token", "ghp_secret123", None, None)
        .await
        .expect("Failed to create key");

    // Update app_name to None (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(None),
        ..Default::default()
    };

    vault.keys().update(&id, update_request).await.expect("Failed to update key");

    // Verify the key can be accessed with no app_name
    let updated_key = vault.keys().get("", "token").await.expect("Key not found");
    assert!(updated_key.app_name.is_none());
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old app_name no longer works
    let result = vault.keys().get("github", "token").await;
    assert!(result.is_err());

    // Cleanup
    let _ = std::fs::remove_file(db_path);
}

/// Test re-encryption when app_name changes from None to Some
#[tokio::test]
async fn test_inline_edit_from_no_app_name() {
    let (vault, db_path) = setup_test_vault("no_app_name").await;

    // Create a key with no app_name
    let id = vault.keys()
        .create(None, "token", "ghp_secret123", None, None)
        .await
        .expect("Failed to create key");

    // Update app_name to "gitlab" (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(Some("gitlab".to_string())),
        ..Default::default()
    };

    vault.keys().update(&id, update_request).await.expect("Failed to update key");

    // Verify the key can be accessed with new app_name
    let updated_key = vault.keys().get("gitlab", "token").await.expect("Key not found");
    assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "ghp_secret123");

    // Verify old access method no longer works
    let result = vault.keys().get("", "token").await;
    assert!(result.is_err());

    // Cleanup
    let _ = std::fs::remove_file(db_path);
}

/// Test re-encryption with value change and app_name change
#[tokio::test]
async fn test_inline_edit_value_and_app_name_change() {
    let (vault, db_path) = setup_test_vault("value_app_change").await;

    // Create a key with app_name "github" and value "old_value"
    let id = vault.keys()
        .create(Some("github"), "token", "old_value", None, None)
        .await
        .expect("Failed to create key");

    // Update both app_name and value (simulating inline edit)
    let update_request = UpdateKeyRequest {
        app_name: Some(Some("gitlab".to_string())),
        key_value: Some("new_value".to_string()),
        ..Default::default()
    };

    vault.keys().update(&id, update_request).await.expect("Failed to update key");

    // Verify the key can be accessed with new app_name and has new value
    let updated_key = vault.keys().get("gitlab", "token").await.expect("Key not found");
    assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "new_value");

    // Verify old app_name no longer works
    let result = vault.keys().get("github", "token").await;
    assert!(result.is_err());

    // Cleanup
    let _ = std::fs::remove_file(db_path);
}

/// Test error handling when updating non-existent key
#[tokio::test]
async fn test_inline_edit_non_existent_key() {
    let (vault, db_path) = setup_test_vault("non_existent").await;

    // Try to update a non-existent key
    let update_request = UpdateKeyRequest {
        key_name: Some("new-name".to_string()),
        ..Default::default()
    };

    let result = vault.keys().update("non-existent-id", update_request).await;
    assert!(result.is_err());

    // Cleanup
    let _ = std::fs::remove_file(db_path);
}

/// Test partial update - only description (no re-encryption needed)
#[tokio::test]
async fn test_inline_edit_metadata_only() {
    let (vault, db_path) = setup_test_vault("metadata_only").await;

    // Create a key with description
    let id = vault.keys()
        .create(Some("github"), "token", "ghp_secret123", None, Some("Old description"))
        .await
        .expect("Failed to create key");

    // Update only description (should not trigger re-encryption)
    let update_request = UpdateKeyRequest {
        description: Some(Some("New description".to_string())),
        ..Default::default()
    };

    vault.keys().update(&id, update_request).await.expect("Failed to update key");

    // Verify only description changed
    let updated_key = vault.keys().get("github", "token").await.expect("Key not found");
    assert_eq!(updated_key.app_name, Some("github".to_string()));
    assert_eq!(updated_key.key_name, "token");
    assert_eq!(updated_key.key_value, "ghp_secret123");
    assert_eq!(updated_key.description, Some("New description".to_string()));

    // Cleanup
    let _ = std::fs::remove_file(db_path);
}
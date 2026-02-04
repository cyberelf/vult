//! Integration tests for auto-lock functionality
//!
//! These tests verify the complete auto-lock flow including:
//! - Activity counter incrementing
//! - Auto-lock triggering after timeout
//! - Activity updates resetting the timer
//! - Lock/unlock state transitions

use std::sync::Arc;
use std::time::Duration;
use vult::auth::AuthManager;
use vult::database::VaultDb;

#[tokio::test]
async fn test_auto_lock_integration_flow() {
    // Create vault with 2-second timeout for fast testing
    let db = Arc::new(VaultDb::new("sqlite::memory:").await.unwrap());
    let auth = Arc::new(AuthManager::new(
        db,
        Some(Duration::from_secs(2)),
    ));

    // Start the activity counter background task
    auth.start_activity_counter();

    // Initialize and unlock
    auth.initialize("integrationTest123").await.unwrap();
    assert!(auth.is_unlocked().await, "Vault should be unlocked after initialization");

    // Wait for timeout
    tokio::time::sleep(Duration::from_millis(2500)).await;

    // Should be ready for auto-lock
    assert!(auth.should_auto_lock().await, "Should be ready for auto-lock after timeout");

    // Actually lock it
    auth.lock().await.unwrap();
    assert!(!auth.is_unlocked().await, "Vault should be locked");
}

#[tokio::test]
async fn test_activity_prevents_auto_lock_integration() {
    let db = Arc::new(VaultDb::new("sqlite::memory:").await.unwrap());
    let auth = Arc::new(AuthManager::new(
        db,
        Some(Duration::from_secs(3)),
    ));

    auth.start_activity_counter();
    auth.initialize("activityPreventTest").await.unwrap();

    // Simulate user activity every 2 seconds for 8 seconds
    for _ in 0..4 {
        tokio::time::sleep(Duration::from_secs(2)).await;
        auth.update_activity().await;
        assert!(!auth.should_auto_lock().await, "Activity should prevent auto-lock");
    }

    // Now stop activity and wait for timeout
    tokio::time::sleep(Duration::from_secs(4)).await;
    assert!(auth.should_auto_lock().await, "Should auto-lock after inactivity");
}

#[tokio::test]
async fn test_lock_stops_counter_integration() {
    let db = Arc::new(VaultDb::new("sqlite::memory:").await.unwrap());
    let auth = Arc::new(AuthManager::new(
        db,
        Some(Duration::from_secs(5)),
    ));

    auth.start_activity_counter();
    auth.initialize("lockStopsCounter").await.unwrap();

    // Let counter increment
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    let counter_before_lock = {
        let state = auth.get_session_state().await;
        state.last_activity_secs
    };
    assert!(counter_before_lock >= 1, "Counter should have incremented");

    // Lock the vault
    auth.lock().await.unwrap();

    // Counter should be reset
    let state = auth.get_session_state().await;
    assert_eq!(state.last_activity_secs, 0, "Counter should be reset to 0 when locked");

    // Wait and verify counter doesn't increment while locked
    tokio::time::sleep(Duration::from_secs(2)).await;
    let state = auth.get_session_state().await;
    assert_eq!(state.last_activity_secs, 0, "Counter should not increment while locked");
}

#[tokio::test]
async fn test_unlock_restarts_counter_integration() {
    let db = Arc::new(VaultDb::new("sqlite::memory:").await.unwrap());
    let auth = Arc::new(AuthManager::new(
        db,
        Some(Duration::from_secs(5)),
    ));

    auth.start_activity_counter();
    auth.initialize("unlockRestartsCounter").await.unwrap();

    // Let counter increment a bit
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Lock the vault
    auth.lock().await.unwrap();
    
    // Counter should be reset to 0 when locked
    let state_after_lock = auth.get_session_state().await;
    assert_eq!(state_after_lock.last_activity_secs, 0, "Counter should be 0 after lock");
    
    // Unlock the vault
    auth.unlock("unlockRestartsCounter").await.unwrap();

    // Counter should restart from 0 after unlock
    // Wait a bit for the counter to increment
    tokio::time::sleep(Duration::from_secs(2)).await;
    let state = auth.get_session_state().await;
    assert!(state.is_unlocked, "Vault should be unlocked");
    // Counter should be around 2 seconds (give some tolerance for execution time)
    assert!(state.last_activity_secs >= 1 && state.last_activity_secs <= 4,
        "Counter should be between 1-4 seconds after unlock, got {}", state.last_activity_secs);
}

#[tokio::test]
async fn test_long_running_counter() {
    let db = Arc::new(VaultDb::new("sqlite::memory:").await.unwrap());
    let auth = Arc::new(AuthManager::new(
        db,
        Some(Duration::from_secs(60)),
    ));

    auth.start_activity_counter();
    auth.initialize("longRunningCounter").await.unwrap();

    // Wait for counter to increment several times
    tokio::time::sleep(Duration::from_secs(5)).await;
    let state = auth.get_session_state().await;
    
    // Counter should have incremented to around 5 seconds
    assert!(state.is_unlocked, "Vault should be unlocked");
    assert!(state.last_activity_secs >= 4 && state.last_activity_secs <= 6,
        "Counter should be around 5 seconds, got {}", state.last_activity_secs);
}

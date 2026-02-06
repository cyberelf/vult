//! Vult GUI Application - Tauri Desktop Interface
//!
//! This is the graphical user interface for Vult, built with Tauri.
//! For command-line usage, see the `vult` binary.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::Arc;

use vult::clipboard::ClipboardManager;
use vult::gui::{commands, AuthManager};
use vult::services::VaultManager;

/// Get the default vault database path.
///
/// On Linux: ~/.vult/vault.db
/// On Windows: %USERPROFILE%\.vult\vault.db
fn get_vault_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".vult");
    std::fs::create_dir_all(&path).ok();
    path.push("vault.db");
    path
}

#[tokio::main]
async fn main() {
    // Initialize the vault database
    let vault_path = get_vault_path();

    // Ensure the directory exists
    if let Some(parent) = vault_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create .vult directory");
    }

    // Try different path formats for Windows
    let vault_str = vault_path.to_str().expect("Invalid path");
    let db_path = format!("sqlite://{}?mode=rwc", vault_str.replace('\\', "/"));

    eprintln!("Database path: {}", db_path);

    // Initialize VaultManager - the main entry point for vault operations
    let vault = Arc::new(
        VaultManager::new(&db_path)
            .await
            .expect("Failed to initialize vault"),
    );

    // Initialize authentication manager with 5-minute auto-lock
    // This wraps VaultManager and adds GUI-specific features (auto-lock, events)
    let auth_manager = Arc::new(AuthManager::new(
        Arc::clone(&vault),
        Some(tokio::time::Duration::from_secs(300)),
    ));

    // Start activity counter for auto-lock
    auth_manager.start_activity_counter();

    // Initialize clipboard manager
    let clipboard_manager = Arc::new(ClipboardManager::new().unwrap());
    clipboard_manager.start_auto_clear_checker();

    // Clone auth_manager before passing to Tauri
    let auth_for_setup = Arc::clone(&auth_manager);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(auth_manager)
        .manage(clipboard_manager)
        .invoke_handler(tauri::generate_handler![
            commands::init_vault,
            commands::unlock_vault,
            commands::lock_vault,
            commands::get_auth_state,
            commands::is_initialized,
            commands::change_pin,
            commands::create_api_key,
            commands::get_api_key,
            commands::list_api_keys,
            commands::search_api_keys,
            commands::update_api_key,
            commands::delete_api_key,
            commands::copy_to_clipboard,
            commands::update_activity,
            commands::check_auto_lock,
        ])
        .setup(move |app| {
            // Start auto-lock checker with app handle for event emission
            let auth_clone = Arc::clone(&auth_for_setup);
            let app_handle = app.handle().clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
                loop {
                    interval.tick().await;
                    if auth_clone.should_auto_lock().await {
                        let _ = auth_clone.lock_with_event(&app_handle).await;
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

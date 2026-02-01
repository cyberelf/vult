// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod clipboard;
mod commands;
mod crypto;
mod database;

use auth::AuthManager;
use clipboard::ClipboardManager;
use database::VaultDb;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;

fn get_vault_path() -> PathBuf {
    let mut path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push(".vult");
    std::fs::create_dir_all(&path).ok();
    path.push("vault.db");
    path
}

#[tokio::main]
async fn main() {
    // Initialize the vault database
    let vault_path = get_vault_path();
    let db_path = format!("sqlite:{}", vault_path.display());
    let db = Arc::new(
        VaultDb::new(&db_path)
            .await
            .expect("Failed to initialize vault database"),
    );

    // Initialize authentication manager
    let auth_manager = Arc::new(AuthManager::new(
        Arc::clone(&db),
        Some(tokio::time::Duration::from_secs(300)), // 5 minutes
    ));

    // Start activity counter for auto-lock
    auth_manager.start_activity_counter();

    // Initialize clipboard manager
    let clipboard_manager = Arc::new(ClipboardManager::new().unwrap());
    clipboard_manager.start_auto_clear_checker();

    // Start auto-lock checker
    let auth_clone = Arc::clone(&auth_manager);
    let app_handle_clone = std::sync::Arc::new(std::sync::Mutex::new(None::<tauri::AppHandle>));
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            if auth_clone.should_auto_lock().await {
                // Emit event to frontend to trigger lock UI
                if let Some(handle) = app_handle_clone.lock().unwrap().as_ref() {
                    let _ = handle.emit_all("vault-auto-locked", ());
                }
                let _ = auth_clone.lock().await;
            }
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(db)
        .manage(auth_manager)
        .manage(clipboard_manager)
        .invoke_handler(tauri::generate_handler![
            commands::init_vault,
            commands::unlock_vault,
            commands::lock_vault,
            commands::get_auth_state,
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
        .setup(|app| {
            let handle = app.handle().clone();
            if let Ok(mut guard) = app_handle_clone.lock() {
                *guard = Some(handle);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//! Tauri commands for the vault
//!
//! These commands are called from the frontend.

use crate::auth::{AuthManager, SessionState, validate_pin};
use crate::database::{CreateApiKey, UpdateApiKey, VaultDb, ApiKey, ApiKeyWithSecret};
use crate::crypto::VaultKey;
use crate::clipboard::ClipboardManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// Response type for commands
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> CommandResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

/// Authentication state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub is_initialized: bool,
    pub is_unlocked: bool,
}

/// Initializes the vault with a new PIN
#[tauri::command]
pub async fn init_vault(
    pin: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<()>, String> {
    // Validate PIN
    validate_pin(&pin).map_err(|e| e.to_string())?;

    auth_manager
        .initialize(&pin)
        .await
        .map_err(|e| e.to_string())?;

    Ok(CommandResponse::success(()))
}

/// Unlocks the vault with a PIN
#[tauri::command]
pub async fn unlock_vault(
    pin: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<()>, String> {
    auth_manager.unlock(&pin).await.map_err(|e| e.to_string())?;
    Ok(CommandResponse::success(()))
}

/// Locks the vault
#[tauri::command]
pub async fn lock_vault(
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<()>, String> {
    auth_manager.lock().await.map_err(|e| e.to_string())?;
    Ok(CommandResponse::success(()))
}

/// Gets the current authentication state
#[tauri::command]
pub async fn get_auth_state(
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<AuthState>, String> {
    let is_initialized = auth_manager.is_initialized().await.map_err(|e| e.to_string())?;
    let is_unlocked = auth_manager.is_unlocked().await;

    Ok(CommandResponse::success(AuthState {
        is_initialized,
        is_unlocked,
    }))
}

/// Changes the PIN
#[tauri::command]
pub async fn change_pin(
    old_pin: String,
    new_pin: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<()>, String> {
    validate_pin(&new_pin).map_err(|e| e.to_string())?;
    auth_manager
        .change_pin(&old_pin, &new_pin)
        .await
        .map_err(|e| e.to_string())?;
    Ok(CommandResponse::success(()))
}

/// Creates a new API key
#[tauri::command]
pub async fn create_api_key(
    input: CreateApiKey,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
    db: tauri::State<'_, Arc<VaultDb>>,
) -> Result<CommandResponse<ApiKeyWithSecret>, String> {
    let key = auth_manager.get_vault_key().await.map_err(|e| e.to_string())?;
    let result = db
        .create_api_key(input, &key)
        .await
        .map_err(|e| e.to_string())?;
    Ok(CommandResponse::success(result))
}

/// Gets an API key by ID
#[tauri::command]
pub async fn get_api_key(
    id: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
    db: tauri::State<'_, Arc<VaultDb>>,
) -> Result<CommandResponse<ApiKeyWithSecret>, String> {
    let key = auth_manager.get_vault_key().await.map_err(|e| e.to_string())?;
    let result = db.get_api_key(&id, &key).await.map_err(|e| e.to_string())?;
    Ok(CommandResponse::success(result))
}

/// Lists all API keys
#[tauri::command]
pub async fn list_api_keys(
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
    db: tauri::State<'_, Arc<VaultDb>>,
) -> Result<CommandResponse<Vec<ApiKey>>, String> {
    // Update activity to prevent auto-lock
    auth_manager.update_activity().await;
    let result = db.list_api_keys().await.map_err(|e| e.to_string())?;
    Ok(CommandResponse::success(result))
}

/// Searches API keys
#[tauri::command]
pub async fn search_api_keys(
    query: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
    db: tauri::State<'_, Arc<VaultDb>>,
) -> Result<CommandResponse<Vec<ApiKey>>, String> {
    auth_manager.update_activity().await;
    let result = db
        .search_api_keys(&query)
        .await
        .map_err(|e| e.to_string())?;
    Ok(CommandResponse::success(result))
}

/// Updates an API key
#[tauri::command]
pub async fn update_api_key(
    input: UpdateApiKey,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
    db: tauri::State<'_, Arc<VaultDb>>,
) -> Result<CommandResponse<ApiKeyWithSecret>, String> {
    let key = auth_manager.get_vault_key().await.map_err(|e| e.to_string())?;
    let result = db
        .update_api_key(input, &key)
        .await
        .map_err(|e| e.to_string())?;
    Ok(CommandResponse::success(result))
}

/// Deletes an API key
#[tauri::command]
pub async fn delete_api_key(
    id: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
    db: tauri::State<'_, Arc<VaultDb>>,
) -> Result<CommandResponse<()>, String> {
    let key = auth_manager.get_vault_key().await.map_err(|e| e.to_string())?;
    db.delete_api_key(&id, &key).await.map_err(|e| e.to_string())?;
    Ok(CommandResponse::success(()))
}

/// Copies an API key to clipboard with auto-clear
#[tauri::command]
pub async fn copy_to_clipboard(
    id: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
    db: tauri::State<'_, Arc<VaultDb>>,
    clipboard: tauri::State<'_, Arc<ClipboardManager>>,
) -> Result<CommandResponse<String>, String> {
    let key = auth_manager.get_vault_key().await.map_err(|e| e.to_string())?;
    let api_key = db.get_api_key(&id, &key).await.map_err(|e| e.to_string())?;

    clipboard
        .copy_with_timeout(api_key.key_value.clone(), Duration::from_secs(30))
        .await;

    Ok(CommandResponse::success(api_key.key_value))
}

/// Updates user activity (prevents auto-lock)
#[tauri::command]
pub async fn update_activity(
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<()>, String> {
    auth_manager.update_activity().await;
    Ok(CommandResponse::success(()))
}

/// Checks if auto-lock should trigger
#[tauri::command]
pub async fn check_auto_lock(
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<bool>, String> {
    let should_lock = auth_manager.should_auto_lock().await;
    Ok(CommandResponse::success(should_lock))
}

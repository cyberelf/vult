//! Tauri commands for the vault
//!
//! These commands are thin adapters that call VaultManager services.
//! They handle the Tauri IPC layer and convert between library types
//! and frontend-friendly formats.

use super::auth_manager::AuthManager;
use crate::clipboard::ClipboardManager;
use crate::core::validate_pin;
use crate::database::{ApiKey, ApiKeyWithSecret, CreateApiKey, UpdateApiKey};
use crate::services::key_service::UpdateKeyRequest;
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

// =============================================================================
// Authentication Commands
// =============================================================================

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
    let is_initialized = auth_manager
        .is_initialized()
        .await
        .map_err(|e| e.to_string())?;
    let is_unlocked = auth_manager.is_unlocked().await;

    Ok(CommandResponse::success(AuthState {
        is_initialized,
        is_unlocked,
    }))
}

/// Checks if the vault has been initialized
#[tauri::command]
pub async fn is_initialized(
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<bool, String> {
    auth_manager
        .is_initialized()
        .await
        .map_err(|e| e.to_string())
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

// =============================================================================
// Key Management Commands (using VaultManager.keys() service)
// =============================================================================

/// Creates a new API key
#[tauri::command]
pub async fn create_api_key(
    input: CreateApiKey,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<ApiKeyWithSecret>, String> {
    auth_manager.update_activity().await;

    // Create the key and get the ID
    let id = auth_manager
        .vault()
        .keys()
        .create(
            input.app_name.as_deref(),
            &input.key_name,
            &input.key_value,
            input.api_url.as_deref(),
            input.description.as_deref(),
        )
        .await
        .map_err(|e| e.to_string())?;

    // Fetch the created key to return full details
    let key = auth_manager
        .vault()
        .keys()
        .get_by_id(&id)
        .await
        .map_err(|e| e.to_string())?;

    // Convert to ApiKeyWithSecret for frontend compatibility
    let result = ApiKeyWithSecret {
        api_key: ApiKey {
            id: key.id,
            app_name: key.app_name,
            key_name: key.key_name,
            api_url: key.api_url,
            description: key.description,
            created_at: key.created_at,
            updated_at: key.updated_at,
        },
        key_value: key.key_value,
    };

    Ok(CommandResponse::success(result))
}

/// Gets an API key by ID
#[tauri::command]
pub async fn get_api_key(
    id: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<ApiKeyWithSecret>, String> {
    auth_manager.update_activity().await;

    let key = auth_manager
        .vault()
        .keys()
        .get_by_id(&id)
        .await
        .map_err(|e| e.to_string())?;

    let result = ApiKeyWithSecret {
        api_key: ApiKey {
            id: key.id,
            app_name: key.app_name,
            key_name: key.key_name,
            api_url: key.api_url,
            description: key.description,
            created_at: key.created_at,
            updated_at: key.updated_at,
        },
        key_value: key.key_value,
    };

    Ok(CommandResponse::success(result))
}

/// Lists all API keys
#[tauri::command]
pub async fn list_api_keys(
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<Vec<ApiKey>>, String> {
    auth_manager.update_activity().await;

    let keys = auth_manager
        .vault()
        .keys()
        .list()
        .await
        .map_err(|e| e.to_string())?;

    // Convert ApiKeyMetadata to ApiKey for frontend compatibility
    let result: Vec<ApiKey> = keys
        .into_iter()
        .map(|m| ApiKey {
            id: m.id,
            app_name: m.app_name,
            key_name: m.key_name,
            api_url: m.api_url,
            description: m.description,
            created_at: m.created_at,
            updated_at: m.updated_at,
        })
        .collect();

    Ok(CommandResponse::success(result))
}

/// Searches API keys
#[tauri::command]
pub async fn search_api_keys(
    query: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<Vec<ApiKey>>, String> {
    auth_manager.update_activity().await;

    let keys = auth_manager
        .vault()
        .keys()
        .search(&query)
        .await
        .map_err(|e| e.to_string())?;

    let result: Vec<ApiKey> = keys
        .into_iter()
        .map(|m| ApiKey {
            id: m.id,
            app_name: m.app_name,
            key_name: m.key_name,
            api_url: m.api_url,
            description: m.description,
            created_at: m.created_at,
            updated_at: m.updated_at,
        })
        .collect();

    Ok(CommandResponse::success(result))
}

/// Updates an API key
#[tauri::command]
pub async fn update_api_key(
    input: UpdateApiKey,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<ApiKeyWithSecret>, String> {
    auth_manager.update_activity().await;

    // Build the update request from the frontend input
    // For partial updates: frontend sends None for fields to keep unchanged
    let request = UpdateKeyRequest {
        app_name: input.app_name.map(Some), // Option<String> -> Option<Option<String>>
        key_name: input.key_name,
        key_value: input.key_value,
        api_url: input.api_url, // Already Option<Option<String>>
        description: input.description, // Already Option<Option<String>>
    };

    auth_manager
        .vault()
        .keys()
        .update(&input.id, request)
        .await
        .map_err(|e| e.to_string())?;

    // Fetch updated metadata only (no decryption for partial updates)
    let metadata = auth_manager
        .vault()
        .keys()
        .list()
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|k| k.id == input.id)
        .ok_or_else(|| "Key not found after update".to_string())?;

    let result = ApiKeyWithSecret {
        api_key: ApiKey {
            id: metadata.id,
            app_name: metadata.app_name,
            key_name: metadata.key_name,
            api_url: metadata.api_url,
            description: metadata.description,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
        },
        // For metadata updates, return masked value (not decrypted)
        key_value: "••••••••".to_string(),
    };

    Ok(CommandResponse::success(result))
}

/// Deletes an API key
#[tauri::command]
pub async fn delete_api_key(
    id: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
) -> Result<CommandResponse<()>, String> {
    auth_manager.update_activity().await;

    auth_manager
        .vault()
        .keys()
        .delete(&id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(CommandResponse::success(()))
}

// =============================================================================
// Clipboard Commands
// =============================================================================

/// Copies an API key to clipboard with auto-clear
#[tauri::command]
pub async fn copy_to_clipboard(
    id: String,
    auth_manager: tauri::State<'_, Arc<AuthManager>>,
    clipboard: tauri::State<'_, Arc<ClipboardManager>>,
) -> Result<CommandResponse<String>, String> {
    auth_manager.update_activity().await;

    let api_key = auth_manager
        .vault()
        .keys()
        .get_by_id(&id)
        .await
        .map_err(|e| e.to_string())?;

    clipboard
        .copy_with_timeout(api_key.key_value.clone(), Duration::from_secs(30))
        .await;

    Ok(CommandResponse::success(api_key.key_value))
}

// =============================================================================
// Activity Management Commands
// =============================================================================

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

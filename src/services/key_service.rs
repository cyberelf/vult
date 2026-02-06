//! Key management service - API key CRUD operations
//!
//! This service provides high-level operations for managing API keys,
//! with automatic encryption and decryption.
//!
//! # Example
//!
//! ```rust,ignore
//! // Create a key
//! key_service.create("github", "token", "ghp_xxx...", None, None).await?;
//!
//! // List all keys (metadata only)
//! let keys = key_service.list().await?;
//!
//! // Get full key with decrypted value
//! let key = key_service.get("github", "token").await?;
//! println!("Value: {}", key.key_value);
//! ```

use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::crypto::EncryptedData;
use crate::database::VaultDb;
use crate::error::{Result, VaultError};

use super::{AuthService, CryptoService};

/// Complete API key with decrypted value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Unique identifier
    pub id: String,
    /// Application name (e.g., "github", "aws")
    pub app_name: Option<String>,
    /// Key name (e.g., "token", "api-key")
    pub key_name: String,
    /// Decrypted key value
    pub key_value: String,
    /// Optional API URL
    pub api_url: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// API key metadata without the decrypted value.
///
/// Used for listing and searching to avoid decryption overhead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyMetadata {
    /// Unique identifier
    pub id: String,
    /// Application name
    pub app_name: Option<String>,
    /// Key name
    pub key_name: String,
    /// Optional API URL
    pub api_url: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Request to create a new API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKeyRequest {
    /// Application name (optional)
    pub app_name: Option<String>,
    /// Key name (required)
    pub key_name: String,
    /// Key value (required)
    pub key_value: String,
    /// Optional API URL
    pub api_url: Option<String>,
    /// Optional description
    pub description: Option<String>,
}

/// Request to update an existing API key.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateKeyRequest {
    /// New app name (None = keep existing)
    pub app_name: Option<Option<String>>,
    /// New key name (None = keep existing)
    pub key_name: Option<String>,
    /// New key value (None = keep existing, Some = re-encrypt)
    pub key_value: Option<String>,
    /// New API URL (None = keep existing)
    pub api_url: Option<Option<String>>,
    /// New description (None = keep existing)
    pub description: Option<Option<String>>,
}

/// Key management service.
///
/// Provides CRUD operations for API keys with automatic encryption/decryption.
/// Requires the vault to be unlocked for all operations.
pub struct KeyService {
    db: Arc<VaultDb>,
    crypto: Arc<CryptoService>,
    auth: Arc<AuthService>,
}

impl KeyService {
    /// Creates a new key service.
    pub fn new(db: Arc<VaultDb>, crypto: Arc<CryptoService>, auth: Arc<AuthService>) -> Self {
        Self { db, crypto, auth }
    }

    /// Ensures the vault is unlocked before operations.
    async fn require_unlocked(&self) -> Result<()> {
        if !self.auth.is_unlocked_async().await {
            return Err(VaultError::Locked);
        }
        Ok(())
    }

    /// Creates a new API key.
    ///
    /// # Arguments
    ///
    /// * `app_name` - Application name (e.g., "github")
    /// * `key_name` - Key name (e.g., "token")
    /// * `key_value` - The secret key value
    /// * `api_url` - Optional API URL
    /// * `description` - Optional description
    ///
    /// # Returns
    ///
    /// The ID of the created key.
    ///
    /// # Errors
    ///
    /// - [`VaultError::Locked`] if vault is locked
    /// - [`VaultError::DuplicateKey`] if key already exists
    pub async fn create(
        &self,
        app_name: Option<&str>,
        key_name: &str,
        key_value: &str,
        api_url: Option<&str>,
        description: Option<&str>,
    ) -> Result<String> {
        self.require_unlocked().await?;

        let master_key = self.auth.get_vault_key().await?;
        let app_name_str = app_name.unwrap_or("");

        // Encrypt with per-key encryption
        let (encrypted, salt) =
            self.crypto
                .encrypt_api_key(key_value, &master_key, app_name_str, key_name)?;

        // Generate ID
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();

        // Insert into database
        sqlx::query(
            r#"
            INSERT INTO api_keys (id, app_name, key_name, api_url, description, 
                                  encrypted_key_value, nonce, key_salt, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
        )
        .bind(&id)
        .bind(app_name)
        .bind(key_name)
        .bind(api_url)
        .bind(description)
        .bind(&encrypted.ciphertext)
        .bind(&encrypted.nonce)
        .bind(&salt[..])
        .bind(now)
        .bind(now)
        .execute(&self.db.pool)
        .await
        .map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("UNIQUE constraint") {
                VaultError::duplicate_key(app_name.unwrap_or(""), key_name)
            } else {
                VaultError::Database(err_str)
            }
        })?;

        Ok(id)
    }

    /// Gets an API key with its decrypted value.
    ///
    /// # Arguments
    ///
    /// * `app_name` - Application name
    /// * `key_name` - Key name
    ///
    /// # Errors
    ///
    /// - [`VaultError::Locked`] if vault is locked
    /// - [`VaultError::NotFound`] if key doesn't exist
    pub async fn get(&self, app_name: &str, key_name: &str) -> Result<ApiKey> {
        self.require_unlocked().await?;

        let master_key = self.auth.get_vault_key().await?;

        // Query the key
        let row = sqlx::query(
            r#"
            SELECT id, app_name, key_name, api_url, description,
                   encrypted_key_value, nonce, key_salt, created_at, updated_at
            FROM api_keys
            WHERE (app_name = ?1 OR (app_name IS NULL AND ?1 = '')) AND key_name = ?2
            "#,
        )
        .bind(app_name)
        .bind(key_name)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| VaultError::Database(e.to_string()))?
        .ok_or_else(|| VaultError::key_not_found(app_name, key_name))?;

        // Extract fields
        let id: String = row.get("id");
        let db_app_name: Option<String> = row.get("app_name");
        let db_key_name: String = row.get("key_name");
        let api_url: Option<String> = row.get("api_url");
        let description: Option<String> = row.get("description");
        let encrypted_value: Vec<u8> = row.get("encrypted_key_value");
        let nonce: Vec<u8> = row.get("nonce");
        let key_salt: Vec<u8> = row.get("key_salt");
        let created_at: i64 = row.get("created_at");
        let updated_at: i64 = row.get("updated_at");

        // Decrypt the value
        let mut salt_array = [0u8; 32];
        if key_salt.len() == 32 {
            salt_array.copy_from_slice(&key_salt);
        }

        let encrypted = EncryptedData {
            ciphertext: encrypted_value,
            nonce,
        };

        let key_value = self.crypto.decrypt_api_key(
            &encrypted,
            &master_key,
            db_app_name.as_deref().unwrap_or(""),
            &db_key_name,
            &salt_array,
        )?;

        Ok(ApiKey {
            id,
            app_name: db_app_name,
            key_name: db_key_name,
            key_value,
            api_url,
            description,
            created_at: DateTime::from_timestamp(created_at, 0).unwrap_or_default(),
            updated_at: DateTime::from_timestamp(updated_at, 0).unwrap_or_default(),
        })
    }

    /// Gets an API key by ID with its decrypted value.
    pub async fn get_by_id(&self, id: &str) -> Result<ApiKey> {
        self.require_unlocked().await?;

        let master_key = self.auth.get_vault_key().await?;

        let row = sqlx::query(
            r#"
            SELECT id, app_name, key_name, api_url, description,
                   encrypted_key_value, nonce, key_salt, created_at, updated_at
            FROM api_keys
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| VaultError::Database(e.to_string()))?
        .ok_or_else(|| VaultError::NotFound(id.to_string()))?;

        // Extract and decrypt (same as get())
        let db_app_name: Option<String> = row.get("app_name");
        let db_key_name: String = row.get("key_name");
        let api_url: Option<String> = row.get("api_url");
        let description: Option<String> = row.get("description");
        let encrypted_value: Vec<u8> = row.get("encrypted_key_value");
        let nonce: Vec<u8> = row.get("nonce");
        let key_salt: Vec<u8> = row.get("key_salt");
        let created_at: i64 = row.get("created_at");
        let updated_at: i64 = row.get("updated_at");

        let mut salt_array = [0u8; 32];
        if key_salt.len() == 32 {
            salt_array.copy_from_slice(&key_salt);
        }

        let encrypted = EncryptedData {
            ciphertext: encrypted_value,
            nonce,
        };

        let key_value = self.crypto.decrypt_api_key(
            &encrypted,
            &master_key,
            db_app_name.as_deref().unwrap_or(""),
            &db_key_name,
            &salt_array,
        )?;

        Ok(ApiKey {
            id: id.to_string(),
            app_name: db_app_name,
            key_name: db_key_name,
            key_value,
            api_url,
            description,
            created_at: DateTime::from_timestamp(created_at, 0).unwrap_or_default(),
            updated_at: DateTime::from_timestamp(updated_at, 0).unwrap_or_default(),
        })
    }

    /// Lists all API keys (metadata only, no decryption).
    ///
    /// # Returns
    ///
    /// Vector of key metadata without decrypted values.
    pub async fn list(&self) -> Result<Vec<ApiKeyMetadata>> {
        self.require_unlocked().await?;

        let rows = sqlx::query(
            r#"
            SELECT id, app_name, key_name, api_url, description, created_at, updated_at
            FROM api_keys
            ORDER BY app_name, key_name
            "#,
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| VaultError::Database(e.to_string()))?;

        let keys = rows
            .into_iter()
            .map(|row| ApiKeyMetadata {
                id: row.get("id"),
                app_name: row.get("app_name"),
                key_name: row.get("key_name"),
                api_url: row.get("api_url"),
                description: row.get("description"),
                created_at: DateTime::from_timestamp(row.get("created_at"), 0).unwrap_or_default(),
                updated_at: DateTime::from_timestamp(row.get("updated_at"), 0).unwrap_or_default(),
            })
            .collect();

        Ok(keys)
    }

    /// Searches for API keys matching a query.
    ///
    /// Searches in app_name, key_name, and description fields.
    /// Case-insensitive partial matching.
    pub async fn search(&self, query: &str) -> Result<Vec<ApiKeyMetadata>> {
        self.require_unlocked().await?;

        let pattern = format!("%{}%", query);

        let rows = sqlx::query(
            r#"
            SELECT id, app_name, key_name, api_url, description, created_at, updated_at
            FROM api_keys
            WHERE app_name LIKE ?1 OR key_name LIKE ?1 OR description LIKE ?1
            ORDER BY app_name, key_name
            "#,
        )
        .bind(&pattern)
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| VaultError::Database(e.to_string()))?;

        let keys = rows
            .into_iter()
            .map(|row| ApiKeyMetadata {
                id: row.get("id"),
                app_name: row.get("app_name"),
                key_name: row.get("key_name"),
                api_url: row.get("api_url"),
                description: row.get("description"),
                created_at: DateTime::from_timestamp(row.get("created_at"), 0).unwrap_or_default(),
                updated_at: DateTime::from_timestamp(row.get("updated_at"), 0).unwrap_or_default(),
            })
            .collect();

        Ok(keys)
    }

    /// Updates an existing API key.
    ///
    /// Only provided fields are updated. If `key_value` is provided,
    /// or if `app_name`/`key_name` changes, the key will be re-encrypted.
    pub async fn update(&self, id: &str, request: UpdateKeyRequest) -> Result<()> {
        self.require_unlocked().await?;

        // First, get the existing key
        let existing = self.get_by_id(id).await?;

        let now = Utc::now().timestamp();

        // Determine new values (None = keep existing, Some(value) = update)
        let new_app_name = match request.app_name {
            None => existing.app_name.clone(),
            Some(val) => val,
        };
        let new_key_name = request.key_name.unwrap_or(existing.key_name.clone());
        let new_api_url = match request.api_url {
            None => existing.api_url.clone(),
            Some(val) => val,
        };
        let new_description = match request.description {
            None => existing.description.clone(),
            Some(val) => val,
        };

        // Check if app_name or key_name changed (requires re-encryption)
        let app_changed = new_app_name != existing.app_name;
        let key_changed = new_key_name != existing.key_name;

        // Re-encrypt if key_value changed OR if app_name/key_name changed
        let needs_reencrypt = request.key_value.is_some() || app_changed || key_changed;

        if needs_reencrypt {
            // Use the new value if provided, otherwise use existing decrypted value
            let value_to_encrypt = if let Some(new_value) = request.key_value {
                new_value
            } else {
                existing.key_value
            };

            let master_key = self.auth.get_vault_key().await?;
            let (encrypted, salt) = self.crypto.encrypt_api_key(
                &value_to_encrypt,
                &master_key,
                new_app_name.as_deref().unwrap_or(""),
                &new_key_name,
            )?;

            sqlx::query(
                r#"
                UPDATE api_keys
                SET app_name = ?1, key_name = ?2, api_url = ?3, description = ?4,
                    encrypted_key_value = ?5, nonce = ?6, key_salt = ?7, updated_at = ?8
                WHERE id = ?9
                "#,
            )
            .bind(&new_app_name)
            .bind(&new_key_name)
            .bind(&new_api_url)
            .bind(&new_description)
            .bind(&encrypted.ciphertext)
            .bind(&encrypted.nonce)
            .bind(&salt[..])
            .bind(now)
            .bind(id)
            .execute(&self.db.pool)
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;
        } else {
            // Update metadata only (no encryption context changes)
            sqlx::query(
                r#"
                UPDATE api_keys
                SET app_name = ?1, key_name = ?2, api_url = ?3, description = ?4, updated_at = ?5
                WHERE id = ?6
                "#,
            )
            .bind(&new_app_name)
            .bind(&new_key_name)
            .bind(&new_api_url)
            .bind(&new_description)
            .bind(now)
            .bind(id)
            .execute(&self.db.pool)
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;
        }

        Ok(())
    }

    /// Deletes an API key.
    ///
    /// # Returns
    ///
    /// The metadata of the deleted key for confirmation.
    pub async fn delete(&self, id: &str) -> Result<ApiKeyMetadata> {
        self.require_unlocked().await?;

        // First get the key metadata
        let row = sqlx::query(
            r#"
            SELECT id, app_name, key_name, api_url, description, created_at, updated_at
            FROM api_keys
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| VaultError::Database(e.to_string()))?
        .ok_or_else(|| VaultError::NotFound(id.to_string()))?;

        let metadata = ApiKeyMetadata {
            id: row.get("id"),
            app_name: row.get("app_name"),
            key_name: row.get("key_name"),
            api_url: row.get("api_url"),
            description: row.get("description"),
            created_at: DateTime::from_timestamp(row.get("created_at"), 0).unwrap_or_default(),
            updated_at: DateTime::from_timestamp(row.get("updated_at"), 0).unwrap_or_default(),
        };

        // Delete the key
        sqlx::query("DELETE FROM api_keys WHERE id = ?1")
            .bind(id)
            .execute(&self.db.pool)
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;

        Ok(metadata)
    }

    /// Deletes an API key by app_name and key_name.
    pub async fn delete_by_name(&self, app_name: &str, key_name: &str) -> Result<ApiKeyMetadata> {
        self.require_unlocked().await?;

        // First get the key
        let key = self.get(app_name, key_name).await?;
        self.delete(&key.id).await
    }

    /// Counts total number of API keys.
    pub async fn count(&self) -> Result<i64> {
        self.require_unlocked().await?;

        let row = sqlx::query("SELECT COUNT(*) as count FROM api_keys")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;

        Ok(row.get("count"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create fully configured test services
    async fn setup_test_services() -> (KeyService, Arc<AuthService>) {
        let db = Arc::new(VaultDb::new("sqlite::memory:").await.unwrap());
        let crypto = Arc::new(CryptoService::new());
        let auth = Arc::new(AuthService::new(Arc::clone(&db), Arc::clone(&crypto)));

        // Initialize vault
        auth.init_vault("test-pin-123").await.unwrap();

        let key_service = KeyService::new(Arc::clone(&db), Arc::clone(&crypto), Arc::clone(&auth));

        (key_service, auth)
    }

  
    #[tokio::test]
    async fn test_create_key() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(Some("github"), "token", "ghp_test123", None, None)
            .await
            .unwrap();

        assert!(!id.is_empty());
        // ID should be a valid UUID
        assert!(Uuid::parse_str(&id).is_ok());
    }

    #[tokio::test]
    async fn test_create_and_get_key() {
        let (service, _auth) = setup_test_services().await;

        service
            .create(
                Some("github"),
                "token",
                "ghp_secret_value",
                Some("https://api.github.com"),
                Some("My GitHub token"),
            )
            .await
            .unwrap();

        let key = service.get("github", "token").await.unwrap();

        assert_eq!(key.app_name, Some("github".to_string()));
        assert_eq!(key.key_name, "token");
        assert_eq!(key.key_value, "ghp_secret_value");
        assert_eq!(key.api_url, Some("https://api.github.com".to_string()));
        assert_eq!(key.description, Some("My GitHub token".to_string()));
    }

    #[tokio::test]
    async fn test_create_key_without_app_name() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(None, "standalone-key", "secret123", None, None)
            .await
            .unwrap();

        assert!(!id.is_empty());

        let key = service.get_by_id(&id).await.unwrap();
        assert!(key.app_name.is_none());
        assert_eq!(key.key_name, "standalone-key");
    }

    #[tokio::test]
    async fn test_duplicate_key_error() {
        let (service, _auth) = setup_test_services().await;

        service
            .create(Some("github"), "token", "value1", None, None)
            .await
            .unwrap();

        let result = service
            .create(Some("github"), "token", "value2", None, None)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_keys() {
        let (service, _auth) = setup_test_services().await;

        service
            .create(Some("github"), "token", "v1", None, None)
            .await
            .unwrap();
        service
            .create(Some("aws"), "secret", "v2", None, None)
            .await
            .unwrap();
        service
            .create(None, "standalone", "v3", None, None)
            .await
            .unwrap();

        let keys = service.list().await.unwrap();

        assert_eq!(keys.len(), 3);
    }

    #[tokio::test]
    async fn test_list_empty() {
        let (service, _auth) = setup_test_services().await;

        let keys = service.list().await.unwrap();
        assert!(keys.is_empty());
    }

    #[tokio::test]
    async fn test_search_keys() {
        let (service, _auth) = setup_test_services().await;

        service
            .create(
                Some("github"),
                "personal",
                "v1",
                None,
                Some("Personal access token"),
            )
            .await
            .unwrap();
        service
            .create(Some("github"), "work", "v2", None, Some("Work token"))
            .await
            .unwrap();
        service
            .create(Some("aws"), "secret", "v3", None, None)
            .await
            .unwrap();

        let results = service.search("github").await.unwrap();
        assert_eq!(results.len(), 2);

        let results = service.search("personal").await.unwrap();
        assert_eq!(results.len(), 1);

        let results = service.search("token").await.unwrap();
        assert_eq!(results.len(), 2); // matches description "token"
    }

    #[tokio::test]
    async fn test_delete_key() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(Some("github"), "token", "secret", None, None)
            .await
            .unwrap();

        assert!(service.get_by_id(&id).await.is_ok());

        service.delete(&id).await.unwrap();

        let result = service.get_by_id(&id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_by_name() {
        let (service, _auth) = setup_test_services().await;

        service
            .create(Some("github"), "token", "secret", None, None)
            .await
            .unwrap();

        service.delete_by_name("github", "token").await.unwrap();

        let result = service.get("github", "token").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_key_value() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(Some("github"), "token", "old_value", None, None)
            .await
            .unwrap();

        let update = UpdateKeyRequest {
            key_value: Some("new_value".to_string()),
            ..Default::default()
        };

        service.update(&id, update).await.unwrap();

        let key = service.get_by_id(&id).await.unwrap();
        assert_eq!(key.key_value, "new_value");
    }

    #[tokio::test]
    async fn test_count() {
        let (service, _auth) = setup_test_services().await;

        assert_eq!(service.count().await.unwrap(), 0);

        service
            .create(Some("a"), "k1", "v1", None, None)
            .await
            .unwrap();
        assert_eq!(service.count().await.unwrap(), 1);

        service
            .create(Some("b"), "k2", "v2", None, None)
            .await
            .unwrap();
        assert_eq!(service.count().await.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_operations_require_unlocked() {
        let (service, auth) = setup_test_services().await;

        // Create a key while unlocked
        service
            .create(Some("github"), "token", "secret", None, None)
            .await
            .unwrap();

        // Lock the vault
        auth.lock().await.unwrap();

        // All operations should fail
        let result = service.create(Some("new"), "key", "v", None, None).await;
        assert!(result.is_err());

        let result = service.list().await;
        assert!(result.is_err());

        let result = service.get("github", "token").await;
        assert!(result.is_err());
    }

    // Test re-encryption when app_name changes
    #[tokio::test]
    async fn test_update_key_with_app_name_change() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(Some("github"), "token", "ghp_secret123", None, None)
            .await
            .unwrap();

        // Get the original key to verify value remains the same
        let original_key = service.get_by_id(&id).await.unwrap();
        assert_eq!(original_key.app_name, Some("github".to_string()));
        assert_eq!(original_key.key_name, "token");
        assert_eq!(original_key.key_value, "ghp_secret123");

        // Update app_name from "github" to "gitlab"
        let update = UpdateKeyRequest {
            app_name: Some(Some("gitlab".to_string())),
            ..Default::default()
        };

        service.update(&id, update).await.unwrap();

        // Verify the update worked
        let updated_key = service.get_by_id(&id).await.unwrap();
        assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
        assert_eq!(updated_key.key_name, "token");
        assert_eq!(updated_key.key_value, "ghp_secret123"); // Value should remain the same

        // Verify the key can be accessed with new app_name
        let key_by_name = service.get("gitlab", "token").await.unwrap();
        assert_eq!(key_by_name.key_value, "ghp_secret123");

        // Verify old app_name no longer works
        let result = service.get("github", "token").await;
        assert!(result.is_err());
    }

    // Test re-encryption when key_name changes
    #[tokio::test]
    async fn test_update_key_with_key_name_change() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(Some("github"), "token", "ghp_secret123", None, None)
            .await
            .unwrap();

        // Get the original key
        let original_key = service.get_by_id(&id).await.unwrap();
        assert_eq!(original_key.app_name, Some("github".to_string()));
        assert_eq!(original_key.key_name, "token");
        assert_eq!(original_key.key_value, "ghp_secret123");

        // Update key_name from "token" to "access-token"
        let update = UpdateKeyRequest {
            key_name: Some("access-token".to_string()),
            ..Default::default()
        };

        service.update(&id, update).await.unwrap();

        // Verify the update worked
        let updated_key = service.get_by_id(&id).await.unwrap();
        assert_eq!(updated_key.app_name, Some("github".to_string()));
        assert_eq!(updated_key.key_name, "access-token");
        assert_eq!(updated_key.key_value, "ghp_secret123");

        // Verify the key can be accessed with new key_name
        let key_by_name = service.get("github", "access-token").await.unwrap();
        assert_eq!(key_by_name.key_value, "ghp_secret123");

        // Verify old key_name no longer works
        let result = service.get("github", "token").await;
        assert!(result.is_err());
    }

    // Test re-encryption when both app_name and key_name change
    #[tokio::test]
    async fn test_update_key_with_both_name_changes() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(Some("github"), "token", "ghp_secret123", None, None)
            .await
            .unwrap();

        // Get the original key
        let original_key = service.get_by_id(&id).await.unwrap();
        assert_eq!(original_key.app_name, Some("github".to_string()));
        assert_eq!(original_key.key_name, "token");
        assert_eq!(original_key.key_value, "ghp_secret123");

        // Update both app_name and key_name
        let update = UpdateKeyRequest {
            app_name: Some(Some("gitlab".to_string())),
            key_name: Some("access-token".to_string()),
            ..Default::default()
        };

        service.update(&id, update).await.unwrap();

        // Verify the update worked
        let updated_key = service.get_by_id(&id).await.unwrap();
        assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
        assert_eq!(updated_key.key_name, "access-token");
        assert_eq!(updated_key.key_value, "ghp_secret123");

        // Verify the key can be accessed with new names
        let key_by_name = service.get("gitlab", "access-token").await.unwrap();
        assert_eq!(key_by_name.key_value, "ghp_secret123");

        // Verify old names no longer work
        let result1 = service.get("github", "token").await;
        assert!(result1.is_err());

        let result2 = service.get("github", "access-token").await;
        assert!(result2.is_err());

        let result3 = service.get("gitlab", "token").await;
        assert!(result3.is_err());
    }

    // Test re-encryption when key_value changes with app_name
    #[tokio::test]
    async fn test_update_key_value_with_app_name_change() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(Some("github"), "token", "old_secret", None, None)
            .await
            .unwrap();

        // Update both key_value and app_name
        let update = UpdateKeyRequest {
            app_name: Some(Some("gitlab".to_string())),
            key_value: Some("new_secret".to_string()),
            ..Default::default()
        };

        service.update(&id, update).await.unwrap();

        // Verify the update worked
        let updated_key = service.get_by_id(&id).await.unwrap();
        assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
        assert_eq!(updated_key.key_name, "token");
        assert_eq!(updated_key.key_value, "new_secret");

        // Verify the key can be accessed with new app_name and has new value
        let key_by_name = service.get("gitlab", "token").await.unwrap();
        assert_eq!(key_by_name.key_value, "new_secret");
    }

    // Test update with empty app_name change
    #[tokio::test]
    async fn test_update_key_with_app_name_to_none() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(Some("github"), "token", "secret123", None, None)
            .await
            .unwrap();

        // Change app_name from Some("github") to None
        let update = UpdateKeyRequest {
            app_name: Some(None), // This sets app_name to NULL
            ..Default::default()
        };

        service.update(&id, update).await.unwrap();

        // Verify the update worked
        let updated_key = service.get_by_id(&id).await.unwrap();
        assert!(updated_key.app_name.is_none());
        assert_eq!(updated_key.key_name, "token");
        assert_eq!(updated_key.key_value, "secret123");

        // Verify the key can be accessed with no app_name
        let key_by_name = service.get("", "token").await.unwrap();
        assert_eq!(key_by_name.key_value, "secret123");
    }

    // Test update from no app_name to app_name
    #[tokio::test]
    async fn test_update_key_from_no_app_name_to_app_name() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(None, "token", "secret123", None, None)
            .await
            .unwrap();

        // Change app_name from None to Some("gitlab")
        let update = UpdateKeyRequest {
            app_name: Some(Some("gitlab".to_string())),
            ..Default::default()
        };

        service.update(&id, update).await.unwrap();

        // Verify the update worked
        let updated_key = service.get_by_id(&id).await.unwrap();
        assert_eq!(updated_key.app_name, Some("gitlab".to_string()));
        assert_eq!(updated_key.key_name, "token");
        assert_eq!(updated_key.key_value, "secret123");

        // Verify the key can be accessed with new app_name
        let key_by_name = service.get("gitlab", "token").await.unwrap();
        assert_eq!(key_by_name.key_value, "secret123");

        // Verify old access method no longer works
        let result = service.get("", "token").await;
        assert!(result.is_err());
    }

    // Test error handling when updating non-existent key
    #[tokio::test]
    async fn test_update_non_existent_key() {
        let (service, _auth) = setup_test_services().await;

        let update = UpdateKeyRequest {
            key_name: Some("new-name".to_string()),
            ..Default::default()
        };

        let result = service.update("non-existent-id", update).await;
        assert!(result.is_err());
    }

    // Test partial update - only description
    #[tokio::test]
    async fn test_update_key_metadata_only() {
        let (service, _auth) = setup_test_services().await;

        let id = service
            .create(Some("github"), "token", "secret123", None, Some("Old description"))
            .await
            .unwrap();

        // Update only description
        let update = UpdateKeyRequest {
            description: Some(Some("New description".to_string())),
            ..Default::default()
        };

        service.update(&id, update).await.unwrap();

        // Verify only description changed
        let updated_key = service.get_by_id(&id).await.unwrap();
        assert_eq!(updated_key.app_name, Some("github".to_string()));
        assert_eq!(updated_key.key_name, "token");
        assert_eq!(updated_key.key_value, "secret123");
        assert_eq!(updated_key.description, Some("New description".to_string()));
    }
}

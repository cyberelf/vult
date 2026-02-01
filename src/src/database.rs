//! Database operations for the vault
//!
//! Uses SQLite with SQLx for encrypted storage of API keys.

use crate::crypto::{CryptoError, EncryptedData, VaultKey};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, FromRow, Pool, Row, Sqlite};
use thiserror::Error;
use uuid::Uuid;

/// Errors related to database operations
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Crypto error: {0}")]
    Crypto(#[from] CryptoError),

    #[error("API key not found")]
    NotFound,

    #[error("Duplicate API key: {app_name}/{key_name}")]
    Duplicate { app_name: String, key_name: String },

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Result type for database operations
pub type Result<T> = std::result::Result<T, DbError>;

/// Represents an API key stored in the vault
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub app_name: String,
    pub key_name: String,
    pub api_url: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// API key with the decrypted key value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyWithSecret {
    #[serde(flatten)]
    pub api_key: ApiKey,
    pub key_value: String,
}

/// Input for creating a new API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKey {
    pub app_name: String,
    pub key_name: String,
    pub api_url: Option<String>,
    pub description: Option<String>,
    pub key_value: String,
}

/// Input for updating an existing API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApiKey {
    pub id: String,
    pub app_name: Option<String>,
    pub key_name: Option<String>,
    pub api_url: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub key_value: Option<String>,
}

/// Database row for encrypted API key storage
struct EncryptedApiKeyRow {
    id: String,
    app_name: String,
    key_name: String,
    api_url: Option<String>,
    description: Option<String>,
    encrypted_key_value: Vec<u8>,
    nonce: Vec<u8>,
    created_at: i64,
    updated_at: i64,
}

impl FromRow<'_, SqliteRow> for EncryptedApiKeyRow {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            app_name: row.try_get("app_name")?,
            key_name: row.try_get("key_name")?,
            api_url: row.try_get("api_url")?,
            description: row.try_get("description")?,
            encrypted_key_value: row.try_get("encrypted_key_value")?,
            nonce: row.try_get("nonce")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

/// The vault database
pub struct VaultDb {
    pool: Pool<Sqlite>,
}

impl VaultDb {
    /// Creates a new vault database connection pool
    pub async fn new(database_path: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_path).await?;
        let db = Self { pool };
        db.init().await?;
        Ok(db)
    }

    /// Initializes the database schema
    async fn init(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS api_keys (
                id TEXT PRIMARY KEY,
                app_name TEXT NOT NULL,
                key_name TEXT NOT NULL,
                api_url TEXT,
                description TEXT,
                encrypted_key_value BLOB NOT NULL,
                nonce BLOB NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                UNIQUE(app_name, key_name COLLATE NOCASE)
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Creates a new API key with encrypted value
    pub async fn create_api_key(
        &self,
        input: CreateApiKey,
        key: &VaultKey,
    ) -> Result<ApiKeyWithSecret> {
        // Validate inputs
        if input.app_name.trim().is_empty() {
            return Err(DbError::InvalidInput("app_name cannot be empty".to_string()));
        }
        if input.key_name.trim().is_empty() {
            return Err(DbError::InvalidInput("key_name cannot be empty".to_string()));
        }
        if input.key_value.trim().is_empty() {
            return Err(DbError::InvalidInput("key_value cannot be empty".to_string()));
        }

        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Encrypt the key value
        let plaintext = input.key_value.as_bytes();
        let encrypted = crate::crypto::encrypt(plaintext, key)?;

        // Insert into database
        sqlx::query(
            r#"
            INSERT INTO api_keys (id, app_name, key_name, api_url, description, encrypted_key_value, nonce, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
        )
        .bind(&id)
        .bind(&input.app_name)
        .bind(&input.key_name)
        .bind(&input.api_url)
        .bind(&input.description)
        .bind(&encrypted.ciphertext)
        .bind(&encrypted.nonce)
        .bind(now.timestamp())
        .bind(now.timestamp())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                DbError::Duplicate {
                    app_name: input.app_name.clone(),
                    key_name: input.key_name.clone(),
                }
            } else {
                DbError::Database(e)
            }
        })?;

        Ok(ApiKeyWithSecret {
            api_key: ApiKey {
                id,
                app_name: input.app_name,
                key_name: input.key_name,
                api_url: input.api_url,
                description: input.description,
                created_at: now,
                updated_at: now,
            },
            key_value: input.key_value,
        })
    }

    /// Gets an API key by ID with decrypted value
    pub async fn get_api_key(&self, id: &str, key: &VaultKey) -> Result<ApiKeyWithSecret> {
        let row = sqlx::query_as::<_, EncryptedApiKeyRow>(
            "SELECT id, app_name, key_name, api_url, description, encrypted_key_value, nonce, created_at, updated_at FROM api_keys WHERE id = ?1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        self.decrypt_row(row, key)
    }

    /// Lists all API keys (without decrypted values)
    pub async fn list_api_keys(&self) -> Result<Vec<ApiKey>> {
        let rows = sqlx::query_as::<_, EncryptedApiKeyRow>(
            "SELECT id, app_name, key_name, api_url, description, encrypted_key_value, nonce, created_at, updated_at FROM api_keys ORDER BY app_name, key_name"
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(ApiKey {
                    id: row.id,
                    app_name: row.app_name,
                    key_name: row.key_name,
                    api_url: row.api_url,
                    description: row.description,
                    created_at: DateTime::from_timestamp(row.created_at, 0).unwrap(),
                    updated_at: DateTime::from_timestamp(row.updated_at, 0).unwrap(),
                })
            })
            .collect()
    }

    /// Searches API keys by app name, key name, or description
    pub async fn search_api_keys(&self, query: &str) -> Result<Vec<ApiKey>> {
        let pattern = format!("%{}%", query);

        let rows = sqlx::query_as::<_, EncryptedApiKeyRow>(
            r#"
            SELECT id, app_name, key_name, api_url, description, encrypted_key_value, nonce, created_at, updated_at
            FROM api_keys
            WHERE app_name LIKE ?1 OR key_name LIKE ?1 OR description LIKE ?1
            ORDER BY app_name, key_name
            "#
        )
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(ApiKey {
                    id: row.id,
                    app_name: row.app_name,
                    key_name: row.key_name,
                    api_url: row.api_url,
                    description: row.description,
                    created_at: DateTime::from_timestamp(row.created_at, 0).unwrap(),
                    updated_at: DateTime::from_timestamp(row.updated_at, 0).unwrap(),
                })
            })
            .collect()
    }

    /// Updates an existing API key
    pub async fn update_api_key(
        &self,
        input: UpdateApiKey,
        key: &VaultKey,
    ) -> Result<ApiKeyWithSecret> {
        let mut existing = self.get_api_key(&input.id, key).await?;

        let now = Utc::now();

        // Update fields if provided
        if let Some(app_name) = input.app_name {
            existing.api_key.app_name = app_name;
        }
        if let Some(key_name) = input.key_name {
            existing.api_key.key_name = key_name;
        }
        if let Some(api_url) = input.api_url {
            existing.api_key.api_url = api_url;
        }
        if let Some(description) = input.description {
            existing.api_key.description = description;
        }
        if let Some(key_value) = input.key_value {
            existing.key_value = key_value;
        }
        existing.api_key.updated_at = now;

        // Encrypt the new key value
        let encrypted = crate::crypto::encrypt(existing.key_value.as_bytes(), key)?;

        sqlx::query(
            r#"
            UPDATE api_keys
            SET app_name = ?1, key_name = ?2, api_url = ?3, description = ?4,
                encrypted_key_value = ?5, nonce = ?6, updated_at = ?7
            WHERE id = ?8
            "#,
        )
        .bind(&existing.api_key.app_name)
        .bind(&existing.api_key.key_name)
        .bind(&existing.api_key.api_url)
        .bind(&existing.api_key.description)
        .bind(&encrypted.ciphertext)
        .bind(&encrypted.nonce)
        .bind(now.timestamp())
        .bind(&input.id)
        .execute(&self.pool)
        .await?;

        Ok(existing)
    }

    /// Deletes an API key
    pub async fn delete_api_key(&self, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM api_keys WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    /// Decrypts an API key row
    fn decrypt_row(&self, row: EncryptedApiKeyRow, key: &VaultKey) -> Result<ApiKeyWithSecret> {
        let encrypted = EncryptedData {
            ciphertext: row.encrypted_key_value,
            nonce: row.nonce,
        };

        let decrypted = crate::crypto::decrypt(&encrypted, key)?;
        let key_value = String::from_utf8(decrypted)
            .map_err(|_| DbError::Crypto(CryptoError::Decryption("Invalid UTF-8".to_string())))?;

        Ok(ApiKeyWithSecret {
            api_key: ApiKey {
                id: row.id,
                app_name: row.app_name,
                key_name: row.key_name,
                api_url: row.api_url,
                description: row.description,
                created_at: DateTime::from_timestamp(row.created_at, 0).unwrap(),
                updated_at: DateTime::from_timestamp(row.updated_at, 0).unwrap(),
            },
            key_value,
        })
    }

    /// Gets the count of API keys in the vault
    pub async fn count(&self) -> Result<i64> {
        let result = sqlx::query("SELECT COUNT(*) as count FROM api_keys")
            .fetch_one(&self.pool)
            .await?;

        Ok(result.get("count"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> VaultDb {
        VaultDb::new("sqlite::memory:").await.unwrap()
    }

    fn derive_test_key() -> VaultKey {
        let mut salt = [0u8; 32];
        for i in 0..32 {
            salt[i] = i as u8;
        }
        crate::crypto::derive_key_from_pin("123456", &salt).unwrap()
    }

    #[tokio::test]
    async fn test_create_api_key() {
        let db = setup_test_db().await;
        let key = derive_test_key();

        let input = CreateApiKey {
            app_name: "TestApp".to_string(),
            key_name: "prod_key".to_string(),
            api_url: Some("https://api.example.com".to_string()),
            description: Some("Production API key".to_string()),
            key_value: "sk_test_1234567890".to_string(),
        };

        let result = db.create_api_key(input, &key).await.unwrap();
        assert_eq!(result.api_key.app_name, "TestApp");
        assert_eq!(result.api_key.key_name, "prod_key");
        assert_eq!(result.key_value, "sk_test_1234567890");
    }

    #[tokio::test]
    async fn test_duplicate_rejected() {
        let db = setup_test_db().await;
        let key = derive_test_key();

        let input = CreateApiKey {
            app_name: "TestApp".to_string(),
            key_name: "prod_key".to_string(),
            api_url: None,
            description: None,
            key_value: "sk_test_1234567890".to_string(),
        };

        db.create_api_key(input.clone(), &key).await.unwrap();

        let result = db.create_api_key(input, &key).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_api_key() {
        let db = setup_test_db().await;
        let key = derive_test_key();

        let input = CreateApiKey {
            app_name: "TestApp".to_string(),
            key_name: "prod_key".to_string(),
            api_url: None,
            description: None,
            key_value: "sk_test_1234567890".to_string(),
        };

        let created = db.create_api_key(input, &key).await.unwrap();
        let retrieved = db.get_api_key(&created.api_key.id, &key).await.unwrap();

        assert_eq!(retrieved.key_value, "sk_test_1234567890");
    }

    #[tokio::test]
    async fn test_list_api_keys() {
        let db = setup_test_db().await;
        let key = derive_test_key();

        db.create_api_key(CreateApiKey {
            app_name: "App1".to_string(),
            key_name: "key1".to_string(),
            api_url: None,
            description: None,
            key_value: "value1".to_string(),
        }, &key).await.unwrap();

        db.create_api_key(CreateApiKey {
            app_name: "App2".to_string(),
            key_name: "key2".to_string(),
            api_url: None,
            description: None,
            key_value: "value2".to_string(),
        }, &key).await.unwrap();

        let keys = db.list_api_keys().await.unwrap();
        assert_eq!(keys.len(), 2);
    }

    #[tokio::test]
    async fn test_search_api_keys() {
        let db = setup_test_db().await;
        let key = derive_test_key();

        db.create_api_key(CreateApiKey {
            app_name: "GitHub".to_string(),
            key_name: "personal_token".to_string(),
            api_url: Some("https://api.github.com".to_string()),
            description: Some("Personal access token".to_string()),
            key_value: "ghp_xxxxx".to_string(),
        }, &key).await.unwrap();

        let results = db.search_api_keys("github").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].app_name, "GitHub");

        let results = db.search_api_keys("token").await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_api_key() {
        let db = setup_test_db().await;
        let key = derive_test_key();

        let created = db.create_api_key(CreateApiKey {
            app_name: "TestApp".to_string(),
            key_name: "key1".to_string(),
            api_url: None,
            description: None,
            key_value: "value1".to_string(),
        }, &key).await.unwrap();

        db.delete_api_key(&created.api_key.id).await.unwrap();

        let result = db.get_api_key(&created.api_key.id, &key).await;
        assert!(matches!(result, Err(DbError::NotFound)));
    }
}

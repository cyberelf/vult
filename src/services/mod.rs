//! High-level service layer for vault operations
//!
//! This module provides the main entry point for the vault library through
//! [`VaultManager`], which orchestrates all vault operations.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     VaultManager                            │
//! │         (Orchestrates services, public API)                 │
//! └─────────────────────────────────────────────────────────────┘
//!                              │
//!     ┌────────────────────────┼────────────────────────┐
//!     ▼                        ▼                        ▼
//! ┌──────────┐          ┌──────────┐          ┌──────────────┐
//! │AuthService│         │KeyService │         │CryptoService  │
//! │           │         │           │         │               │
//! └──────────┘          └──────────┘          └──────────────┘
//!                              │
//!                              ▼
//!                       ┌──────────┐
//!                       │ VaultDb  │
//!                       │(storage) │
//!                       └──────────┘
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use vult::services::VaultManager;
//!
//! let vault = VaultManager::new("sqlite://path/to/vault.db").await?;
//!
//! // Access services
//! vault.auth().unlock("my-pin").await?;
//! let keys = vault.keys().list().await?;
//! ```

mod auth_service;
mod crypto_service;
pub mod key_service;
mod vault_manager;

// Re-export main types
pub use auth_service::AuthService;
pub use crypto_service::CryptoService;
pub use key_service::KeyService;
pub use vault_manager::VaultManager;

// Re-export data types used in the API
pub use key_service::{ApiKey, ApiKeyMetadata, CreateKeyRequest, UpdateKeyRequest};

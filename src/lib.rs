//! # Vult - Secure API Key Vault Library
//!
//! Vult provides secure storage and management of API keys with PIN-based
//! authentication and AES-256-GCM encryption.
//!
//! ## Features
//!
//! - **Secure Storage**: API keys encrypted at rest using AES-256-GCM
//! - **PIN Authentication**: Master key derived from PIN using Argon2id
//! - **Per-Key Encryption**: Each key has unique encryption parameters
//! - **Cross-Platform**: Works on Windows and Linux
//!
//! ## Architecture
//!
//! The library is organized into layers:
//!
//! - **Core**: Constants, types, and validation ([`core`])
//! - **Services**: High-level operations ([`services::VaultManager`])
//! - **Foundation**: Cryptography, database, clipboard
//! - **Error Handling**: Unified [`error::VaultError`] type
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use vult::services::VaultManager;
//!
//! #[tokio::main]
//! async fn main() -> vult::error::Result<()> {
//!     // Initialize the vault
//!     let vault = VaultManager::new("sqlite://~/.vult/vault.db?mode=rwc").await?;
//!
//!     // Initialize with a PIN (first time only)
//!     vault.auth().init_vault("my-secure-pin").await?;
//!
//!     // Unlock the vault
//!     vault.auth().unlock("my-secure-pin").await?;
//!
//!     // Create an API key
//!     vault.keys().create("github", "token", "ghp_xxx...", None, None).await?;
//!
//!     // Retrieve the key
//!     let key = vault.keys().get("github", "token").await?;
//!     println!("Key value: {}", key.key_value);
//!
//!     // Lock when done
//!     vault.auth().lock().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Security Model
//!
//! - PIN is never stored; only a verification hash
//! - Master key derived using Argon2id (memory-hard)
//! - Each API key encrypted with unique derived key
//! - Sensitive data zeroized when no longer needed
//! - No recovery mechanism by design (lost PIN = lost data)

// =============================================================================
// Public modules - Library API
// =============================================================================

/// Core types, constants, and validation utilities
pub mod core;

/// Clipboard operations with auto-clear
pub mod clipboard;

/// Cryptographic operations (Argon2id, AES-256-GCM)
pub mod crypto;

/// Database operations (SQLite with SQLx)
pub mod database;

/// Unified error types
pub mod error;

/// High-level service layer
pub mod services;

// =============================================================================
// GUI-specific modules - Only available with gui feature
// =============================================================================

/// GUI-specific modules for Tauri desktop application.
///
/// This module provides:
/// - `AuthManager` - Authentication with auto-lock and Tauri event support
/// - `commands` - Tauri IPC command handlers
///
/// For CLI or library use, use [`services::AuthService`] instead.
#[cfg(feature = "gui")]
pub mod gui;

// =============================================================================
// Re-exports for convenience
// =============================================================================

pub use error::{Result, VaultError};
pub use services::VaultManager;

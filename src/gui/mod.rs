//! GUI-specific modules for Tauri desktop application.
//!
//! This module contains functionality specific to the GUI binary:
//! - `AuthManager` - Authentication with auto-lock and Tauri event support
//! - `commands` - Tauri IPC command handlers
//!
//! These modules are compiled only when the `gui` feature is enabled.

mod auth_manager;
pub mod commands;

pub use auth_manager::{AuthError, AuthManager, Result, SessionState};

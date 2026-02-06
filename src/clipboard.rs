//! Clipboard manager with auto-clear functionality.
//!
//! This module provides secure clipboard operations with automatic
//! clearing after a timeout period.
//!
//! # Features
//!
//! - Copy text to clipboard with auto-clear timeout
//! - Configurable timeout (default: 30 seconds)
//! - Thread-safe async operations
//! - Restores original clipboard content after clear
//!
//! # Security
//!
//! - Clipboard contents are automatically cleared after timeout
//! - Original clipboard content is restored when possible
//! - Prevents sensitive data from lingering in clipboard
//!
//! # Library Functions
//!
//! For simple one-shot clipboard operations (CLI, library use):
//! ```rust,ignore
//! use vult::clipboard::{copy_to_clipboard, clear_clipboard};
//!
//! copy_to_clipboard("secret")?;
//! // Later...
//! clear_clipboard()?;
//! ```
//!
//! # GUI Manager
//!
//! For GUI applications needing persistent auto-clear:
//! ```rust,ignore
//! use vult::clipboard::ClipboardManager;
//! use std::time::Duration;
//!
//! let manager = ClipboardManager::new()?;
//!
//! // Copy with 45-second auto-clear
//! manager.copy_with_timeout("secret".to_string(), Duration::from_secs(45)).await;
//!
//! // Start background clearing task
//! manager.start_auto_clear_checker();
//! ```

use arboard::Clipboard;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{interval, Instant};

use crate::VaultError;

// ============================================================================
// Simple Library Functions (for CLI and library use)
// ============================================================================

/// Copy text to system clipboard.
///
/// This is a simple synchronous copy operation for CLI and library use.
/// For GUI applications with auto-clear, use `ClipboardManager` instead.
///
/// # Example
///
/// ```rust,ignore
/// use vult::clipboard::copy_to_clipboard;
///
/// copy_to_clipboard("my-api-key")?;
/// ```
pub fn copy_to_clipboard(text: &str) -> Result<(), VaultError> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| VaultError::Clipboard(format!("Failed to access clipboard: {}", e)))?;
    clipboard
        .set_text(text)
        .map_err(|e| VaultError::Clipboard(format!("Failed to copy to clipboard: {}", e)))?;
    Ok(())
}

/// Clear the system clipboard.
///
/// Sets the clipboard content to an empty string.
///
/// # Example
///
/// ```rust,ignore
/// use vult::clipboard::clear_clipboard;
///
/// clear_clipboard()?;
/// ```
pub fn clear_clipboard() -> Result<(), VaultError> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| VaultError::Clipboard(format!("Failed to access clipboard: {}", e)))?;
    clipboard
        .set_text("")
        .map_err(|e| VaultError::Clipboard(format!("Failed to clear clipboard: {}", e)))?;
    Ok(())
}

/// Get current clipboard text content.
///
/// Returns None if clipboard is empty or contains non-text data.
///
/// # Example
///
/// ```rust,ignore
/// use vult::clipboard::get_clipboard_text;
///
/// if let Some(text) = get_clipboard_text()? {
///     println!("Clipboard contains: {}", text);
/// }
/// ```
pub fn get_clipboard_text() -> Result<Option<String>, VaultError> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| VaultError::Clipboard(format!("Failed to access clipboard: {}", e)))?;
    match clipboard.get_text() {
        Ok(text) if !text.is_empty() => Ok(Some(text)),
        Ok(_) => Ok(None),
        Err(_) => Ok(None), // Non-text content returns None
    }
}

// ============================================================================
// ClipboardManager (for GUI with auto-clear)
// ============================================================================

/// Clipboard manager that auto-clears after a timeout
pub struct ClipboardManager {
    inner: Arc<Mutex<ClipboardManagerInner>>,
}

struct ClipboardManagerInner {
    clipboard: Option<Clipboard>,
    last_copied_at: Option<Instant>,
    timeout: Duration,
    original_content: Option<String>,
}

impl ClipboardManager {
    /// Creates a new clipboard manager
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            inner: Arc::new(Mutex::new(ClipboardManagerInner {
                clipboard: Clipboard::new().ok(),
                last_copied_at: None,
                timeout: Duration::from_secs(30),
                original_content: None,
            })),
        })
    }

    /// Copies text to clipboard with auto-clear after timeout
    pub async fn copy_with_timeout(&self, text: String, timeout: Duration) {
        let mut inner = self.inner.lock().await;

        // Save original content
        let original = inner.clipboard.as_mut().and_then(|cb| cb.get_text().ok());

        // Copy new content
        if let Some(clipboard) = inner.clipboard.as_mut() {
            let _ = clipboard.set_text(&text);
        }

        inner.original_content = original;
        inner.last_copied_at = Some(Instant::now());
        inner.timeout = timeout;

        drop(inner);

        // Spawn background task to clear clipboard
        let inner_clone = Arc::clone(&self.inner);
        tokio::spawn(async move {
            tokio::time::sleep(timeout).await;
            let mut inner = inner_clone.lock().await;

            // Only clear if we're still the last one who copied
            if let Some(last_copied) = inner.last_copied_at {
                if last_copied.elapsed() >= timeout {
                    // Restore original content or clear
                    let original = inner.original_content.clone();
                    if let Some(clipboard) = inner.clipboard.as_mut() {
                        if let Some(orig) = original {
                            let _ = clipboard.set_text(&orig);
                        } else {
                            let _ = clipboard.set_text("");
                        }
                    }
                    inner.last_copied_at = None;
                    inner.original_content = None;
                }
            }
        });
    }

    /// Immediately clears the clipboard
    pub async fn clear_now(&self) {
        let mut inner = self.inner.lock().await;
        let original = inner.original_content.clone();
        if let Some(clipboard) = inner.clipboard.as_mut() {
            if let Some(orig) = original {
                let _ = clipboard.set_text(&orig);
            } else {
                let _ = clipboard.set_text("");
            }
        }
        inner.last_copied_at = None;
        inner.original_content = None;
    }

    /// Starts the background auto-clear checker
    pub fn start_auto_clear_checker(&self) {
        let inner_clone = Arc::clone(&self.inner);
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(1));
            loop {
                ticker.tick().await;
                let mut inner = inner_clone.lock().await;

                if let Some(last_copied) = inner.last_copied_at {
                    if last_copied.elapsed() >= inner.timeout {
                        let original = inner.original_content.clone();
                        if let Some(clipboard) = inner.clipboard.as_mut() {
                            if let Some(orig) = original {
                                let _ = clipboard.set_text(&orig);
                            } else {
                                let _ = clipboard.set_text("");
                            }
                        }
                        inner.last_copied_at = None;
                        inner.original_content = None;
                    }
                }
            }
        });
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            inner: Arc::new(Mutex::new(ClipboardManagerInner {
                clipboard: None,
                last_copied_at: None,
                timeout: Duration::from_secs(30),
                original_content: None,
            })),
        })
    }
}

//! Biometric authentication module for platform-specific implementations.
//!
//! This module provides platform-specific biometric authentication:
//! - Windows Hello on Windows 10 (1903+) and Windows 11
//!
//! Platform support is controlled by feature flags:
//! - `windows-biometric`: Enables Windows Hello support
//!
//! # Example
//!
//! ```rust,ignore
//! use vult::biometric::WindowsHelloProvider;
//! use vult::core::BiometricProvider;
//!
//! let provider = WindowsHelloProvider::new();
//! let availability = provider.check_availability().await;
//! if availability == BiometricAvailability::Available {
//!     let verified = provider.verify("Unlock Vult").await?;
//! }
//! ```

#[cfg(all(windows, feature = "windows-biometric"))]
mod windows_hello;

#[cfg(all(windows, feature = "windows-biometric"))]
pub use windows_hello::WindowsHelloProvider;

// Mock provider for testing
// Available in library unit tests or when windows-biometric feature is enabled (for integration tests)
pub mod mock;
pub use mock::MockBiometricProvider;

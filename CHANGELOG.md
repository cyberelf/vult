# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2026-02-10

### Fixed
- **Critical PIN verification vulnerability**: Repaired an issue where PIN verification used only the first byte of the derived key (leading to a ~0.4% collision rate). Fixes include:
  - Use of full Blake3 hash for verification instead of a single byte.
  - Added per-key salt into the verification hash for defense-in-depth.
  - Implemented automatic on-open migration supporting three legacy formats to preserve existing vaults.
- **Cryptography and build fixes**:
  - Corrected RNG import in `src/crypto.rs` (RngCore) and other compile issues introduced during refactors.
- **Dependency and policy fixes**:
  - Resolved `cargo-deny` and license compliance issues reported by CI.
  - Added advisory ignores for transitive Tauri/Linux dependencies that are not applicable to our Windows/macOS packaging.

### Changed
- **Security tooling**: Consolidated security checks under `cargo-deny` (replacing ad-hoc `cargo-audit` runs) and updated CI and local scripts accordingly.

### Technical
- Added `blake3` and `subtle` for secure hash and constant-time comparisons.
- Updated `deny.toml` with targeted advisory and license allowances.
- Migration and tests: added migration path and tests to ensure existing vaults remain unlockable.

## [0.2.1] - 2026-02-10

### Added
- **Windows Hello Integration**: Optional biometric authentication for Windows 10 (1903+) and Windows 11
  - Optional `windows-biometric` feature flag
  - BiometricProvider trait abstraction for platform-specific implementations
  - WindowsHelloProvider using UserConsentVerifier API with desktop HWND support
  - BiometricAvailability detection (Available, NotConfigured, DeviceNotPresent, NotSupported)
  - Automatic PIN fallback when biometric fails or is unavailable
  - User setting to enable/disable Windows Hello
  - MockBiometricProvider for testing
- **Secure Credential Storage**: Platform-native credential storage for biometric setup
  - Windows DPAPI-based credential store with per-vault isolation
  - Credential validation before storage
  - Automatic cleanup on disable
- **GUI Enhancements**: Improved unlock screen UI
  - Windows Hello button with biometric icon
  - Toggle buttons for switching between PIN and biometric auth
  - Consistent button styling across authentication methods
  - Biometric settings in vault settings screen

### Fixed
- **Windows Hello Modal Z-Order**: Fixed modal appearing behind main window
  - Used IUserConsentVerifierInterop desktop API with HWND parameter
  - Proper window parenting for system modal dialogs
  - Added raw-window-handle support for HWND extraction from Tauri window

### Changed
- Updated authentication specification with Windows Hello requirements
- Enhanced AGENTS.md with security architecture principles and biometric integration guidelines
- Added comprehensive Windows Hello integration documentation to LESSONS.md

### Technical
- Added windows-rs 0.58 with Security_Credentials_UI and Win32_System_WinRT features
- Added raw-window-handle 0.6 for native window handle extraction
- Added async-trait 0.1 for BiometricProvider trait objects
- 6 new integration tests for biometric functionality
- Updated Tauri capabilities for new biometric commands

## [0.2.0] - 2026-02-07

### Added
- **CLI Binary**: Full command-line interface (`vult`) for vault operations
  - `vult init` - Initialize vault with PIN
  - `vult add` - Add API keys (interactive or from stdin)
  - `vult get` - Retrieve keys (with --copy for clipboard)
  - `vult list` - List all keys (table or JSON format)
  - `vult search` - Search keys by name/description
  - `vult update` - Update key value or metadata
  - `vult delete` - Remove keys (with confirmation)
  - `vult change-pin` - Change vault PIN
  - `vult status` - Show vault status
- **Library Architecture**: Separated vault logic into reusable library
  - VaultManager as main entry point
  - AuthService, KeyService, CryptoService, StorageService
  - VaultError unified error type with exit codes and suggestions
- **Environment Variable Support**: `VULT_PIN` for scripting (with security warning)
- **Exit Codes**: Proper exit codes (0-10) for scripting integration
- **Ctrl+C Handling**: Graceful interrupt handling in CLI
- **Comprehensive Documentation**:
  - CLI_GUIDE.md with usage examples
  - ARCHITECTURE.md for developers
  - Enhanced rustdoc comments
- **Property-Based Testing**: 11 proptest tests for crypto operations
- **Service Unit Tests**: 43 tests for AuthService, KeyService, VaultManager
- **Version Badge**: Added version indicator (v0.2.0) to UI header

### Changed
- **BREAKING**: Migrated frontend from vanilla JavaScript to SvelteKit
- UI now uses Svelte 5 with Runes ($state, $props, $derived)
- TypeScript strict mode enabled for type safety
- Tailwind CSS v4 with new `@import` and `@theme` syntax
- Replaced custom components with shadcn-svelte components
- Vite build system for faster development and optimized production builds
- Library compiles independently (no GUI dependencies required)

### Added (SvelteKit Migration)
- SvelteKit SPA mode (SSR disabled for desktop app)
- Comprehensive TypeScript type definitions for all API types
- Type-safe Tauri command wrappers
- Svelte stores for global state management (vault, ui, clipboard)
- Toast notification system for user feedback
- Activity tracking service for auto-lock functionality
- Vitest testing framework with 55 passing tests
- Responsive design improvements with proper mobile support

### Removed
- Old vanilla JS implementation in `ui/` directory

## [0.1.0] - 2026-02-02

### Added
- Initial release of Vult - Secure API Key Vault
- PIN-based authentication with Argon2id key derivation
- AES-256-GCM encryption for stored API keys
- Per-key encryption with unique derived keys for each API key
- Auto-lock functionality after 5 minutes of inactivity
- Clipboard integration with auto-clear after 45 seconds
- Table-based UI with inline editing capabilities
- Search functionality across app name, key name, and description
- Database schema version tracking with automatic migrations
- Backup creation before database migrations
- Cross-platform support (Windows, macOS, Linux)

### Security Features
- Master key derived from PIN using Argon2id (64 MiB memory, 3 iterations)
- Per-key encryption using unique derived keys from master key + key context
- Each API key encrypted with individual salt
- Secure memory handling with zeroize
- PIN validation with minimum 6 character requirement

### Database
- SQLite database stored at `~/.vult/vault.db`
- Schema versioning system (current version: 2)
- Automatic migration from older schema versions
- Protection against opening databases with newer versions
- Automatic cleanup of orphaned tables

### API Key Management
- Create, read, update, and delete API keys
- Optional fields: app name, API URL, description
- Show/hide key values with toggle button
- Copy keys to clipboard with visual feedback
- Inline editing in table view

[0.2.2]: https://github.com/cyberelf/vult/releases/tag/v0.2.2
[0.2.1]: https://github.com/cyberelf/vult/releases/tag/v0.2.1
[0.2.0]: https://github.com/cyberelf/vult/releases/tag/v0.2.0
[0.1.0]: https://github.com/yourusername/vult/releases/tag/v0.1.0

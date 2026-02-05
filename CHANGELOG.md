# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[0.1.0]: https://github.com/yourusername/vult/releases/tag/v0.1.0

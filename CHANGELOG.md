# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-XX

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

# Change: Add Secure API Key Vault

## Why
API keys are sensitive credentials that need secure storage and controlled access. Currently, users must manage API keys insecurely (plaintext files, clipboard, password managers not designed for API keys). A dedicated, secure vault with device authentication provides a specialized solution for managing API credentials.

## What Changes

### Core Capabilities
- Add encrypted storage system for API keys with app-recognized encryption
- Add device authentication layer supporting both biometric and PIN-based auth
- Add CRUD operations for API key management with properties:
  - App name
  - Key name
  - API URL
  - Description

### Security Features
- Encryption at rest using platform-secure key derivation
- Device authentication required for all vault access (view/edit/delete)
- Biometric primary authentication with PIN fallback
- Single-user vault design per device

### Cross-Platform Support
- Windows, macOS, and Linux desktop support
- Platform-native biometric integration (Windows Hello, Touch ID, etc.)

## Impact

### Affected Specs
- **NEW** `secure-storage` - Encrypted data storage capability
- **NEW** `device-authentication` - Biometric and PIN authentication
- **NEW** `api-key-management` - API key CRUD operations

### Affected Code
- Core application structure (new project)
- Encryption module (new)
- Platform integration layer (new)
- UI components (new)

### Dependencies
- Cross-platform desktop framework (Tauri recommended for Rust)
- Encryption libraries (age or rust-argon2 + AES-GCM)
- Platform biometric APIs (webauthn-rs or similar)

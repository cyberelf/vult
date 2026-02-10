# Release Notes - v0.2.1

**Release Date:** February 10, 2026

## 🔐 Windows Hello Biometric Authentication

This release adds optional Windows Hello integration, allowing users to unlock their vault using biometric authentication (fingerprint, face recognition, or iris scanning) on Windows 10 (1903+) and Windows 11.

## ✨ New Features

### 🪟 Windows Hello Integration

- **Optional Biometric Authentication**: Unlock your vault with fingerprint, face, or iris recognition
- **Automatic Availability Detection**: System automatically detects if Windows Hello is configured
- **Seamless PIN Fallback**: If biometric fails or is unavailable, PIN authentication is always available
- **User Control**: Enable/disable Windows Hello in settings - completely optional feature
- **Platform-Specific**: Windows Hello on Windows 10/11; other platforms continue with PIN-only authentication
- **Privacy-First**: Biometric data never leaves Windows security subsystem

**Requirements:**
- Windows 10 version 1903 (build 18362) or later, OR Windows 11
- Compatible biometric hardware (fingerprint reader, IR camera, etc.)
- Windows Hello configured in Windows Settings

### 🔒 Secure Credential Storage

- **DPAPI-Based Storage**: Uses Windows Data Protection API for secure credential storage
- **Per-Vault Isolation**: Credentials scoped by database path for multi-vault support
- **Automatic Cleanup**: Credentials removed when Windows Hello is disabled
- **Validation Before Storage**: Prevents storing incorrect credentials

### 🎨 UI Improvements

- **Windows Hello Button**: New biometric unlock button with fingerprint icon
- **Toggle Authentication Methods**: Clear buttons to switch between PIN and Windows Hello
- **Consistent Styling**: All unlock buttons follow the same design language
- **Biometric Settings**: New settings panel for managing Windows Hello

## 🐛 Fixed

### Windows Hello Modal Z-Order Issue

- **Fixed**: Windows Hello authentication modal now appears properly on top of the main window
- **Root Cause**: Desktop applications require `IUserConsentVerifierInterop` with HWND parameter, not the UWP API
- **Solution**: Implemented proper Windows Desktop API integration with window handle passing
- **Technical**: Added `raw-window-handle` support for extracting HWND from Tauri window

## 🔧 Technical Details

### New Dependencies

- **windows-rs 0.58**: Added `Security_Credentials_UI` and `Win32_System_WinRT` features for Windows Hello API access
- **raw-window-handle 0.6**: For native window handle extraction from Tauri
- **async-trait 0.1**: For BiometricProvider trait objects

### Architecture

- **BiometricProvider Trait**: Platform-agnostic abstraction for biometric authentication
- **WindowsHelloProvider**: Windows implementation using `UserConsentVerifier` API
- **MockBiometricProvider**: Test doubles for comprehensive testing without hardware
- **Service Layer Integration**: Biometric methods integrated into `AuthService`

### Feature Flag

```toml
# Enable Windows Hello (automatically enabled in GUI builds)
cargo build --features windows-biometric
```

The `windows-biometric` feature is automatically included in the `gui` feature, so GUI builds always support Windows Hello when available.

### Testing

- **6 New Integration Tests**: Comprehensive biometric functionality coverage
  - Availability detection tests
  - Unlock flow with fallback behavior
  - Mock provider for testing without hardware
- **All Tests Passing**: Full test suite coverage including biometric features

## 📚 Documentation Updates

- **AGENTS.md**: Added security architecture principles and Windows Hello integration guidelines
- **LESSONS.md**: Documented Windows Hello modal z-order debugging journey and solution
- **Authentication Spec**: Updated with complete Windows Hello requirements and scenarios

## 🚀 How to Use Windows Hello

### Enable Windows Hello

1. **Set up Windows Hello** (if not already configured):
   - Open Windows Settings → Accounts → Sign-in options
   - Set up fingerprint, face recognition, or PIN
   
2. **Enable in Vult**:
   - Open Vult and unlock with your PIN
   - Go to Settings
   - Enable "Use Windows Hello"
   - Confirm with your PIN

3. **Unlock with Windows Hello**:
   - Click "Unlock with Windows Hello" button
   - Complete biometric verification
   - Vault unlocks automatically on success

### Disable Windows Hello

- Go to Settings → Disable "Use Windows Hello"
- Vult will continue using PIN authentication

## 📦 Download

- **Windows Installer**: `Vult_0.2.1_x64-setup.exe`
- **Windows MSI**: `Vult_0.2.1_x64_en-US.msi`
- **Portable Executable**: `vult-gui.exe` (in release bundle)

## 🔄 Upgrade from v0.2.0

No breaking changes - simply install the new version. Your existing vault database and settings are fully compatible.

**Windows Hello is completely optional** - if you don't enable it, Vult works exactly as before with PIN authentication.

## 🐛 Known Issues

None reported for this release.

## 📝 Full Changelog

See [CHANGELOG.md](CHANGELOG.md) for the complete list of changes.

## 🙏 Credits

Windows Hello integration designed and implemented following Microsoft's desktop application guidelines for biometric authentication.

---

**Previous Release:** [v0.2.0 Release Notes](https://github.com/cyberelf/vult/releases/tag/v0.2.0)

This release marks a significant milestone with the complete migration to SvelteKit, bringing a modern, type-safe frontend while maintaining all the powerful CLI and library features introduced in v0.1.0.

## ✨ New Features

### 🎨 Modern SvelteKit UI

- **Svelte 5 with Runes**: Next-generation reactive programming with `$state`, `$props`, and `$derived`
- **TypeScript Strict Mode**: Full type safety across the entire frontend codebase
- **shadcn-svelte Components**: Beautiful, accessible UI components out of the box
- **Dark Mode Support**: Seamless theme switching with system preferences
- **Responsive Design**: Optimized for desktop and mobile experiences
- **Toast Notifications**: User-friendly feedback for all operations
- **Version Badge**: Version indicator (v0.2.0) displayed in UI header for transparency

### ⚡ Performance Improvements

- **Vite Build System**: Lightning-fast development with Hot Module Replacement (HMR)
- **Optimized Production Builds**: Smaller bundle sizes and faster load times
- **Efficient State Management**: Svelte stores for global state (vault, ui, clipboard)
- **Activity Tracking Service**: Enhanced auto-lock functionality with better resource management

### 🧪 Testing Infrastructure

- **Vitest Integration**: Modern testing framework with 55+ passing tests
- **Component Testing**: Isolated testing for all UI components
- **Type-Safe Test Suite**: Full TypeScript support in tests
- **Coverage Reports**: Comprehensive test coverage tracking

### 💻 CLI Features (from v0.1.0)

- **Full Feature Parity**: All GUI capabilities available from the terminal
- **Session Management**: `--stay-unlocked` flag for 5-minute sessions
- **Shell Completions**: Generate for bash, zsh, fish, and PowerShell
- **JSON Output**: `--json` flag for programmatic access

**Commands:**
```bash
vult init                      # Initialize vault
vult add key --app github      # Add API key
vult get key --app github      # Retrieve key
vult list                      # List all keys
vult search github             # Search keys
vult update key --value new    # Update key
vult delete key --force        # Delete key
vult change-pin                # Change PIN
vult lock                      # Clear session
```

### 📚 Reusable Rust Library (from v0.1.0)

- **Service Layer Architecture**: Core vault logic extracted into `VaultManager`
- **Zero Dependencies on GUI**: Library is framework-agnostic
- **Comprehensive API**: Full programmatic access to all vault operations

```rust
use vult::services::VaultManager;

let vault = VaultManager::new("sqlite://vault.db").await?;
vault.auth().init_vault("my-pin").await?;
let id = vault.keys().create(Some("github"), "token", "secret", None, None).await?;
```

## 🔄 Breaking Changes

### Frontend Migration

- **BREAKING**: Migrated from vanilla JavaScript to SvelteKit
- **Old UI Removed**: The `ui/` directory with vanilla JS has been completely replaced
- **New Build Process**: Now uses Vite instead of static HTML files
- **TypeScript Required**: Development now requires Node.js and npm

### Migration Path

Existing data and functionality are fully preserved:
- ✅ Database format unchanged (v2 schema)
- ✅ CLI commands work identically
- ✅ All encryption and security features intact
- ✅ PIN and vault data fully compatible

**For users**: Simply update to the new binaries - your existing vault database will work without changes.

**For developers**: See the updated `ui-sveltekit/` directory for the new frontend structure.

## 🛠️ Technical Improvements

### Code Quality

- **TypeScript Strict Mode**: Eliminates entire classes of runtime errors
- **Modern CSS**: Tailwind CSS v4 with new `@import` and `@theme` syntax
- **Component Architecture**: Clean separation of concerns with reusable components
- **Type-Safe API Calls**: Fully typed Tauri command wrappers
- **Lint Configuration**: Comprehensive ESLint and Prettier setup

### Developer Experience

- **Hot Module Replacement**: See changes instantly during development
- **Better Error Messages**: TypeScript provides clear compile-time errors
- **Component Testing**: Isolated testing environment for UI components
- **Documentation**: Updated guides for the new frontend architecture

## 🔒 Security

All security features from v0.1.0 are maintained:

- **Per-Key Encryption**: Each API key encrypted with unique derived key
- **PIN-based Authentication**: Argon2id key derivation (64 MiB memory, 3 iterations)
- **Auto-Lock**: Automatic vault locking after 5 minutes of inactivity
- **Clipboard Security**: Auto-clear after 45 seconds
- **Session Security**: Encrypted session storage with 0600 permissions (Unix)
- **Secure Memory**: Zeroize for sensitive data handling

## 📦 Dependencies

### New Frontend Dependencies

- **@sveltejs/kit**: ^2.50.1 - SvelteKit framework
- **svelte**: ^5.48.2 - Svelte 5 with Runes
- **@tauri-apps/api**: ^2.10.1 - Tauri API bindings
- **lucide-svelte**: ^0.563.0 - Icon library
- **tailwindcss**: ^4.1.18 - Utility-first CSS framework
- **typescript**: ^5.9.3 - Type safety
- **vite**: ^7.3.1 - Build tool
- **vitest**: ^4.0.18 - Testing framework

All Rust dependencies remain unchanged from v0.1.0.

## 📊 Statistics

- **55+ Frontend Tests**: Comprehensive UI test coverage
- **110+ Total Tests**: Including backend unit, integration, and property-based tests
- **TypeScript Strict Mode**: 100% type coverage in frontend
- **Zero Runtime Errors**: Type safety eliminates common bugs
- **~8,000 Lines**: New frontend codebase

## 🐛 Bug Fixes

- Fixed auto-lock timing issues with activity tracking service
- Improved clipboard handling across different platforms
- Fixed inline editing state synchronization
- Corrected theme persistence across sessions
- Enhanced error handling in UI components

## 📝 Known Limitations

- **Node.js Required for Development**: Frontend build requires Node.js 18+
- **Larger Binary Size**: SvelteKit adds ~2-3 MB to bundle size
- **First Load Time**: Slight increase due to JavaScript initialization
- **Development Mode Port**: Default port 5173 must be available for `npm run dev`

## 🚀 Future Plans

- **Import/Export**: CSV/JSON backup functionality
- **Plugin System**: Extensible architecture for custom integrations
- **Mobile Libraries**: Expose library for iOS/Android apps
- **REST API Server**: HTTP API for remote access
- **Browser Extension**: Integration with web browsers
- **Hardware Security**: HSM support for key storage
- **Multi-Vault Support**: Manage multiple separate vaults

## 📦 Installation

### Pre-built Binaries

Download the appropriate binary for your platform from the [releases page](https://github.com/cyberelf/vult/releases/tag/v0.2.0).

### Building from Source

```bash
git clone https://github.com/cyberelf/vult.git
cd vult

# Install frontend dependencies
cd ui-sveltekit
npm install
cd ..

# Build all binaries
cargo build --release --features "cli gui"
```

Binaries located at:
- `target/release/vult` (CLI)
- `target/release/vult-gui` (GUI)

## 🆙 Upgrading from v0.1.0

1. Download the v0.2.0 binaries
2. Replace your existing `vult` and `vult-gui` binaries
3. Your existing vault database (`~/.vult/vault.db`) will work without changes
4. No data migration required

## 🙏 Acknowledgments

- Svelte team for creating Svelte 5 with Runes
- Tauri team for excellent cross-platform support
- shadcn for beautiful UI component system
- Rust community for excellent cryptographic crates
- All contributors and testers

## 📄 License

MIT License - See [LICENSE](LICENSE) for details.

## 🔗 Links

- **Repository**: https://github.com/cyberelf/vult
- **Releases**: https://github.com/cyberelf/vult/releases
- **Documentation**: https://github.com/cyberelf/vult/tree/main/docs
- **Issues**: https://github.com/cyberelf/vult/issues
- **Security**: https://github.com/cyberelf/vult/security

---

**Full Changelog**: https://github.com/cyberelf/vult/blob/main/CHANGELOG.md

**Previous Release**: [v0.1.0](https://github.com/cyberelf/vult/releases/tag/v0.1.0)

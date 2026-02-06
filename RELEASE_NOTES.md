# Release Notes - v0.1.0

**Release Date:** February 5, 2026

## 🎉 Major Release: Backend Library & CLI

This release introduces a major architectural change, transforming Vult from a GUI-only application into a comprehensive, multi-interface secure API key vault with a reusable Rust library.

## ✨ New Features

### 📚 Reusable Rust Library

- **Service Layer Architecture**: Core vault logic extracted into `VaultManager` with clean service APIs
- **Zero Dependencies on GUI**: Library is framework-agnostic and can be used in any Rust project
- **Comprehensive API**: Full programmatic access to all vault operations

```rust
use vult::services::VaultManager;

let vault = VaultManager::new("sqlite://vault.db").await?;
vault.auth().init_vault("my-pin").await?;
let id = vault.keys().create(Some("github"), "token", "secret", None, None).await?;
```

### 💻 Command-Line Interface (CLI)

- **Full Feature Parity**: All GUI capabilities available from the terminal
- **Session Management**: `--stay-unlocked` flag for 5-minute sessions (no repeated PIN entry)
- **Shell Completions**: Generate for bash, zsh, fish, and PowerShell
- **JSON Output**: `--json` flag for programmatic access and scripting
- **Secure Input**: PIN entry uses secure terminal input (never echoes)

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

**Session Mode:**
```bash
vult add key1 --stay-unlocked  # Creates 5-minute session
vult list                       # Uses session, no PIN needed
vult get key1                   # Still using session
vult lock                       # Manually clear session
```

### 🔒 Enhanced Security

- **Per-Key Encryption**: Each API key encrypted with unique derived key
- **Session Security**: Sessions store encrypted PIN with AES-256-GCM
- **Automatic Session Cleanup**: Ctrl+C and process exit clear sessions
- **Restricted Permissions**: Session files created with 0600 permissions (Unix)
- **Property-Based Testing**: Cryptographic properties verified with proptest
- **Security Policy**: Vulnerability disclosure policy in `.well-known/security.txt`

## 🛠️ Improvements

### Code Quality

- **Clippy Configuration**: Strict linting with pedantic and nursery lints
- **cargo-deny**: License and dependency security checks
- **Pre-commit Hooks**: Automatic formatting and linting
- **CI/CD Pipeline**: Automated testing, linting, and security scanning
- **Dependabot**: Automated dependency updates

### Documentation

- **Architecture Guide**: Complete system design documentation
- **CLI Guide**: Comprehensive command examples
- **Migration Guide**: Step-by-step upgrade instructions
- **Security Testing Guide**: Security verification procedures
- **Contributing Guide**: Developer onboarding and conventions

### Testing

- **91 Library Tests**: Unit tests for all core functionality
- **19 CLI Tests**: End-to-end integration tests
- **Property-Based Tests**: 7 cryptographic property tests
- **Auto-lock Tests**: GUI auto-lock behavior verification

## 🔄 Changes

### Breaking Changes

- **Library API Redesigned**: `VaultDb` replaced with `VaultManager` and service layer
- **Error Types Unified**: Single `VaultError` type for all operations
- **Feature Flags**: `gui` and `cli` features control binary builds

**Migration Required:** See [MIGRATION.md](MIGRATION.md) for upgrade instructions.

### Non-Breaking Changes

- **Database Schema**: Backward compatible (v2)
  - Automatic migrations with backups
  - Shared database between GUI and CLI
- **GUI Functionality**: All existing features preserved
- **Security Model**: No changes to encryption or key derivation

## 📦 Binaries

### Downloads

| Platform | Architecture | Binary | Size |
|----------|-------------|--------|------|
| Linux | x86_64 | `vult-0.1.0-linux-x86_64.tar.gz` | ~15 MB |
| Windows | x86_64 | `vult-0.1.0-windows-x86_64.zip` | ~12 MB |
| macOS | x86_64 | `vult-0.1.0-macos-x86_64.tar.gz` | ~14 MB |

Each archive contains:
- `vult` - CLI binary
- `vult-gui` - GUI binary
- `README.md`, `LICENSE`, `CHANGELOG.md`

### Installation

**Linux/macOS:**
```bash
tar -xzf vult-0.1.0-linux-x86_64.tar.gz
cd vult-0.1.0-linux-x86_64
cp vult ~/.local/bin/          # CLI
cp vult-gui ~/.local/bin/      # GUI (optional)
```

**Windows:**
```powershell
# Extract ZIP
# Copy vult.exe to a directory in PATH
# Copy vult-gui.exe for GUI access
```

### Building from Source

```bash
git clone https://github.com/cyberelf/vult.git
cd vult
cargo build --release --features "cli gui"
```

Binaries located at:
- `target/release/vult` (CLI)
- `target/release/vult-gui` (GUI)

## 📊 Statistics

- **193 Implementation Tasks**: 75% complete (144/193)
- **49 Lines of Code Changed**: ~12,000 additions
- **New Files**: 15+ documentation and config files
- **Test Coverage**: 110+ tests across unit, integration, and property-based
- **Dependencies**: 2 new (clap, clap_complete for CLI)

## 🐛 Bug Fixes

- Fixed database migration backup mechanism
- Improved error messages and hints
- Fixed clipboard auto-clear race condition
- Corrected session file permissions on Unix

## 📝 Known Limitations

- **change-pin with CLI**: Requires interactive input (can't be automated with VULT_PIN)
- **Clipboard Testing**: Automated clipboard tests difficult due to system dependencies
- **Cross-Platform Testing**: Manual verification required for Windows/macOS
- **Formal Verification**: Not yet implemented (planned for future releases)

## 🚀 Future Plans

- **Import/Export**: CSV/JSON backup functionality
- **Mobile Libraries**: Expose library for iOS/Android apps
- **REST API Server**: HTTP API for remote access
- **Browser Extension**: Integration with web browsers
- **Hardware Security**: HSM support for key storage

## 🙏 Acknowledgments

- Rust community for excellent cryptographic crates
- Tauri team for cross-platform framework
- Contributors and testers

## 📄 License

MIT License - See [LICENSE](LICENSE) for details.

## 🔗 Links

- **Repository**: https://github.com/cyberelf/vult
- **Documentation**: https://github.com/cyberelf/vult/tree/main/docs
- **Issues**: https://github.com/cyberelf/vult/issues
- **Security**: https://github.com/cyberelf/vult/security

---

**Full Changelog**: https://github.com/cyberelf/vult/blob/main/CHANGELOG.md

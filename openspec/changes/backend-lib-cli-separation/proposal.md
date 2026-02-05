# Change: Backend Library and CLI Separation

## Why

The current Vult architecture tightly couples the Tauri GUI application with the core vault logic. All core functionality (authentication, encryption, database operations) is embedded in modules that are only accessible through the GUI application. This creates several problems:

- **Limited Access Patterns**: Users must use the GUI even for simple operations like listing keys or copying a single value
- **Reusability**: Core vault logic cannot be reused in other contexts (scripts, CI/CD, automation)
- **Testing**: Integration testing requires the full Tauri app, making tests slower and more complex
- **Developer Experience**: Contributors must understand Tauri to work with core vault logic
- **Scripting & Automation**: No way to integrate vault operations into shell scripts or automated workflows

A clean separation between the library (core logic) and binaries (GUI + CLI) enables:
- Command-line access for power users and automation
- Better testing with unit tests against the library API
- Potential for future integrations (browser extensions, mobile apps, API server)
- Clear architectural boundaries and maintainability

## What Changes

### Core Architecture Restructure

**Library Layer** (`src/lib.rs`)
- Expose public API for all vault operations:
  - `VaultManager` - High-level vault operations orchestrator
  - `AuthService` - PIN/session management without Tauri coupling
  - `CryptoService` - Encryption/decryption operations
  - `StorageService` - Database operations via SQLx
  - `KeyService` - API key CRUD operations
- Remove all Tauri-specific dependencies from core modules
- Make all core modules public with well-defined interfaces
- Add library-level error types (separate from Tauri command errors)

**Binary Layer** (`src/bin/`)
- `vult-gui` (current `main.rs`) - Tauri GUI application wrapper
  - Thin adapter layer converting library calls to Tauri commands
  - Manages GUI-specific state (Arc wrappers, auto-lock UI)
- `vult` - New CLI application
  - Command-line interface using `clap` or similar
  - Interactive PIN prompt for authentication
  - Commands: `init`, `add`, `get`, `list`, `update`, `delete`, `search`, `lock`
  - Output formats: human-readable table, JSON, or raw value
  - Clipboard integration for secure copy operations

### CLI Commands

```bash
# Initialize vault with PIN
vult init

# Add new API key (prompts for values)
vult add <app-name> <key-name>

# Get and copy to clipboard
vult get <app-name> <key-name> [--copy]

# List all keys
vult list [--json]

# Search keys
vult search <query>

# Update key value
vult update <app-name> <key-name>

# Delete key
vult delete <app-name> <key-name>

# Change PIN
vult change-pin
```

### Module Refactoring

- **`auth.rs`**: Remove Tauri State dependency, expose standalone `AuthService`
- **`crypto.rs`**: Already mostly independent, make public API cleaner
- **`database.rs`**: Already independent, expose public interface
- **`commands.rs`**: Becomes thin adapter layer calling library functions
- **`clipboard.rs`**: Split into library (clipboard operations) and GUI-specific (auto-clear thread)

### Configuration & State Management

- Support both interactive (GUI) and non-interactive (CLI) modes
- CLI uses same `~/.vult/vault.db` database path
- Session management for CLI (optional `--stay-unlocked` flag)
- Environment variable support for automation: `VULT_PIN` (with security warnings)
- Platform support: Windows and Linux only

## Impact

### Affected Specs

- **NEW** `library-api` - Public library interface for vault operations
- **NEW** `cli-interface` - Command-line interface specification
- **MODIFIED** `vault-architecture` - Separation of concerns: lib vs. binaries
- **MODIFIED** `authentication` - Remove Tauri coupling, support CLI authentication
- **MODIFIED** `secure-storage` - Make storage layer library-accessible
- **MODIFIED** `api-key-management` - Expose through library interface

### Affected Code

**New Files**:
- `src/bin/vult-gui.rs` - Tauri GUI binary (relocated from `main.rs`)
- `src/bin/vult.rs` - New CLI binary
- `src/services/mod.rs` - Service layer abstractions
- `src/services/vault_manager.rs` - High-level vault orchestration
- `src/services/auth_service.rs` - Authentication service
- `src/services/key_service.rs` - Key management service
- `src/error.rs` - Library-level error types

**Modified Files**:
- `src/lib.rs` - Expose public library API
- `src/main.rs` - Becomes symlink or redirects to `bin/vult-gui.rs`
- `src/auth.rs` - Remove Tauri State, expose service API
- `src/commands.rs` - Thin adapter for Tauri commands
- `src/clipboard.rs` - Split GUI-specific from library logic
- `Cargo.toml` - Add CLI dependencies (clap), configure binaries

**Unchanged (but exposed)**:
- `src/crypto.rs` - Already independent
- `src/database.rs` - Already independent

### Dependencies

**New Dependencies**:
- `clap` ~4.5 - CLI argument parsing and command structure
- `dialoguer` ~0.11 - Interactive CLI prompts (PIN entry)
- `tabled` ~0.15 or `comfy-table` ~7.1 - Terminal table formatting
- `rpassword` ~7.3 - Secure password/PIN input

**Optional**:
- `colored` ~2.1 - Terminal color output
- `indicatif` ~0.17 - Progress indicators for long operations

### Breaking Changes

**For End Users**: None - GUI app remains unchanged in functionality

**For Developers**:
- Module structure changes (imports may need updating in tests)
- Binary build process changes (now builds two binaries)
- Any external code depending on internal modules will break

### Migration Path

1. Existing GUI users - No migration needed, transparent change
2. New CLI users - Install and run `vult init` to reuse existing vault or create new one
3. Developers - Update imports for refactored modules

### Security Considerations

- CLI must maintain same security model as GUI (encrypted at rest, authenticated access)
- PIN entry via secure terminal input (no echo)
- Clipboard auto-clear in CLI mode (configurable timeout)
- Warning when using `VULT_PIN` environment variable (security risk)
- Session tokens for CLI unlock (optional, explicit user choice)

# Vult Architecture

This document describes the architecture of the Vult secure API key vault.

## Overview

Vult is a secure API key management system built with Rust. It provides:
- Encrypted storage of API keys at rest
- PIN-based authentication with memory-hard key derivation
- Cross-platform support (Windows, Linux)
- Both GUI (Tauri) and CLI interfaces

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           User Interfaces                               │
├─────────────────────────────┬───────────────────────────────────────────┤
│       GUI (Tauri)           │              CLI (clap)                   │
│    src/bin/vult-gui.rs      │           src/bin/vult.rs                 │
└─────────────────────────────┴───────────────────────────────────────────┘
                                      │
                              ┌───────┴───────┐
                              │               │
                              ▼               ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        Adapter Layer                                    │
│    GUI: src/gui/commands.rs (Tauri IPC handlers)                        │
│    CLI: Built into binary (uses services directly)                      │
└─────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                       Service Layer (Library)                           │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │                        VaultManager                                │ │
│  │              (Orchestrates all services)                           │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│           │                │                │                │          │
│           ▼                ▼                ▼                ▼          │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌───────────────┐  │
│  │ AuthService  │ │ KeyService   │ │CryptoService │ │StorageService │  │
│  │              │ │              │ │              │ │               │  │
│  │ - init()     │ │ - create()   │ │ - encrypt()  │ │ - store()     │  │
│  │ - unlock()   │ │ - get()      │ │ - decrypt()  │ │ - retrieve()  │  │
│  │ - lock()     │ │ - list()     │ │ - derive_key │ │ - delete()    │  │
│  │ - change_pin │ │ - update()   │ │              │ │               │  │
│  └──────────────┘ │ - delete()   │ └──────────────┘ └───────────────┘  │
│                   │ - search()   │                                      │
│                   └──────────────┘                                      │
└─────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        Core & Foundation                                │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌───────────────┐  │
│  │  core/       │ │  crypto.rs   │ │ database.rs  │ │ clipboard.rs  │  │
│  │              │ │              │ │              │ │               │  │
│  │ - types.rs   │ │ - Argon2id   │ │ - SQLite     │ │ - Auto-clear  │  │
│  │ - validate   │ │ - AES-GCM    │ │ - Migrations │ │ - Platform    │  │
│  │ - constants  │ │ - per-key    │ │ - CRUD       │ │               │  │
│  └──────────────┘ └──────────────┘ └──────────────┘ └───────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          Storage Layer                                  │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                      SQLite Database                             │  │
│  │                     ~/.vult/vault.db                             │  │
│  └──────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
vult/
├── src/
│   ├── lib.rs              # Library entry point, public exports
│   ├── main.rs             # GUI entry point (when building with tauri)
│   ├── error.rs            # VaultError enum - unified error handling
│   │
│   ├── core/               # Core types and utilities
│   │   ├── mod.rs          # Module exports
│   │   └── types.rs        # Constants, PIN validation, shared types
│   │
│   ├── crypto.rs           # Cryptographic operations (Argon2id, AES-GCM)
│   ├── database.rs         # SQLite operations
│   ├── clipboard.rs        # Clipboard with auto-clear
│   │
│   ├── services/           # Business logic layer
│   │   ├── mod.rs          # Service module exports
│   │   ├── vault_manager.rs    # Main orchestrator
│   │   ├── auth_service.rs     # Authentication operations
│   │   ├── key_service.rs      # Key CRUD operations
│   │   ├── crypto_service.rs   # Crypto wrapper
│   │   └── storage_service.rs  # Storage wrapper
│   │
│   ├── gui/               # GUI-specific modules (feature-gated)
│   │   ├── mod.rs          # Module exports
│   │   ├── auth_manager.rs # AuthManager with Tauri event support
│   │   └── commands.rs     # Tauri IPC command handlers
│   │
│   └── bin/
│       ├── vult.rs         # CLI binary
│       └── vult-gui.rs     # GUI binary (Tauri)
│
├── ui-sveltekit/           # Frontend (SvelteKit + Tailwind)
├── tests/                  # Integration tests
└── docs/                   # Documentation
```

## Module Responsibilities

### Core Module (`src/core/`)

Foundation layer with no business logic dependencies:

- **`types.rs`**: Constants (PIN lengths, timeouts), PIN validation
- Shared types used across the library
- No async code, no database access

### Services Layer (`src/services/`)

Business logic with clean interfaces:

- **`VaultManager`**: Entry point, creates and wires services
- **`AuthService`**: PIN operations, session management (CLI/library use)
- **`KeyService`**: API key CRUD with encryption
- **`CryptoService`**: Encryption/decryption wrapper
- **`StorageService`**: Database wrapper (future use)

### GUI Module (`src/gui/`)

**Feature-gated** (`gui` feature) - Tauri-specific functionality:

- **`AuthManager`**: Authentication with auto-lock and Tauri event emission
- Activity tracking and background counter
- Emits `vault_locked` events for frontend synchronization

## Security Architecture

### Cryptographic Design

```
                    User PIN
                        │
                        ▼
            ┌───────────────────────┐
            │      Argon2id         │
            │  (Memory: 64MB)       │
            │  (Iterations: 3)      │
            │  (Parallelism: 4)     │
            └───────────────────────┘
                        │
                        ▼
                  Master Key (256-bit)
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ▼               ▼               ▼
   ┌─────────┐    ┌─────────┐    ┌─────────┐
   │ Per-Key │    │ Per-Key │    │ Per-Key │
   │Derivation│   │Derivation│   │Derivation│
   │ + Salt  │    │ + Salt  │    │ + Salt  │
   └─────────┘    └─────────┘    └─────────┘
        │               │               │
        ▼               ▼               ▼
   ┌─────────┐    ┌─────────┐    ┌─────────┐
   │AES-256- │    │AES-256- │    │AES-256- │
   │   GCM   │    │   GCM   │    │   GCM   │
   │ Key 1   │    │ Key 2   │    │ Key 3   │
   └─────────┘    └─────────┘    └─────────┘
```

### Key Points

1. **No PIN Storage**: PIN is never stored; only a verification hash
2. **Per-Key Encryption**: Each API key uses a unique derived encryption key
3. **Memory-Hard KDF**: Argon2id resists GPU/ASIC attacks
4. **Authenticated Encryption**: AES-256-GCM provides confidentiality + integrity
5. **Zeroization**: Sensitive data cleared from memory when no longer needed

### Session Management

```
┌─────────────────────────────────────────────────────────────────┐
│                        Session State                             │
├─────────────────────────────────────────────────────────────────┤
│  Locked State:                                                   │
│    - No master key in memory                                     │
│    - All key operations fail                                     │
│    - Only init() and unlock() allowed                            │
├─────────────────────────────────────────────────────────────────┤
│  Unlocked State:                                                 │
│    - Master key held in memory (Arc<RwLock<Option<VaultKey>>>)   │
│    - All operations available                                    │
│    - Auto-lock after inactivity (GUI only)                       │
└─────────────────────────────────────────────────────────────────┘
```

## Database Schema

### Current Schema (Version 2)

```sql
-- Schema version tracking
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY
);

-- PIN verification hash
CREATE TABLE vault_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Encrypted API keys
CREATE TABLE api_keys (
    id TEXT PRIMARY KEY,
    app_name TEXT,                    -- Optional app identifier
    key_name TEXT NOT NULL,           -- Key identifier within app
    encrypted_key BLOB NOT NULL,      -- AES-GCM ciphertext
    nonce BLOB NOT NULL,              -- 12-byte random nonce
    key_salt BLOB NOT NULL,           -- 32-byte per-key salt
    description TEXT,                 -- Optional description
    expires_at TEXT,                  -- Optional expiration
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(app_name, key_name)        -- Prevent duplicates
);
```

### Migration Strategy

1. Schema version is checked on startup
2. Migrations run automatically if needed
3. Database is backed up before migration
4. Application refuses to open newer schema versions

## Service Layer Design

### VaultManager

Central orchestrator that owns all services:

```rust
pub struct VaultManager {
    auth_service: Arc<AuthService>,
    key_service: Arc<KeyService>,
    crypto_service: Arc<CryptoService>,
    storage_service: Arc<StorageService>,
}
```

**Responsibilities:**
- Initialize database connection
- Create and wire up services
- Provide service accessors

### AuthService

Manages authentication and session state:

```rust
pub struct AuthService {
    db: Arc<VaultDb>,
    state: Arc<RwLock<AuthState>>,
}
```

**Methods:**
- `init_vault(pin)` - First-time setup
- `unlock(pin)` - Verify PIN, load master key
- `lock()` - Clear master key from memory
- `is_unlocked()` - Check session state
- `change_pin(old, new)` - Update PIN

### KeyService

Manages API key CRUD operations:

```rust
pub struct KeyService {
    db: Arc<VaultDb>,
    state: Arc<RwLock<AuthState>>,
}
```

**Methods:**
- `create(app, key, value, desc, expires)` - Add new key
- `get(app, key)` - Retrieve and decrypt key
- `list()` - List all keys (metadata only)
- `search(query)` - Search keys
- `update(app, key, changes)` - Update key
- `delete(app, key)` - Remove key

## Error Handling

All library code uses the unified `VaultError` type:

```rust
pub enum VaultError {
    // Authentication
    InvalidPin,
    PinTooShort,
    Locked,
    TooManyAttempts,
    
    // Not Found
    KeyNotFound { app_name: String, key_name: String },
    NotInitialized,
    
    // Conflicts
    DuplicateKey { app_name: String, key_name: String },
    AlreadyInitialized,
    
    // Cryptographic
    DecryptionFailed,
    EncryptionFailed(String),
    
    // Database
    Database(String),
    
    // etc.
}
```

Each variant maps to:
- An exit code (for CLI)
- A user-friendly message
- An optional suggestion

## Feature Flags

```toml
[features]
default = ["gui", "custom-protocol"]
gui = ["tauri", "tauri-plugin-*"]
cli = ["clap", "dialoguer", "comfy-table", "rpassword", "colored"]
```

- **gui**: Tauri desktop application
- **cli**: Command-line interface
- Library compiles without either feature

## Testing Strategy

### Unit Tests

Each service has unit tests in its module:
- `auth_service::tests`
- `key_service::tests`
- `crypto::tests`

### Property-Based Tests

Critical crypto operations are tested with proptest:
- Encryption/decryption roundtrip
- Key derivation determinism
- Per-key uniqueness

### Integration Tests

Full workflow tests in `tests/` directory:
- Init → add → get → delete
- PIN change
- Concurrent access

## Platform Considerations

### Windows
- Database: `%APPDATA%\vult\vault.db`
- Clipboard: Windows Clipboard API

### Linux
- Database: `~/.vult/vault.db`
- Clipboard: X11/Wayland via arboard

## Future Considerations

1. **macOS Support**: Planned for future release
2. **Biometric Authentication**: Windows Hello, Touch ID
3. **Import/Export**: CSV/JSON vault backup
4. **Key Rotation**: Automatic re-encryption on schedule
5. **Audit Logging**: Track key access

---

For more details, see the [rustdoc documentation](../target/doc/vult/index.html) or the [CLI Guide](./CLI_GUIDE.md).

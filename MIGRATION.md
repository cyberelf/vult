# Migration Guide

This guide helps developers migrate their code when updating Vult.

## Migrating from v0.0.x to v0.1.0 (Backend Library Separation)

### Overview

Version 0.1.0 introduces a major architectural change:
- Core logic extracted into a reusable library
- Service layer pattern with `VaultManager`
- CLI binary added for command-line access
- GUI adapted to use the library

### Breaking Changes

#### 1. Database Module No Longer Exported Directly

**Before (v0.0.x):**
```rust
use vult::database::VaultDb;

let db = VaultDb::new("sqlite://path").await?;
db.create_api_key(input, &vault_key).await?;
```

**After (v0.1.0):**
```rust
use vult::services::VaultManager;

let vault = VaultManager::new("sqlite://path").await?;
vault.auth().unlock(pin).await?;
vault.keys().create(app, name, value, url, desc).await?;
```

#### 2. Authentication Pattern Changed

**Before (v0.0.x):**
```rust
use vult::auth::{derive_key_from_pin, generate_salt};

let salt = generate_salt();
let key = derive_key_from_pin(pin, &salt)?;
// Manual key management
```

**After (v0.1.0):**
```rust
use vult::services::VaultManager;

let vault = VaultManager::new(db_url).await?;
vault.auth().init_vault(pin).await?; // First time
vault.auth().unlock(pin).await?;      // Subsequent
let is_unlocked = vault.is_unlocked();
```

#### 3. Key Operations Now Through KeyService

**Before (v0.0.x):**
```rust
db.create_api_key(CreateApiKey { ... }, &vault_key).await?;
let key = db.get_api_key(&id, &vault_key).await?;
```

**After (v0.1.0):**
```rust
let id = vault.keys().create(app, name, value, url, desc).await?;
let key = vault.keys().get_by_id(&id).await?;
```

#### 4. Error Types Unified

**Before (v0.0.x):**
```rust
use vult::database::DbError;
use vult::crypto::CryptoError;
```

**After (v0.1.0):**
```rust
use vult::VaultError; // Single error type for all operations
```

### Feature Flags

New feature flags control binary builds:

**Library only:**
```toml
[dependencies]
vult = { version = "0.1", default-features = false }
```

**With CLI:**
```toml
[dependencies]
vult = { version = "0.1", features = ["cli"] }
```

**With GUI:**
```toml
[dependencies]
vult = { version = "0.1", features = ["gui"] }
```

### GUI-Specific Changes

#### AuthManager Now Wraps VaultManager

**Before (v0.0.x):**
```rust
use vult::database::VaultDb;
use vult::gui::AuthManager;

let db = Arc::new(VaultDb::new(db_url).await?);
let auth = Arc::new(AuthManager::new(db, timeout));
```

**After (v0.1.0):**
```rust
use vult::services::VaultManager;
use vult::gui::AuthManager;

let vault = Arc::new(VaultManager::new(db_url).await?);
let auth = Arc::new(AuthManager::new(vault, timeout));
```

#### Commands Now Delegate to Services

**Before (v0.0.x):**
```rust
#[tauri::command]
pub async fn create_api_key(
    input: CreateApiKey,
    auth: State<'_, Arc<AuthManager>>,
    db: State<'_, Arc<VaultDb>>,
) -> Result<ApiKey> {
    let key = auth.get_vault_key().await?;
    db.create_api_key(input, &key).await
}
```

**After (v0.1.0):**
```rust
#[tauri::command]
pub async fn create_api_key(
    input: CreateApiKey,
    auth: State<'_, Arc<AuthManager>>,
) -> Result<ApiKey> {
    auth.vault()
        .keys()
        .create(input.app, input.name, input.value, input.url, input.desc)
        .await
}
```

### New Capabilities

#### CLI Access

Version 0.1.0 adds a command-line interface:

```bash
vult init                    # Initialize vault
vult add mykey --app github  # Add key
vult get mykey --app github  # Retrieve key
vult list                    # List all keys
```

#### Session Management

CLI supports session mode to avoid re-entering PIN:

```bash
vult add key1 --stay-unlocked   # Creates 5-minute session
vult list                        # Uses session, no PIN needed
vult lock                        # Manually clear session
```

#### Library-First Design

Core vault logic now available as a library:

```rust
use vult::services::VaultManager;

let vault = VaultManager::new("sqlite://vault.db").await?;
vault.auth().init_vault("my-pin").await?;

let id = vault.keys().create(
    Some("github"),
    "token",
    "ghp_abc123",
    Some("https://api.github.com"),
    Some("Personal access token"),
).await?;

let key = vault.keys().get_by_id(&id).await?;
println!("Value: {}", key.key_value);
```

### Migration Checklist

- [ ] Update `vult` dependency to `0.1.0`
- [ ] Replace `VaultDb` imports with `VaultManager`
- [ ] Update authentication pattern to use `vault.auth()`
- [ ] Update key operations to use `vault.keys()`
- [ ] Replace `DbError` and `CryptoError` with `VaultError`
- [ ] Update GUI `AuthManager` initialization (if using GUI)
- [ ] Update Tauri commands to delegate to services (if using GUI)
- [ ] Run tests to verify migration
- [ ] Review new CLI capabilities for automation opportunities

### Support

If you encounter migration issues:
- Check [ARCHITECTURE.md](ARCHITECTURE.md) for design details
- Review [examples/](examples/) for usage patterns
- Open an issue on GitHub

### Database Compatibility

**Good news:** Database format is backward compatible!

- v0.1.0 databases work with v0.0.x (no schema changes for core tables)
- Migrations run automatically on startup
- Backups created before migrations

You can run v0.0.x GUI and v0.1.0 CLI against the same database.

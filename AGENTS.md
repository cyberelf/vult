# Vult - AI Assistant Instructions

Instructions for AI assistants working on the Vult project.

## Quick Project Overview

**Vult** is a cross-platform secure API key vault built with:
- **Backend**: Rust + Tauri v2
- **Frontend**: Vanilla JavaScript + HTML + CSS
- **Database**: SQLite with SQLx
- **Security**: Argon2id + AES-256-GCM encryption

**Current Version**: 0.1.0

## Project Architecture

```
vult/
├── src/                    # Rust backend (Tauri commands)
│   ├── main.rs            # Application entry point, DB path setup
│   ├── auth.rs            # PIN validation, session management, auto-lock
│   ├── commands.rs        # Tauri command handlers (IPC)
│   ├── crypto.rs          # Argon2id, AES-GCM, per-key encryption
│   ├── database.rs        # SQLite operations, migrations, schema versioning
│   └── clipboard.rs       # Clipboard auto-clear (45s timeout)
├── ui/                     # Frontend UI
│   ├── index.html         # Main HTML structure
│   ├── app.js             # Application logic, state management
│   └── styles.css         # Dark theme styling
├── capabilities/          # Tauri capabilities (IPC allowlists)
├── gen/schemas/           # Generated Tauri schemas
└── openspec/              # Spec-driven development (see below)
```

## Key Technical Decisions

### Security
- **No PIN Recovery**: By design - if you forget your PIN, your data is permanently inaccessible
- **Per-Key Encryption**: Each API key encrypted with unique derived key (master key + app_name + key_name + salt)
- **In-Memory Only**: Decrypted keys never written to disk
- **Auto-Lock**: 5 minutes of inactivity (configurable in `src/main.rs`)

### Database
- **Location**: `~/.vult/vault.db`
- **Schema Version**: Currently v2 (tracked in `schema_version` table)
- **Migrations**: Automatic on app startup with backup protection
- **Version Guard**: Blocks opening databases with newer schema versions

### Frontend
- **State Management**: Simple in-memory objects (`allKeys`, `keyVisibility`, `keyEditStates`, `keyData`)
- **Table-Based UI**: Inline editing, show/hide toggle, copy to clipboard
- **No Framework**: Vanilla JS for simplicity and security auditability

## Common Tasks

### Adding a New Tauri Command
1. Add function to `src/commands.rs` with `#[tauri::command]`
2. Register in `src/main.rs` invoke_handler
3. Add to `capabilities/` (allowlist)
4. Call from frontend: `invoke('command_name', { param: value })`

### Database Schema Changes
1. Increment `SCHEMA_VERSION` in `src/database.rs`
2. Add migration case in `run_migration()`
3. Update `EncryptedApiKeyRow` struct if needed
4. Add tests for migration
5. Document in `CHANGELOG.md`

### Testing
```bash
# Run all tests
cargo test

# Run specific module
cargo test --lib database
cargo test --lib crypto
cargo test --lib auth

# Run with output
cargo test -- --nocapture
```

## OpenSpec Instructions

When making changes that involve:
- New features or functionality
- Breaking changes (API, schema, security)
- Architecture changes
- Performance optimizations
- Security pattern updates

Use the OpenSpec workflow below:

## Code Conventions

### Rust
- Use `Result<T>` type aliases for error handling
- Prefer `thiserror` for custom error types
- Use `zeroize` for sensitive data (PINs, keys)
- Follow standard Rust formatting (`cargo fmt`)
- Run clippy before committing (`cargo clippy -- -D warnings`)

### Database
- All migrations must be backward-compatible or include data migration
- Test with both empty and populated databases
- Always clean up orphaned tables (e.g., `api_keys_v2`, `api_keys_new`)

### Frontend
- Use `escapeHtml()` for all user-generated content
- Call `update_activity()` before sensitive operations
- Clear sensitive data on lock (`keyData = {}`)

## Security Considerations

### Never Do
- ❌ Log PINs or decrypted keys
- ❌ Store decrypted keys in variables longer than needed
- ❌ Write sensitive data to disk without encryption
- ❌ Skip validation for "convenience"
- ❌ Remove the 6-character minimum PIN requirement

### Always Do
- ✅ Use per-key encryption
- ✅ Clear clipboard after timeout
- ✅ Update activity on user interaction
- ✅ Validate all inputs
- ✅ Test encryption/decryption roundtrips

## Dependencies

Key dependencies and their purposes:
- `tauri 2.1` - Desktop framework
- `sqlx 0.8` - Async SQL toolkit for SQLite
- `argon2 0.5` - Password hashing (PIN → master key)
- `aes-gcm 0.10` - Authenticated encryption
- `zeroize 1.8` - Secure memory clearing
- `arboard 3.4` - Clipboard management
- `thiserror 2.0` - Error handling
- `chrono 0.4` - Timestamps

## Debugging

### Enable Tauri DevTools
Already enabled in `Cargo.toml`:
```toml
tauri = { version = "2.1", features = ["devtools"] }
```
Press `F12` in the running app to open dev tools.

### Database Inspection
```bash
# Open database
sqlite3 ~/.vult/vault.db

# View tables
.tables

# View schema
.schema

# Query API keys (encrypted)
SELECT id, app_name, key_name FROM api_keys;

# Check schema version
SELECT * FROM schema_version;
```

### Log Output
The app uses `eprintln!` for logging to stderr, visible in the terminal when running `cargo tauri dev`.

## Common Issues

### "no such table: api_keys"
- Database needs migration
- Check `schema_version` table exists
- Run migrations on startup

### "table api_keys has no column named key_salt"
- Old database schema (v1)
- Migration should run automatically
- If fails, delete `~/.vult/vault.db` and start fresh (data loss!)

### Per-Key Encryption Issues
- Each key has unique `key_salt` (32 bytes)
- Derive per-key key: `derive_per_key_encryption_key(master_key, app_name, key_name, salt)`
- Old migrated keys may have all-zero salts (call `reencrypt_all_keys()`)


## Take Record of Lessons

When finding yourself making simple mistakes or repetitive failures, take note in [LESSONS.md](LESSONS.md) for future reference.

## Version Release Checklist

When preparing a new release:
- [ ] Update version in `Cargo.toml`
- [ ] Update version in `tauri.conf.json`
- [ ] Add changelog entry to `CHANGELOG.md`
- [ ] Run full test suite: `cargo test`
- [ ] Build release: `cargo tauri build`
- [ ] Test built application
- [ ] Update README.md if needed
- [ ] Create git tag: `git tag v0.x.0`
- [ ] Push tag: `git push --tags`

## Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Argon2 Specifications](https://github.com/P-H-C/phc-winner-argon2)
- [OpenSpec Workflow](openspec/AGENTS.md)

## License

MIT License - see [LICENSE](LICENSE) for details.

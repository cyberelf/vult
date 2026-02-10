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
│   ├── main.rs            # GUI entry point
│   ├── lib.rs             # Library entry point
│   ├── core/              # Core types, constants, validation
│   │   └── types.rs       # PIN validation, constants
│   ├── services/          # Business logic layer
│   │   ├── vault_manager.rs   # Main orchestrator
│   │   ├── auth_service.rs    # Authentication (CLI/library)
│   │   ├── key_service.rs     # Key CRUD operations
│   │   └── crypto_service.rs  # Encryption wrapper
│   ├── gui/               # GUI-specific (feature-gated)
│   │   ├── auth_manager.rs    # AuthManager with Tauri events
│   │   └── commands.rs        # Tauri IPC command handlers
│   ├── crypto.rs          # Argon2id, AES-GCM, per-key encryption
│   ├── database.rs        # SQLite operations, migrations
│   ├── clipboard.rs       # Clipboard auto-clear (45s timeout)
│   └── bin/
│       ├── vult.rs        # CLI binary
│       └── vult-gui.rs    # GUI binary (Tauri)
├── ui-sveltekit/          # Frontend (SvelteKit + Tailwind)
├── capabilities/          # Tauri capabilities (IPC allowlists)
├── gen/schemas/           # Generated Tauri schemas
└── openspec/              # Spec-driven development
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

## Ad-Hoc Question Workflow

When responding to random user questions (not part of OpenSpec workflow), follow this quality control process:

### Step 1: Identify Question Type

Classify the question into one of these categories:

1. **Information Request**: User asking about code, architecture, documentation
   - No code changes needed
   - Provide answer with references
   - Skip to quality checks unnecessary

2. **Code Modification Request**: User asking to fix, improve, or add code
   - Requires code changes
   - **MUST follow quality gates below**
   - Examples: "fix this bug", "add this feature", "improve this UI"

3. **Investigation Request**: User asking to debug or analyze
   - May or may not require changes
   - If changes made, apply quality gates

### Step 2: For Code Modifications - Apply Quality Gates

**MANDATORY after ANY code change:**

**Quick Check Script, linting formatting and type checking** 
```bash
# One-line quality check for backend
./scripts/quick_check.sh backend

# One-line quality check for frontend
./scripts/quick_check.sh frontend

# Check everything
./scripts/quick_check.sh all
```

**Manual Quality Gates, related tests**:

```bash
# Run related tests for backend changes
cargo test --features "cli gui" --lib <relevant_module>

# run related tests for frontend changes
cd ui-sveltekit && npm run test -- <relevant_test_file>
```

### Step 3: Report Quality Gate Results

**If quality gates pass:**
- Provide "✅ Quality checks passed" summary
- List what was checked (clippy, tests, build)

**If quality gates fail:**
- **STOP and fix issues before continuing**
- Show errors to user
- Fix issues
- Re-run quality gates
- Only proceed after all gates pass


### Quality Gate Checklist by Change Type

| Change Type | Required Checks |
|------------|----------------|
| Rust source files | `cargo clippy`, `cargo check`, `cargo test` |
| Tauri commands | `cargo clippy`, frontend build, type check |
| Frontend files | `npm run build` (type check recommended but not blocking) |
| Database schema | Migration tests, integration tests |
| Security code | Full test suite + security tests |
| Documentation only | None (but verify markdown syntax) |

**Note**: Frontend `npm run check` (strict type checking) currently has known issues documented in [docs/FRONTEND_TYPE_ISSUES.md](docs/FRONTEND_TYPE_ISSUES.md). Production build (`npm run build`) succeeds and is the primary quality gate for frontend changes. Strive to fix type errors but don't block commits if only type check fails while build succeeds.

### When to Skip Quality Gates

Only skip quality gates for:
- Pure documentation edits (README, AGENTS.md, comments)
- Configuration file updates (without code impact)
- Simple typo fixes in strings

**Constitution**: For any code modification, running quality gates is MANDATORY, not optional. Skipping quality checks violates project standards and can introduce bugs.

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

### Backend-Frontend API Consistency

**CRITICAL**: The Rust backend and TypeScript frontend communicate via Tauri IPC. Type mismatches cause silent failures that are hard to debug.

#### Mandatory Rules for Tauri Commands

1. **Always use `CommandResponse<T>` wrapper**:
   ```rust
   // ✅ CORRECT - Consistent response format
   #[tauri::command]
   pub async fn my_command() -> Result<CommandResponse<MyData>, String> {
       Ok(CommandResponse::success(data))
   }
   
   // ❌ WRONG - Raw data breaks frontend expectations
   #[tauri::command]
   pub async fn my_command() -> Result<MyData, String> {
       Ok(data)
   }
   ```

2. **Always use `snake_case` serialization for shared enums**:
   ```rust
   // ✅ CORRECT - Frontend expects snake_case
   #[derive(Serialize, Deserialize)]
   #[serde(rename_all = "snake_case")]
   pub enum MyEnum {
       Available,           // Serializes as "available"
       NotConfigured,       // Serializes as "not_configured"
   }
   
   // ❌ WRONG - Serializes as "Available", "NotConfigured" (PascalCase)
   #[derive(Serialize, Deserialize)]
   pub enum MyEnum {
       Available,
       NotConfigured,
   }
   ```

3. **Keep TypeScript types synchronized**:
   - After adding/modifying Rust types, immediately update `ui-sveltekit/src/lib/types/api.ts`
   - Use exact same casing convention (snake_case for enums, camelCase for fields)
   - Document the Rust source in TypeScript comments

4. **Frontend must unwrap `CommandResponse`**:
   ```typescript
   // ✅ CORRECT - Unwrap the response wrapper
   export async function myCommand(): Promise<MyData> {
     const response = await invoke<CommandResponse<MyData>>('my_command');
     if (!response.success || !response.data) {
       throw new Error(response.error || 'Command failed');
     }
     return response.data;
   }
   
   // ❌ WRONG - Returns wrapped data
   export async function myCommand(): Promise<MyData> {
     return await invoke<MyData>('my_command');
   }
   ```

5. **Test at the API boundary**:
   ```bash
   # After adding new Tauri commands:
   # 1. Test backend
   cargo test
   
   # 2. Build frontend to catch type errors
   cd ui-sveltekit && npm run build
   
   # 3. Run dev to verify runtime behavior
   cargo tauri dev
   ```

#### Common Pitfalls

- 🔴 **Enum serialization mismatch**: Rust PascalCase vs TypeScript snake_case
- 🔴 **Missing CommandResponse wrapper**: Frontend expects wrapper but command returns raw data
- 🔴 **Feature flags not enabled**: Adding GUI-required features but forgetting to include in `gui` feature
- 🔴 **Type drift**: Updating Rust types without updating TypeScript definitions

#### Verification Checklist

When adding new Tauri commands, verify:
- [ ] Command returns `CommandResponse<T>`
- [ ] Shared enums have `#[serde(rename_all = "snake_case")]`
- [ ] TypeScript types match Rust types (check casing!)
- [ ] Frontend unwraps the response wrapper
- [ ] Frontend build completes without errors
- [ ] Feature flags include all required dependencies

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

## Windows Hello Integration

Vult supports optional biometric authentication using Windows Hello on Windows 10 (1903+) and Windows 11.

### Key Points
- **Optional Feature**: Enabled via `windows-biometric` cargo feature flag
- **PIN Fallback**: Windows Hello failure automatically falls back to PIN authentication
- **User Control**: Users can enable/disable Windows Hello in settings
- **Platform-Specific**: Windows-only via `windows-rs` crate
- **No Data Storage**: Biometric templates never leave Windows Hello, Vult only receives yes/no verification

### Architecture
- **BiometricProvider Trait**: Platform-agnostic abstraction in `src/core/types.rs`  
- **WindowsHelloProvider**: Windows implementation in `src/biometric/windows_hello.rs`
- **MockBiometricProvider**: Test doubles in `src/biometric/mock.rs`
- **AuthService Integration**: Biometric methods available when provider is set
- **GUI Integration**: Settings toggle + unlock screen biometric button (SvelteKit)

### Testing
```bash
# Run biometric unit tests (Windows Hello result mapping)
cargo test --lib biometric

# Run integration tests (requires windows-biometric feature)
cargo test --features windows-biometric --test biometric_integration_test

# Run biometric availability tests
cargo test --test biometric_availability_test

# Mock provider allows testing without real hardware
```

### Feature Configuration
**IMPORTANT**: The `windows-biometric` feature is automatically enabled when building the GUI:

```toml
# In Cargo.toml
[features]
gui = ["dep:tauri", "dep:tauri-plugin-shell", "dep:tauri-build", "custom-protocol", "windows-biometric"]
```

This means:
- `cargo tauri dev` - Windows Hello is enabled ✓
- `cargo tauri build` - Windows Hello is enabled ✓
- `cargo build --bin vult-gui` - Windows Hello is enabled ✓
- `cargo build --bin vult` (CLI) - Windows Hello is NOT enabled (as expected)

### Troubleshooting
If Windows Hello shows "Not supported on this system":

1. **Check feature flag is compiled in**:
   ```bash
   # Should show "[DEBUG] windows-biometric feature is ENABLED" in logs
   cargo run --bin vult-gui
   ```

2. **Verify tests pass**:
   ```bash
   cargo test --test biometric_availability_test -- --nocapture
   # Should show "Available" or "NotConfigured", never "NotSupported" if feature is enabled
   ```

3. **Common issues**:
   - Feature not in default build → Fix: Already included in `gui` feature
   - Provider not initialized → Fix: VaultManager.new() creates provider when feature enabled
   - Frontend not unwrapping CommandResponse → Fix: checkBiometricAvailable() unwraps .data

### Common Scenarios
- **Biometric Available**: Show Windows Hello button on unlock screen
- **Not Configured**: Show message prompting user to set up Windows Hello in Windows Settings
- **Hardware Missing**: Fall back to PIN-only mode
- **Verification Failed**: Keep unlock screen open, allow PIN entry

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
- `windows 0.58` - Windows Hello API bindings (feature: `windows-biometric`)
- `async-trait 0.1` - Async trait support for BiometricProvider trait objects

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

## Security Architecture Principles

### Layer Separation and Responsibility

Vult's security architecture follows strict separation of concerns across three layers:

**Service Layer** (`src/services/`):
- **AuthService**: PIN validation, vault unlock/lock, master key management, authentication state
- **CryptoService**: Encryption/decryption operations, key derivation
- **KeyService**: API key CRUD operations requiring unlocked vault

**Platform Layer** (feature-gated in `src/biometric/`, `src/gui/`):
- Platform-specific secure credential storage (DPAPI, Keychain, etc.)
- Biometric provider abstractions
- GUI-specific authentication state management

**Data Layer** (`src/database.rs`, `src/crypto.rs`):
- Encrypted storage operations
- Schema migrations with data protection
- Low-level cryptographic primitives

**Constitution**: Security features MUST respect layer boundaries. Service layer must not contain platform-specific code. Platform layer must not implement core authentication logic. Data layer must not make security policy decisions.

### Authentication Pattern Requirements

When implementing any authentication mechanism (PIN, biometric, hardware key, etc.), the complete system requires these components:

1. **Enrollment/Setup Flow**
   - User registers authentication credential
   - System validates and stores credential securely
   - Clear error handling for setup failures

2. **Credential Storage**
   - Encrypted at rest using platform-native APIs
   - Per-vault isolation for multi-database support
   - Automatic cleanup on disable/uninstall

3. **Verification and Retrieval**
   - Authenticate user identity
   - Retrieve stored credential only after successful verification
   - Pass credential to core authentication system

4. **Lifecycle Management**
   - Enable/disable mechanism
   - Update stored credentials on change
   - Revoke access on security events

5. **Fallback and Recovery**
   - Alternative authentication methods available
   - Clear user communication on failure
   - No permanent lockout from legitimate users

**Constitution**: Authentication features are complete systems, not individual functions. Missing any component renders the feature broken or insecure. Design all five components before implementation.

### Secure Credential Storage

**Platform-Native Approach**: Use operating system's secure storage mechanisms:
- **Windows**: DPAPI (Data Protection API) for user-level encryption
- **macOS**: Keychain Services with appropriate access controls  
- **Linux**: Secret Service API via libsecret or gnome-keyring

**Constitution Rules**:
- NEVER store authentication credentials in plaintext
- NEVER invent custom encryption schemes for credentials
- ALWAYS use platform-native secure storage APIs
- ALWAYS scope storage per-vault (use database path for isolation)
- ALWAYS validate credentials before storage (prevent storing incorrect credentials)
- ALWAYS clear credentials from memory immediately after use

### Security Feature Design Process

Before implementing security-critical features (authentication, encryption, access control):

**Phase 1: Architecture Design**
- Document complete flow with sequence diagrams
- Identify all required components and their responsibilities
- Map to existing service/platform/data layer structure
- Define data storage locations and encryption requirements

**Phase 2: Threat Modeling**
- Document intended security properties
- Identify attack vectors and mitigation strategies
- Define trust boundaries and assumptions
- Plan error handling and edge cases

**Phase 3: Platform Research**
- Research platform-standard implementations
- Identify OS-provided security APIs and best practices
- Select appropriate Rust crates with security audits
- Document platform-specific limitations

**Phase 4: Implementation**
- Implement with clear layer separation
- Use feature flags for platform-specific code
- Add comprehensive error handling
- Implement complete user flows (not just happy path)

**Phase 5: Testing**
- Unit tests for each component
- Integration tests for complete flows (setup → use → disable)
- Security property tests (encryption verification, isolation checks)
- Edge case and error path coverage

**Constitution**: Security features require complete design before coding. Skipping design phases leads to incomplete or insecure implementations.

### Testing Requirements for Security Features

Security features MUST include:

**Integration Tests**: Complete user journeys
- Setup/enrollment flow
- Normal operation with authentication
- Failure scenarios (wrong credential, hardware unavailable)
- Disable/revoke flow
- Isolation between multiple vaults

**Security Property Tests**: Verify security guarantees
- Credentials encrypted at rest
- Credentials cleared from memory after use
- Authentication fails with incorrect credentials
- Fallback mechanisms work correctly
- No credential leakage in logs/errors

**Platform Coverage**: Test on all supported platforms
- Feature availability detection
- Platform-specific error handling
- Graceful degradation when hardware unavailable

**Constitution**: Security features without comprehensive tests are considered incomplete and MUST NOT be merged.

### State Management for Authentication

Vult maintains authentication state at multiple levels:

**Core Authentication State** (AuthService):
- `is_unlocked: bool` - Whether vault operations are permitted
- `vault_key: Option<VaultKey>` - Master key for encryption (in-memory only)
- `failed_attempts: u32` - Rate limiting counter

**GUI Authentication State** (AuthManager, feature-gated):
- Auto-lock timer management
- Activity tracking
- Cross-component state synchronization via Tauri events

**Frontend State** (ui-sveltekit):
- Screen routing (setup/unlock/vault)
- Biometric availability and preferences
- User-facing loading and error states

**Constitution**: Authentication state transitions MUST flow from service layer → GUI layer → frontend. Frontend cannot directly modify core authentication state. All state changes emit appropriate events for synchronization.

### Cryptographic Operations

**Service Organization**:
- `CryptoService`: High-level operations (derive keys, encrypt/decrypt API keys)
- `crypto.rs`: Low-level primitives (Argon2id, AES-GCM, salt generation)
- Never expose raw cryptographic primitives to frontend or GUI layer

**Key Derivation**:
- Master key: `derive_master_key(pin, salt)` using Argon2id
- Per-key encryption: `derive_per_key_encryption_key(master_key, app_name, key_name, salt)`
- All key material cleared with `zeroize` after use

**Constitution**: 
- Cryptographic operations MUST occur in service or crypto layer only
- Key material MUST NEVER leave Rust code (no serialization to frontend)
- All sensitive data MUST use `zeroize` on drop
- Use standard algorithms (Argon2id, AES-GCM) - no custom crypto

### Error Handling for Security Operations

**Error Type Hierarchy**:
- Use `VaultError` enum for semantic error types
- Security errors (InvalidPin, BiometricFailed, Locked) distinct from system errors
- Never expose internal details in error messages to frontend

**Rate Limiting and Attack Prevention**:
- Track failed authentication attempts
- Implement exponential backoff (2^attempts seconds)
- Clear counter on successful authentication
- Log security events to stderr for audit

**Constitution**: 
- Security error messages MUST be generic (don't leak information)
- Failed attempts MUST be rate-limited
- Security events MUST be logged for audit
- Errors MUST NOT expose sensitive data or internal state

### Multi-Vault Support Considerations

Vult supports multiple vault databases with isolated credentials:

**Isolation Requirements**:
- Each vault has independent authentication state
- Credential storage scoped by database path (hash-based isolation)
- No cross-vault credential reuse
- Auto-lock applies per-vault

**Implementation Pattern**:
- VaultManager instantiated per database path
- Services receive `Arc<VaultDb>` with `db_path` field
- Credential stores use database path for storage key derivation
- Frontend manages active vault selection

**Constitution**: Vault isolation MUST be maintained at all layers. Credential storage MUST NOT be shared between vaults. Testing MUST verify multi-vault isolation.


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

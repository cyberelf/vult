# Design: Backend Library and CLI Separation

## Context

**Problem**: The current Vult architecture tightly couples core vault logic with the Tauri GUI framework, limiting reusability and access patterns.

**Constraints**:
- Must maintain backward compatibility with existing GUI functionality
- Must share same vault database between GUI and CLI
- Must preserve existing security model (encryption, authentication)
- Library must be framework-agnostic (no Tauri dependencies)
- Both binaries must be cross-platform (Windows and Linux only)

**Stakeholders**:
- **End users**: Want CLI access for automation and scripting
- **Power users**: Need command-line integration for workflows
- **Developers**: Want cleaner architecture and testability
- **Future integrations**: Library enables mobile apps, browser extensions, API servers

## Goals / Non-Goals

### Goals
- Extract core vault logic into reusable Rust library
- Provide fully-functional CLI binary with all vault operations
- Maintain existing GUI functionality without breaking changes
- Enable testing library independently of GUI
- Support both interactive (GUI) and scriptable (CLI) workflows
- Share vault database between GUI and CLI seamlessly

### Non-Goals
- Web-based interface (not in this change)
- REST API server (future consideration)
- Mobile app integration (future, but library enables it)
- Changing existing security model or encryption
- GUI redesign or feature additions
- Breaking changes to database schema

## Decisions

### Decision 1: Service Layer Architecture

**Choice**: Introduce service layer (`VaultManager`, `AuthService`, `KeyService`) as public API

**Rationale**:
- Clean separation of concerns (services orchestrate, modules implement)
- Services can be tested independently
- Provides natural boundary for library vs binary
- Matches common architectural patterns in Rust ecosystems
- Services can manage cross-cutting concerns (transactions, logging)

**Structure**:
```rust
// Library exports
pub struct VaultManager {
    auth: Arc<AuthService>,
    keys: Arc<KeyService>,
    crypto: Arc<CryptoService>,
    storage: Arc<StorageService>,
}

// Services coordinate and provide high-level operations
impl VaultManager {
    pub async fn new(db_path: &str) -> Result<Self, VaultError>;
    pub fn auth(&self) -> &AuthService;
    pub fn keys(&self) -> &KeyService;
}
```

**Alternatives Considered**:
- **Flat module exports**: Rejected - harder to coordinate, less ergonomic API
- **Trait-based abstraction**: Over-engineering for current needs
- **Facade pattern only**: Rejected - still couples GUI to implementation

### Decision 2: Binary Structure - src/bin/

**Choice**: Move binaries to `src/bin/` directory with separate entry points

**Rationale**:
- Standard Rust convention for multi-binary crates
- Cargo automatically builds all binaries in `src/bin/`
- Clear separation: library in `src/`, binaries in `src/bin/`
- Each binary can have its own dependencies (feature flags)
- Simplifies build configuration

**File Structure**:
```
src/
├── lib.rs                    # Library root, exports public API
├── services/
│   ├── mod.rs
│   ├── vault_manager.rs
│   ├── auth_service.rs
│   └── key_service.rs
├── auth.rs                   # Core auth logic (pub)
├── crypto.rs                 # Crypto operations (pub)
├── database.rs               # Storage layer (pub)
├── clipboard.rs              # Clipboard utilities (pub)
├── error.rs                  # Error types (pub)
├── commands.rs               # GUI adapter layer (private to GUI)
└── bin/
    ├── vult-gui.rs          # Tauri GUI binary
    └── vult.rs              # CLI binary
```

**Alternatives Considered**:
- **Separate workspace crates**: Too heavy for this use case
- **Keep main.rs**: Confusing which binary is which
- **Single binary with subcommands**: Bloats GUI with CLI deps

### Decision 3: CLI Framework - clap + dialoguer

**Choice**: Use `clap` v4 for CLI structure and `dialoguer` for interactive prompts

**Rationale**:
- **clap v4**: Industry standard, derives pattern, excellent error messages
- **dialoguer**: Secure password input, user-friendly prompts
- Combined: Support both scriptable (`vult get app key`) and interactive modes
- Cross-platform terminal support
- Well-maintained, widely used crates

**CLI Structure**:
```rust
#[derive(Parser)]
#[command(name = "vult", about = "Secure API key vault CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(long, global = true)]
    stay_unlocked: bool,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add { app_name: String, key_name: String },
    Get { app_name: String, key_name: String, #[arg(long)] copy: bool },
    List { #[arg(long)] json: bool },
    // ...
}
```

**Alternatives Considered**:
- **structopt**: Deprecated in favor of clap v3+
- **argh**: Too minimal for our needs
- **Custom parser**: Reinventing the wheel

### Decision 4: Error Handling - Unified VaultError

**Choice**: Create library-level `VaultError` enum, binaries adapt to their context

**Rationale**:
- Library can't return Tauri-specific errors
- CLI needs to map to exit codes and user messages
- Unified error type simplifies library implementation
- Variants cover all failure modes (Auth, Crypto, Database, InvalidState, etc.)
- Can serialize for IPC or log appropriately

**Implementation**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("Authentication failed: {0}")]
    Auth(String),
    
    #[error("Vault is locked")]
    Locked,
    
    #[error("Key not found: {0}")]
    NotFound(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Encryption error: {0}")]
    Crypto(String),
    
    // ...
}

// GUI adapter
impl From<VaultError> for tauri::Error {
    fn from(e: VaultError) -> Self {
        tauri::Error::Anyhow(anyhow::anyhow!(e))
    }
}

// CLI adapter
impl From<VaultError> for ExitCode {
    fn from(e: VaultError) -> Self {
        match e {
            VaultError::Auth(_) => ExitCode::from(1),
            VaultError::NotFound(_) => ExitCode::from(2),
            VaultError::Locked => ExitCode::from(3),
            _ => ExitCode::from(1),
        }
    }
}
```

**Alternatives Considered**:
- **anyhow everywhere**: Loses type information, harder to handle
- **Multiple error types**: More complex, unnecessary granularity

### Decision 5: State Management - Context-Aware

**Choice**: GUI maintains long-lived state, CLI creates ephemeral or session-based state

**Rationale**:
- GUI lifecycle: app runs for hours, state persists
- CLI lifecycle: command runs for seconds, state disposed
- Optional CLI session mode with timeout for multi-command workflows
- Each context owns its state management strategy

**GUI State** (unchanged from current):
```rust
// In vult-gui.rs
tauri::Builder::default()
    .manage(Arc::new(vault_manager))
    .manage(Arc::new(auth_manager))  // GUI-specific wrapper
    .invoke_handler(...)
```

**CLI State** (new):
```rust
// In vult.rs
async fn main() {
    let vault = VaultManager::new(&get_db_path()).await?;
    
    match cli.command {
        Commands::Get { app_name, key_name, copy } => {
            let pin = prompt_pin()?;
            vault.auth().unlock(&pin).await?;
            let key = vault.keys().get(&app_name, &key_name).await?;
            if copy {
                clipboard::copy(&key.value)?;
            } else {
                println!("{}", key.value);
            }
        }
        // ...
    }
}
```

**Session Mode** (optional):
- CLI can save auth token to temporary file (e.g., `/tmp/vult-session-{pid}`)
- Subsequent commands reuse token if not expired (5 min timeout)
- Explicit `vult lock` or timeout clears session
- Security trade-off: convenience vs. exposure

**Alternatives Considered**:
- **Always session-based CLI**: Too permissive, security risk
- **Never session CLI**: Too cumbersome for multi-step workflows
- **Shared state via IPC**: Complexity not worth it

### Decision 6: Authentication Flow - Caller Responsibility

**Choice**: Library provides auth methods, binaries handle PIN collection

**Rationale**:
- Library can't assume input method (GUI dialog vs terminal prompt)
- Separation of concerns: library validates, binaries collect
- Enables testing with pre-provided PINs
- Each binary can implement UX appropriate to its context

**Library API**:
```rust
impl AuthService {
    pub async fn init_vault(&self, pin: &str) -> Result<(), VaultError>;
    pub async fn unlock(&self, pin: &str) -> Result<(), VaultError>;
    pub async fn lock(&self) -> Result<(), VaultError>;
    pub fn is_unlocked(&self) -> bool;
    pub async fn change_pin(&self, old_pin: &str, new_pin: &str) -> Result<(), VaultError>;
}
```

**GUI Usage**:
```rust
// Tauri command in commands.rs
#[tauri::command]
async fn unlock_vault(
    vault: State<'_, Arc<VaultManager>>,
    pin: String,
) -> Result<(), String> {
    vault.auth().unlock(&pin).await
        .map_err(|e| e.to_string())
}
```

**CLI Usage**:
```rust
// In vult.rs
let pin = Password::new()
    .with_prompt("Enter PIN")
    .interact()?;

vault.auth().unlock(&pin).await?;
```

### Decision 7: Output Formatting - Multi-Format Support

**Choice**: CLI supports human-readable (default), JSON (`--json`), and raw value modes

**Rationale**:
- **Human-readable**: Default for interactive use (tables with `comfy-table`)
- **JSON**: Machine-readable for scripts and automation
- **Raw value**: `vult get` outputs just the key for piping (`echo $(vult get github token)`)
- Flexibility enables diverse workflows

**Implementation**:
```rust
match output_format {
    OutputFormat::HumanReadable => {
        let mut table = Table::new();
        table.set_header(vec!["App", "Key", "Created"]);
        for key in keys {
            table.add_row(vec![key.app_name, key.key_name, format_date(key.created_at)]);
        }
        println!("{}", table);
    }
    OutputFormat::Json => {
        println!("{}", serde_json::to_string_pretty(&keys)?);
    }
    OutputFormat::Raw => {
        // Only for single value operations like 'get'
        println!("{}", value);
    }
}
```

**Alternatives Considered**:
- **JSON only**: Poor UX for interactive use
- **Custom format**: Users expect standard formats

## Architecture

### Before (Current)
```
┌─────────────────────────────────────────────┐
│           Tauri GUI Application             │
│  ┌─────────────────────────────────────┐   │
│  │        main.rs (entry point)        │   │
│  └─────────────────────────────────────┘   │
│  ┌─────────────────────────────────────┐   │
│  │          commands.rs                │   │
│  │  (Tauri commands, tightly coupled)  │   │
│  └─────────────────────────────────────┘   │
│  ┌──────┬──────┬──────────┬───────────┐   │
│  │ auth │crypto│ database │ clipboard │   │
│  │  .rs │  .rs │   .rs    │    .rs    │   │
│  └──────┴──────┴──────────┴───────────┘   │
└─────────────────────────────────────────────┘
```

### After (Proposed)
```
┌─────────────────────────────────────────────────────────────────┐
│                      vult Library (src/lib.rs)                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   VaultManager                            │  │
│  │         (Orchestrates services, public API)               │  │
│  └───────────────────────────────────────────────────────────┘  │
│  ┌────────────┬────────────┬──────────────┬─────────────────┐  │
│  │AuthService │ KeyService │ CryptoService│ StorageService  │  │
│  │            │            │              │                 │  │
│  └────────────┴────────────┴──────────────┴─────────────────┘  │
│  ┌──────┬──────┬──────────┬───────────┬────────────────────┐  │
│  │ auth │crypto│ database │ clipboard │ error.rs           │  │
│  │  .rs │  .rs │   .rs    │    .rs    │ (VaultError)       │  │
│  └──────┴──────┴──────────┴───────────┴────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
               │                                    │
               ▼                                    ▼
┌─────────────────────────────┐    ┌─────────────────────────────┐
│    vult-gui Binary          │    │    vult CLI Binary          │
│  (src/bin/vult-gui.rs)      │    │  (src/bin/vult.rs)          │
│                             │    │                             │
│  ┌───────────────────────┐  │    │  ┌───────────────────────┐  │
│  │   Tauri Setup         │  │    │  │   clap Parser         │  │
│  │   commands.rs adapter │  │    │  │   Subcommands         │  │
│  └───────────────────────┘  │    │  └───────────────────────┘  │
│  ┌───────────────────────┐  │    │  ┌───────────────────────┐  │
│  │   UI State (Arc)      │  │    │  │   dialoguer prompts   │  │
│  │   Auto-lock thread    │  │    │  │   Output formatting   │  │
│  └───────────────────────┘  │    │  └───────────────────────┘  │
└─────────────────────────────┘    └─────────────────────────────┘
               │                                    │
               └──────────────┬─────────────────────┘
                              ▼
                    ~/.vult/vault.db (shared)
```

### Data Flow Examples

**GUI: Create Key**
```
User → UI Form → Tauri IPC → commands::create_api_key()
    → VaultManager.keys().create() → KeyService
        → AuthService (check unlocked) → CryptoService.encrypt()
            → StorageService.store() → SQLite
```

**CLI: Get Key**
```
User → `vult get github token` → clap parse
    → VaultManager.new() → prompt_pin() → vault.auth().unlock()
        → vault.keys().get("github", "token")
            → StorageService.retrieve() → CryptoService.decrypt()
                → Output to terminal or clipboard
```

## Security Considerations

### Threat Model (No Changes)
- Library maintains same security properties as current implementation
- Both binaries use same encryption, authentication, storage
- CLI inherits security model: encrypted at rest, authenticated access, auto-clear clipboard

### Additional CLI Security Concerns

**Environment Variable PIN (`VULT_PIN`)**:
- **Risk**: PIN visible in process list, shell history
- **Mitigation**: Display warning when used, document as insecure
- **Use Case**: CI/CD where secret management is handled externally

**Session Mode**:
- **Risk**: Longer exposure window if session persists
- **Mitigation**: 5-minute timeout, require explicit opt-in, clear session on lock
- **Trade-off**: Convenience vs. security (user choice)

**Terminal Output**:
- **Risk**: Key values visible in terminal, may be logged
- **Mitigation**: Use `--copy` flag instead of printing, clear screen recommendation
- **Trade-off**: Scriptability requires output

**Clipboard Timeout**:
- CLI reuses GUI's 45-second clipboard auto-clear
- Timeout starts when copied, not when command exits

## Migration Plan

### Phase 1: Library Extraction (Non-Breaking)
1. Create `src/services/` and implement service layer
2. Update `src/lib.rs` to export services publicly
3. Add `src/error.rs` with `VaultError` enum
4. Refactor auth, crypto, database to remove Tauri dependencies
5. **Test**: Existing GUI still works (adapter layer unchanged)

### Phase 2: Binary Separation
6. Create `src/bin/vult-gui.rs` by moving main.rs logic
7. Update `commands.rs` to use services instead of direct module access
8. Add Cargo.toml [[bin]] entries
9. **Test**: Build and run GUI, verify no regressions

### Phase 3: CLI Implementation
10. Create `src/bin/vult.rs` with clap structure
11. Implement commands: init, add, get, list, search, update, delete, change-pin
12. Add output formatting (table, JSON, raw)
13. **Test**: Full CLI workflow, database sharing with GUI

### Phase 4: Documentation & Polish
14. Update README with CLI examples
15. Add rustdoc comments for library API
16. Create MIGRATION.md for developers
17. Update build scripts and CI

### Rollback Strategy
- Changes are additive (library exports, new binary)
- If issues arise, temporarily disable CLI binary in Cargo.toml
- GUI functionality unaffected (adapter layer maintains compatibility)

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|------------|
| Library API instability | Medium | Thorough testing before 1.0, semantic versioning |
| CLI adds attack surface | Low | Same security model, no network exposure |
| Increased binary size | Low | Feature flags for GUI-only vs CLI-only deps |
| Maintenance burden | Medium | Shared library reduces duplication, worth the investment |
| User confusion (two binaries) | Low | Clear docs, different use cases (GUI = daily use, CLI = automation) |
| Session token file leak | Medium | Temp file with 0600 perms, auto-delete on exit |

## Open Questions

1. **Session Token Storage Location**:
   - *Proposal*: `/tmp/vult-session-{pid}` on Linux, `%TEMP%` on Windows
   - *Security*: File permissions 0600 (Linux) or ACLs (Windows), auto-delete on process exit

2. **CLI Auto-Update**:
   - *Proposal*: Defer to package managers, not in-app
   - GUI may keep auto-update, CLI doesn't

3. **Stdin for Secrets**:
   - *Proposal*: Support `--stdin` for key values in scripts
   - Prevents shell history exposure

4. **Completion Scripts**:
   - *Proposal*: Generate shell completions (bash, zsh, fish) with clap
   - Include in release artifacts

5. **Library Versioning**:
   - *Proposal*: Start at 0.1.0, same as binaries
   - Follow semver strictly for library API

## Testing Strategy

### Code Quality
- **Linting**: Cargo clippy with pedantic and nursery lints enabled
- **Formatting**: Cargo fmt enforced in CI
- **License Compliance**: cargo-deny for dependency auditing
- **Pre-commit Hooks**: Automated checks before commits

### Security Testing
- **Vulnerability Scanning**: cargo-audit for known CVEs
- **Dependency Review**: Dependabot for automated updates
- **Unsafe Code Analysis**: cargo-geiger to minimize unsafe usage
- **SAST**: Static analysis for security issues

### Formal Verification
- **Property-Based Testing**: proptest for crypto operations
  - Encryption/decryption roundtrip properties
  - Per-key encryption uniqueness verification
  - PIN hash determinism and uniqueness
- **Invariant Verification**: Runtime checks in critical paths
- **Memory Safety**: MIRI for undefined behavior detection
- **Zeroization Verification**: Ensure sensitive data is cleared
- **Formal Tools**: Consider Kani/Prusti for crypto module

### Fuzz Testing
- **CLI Input Fuzzing**: cargo-fuzz for argument parsing
  - PIN input validation fuzzing
  - App/key name injection attempts
  - Path traversal attempts
  - Extreme input lengths and special characters
- **Serialization Fuzzing**: JSON output handling
- **Continuous Fuzzing**: 24h+ sessions to find edge cases
- **Regression Suite**: Add crashes to automated tests
- **CI Integration**: Short fuzz runs on each PR

### Platform Testing
- **Windows**: Full test suite on Windows 10/11
- **Linux**: Test on major distros (Ubuntu, Fedora, Arch)
- **Cross-Platform DB**: Verify vault compatibility between platforms

### Performance Testing
- CLI startup time benchmarks (<100ms target)
- Encryption/decryption throughput tests
- Database query performance profiling

## Future Enhancements

- **Library**: Publish to crates.io for third-party use
- **CLI**: Shell completion scripts
- **CLI**: `vult import` and `vult export` commands
- **CLI**: `vult backup` command for vault export
- **Library**: Async trait abstractions for storage backend swapping
- **Integration**: VS Code extension using library
- **Integration**: Browser extension using CLI or library
- **Platform**: macOS support (deferred)

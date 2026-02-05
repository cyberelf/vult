# Implementation Tasks

## 1. Foundation - Library Structure Setup
- [ ] 1.1 Create `src/error.rs` with VaultError enum covering all error cases
- [ ] 1.2 Update `src/lib.rs` to export public modules (auth, crypto, database, clipboard, error)
- [ ] 1.3 Create `src/services/mod.rs` with service module exports
- [ ] 1.4 Add library documentation in lib.rs with usage examples
- [ ] 1.5 Update Cargo.toml with [lib] section and proper feature flags

## 2. Service Layer - VaultManager
- [ ] 2.1 Create `src/services/vault_manager.rs` with VaultManager struct
- [ ] 2.2 Implement `VaultManager::new(db_path)` with database initialization
- [ ] 2.3 Add service accessors (auth(), keys(), crypto(), storage())
- [ ] 2.4 Implement proper Arc-based state management for services
- [ ] 2.5 Add comprehensive rustdoc comments with code examples

## 3. Service Layer - AuthService
- [ ] 3.1 Create `src/services/auth_service.rs` extracting logic from auth.rs
- [ ] 3.2 Remove Tauri State dependencies from auth module
- [ ] 3.3 Implement AuthService methods: init_vault(), unlock(), lock(), is_unlocked()
- [ ] 3.4 Add change_pin() method with re-encryption support
- [ ] 3.5 Implement session state tracking (no auto-lock in library, that's binary responsibility)
- [ ] 3.6 Add unit tests for AuthService

## 4. Service Layer - KeyService
- [ ] 4.1 Create `src/services/key_service.rs` for key management operations
- [ ] 4.2 Implement create() method with per-key encryption
- [ ] 4.3 Implement get() method with decryption
- [ ] 4.4 Implement list() returning ApiKeyMetadata (no decrypted values)
- [ ] 4.5 Implement search() with partial matching
- [ ] 4.6 Implement update() with conditional re-encryption
- [ ] 4.7 Implement delete() returning deleted key info
- [ ] 4.8 Add unit tests for KeyService

## 5. Service Layer - CryptoService and StorageService
- [ ] 5.1 Create `src/services/crypto_service.rs` wrapping crypto module
- [ ] 5.2 Expose derive_master_key(), encrypt(), decrypt() as service methods
- [ ] 5.3 Create `src/services/storage_service.rs` wrapping database module
- [ ] 5.4 Implement initialize(), store_key(), retrieve_key(), delete_key() methods
- [ ] 5.5 Add transaction support methods
- [ ] 5.6 Update database.rs to remove Tauri-specific code

## 6. Data Structures - Library Types
- [ ] 6.1 Create ApiKey struct with all fields (id, app_name, key_name, value, etc.)
- [ ] 6.2 Create ApiKeyMetadata struct (ApiKey without decrypted value)
- [ ] 6.3 Create UpdateKeyRequest struct with optional fields
- [ ] 6.4 Add Serialize/Deserialize derives for IPC compatibility
- [ ] 6.5 Document all public structs with rustdoc

## 7. Error Handling - VaultError Implementation
- [ ] 7.1 Define all VaultError variants (Auth, Crypto, Database, Locked, NotFound, etc.)
- [ ] 7.2 Implement Display and Error traits
- [ ] 7.3 Add From implementations for underlying error types (sqlx, etc.)
- [ ] 7.4 Add context methods for better error messages
- [ ] 7.5 Update all library code to use VaultError

## 8. GUI Binary - Separation and Adapter Layer
- [ ] 8.1 Create `src/bin/vult-gui.rs` and move main.rs logic there
- [ ] 8.2 Update commands.rs to be an adapter layer calling VaultManager services
- [ ] 8.3 Convert Tauri commands to thin wrappers around service calls
- [ ] 8.4 Update State management to wrap VaultManager instead of individual components
- [ ] 8.5 Maintain auto-lock functionality in GUI binary (activity tracking)
- [ ] 8.6 Test GUI for regressions - all existing functionality must work

## 9. CLI Binary - Project Setup
- [ ] 9.1 Add CLI dependencies to Cargo.toml: clap, dialoguer, comfy-table, rpassword
- [ ] 9.2 Create `src/bin/vult.rs` with main() entry point
- [ ] 9.3 Set up clap CLI structure with Commands enum
- [ ] 9.4 Implement global flags: --stay-unlocked, --json, etc.
- [ ] 9.5 Add database path logic (VULT_DB_PATH or default ~/.vult/vault.db)

## 10. CLI Binary - Authentication Commands
- [ ] 10.1 Implement `vult init` command with PIN setup
- [ ] 10.2 Add secure PIN input using rpassword/dialoguer
- [ ] 10.3 Implement PIN confirmation on init and change-pin
- [ ] 10.4 Implement `vult change-pin` command
- [ ] 10.5 Add VULT_PIN environment variable support with security warning
- [ ] 10.6 Test PIN validation and error handling

## 11. CLI Binary - Key Management Commands (Part 1)
- [ ] 11.1 Implement `vult add <app> <key>` with interactive prompts
- [ ] 11.2 Add --stdin flag for reading key value from stdin
- [ ] 11.3 Implement `vult get <app> <key>` outputting raw value
- [ ] 11.4 Add --full flag to get for showing all metadata
- [ ] 11.5 Add --copy flag to copy to clipboard with auto-clear
- [ ] 11.6 Implement `vult list` with table formatting

## 12. CLI Binary - Key Management Commands (Part 2)
- [ ] 12.1 Add --json flag to list for JSON output
- [ ] 12.2 Add --timestamps flag to list for showing dates
- [ ] 12.3 Implement `vult search <query>` with table output
- [ ] 12.4 Implement `vult update <app> <key>` with interactive mode
- [ ] 12.5 Add flags for update: --value, --url, --description
- [ ] 12.6 Implement `vult delete <app> <key>` with confirmation prompt
- [ ] 12.7 Add --force flag to delete to skip confirmation

## 13. CLI Binary - Output Formatting
- [ ] 13.1 Create output formatting module for tables, JSON, raw
- [ ] 13.2 Implement table formatting with comfy-table
- [ ] 13.3 Implement JSON output for list and search
- [ ] 13.4 Add color support for terminal output (check if TTY)
- [ ] 13.5 Format timestamps in human-readable format
- [ ] 13.6 Test output formatting on different terminals

## 14. CLI Binary - Session Management
- [ ] 14.1 Implement optional session token file in /tmp (or %TEMP% on Windows)
- [ ] 14.2 Add --stay-unlocked flag support
- [ ] 14.3 Implement 5-minute session timeout
- [ ] 14.4 Implement `vult lock` command to clear session
- [ ] 14.5 Set proper file permissions (0600) on session file
- [ ] 14.6 Auto-delete session file on process exit

## 15. CLI Binary - Error Handling and Exit Codes
- [ ] 15.1 Map VaultError variants to appropriate exit codes
- [ ] 15.2 Implement user-friendly error messages
- [ ] 15.3 Add suggestion messages for common errors
- [ ] 15.4 Handle Ctrl+C gracefully (cleanup session, lock vault)
- [ ] 15.5 Test error scenarios and exit codes

## 16. Clipboard Integration
- [ ] 16.1 Refactor clipboard.rs to separate library logic from GUI thread management
- [ ] 16.2 Create library clipboard functions: copy(), clear()
- [ ] 16.3 Maintain GUI auto-clear thread in GUI binary
- [ ] 16.4 Implement CLI clipboard operations with 45-second timeout
- [ ] 16.5 Test clipboard on all platforms (Windows, macOS, Linux)

## 17. Configuration - Cargo.toml Updates
- [ ] 17.1 Define [[bin]] entries for vult-gui and vult
- [ ] 17.2 Set up feature flags for GUI-only and CLI-only dependencies
- [ ] 17.3 Ensure library compiles without binary dependencies
- [ ] 17.4 Update build.rs if needed for multi-binary setup
- [ ] 17.5 Test building binaries independently and together

## 18. Testing - Library Tests
- [ ] 18.1 Add unit tests for AuthService (init, unlock, lock, change_pin)
- [ ] 18.2 Add unit tests for KeyService (CRUD operations)
- [ ] 18.3 Add unit tests for CryptoService (encryption/decryption roundtrips)
- [ ] 18.4 Add unit tests for StorageService (database operations)
- [ ] 18.5 Add integration tests for VaultManager orchestration
- [ ] 18.6 Test error handling and edge cases

## 19. Testing - GUI Binary Tests
- [ ] 19.1 Test all existing GUI functionality (no regressions)
- [ ] 19.2 Test adapter layer (commands.rs calling services)
- [ ] 19.3 Test auto-lock behavior still works
- [ ] 19.4 Test clipboard auto-clear in GUI
- [ ] 19.5 Test database sharing with CLI

## 20. Testing - CLI Binary Tests
- [ ] 20.1 Create integration tests for CLI commands
- [ ] 20.2 Test init → add → get → list → delete workflow
- [ ] 20.3 Test search functionality
- [ ] 20.4 Test update operations (value and metadata)
- [ ] 20.5 Test change-pin with re-encryption
- [ ] 20.6 Test --json output format
- [ ] 20.7 Test --copy clipboard functionality
- [ ] 20.8 Test session mode with --stay-unlocked
- [ ] 20.9 Test database sharing with GUI (run both concurrently)

## 21. Testing - Cross-Platform (Windows & Linux)
- [ ] 21.1 Test library on Windows
- [ ] 21.2 Test library on Linux
- [ ] 21.3 Test CLI on Windows
- [ ] 21.4 Test CLI on Linux
- [ ] 21.5 Test GUI on Windows
- [ ] 21.6 Test GUI on Linux
- [ ] 21.7 Test database compatibility between platforms

## 22. Code Quality - Linting and Static Analysis
- [ ] 22.1 Run cargo clippy with -D warnings on all code
- [ ] 22.2 Run cargo fmt --check to verify formatting
- [ ] 22.3 Configure clippy.toml with strict lints (pedantic, nursery)
- [ ] 22.4 Fix all clippy warnings and suggestions
- [ ] 22.5 Add cargo deny configuration for license and security checks
- [ ] 22.6 Run cargo audit for dependency vulnerability scanning
- [ ] 22.7 Set up pre-commit hooks for clippy and fmt
- [ ] 22.8 Add CI job for linting that fails on warnings

## 23. Security - Vulnerability Scanning
- [ ] 23.1 Install and run cargo audit regularly
- [ ] 23.2 Set up Dependabot or similar for dependency updates
- [ ] 23.3 Run cargo-geiger to check unsafe code usage
- [ ] 23.4 Scan dependencies with cargo-deny for known CVEs
- [ ] 23.5 Review and minimize unsafe code blocks
- [ ] 23.6 Add security.txt with vulnerability disclosure policy
- [ ] 23.7 Run SAST tools (cargo-semver-checks for API stability)
- [ ] 23.8 Add regular security scanning to CI pipeline

## 24. Formal Verification - Critical Logic
- [ ] 24.1 Identify critical security functions (encryption, key derivation, auth)
- [ ] 24.2 Write property-based tests for crypto operations using proptest
- [ ] 24.3 Verify encryption/decryption roundtrip properties
- [ ] 24.4 Verify per-key encryption uniqueness (same plaintext → different ciphertext)
- [ ] 24.5 Verify PIN hash properties (deterministic for same PIN, different for different PINs)
- [ ] 24.6 Verify zeroization occurs for sensitive data (PINs, keys)
- [ ] 24.7 Use MIRI to detect undefined behavior in unsafe code
- [ ] 24.8 Document verified properties in rustdoc
- [ ] 24.9 Add invariant checks in critical code paths
- [ ] 24.10 Consider Kani or Prusti for formal verification of crypto module

## 25. Fuzz Testing - CLI Input Validation
- [ ] 25.1 Set up cargo-fuzz for fuzzing CLI inputs
- [ ] 25.2 Create fuzz target for clap argument parsing
- [ ] 25.3 Fuzz PIN input validation (length, character sets)
- [ ] 25.4 Fuzz app_name and key_name inputs (special chars, SQL injection attempts)
- [ ] 25.5 Fuzz key value inputs (binary data, null bytes, extreme lengths)
- [ ] 25.6 Fuzz JSON output serialization
- [ ] 25.7 Fuzz database path inputs (path traversal, invalid paths)
- [ ] 25.8 Run fuzzing for extended periods (24h+ sessions)
- [ ] 25.9 Add found crashes to regression test suite
- [ ] 25.10 Integrate fuzzing into CI (short runs on each PR)

## 26. Documentation - Library API
- [ ] 26.1 Add comprehensive rustdoc comments to VaultManager
- [ ] 26.2 Add rustdoc comments to all service structs and methods
- [ ] 26.3 Add usage examples in rustdoc
- [ ] 26.4 Document error types and when they occur
- [ ] 26.5 Generate and review cargo doc output

## 27. Documentation - User Documentation
- [ ] 27.1 Update README.md with CLI usage section
- [ ] 27.2 Add CLI command examples
- [ ] 27.3 Document installation instructions for both binaries (Windows & Linux)
- [ ] 27.4 Add security considerations for CLI usage
- [ ] 27.5 Document --stay-unlocked risks and VULT_PIN warnings
- [ ] 27.6 Create CLI_GUIDE.md with comprehensive examples
- [ ] 27.7 Document platform-specific considerations (Windows vs Linux paths)

## 28. Documentation - Developer Documentation
- [ ] 28.1 Create ARCHITECTURE.md documenting library structure
- [ ] 28.2 Add contributing guide for library development
- [ ] 28.3 Document service layer patterns
- [ ] 28.4 Create migration guide for developers updating code
- [ ] 28.5 Document build process for both binaries
- [ ] 28.6 Document testing strategy (unit, integration, property-based, fuzz)
- [ ] 28.7 Add security testing documentation

## 29. Build and Release
- [ ] 25.1 Update CI/CD to build both binaries
- [ ] 25.2 Create release scripts for packaging both binaries
- [ ] 25.3 Update version in Cargo.toml to reflect new features
- [ ] 25.4 Test release builds on all platforms
- [ ] 25.5 Create release notes documenting new CLI
- [ ] 25.6 Update CHANGELOG.md

## 30. Optional Enhancements
- [ ] 30.1 Add shell completion scripts (bash, zsh, fish) generation
- [ ] 30.2 Add `vult import` command for CSV/JSON import
- [ ] 30.3 Add `vult export` command for vault backup
- [ ] 30.4 Add `vult doctor` command for vault health check
- [ ] 30.5 Add colored output enhancement with colored crate
- [ ] 30.6 Add progress indicators for long operations
- [ ] 30.7 Consider macOS support as future enhancement

## Notes

**Testing Strategy**:
- Unit tests for each service independently
- Integration tests for VaultManager orchestration
- CLI integration tests using assert_cmd crate
- Property-based tests for critical security functions (proptest)
- Fuzz testing for input validation (cargo-fuzz)
- Formal verification of crypto operations
- Static analysis with clippy (pedantic mode)
- Vulnerability scanning with cargo-audit
- MIRI for undefined behavior detection
- Manual testing on Windows and Linux

**Security Verification**:
- Verify zeroization of sensitive data (PINs, keys)
- Test session file permissions
- Verify clipboard auto-clear timing
- Test error messages don't leak sensitive info

**Performance Considerations**:
- CLI should start quickly (<100ms for simple commands)
- Database connection pooling for multi-command sessions
- Minimize decryption operations
- Optimize for Windows and Linux-specific file I/O patterns

**Breaking Changes**:
- None for end users (GUI unchanged)
- Module structure changes for developers (imports may need updates)

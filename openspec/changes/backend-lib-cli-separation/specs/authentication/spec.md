## ADDED Requirements

N/A - No new authentication requirements.

## MODIFIED Requirements

### MODIFIED: Authentication Context
Authentication is modified to work in both GUI and CLI contexts without framework coupling.

#### Scenario: Framework independence (was: Tauri-coupled)
- **BEFORE**: AuthManager used Tauri State and was tightly coupled to GUI lifecycle
- **AFTER**: AuthService is framework-agnostic and works in any context
- **WHEN** library is used
- **THEN** AuthService SHALL NOT depend on Tauri types
- **AND** AuthService SHALL work in CLI, GUI, or other contexts
- **AND** authentication state SHALL be managed independently

#### Scenario: PIN input (was: GUI dialogs)
- **BEFORE**: PIN prompts used Tauri dialogs and UI components
- **AFTER**: PIN input is responsibility of the caller (binary layer)
- **WHEN** authentication is needed
- **THEN** library SHALL provide auth methods accepting PIN as parameter
- **AND** GUI SHALL handle PIN prompts through UI
- **AND** CLI SHALL handle PIN prompts through terminal input

### MODIFIED: Session Management
Session management is modified to support both interactive GUI and command-line workflows.

#### Scenario: GUI auto-lock (existing behavior maintained)
- **BEFORE**: AuthManager tracked activity and auto-locked after 5 minutes
- **AFTER**: GUI binary still supports auto-lock through activity tracking
- **WHEN** GUI is inactive for 5 minutes
- **THEN** the vault SHALL auto-lock
- **AND** GUI SHALL maintain the existing behavior

#### Scenario: CLI session (new)
- **AFTER**: CLI can optionally maintain short-lived session
- **WHEN** CLI command runs with --stay-unlocked flag
- **THEN** authentication SHALL be cached for subsequent commands
- **AND** session SHALL timeout after 5 minutes
- **AND** session SHALL be explicitly opt-in, not default

#### Scenario: Session state API (new)
- **AFTER**: Library exposes session state query methods
- **WHEN** caller checks authentication state
- **THEN** library SHALL provide `is_unlocked()` method
- **AND** library SHALL provide `is_initialized()` method
- **AND** library SHALL provide `lock()` and `unlock(pin)` methods

### MODIFIED: Authentication Storage
Authentication credential storage is modified to be library-managed rather than GUI-managed.

#### Scenario: Database-backed auth (no change)
- **BEFORE**: PIN hash stored in database via AuthManager
- **AFTER**: PIN hash stored in database via AuthService
- **WHEN** vault is initialized
- **THEN** PIN hash SHALL be stored in database
- **AND** storage mechanism remains the same

#### Scenario: Memory management (enhanced)
- **BEFORE**: Decrypted master key cached in memory during GUI session
- **AFTER**: Master key caching is context-aware
- **WHEN** in GUI context
- **THEN** master key MAY be cached for session duration
- **WHEN** in CLI context
- **THEN** master key MAY be cached for command duration or session if opted-in
- **AND** zeroization SHALL occur when key is no longer needed

### MODIFIED: Authentication Error Handling
Error handling is modified to be library-appropriate rather than GUI-specific.

#### Scenario: Error types (was: Tauri error responses)
- **BEFORE**: Authentication errors returned as Tauri Result types
- **AFTER**: Authentication errors use library VaultError enum
- **WHEN** authentication fails
- **THEN** library SHALL return AuthError variant
- **AND** binaries SHALL adapt errors to their context (Tauri Response or CLI exit code)

#### Scenario: User feedback (new responsibility separation)
- **AFTER**: Library returns structured errors, binaries handle user messaging
- **WHEN** authentication fails
- **THEN** library SHALL return error with context (InvalidPin, NotInitialized, etc.)
- **AND** GUI SHALL display error dialog
- **AND** CLI SHALL display error message and exit with code 1

### MODIFIED: Biometric Authentication
Biometric authentication remains a future feature, but planning considers library architecture.

#### Scenario: Biometric integration planning (deferred)
- **BEFORE**: Planned as Tauri plugin integration
- **AFTER**: Will be library-based with platform-specific implementations
- **WHEN** biometric support is added (future)
- **THEN** library SHALL provide platform abstraction
- **AND** binaries SHALL call platform-specific implementations
- **AND** design SHALL allow CLI to support biometric if platform enables it

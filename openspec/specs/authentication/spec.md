# Authentication Specification

## Purpose

Defines the authentication system for Vult, including PIN-based authentication, session management, and security considerations for both GUI and CLI contexts.

## Requirements

### Requirement: Authentication Context
Authentication works in both GUI and CLI contexts without framework coupling.

#### Scenario: Framework independence
- **WHEN** library is used
- **THEN** AuthService SHALL NOT depend on Tauri types
- **AND** AuthService SHALL work in CLI, GUI, or other contexts
- **AND** authentication state SHALL be managed independently

#### Scenario: PIN input
- **WHEN** authentication is needed
- **THEN** library SHALL provide auth methods accepting PIN as parameter
- **AND** GUI SHALL handle PIN prompts through UI
- **AND** CLI SHALL handle PIN prompts through terminal input

### Requirement: Session Management
Session management supports both interactive GUI and command-line workflows.

#### Scenario: GUI auto-lock
- **WHEN** GUI is inactive for 5 minutes
- **THEN** the vault SHALL auto-lock
- **AND** GUI binary supports auto-lock through activity tracking

#### Scenario: CLI session
- **WHEN** CLI command runs with --stay-unlocked flag
- **THEN** authentication SHALL be cached for subsequent commands
- **AND** session SHALL timeout after 5 minutes
- **AND** session SHALL be explicitly opt-in, not default

#### Scenario: Session state API
- **WHEN** caller checks authentication state
- **THEN** library SHALL provide `is_unlocked()` method
- **AND** library SHALL provide `is_initialized()` method
- **AND** library SHALL provide `lock()` and `unlock(pin)` methods

### Requirement: Authentication Storage
Authentication credential storage is library-managed.

#### Scenario: Database-backed auth
- **WHEN** vault is initialized
- **THEN** PIN hash SHALL be stored in database
- **AND** storage mechanism uses secure hashing

#### Scenario: Memory management
- **WHEN** in GUI context
- **THEN** master key MAY be cached for session duration
- **WHEN** in CLI context
- **THEN** master key MAY be cached for command duration or session if opted-in
- **AND** zeroization SHALL occur when key is no longer needed

### Requirement: Authentication Error Handling
Error handling is library-appropriate rather than GUI-specific.

#### Scenario: Error types
- **WHEN** authentication fails
- **THEN** library SHALL return AuthError variant from VaultError enum
- **AND** binaries SHALL adapt errors to their context (Tauri Response or CLI exit code)

#### Scenario: User feedback
- **WHEN** authentication fails
- **THEN** library SHALL return error with context (InvalidPin, NotInitialized, etc.)
- **AND** GUI SHALL display error dialog
- **AND** CLI SHALL display error message and exit with code 1

### Requirement: Biometric Authentication
Biometric authentication is a future feature with planning for library architecture.

#### Scenario: Biometric integration planning (deferred)
- **WHEN** biometric support is added (future)
- **THEN** library SHALL provide platform abstraction
- **AND** binaries SHALL call platform-specific implementations
- **AND** design SHALL allow CLI to support biometric if platform enables it

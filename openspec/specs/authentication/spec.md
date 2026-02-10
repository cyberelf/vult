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
The system SHALL provide optional Windows Hello biometric authentication for unlocking an initialized vault on Windows 10 (1903+) and Windows 11, using Windows.Security.Credentials.UI APIs and never persisting biometric data.

#### Scenario: Windows Hello availability check
- **WHEN** the app requests biometric availability on Windows
- **THEN** the library SHALL query UserConsentVerifier.CheckAvailabilityAsync
- **AND** the response SHALL indicate Available, NotConfigured, DeviceNotPresent, or NotSupported
- **AND** non-Windows platforms SHALL return NotSupported

#### Scenario: Successful Windows Hello unlock
- **WHEN** a user initiates biometric unlock on a Windows Hello-enabled device
- **AND** Windows Hello verification succeeds
- **THEN** the vault SHALL unlock and start a session

#### Scenario: Fallback to PIN
- **WHEN** Windows Hello is unavailable or the biometric attempt fails
- **THEN** the system SHALL allow the user to unlock with PIN
- **AND** the biometric failure SHALL NOT prevent PIN authentication
- **AND** non-Windows platforms SHALL always use PIN authentication

#### Scenario: Opt-in control
- **WHEN** the user disables Windows Hello in settings
- **THEN** the system SHALL not attempt biometric unlock
- **AND** the system SHALL require PIN for unlock

#### Scenario: Error mapping
- **WHEN** a Windows Hello attempt fails, is cancelled, or times out
- **THEN** the library SHALL return AuthError::BiometricFailed
- **AND** the error SHALL map all non-Verified results to the same error
- **AND** the error SHALL NOT expose Windows API-specific details

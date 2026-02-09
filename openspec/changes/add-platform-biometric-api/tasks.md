## 1. Dependencies and Setup

- [ ] 1.1 Add windows crate dependency to Cargo.toml with Security.Credentials.UI features
- [ ] 1.2 Create windows-biometric feature flag
- [ ] 1.3 Document Windows 10/11 version requirements

## 2. Library API and Abstractions

- [ ] 2.1 Add BiometricAvailability enum and AuthError::BiometricFailed variant
- [ ] 2.2 Define BiometricProvider trait in core module
- [ ] 2.3 Add biometric availability and unlock methods to AuthService

## 3. Windows Hello Implementation

- [ ] 3.1 Create src/biometric/mod.rs module structure
- [ ] 3.2 Implement Windows Hello provider using UserConsentVerifier API
- [ ] 3.3 Add availability detection using UserConsentVerifier::CheckAvailabilityAsync
- [ ] 3.4 Map Windows Hello results to BiometricAvailability and Result types

## 4. Tauri Commands and GUI

- [ ] 4.1 Add check_biometric_available Tauri command
- [ ] 4.2 Add unlock_with_biometric Tauri command
- [ ] 4.3 Update GUI commands.rs to wire biometric operations
- [ ] 4.4 Update capability allowlists for new biometric commands

## 5. GUI Integration

- [ ] 5.1 Add biometric button/option to unlock screen (conditional on availability)
- [ ] 5.2 Implement biometric unlock flow with automatic PIN fallback on failure
- [ ] 5.3 Add user setting to enable/disable Windows Hello
- [ ] 5.4 Display appropriate error messages for biometric failures

## 6. Tests

- [ ] 6.1 Add unit tests for biometric trait and error mapping
- [ ] 6.2 Add mocked Windows Hello provider for testing
- [ ] 6.3 Add integration tests for availability detection
- [ ] 6.4 Add integration tests for unlock flow with fallback behavior

## 7. Documentation

- [ ] 7.1 Update AGENTS.md with Windows Hello integration notes
- [ ] 7.2 Update README with Windows Hello requirements
- [ ] 7.3 Document feature flag usage in Cargo.toml comments

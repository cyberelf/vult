## Context

Authentication is currently PIN-based with session management in both GUI and CLI contexts. The existing spec includes a deferred biometric requirement, and the UI already anticipates a biometric prompt. This change introduces Windows Hello biometric unlock while preserving the PIN fallback and the library-first design.

## Goals / Non-Goals

**Goals:**
- Add Windows Hello biometric unlock in the library layer using windows-rs crate.
- Expose biometric availability and unlock commands to GUI while keeping AuthService framework-neutral.
- Preserve PIN as the primary fallback and support opt-in enablement.
- Provide consistent error mapping without leaking sensitive Windows API details.

**Non-Goals:**
- Support macOS Touch ID or Linux biometrics (future enhancements).
- Replace or remove PIN authentication.
- Implement biometric enrollment or Windows Hello configuration flows.
- Support CLI biometric unlock (GUI-only initially).
- Store biometric templates or any biometric data in Vult storage.

## Decisions

### Decision 1: Use windows-rs for Windows Hello Integration

**Choice**: Use the `windows` crate (windows-rs) to access Windows.Security.Credentials.UI APIs

**Rationale**:
- Official Microsoft-maintained Rust bindings with safe abstractions
- Direct access to UserConsentVerifier API for biometric prompts
- Type-safe, idiomatic Rust API over raw FFI
- Well-documented with examples in the Windows samples repository

**Alternatives considered**:
- `winapi` crate: Lower-level, less maintained, older unsafe FFI patterns
- Custom FFI bindings: Unnecessary reinvention, maintenance burden
- Third-party wrappers: None exist with sufficient maturity

**Implementation approach**:
```rust
use windows::Security::Credentials::UI::{UserConsentVerifier, UserConsentVerificationResult};

pub async fn verify_biometric(message: &str) -> Result<bool> {
    let result = UserConsentVerifier::RequestVerificationAsync(message)?.await?;
    match result {
        UserConsentVerificationResult::Verified => Ok(true),
        _ => Ok(false),
    }
}
```

### Decision 2: Library Abstraction with Windows-Only Implementation

**Choice**: Create a biometric provider trait in the library with Windows Hello as the only implementation

**Rationale**:
- Keeps AuthService framework-neutral while supporting Windows-specific features
- Allows future extension to other platforms without library redesign
- GUI calls biometric methods via Tauri commands, avoiding Tauri types in AuthService

**Alternatives considered**:
- Embed Windows APIs directly in AuthService: Creates Windows-only coupling
- No abstraction layer: Makes future multi-platform support harder

### Decision 3: Explicit Availability Checks

**Choice**: Treat biometric unlock as optional with explicit availability checks and automatic PIN fallback

**Rationale**:
- Not all Windows devices support Windows Hello (requires compatible hardware)
- Clear UX: users know if biometric is available before attempting unlock
- Graceful degradation to PIN on older systems or hardware failures

### Decision 4: Simplified Error Mapping

**Choice**: Map Windows Hello errors to existing AuthError::BiometricFailed variant

**Rationale**:
- Windows API returns UserConsentVerificationResult enum with clear states
- Map all non-success states to a single error for UI simplicity
- Avoid exposing Windows-specific error details to frontend

## Risks / Trade-offs

- **Windows Hello hardware requirement**: Not all Windows devices support biometrics → Mitigate with availability checks and clear UI messaging.
- **Increased binary size**: windows-rs adds ~500KB to release builds → Acceptable trade-off for native Windows integration.
- **Windows version requirement**: Requires Windows 10 1903+ for UserConsentVerifier APIs → Document minimum OS version clearly.
- **Feature flag complexity**: Windows-only feature increases build matrix → Mitigate with clear CI configuration and documentation.

## Migration Plan

1. Add `windows` crate dependency with Windows-only feature flag
2. Create biometric module in library with trait abstraction and Windows Hello implementation
3. Extend AuthService with biometric availability check and unlock methods (keep existing PIN APIs unchanged)
4. Add new Tauri commands for biometric operations
5. Update GUI to display biometric option when available and handle fallback
6. Add unit tests with mocked Windows APIs
7. Add integration tests for availability detection and fallback paths

## Open Questions

- Should we support Windows Hello for Business (enterprise) or only consumer Windows Hello?
- Should biometric setting be stored per-vault or globally in app settings?

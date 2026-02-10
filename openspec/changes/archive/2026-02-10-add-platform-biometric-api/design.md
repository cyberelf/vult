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

### Decision 5: Desktop API with HWND Parameter (Implementation Discovery)

**Choice**: Use `IUserConsentVerifierInterop::RequestVerificationForWindowAsync(HWND, message)` instead of the UWP `UserConsentVerifier::RequestVerificationAsync(message)`

**Rationale**:
- **Critical Issue Discovered**: The UWP API causes Windows Hello modal to appear behind the Tauri window
- Microsoft documentation specifies desktop applications MUST use `IUserConsentVerifierInterop` with HWND parameter
- Passing the window handle (HWND) establishes proper parent-child relationship for modal z-order
- Without HWND, Windows cannot determine which window owns the authentication modal

**Implementation approach**:
```rust
use windows::Win32::System::WinRT::IUserConsentVerifierInterop;
use windows::Win32::Foundation::HWND;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

// Extract HWND from Tauri window
let hwnd = window.window_handle().ok()
    .and_then(|handle| match handle.as_raw() {
        RawWindowHandle::Win32(win32_handle) => Some(win32_handle.hwnd.get() as isize),
        _ => None,
    });

// Use desktop API with HWND
let factory: IUserConsentVerifierInterop = 
    windows::core::factory::<UserConsentVerifier, IUserConsentVerifierInterop>()?;
let async_op = unsafe {
    factory.RequestVerificationForWindowAsync(HWND(hwnd as *mut _), &message)?
};
```

**Additional dependencies required**:
- `raw-window-handle = "0.6"` - Extract native window handle from Tauri
- `Win32_System_WinRT` feature in windows crate - Access to IUserConsentVerifierInterop

**Why this wasn't in original design**:
- UWP API documentation doesn't clearly indicate desktop app requirements
- Modal z-order issue only discovered during GUI integration testing
- Multiple failed attempts with window management APIs (set_always_on_top, request_user_attention) before finding proper solution
- Solution documented in LESSONS.md for future reference

### Decision 6: DPAPI-Based Credential Storage (Implementation Addition)

**Choice**: Use Windows Data Protection API (DPAPI) for secure credential storage with per-vault isolation

**Rationale**:
- Required for biometric unlock to work: store encrypted PIN/vault key that Windows Hello can unlock
- DPAPI provides user-level encryption without requiring explicit key management
- Per-vault isolation achieved by using database path as credential scope key
- Automatic cleanup on disable prevents credential leakage

**Implementation approach**:
```rust
use windows::Win32::Security::Cryptography::{
    CryptProtectData, CryptUnprotectData, CRYPTOPROTECT_UI_FORBIDDEN
};

pub struct CredentialStore;

impl CredentialStore {
    pub fn store(&self, db_path: &str, credential: &[u8]) -> Result<()> {
        let scope = Self::get_scope_key(db_path);
        // DPAPI encrypt with user scope
        let encrypted = encrypt_dpapi(credential, &scope)?;
        // Store in Windows credential manager or registry
        store_credential(&scope, &encrypted)?;
        Ok(())
    }
    
    pub fn retrieve(&self, db_path: &str) -> Result<Option<Vec<u8>>> {
        let scope = Self::get_scope_key(db_path);
        let encrypted = load_credential(&scope)?;
        // DPAPI decrypt with user scope
        decrypt_dpapi(&encrypted, &scope)
    }
}
```

**Security properties**:
- Credentials encrypted at rest with user-level DPAPI protection
- Per-vault isolation prevents credential reuse across databases
- Credentials only accessible by the same Windows user account
- Automatic cleanup when Windows Hello is disabled
- Validation before storage prevents incorrect credential persistence

**Additional features added**:
- `Win32_Security_Cryptography` feature for DPAPI APIs
- Credential validation during setup flow
- Automatic cleanup on biometric disable

## Risks / Trade-offs

- **Windows Hello hardware requirement**: Not all Windows devices support biometrics → Mitigated with availability checks and clear UI messaging.
- **Increased binary size**: windows-rs adds ~500KB to release builds → Acceptable trade-off for native Windows integration.
- **Windows version requirement**: Requires Windows 10 1903+ for UserConsentVerifier APIs → Documented minimum OS version clearly in README and UI.
- **Feature flag complexity**: Windows-only feature increases build matrix → Mitigated with clear CI configuration and documentation.
- **Desktop API complexity**: IUserConsentVerifierInterop requires unsafe code and HWND extraction → Mitigated with proper error handling and documentation in LESSONS.md.
- **Credential storage security**: Storing encrypted credentials for biometric unlock → Mitigated by using DPAPI with user-level protection and per-vault isolation.
- **Modal z-order issues**: Windows Hello modal can appear behind window with wrong API → Solved by using desktop API with HWND parameter (documented in LESSONS.md).

## Migration Plan (As Implemented)

1. ✅ Add `windows` crate dependency with Windows-only feature flag
   - Added `Security_Credentials_UI`, `Win32_System_WinRT`, `Win32_Security_Cryptography` features
   - Added `raw-window-handle = "0.6"` for HWND extraction
   - Added `async-trait = "0.1"` for BiometricProvider trait objects

2. ✅ Create biometric module in library with trait abstraction and Windows Hello implementation
   - Created `src/biometric/mod.rs` module structure
   - Implemented `BiometricProvider` trait with availability and verify methods
   - Implemented `WindowsHelloProvider` using `UserConsentVerifier` API
   - Implemented `MockBiometricProvider` for testing
   - Added `CredentialStore` for DPAPI-based credential storage

3. ✅ Extend AuthService with biometric availability check and unlock methods
   - Added `check_biometric_available()` method
   - Added `unlock_with_biometric(message)` method with PIN fallback
   - Added `unlock_with_biometric_with_window(message, hwnd)` method for desktop API
   - Kept existing PIN APIs unchanged for backwards compatibility

4. ✅ Add new Tauri commands for biometric operations
   - Added `check_biometric_available` command returning BiometricAvailability
   - Added `unlock_with_biometric` command with HWND extraction from Tauri window
   - Updated capability allowlists in `capabilities/default.json`

5. ✅ Update GUI to display biometric option when available and handle fallback
   - Added Windows Hello button to unlock screen with biometric icon
   - Added toggle buttons for switching between PIN and biometric auth
   - Improved button styling consistency across authentication methods
   - Added biometric settings panel in vault settings screen

6. ✅ Add unit tests with mocked Windows APIs
   - Added unit tests for BiometricProvider trait implementation
   - Added unit tests for Windows Hello result mapping
   - Added MockBiometricProvider for testing without hardware

7. ✅ Add integration tests for availability detection and fallback paths
   - Added `biometric_availability_test.rs` for detection testing
   - Added `biometric_integration_test.rs` for unlock flow testing
   - All tests passing with comprehensive coverage

8. ✅ Fix critical Windows Hello modal z-order issue (not in original plan)
   - Discovered modal appearing behind window during testing
   - Researched Microsoft documentation for desktop API requirements
   - Implemented `IUserConsentVerifierInterop` with HWND parameter
   - Added HWND extraction from Tauri window using raw-window-handle
   - Documented complete debugging journey in LESSONS.md

## Implementation Notes

**Critical Discovery**: The original design assumed `UserConsentVerifier::RequestVerificationAsync()` would work correctly for desktop applications. During implementation, we discovered that desktop apps MUST use `IUserConsentVerifierInterop::RequestVerificationForWindowAsync(HWND, message)` to properly parent the authentication modal. This required:
- Additional `raw-window-handle` dependency
- `Win32_System_WinRT` feature for desktop interop API
- HWND extraction from Tauri window
- Factory pattern to get IUserConsentVerifierInterop interface

This was documented in LESSONS.md as a critical lesson: always check for Desktop vs UWP API differences in Windows development.

## Open Questions (Resolved During Implementation)

**Q: Should we support Windows Hello for Business (enterprise) or only consumer Windows Hello?**
- **A**: Supports both. The UserConsentVerifier API works with both consumer and enterprise Windows Hello configurations transparently.

**Q: Should biometric setting be stored per-vault or globally in app settings?**
- **A**: Per-vault. Implemented with credential storage scoped by database path for proper multi-vault isolation.

**Q: How to handle Windows Hello modal appearing behind the main window?**
- **A**: Use IUserConsentVerifierInterop desktop API with HWND parameter instead of UWP API. This was a critical implementation discovery documented in LESSONS.md.

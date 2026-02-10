## Why

Windows users expect Windows Hello biometric unlock for faster, low-friction access while keeping strong device-backed security. Adding Windows Hello support now reduces unlock friction on Windows devices and aligns the vault with Windows 10/11 security capabilities without removing the existing PIN fallback.

## What Changes

- Add Windows Hello biometric unlock using Windows.Security.Credentials.UI APIs via windows-rs crate with availability detection.
- **Critical Implementation Detail**: Use `IUserConsentVerifierInterop` desktop API with HWND parameter (not the UWP API) to properly parent the authentication modal.
- Add DPAPI-based secure credential storage with per-vault isolation for biometric setup.
- Add a user-facing toggle and fallback behavior to PIN on failure or unavailable devices.
- Add error mapping for Windows Hello failures (no sensitive data exposed).
- Update Tauri command surface and allowlists for biometric auth flows.
- Add tests for biometric availability and fallback logic (mocked where needed).
- Windows-only feature flag for biometric support.
- UI improvements: Windows Hello button, toggle buttons for auth method switching, consistent styling.

## Implementation Notes (Post-Implementation)

**Critical Discovery**: During implementation, discovered that desktop applications MUST use `IUserConsentVerifierInterop::RequestVerificationForWindowAsync(HWND, message)` instead of the UWP `UserConsentVerifier::RequestVerificationAsync(message)`. The UWP API causes the Windows Hello modal to appear behind the Tauri window, making it inaccessible.

**Additional Dependencies Required**:
- `raw-window-handle = "0.6"` for extracting HWND from Tauri window
- `Win32_System_WinRT` feature in windows crate for IUserConsentVerifierInterop
- `Win32_Security_Cryptography` feature for DPAPI credential storage
- `async-trait = "0.1"` for BiometricProvider trait objects

**Documented in LESSONS.md**: Complete debugging journey of Windows Hello modal z-order issue with three failed attempts before finding the proper desktop API solution.

## Capabilities

### New Capabilities
- (none)

### Modified Capabilities
- `authentication`: add biometric-based unlock flows, availability checks, and PIN fallback requirements.

## Impact

- Affected code: auth service, GUI auth manager, Tauri commands, capability allowlists, Windows-specific biometric module, credential store.
- Dependencies: 
  - `windows = "0.58"` with features: `Security_Credentials_UI`, `Win32_System_WinRT`, `Win32_Security_Cryptography`, `Win32_Foundation`, `Win32_System_Memory`
  - `raw-window-handle = "0.6"` for HWND extraction from Tauri window
  - `async-trait = "0.1"` for BiometricProvider trait objects
- Tests: 6 new integration tests for biometric auth flows with Windows Hello mocks (all passing).
- Platform: Windows 10 (1903+) and Windows 11 only.
- Build: Windows-biometric feature automatically included in GUI builds.

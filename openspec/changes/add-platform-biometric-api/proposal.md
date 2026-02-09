## Why

Windows users expect Windows Hello biometric unlock for faster, low-friction access while keeping strong device-backed security. Adding Windows Hello support now reduces unlock friction on Windows devices and aligns the vault with Windows 10/11 security capabilities without removing the existing PIN fallback.

## What Changes

- Add Windows Hello biometric unlock using Windows.Security.Credentials.UI APIs via windows-rs crate with availability detection.
- Add a user-facing toggle and fallback behavior to PIN on failure or unavailable devices.
- Add error mapping for Windows Hello failures (no sensitive data exposed).
- Update Tauri command surface and allowlists for biometric auth flows.
- Add tests for biometric availability and fallback logic (mocked where needed).
- Windows-only feature flag for biometric support.

## Capabilities

### New Capabilities
- (none)

### Modified Capabilities
- `authentication`: add biometric-based unlock flows, availability checks, and PIN fallback requirements.

## Impact

- Affected code: auth service, GUI auth manager, Tauri commands, capability allowlists, Windows-specific biometric module.
- Dependencies: `windows` crate (windows-rs) for Windows.Security.Credentials.UI APIs; Windows-only feature flag.
- Tests: new auth flow tests with Windows Hello mocks.
- Platform: Windows 10 (1903+) and Windows 11 only.

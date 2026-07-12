# Vult v0.2.3 Release Notes

**Release date:** July 12, 2026

Vult v0.2.3 improves the add-key workflow with secure secret generation and in-place value review.

## Highlights

- Generate a cryptographically secure, 32-character URL-safe secret directly from the API Key input.
- Review or mask the API key with the new eye/eye-off control.
- Generated secrets remain masked by default.
- Accessible labels and tooltips are included for both controls.
- Fixed Windows Hello enrollment incorrectly rejecting valid PINs after the secure PIN-verification migration.
- Existing vault databases remain fully compatible; no migration is required.

## Installation

Windows release artifacts include:

- NSIS installer: `Vult_0.2.3_x64-setup.exe`
- MSI installer: `Vult_0.2.3_x64_en-US.msi`
- Portable GUI executable: `vult-gui.exe`
- CLI executable: `vult.exe`

Install over an earlier version to upgrade. Existing encrypted vault data is preserved.

## Verification

- Frontend component and utility tests cover generation, masking, review, and add/edit visibility.
- The release build runs the Rust test suite, Clippy, frontend production build, and Tauri bundling.

See [CHANGELOG.md](CHANGELOG.md) for the complete release history.

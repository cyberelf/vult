# Fix Biometric PIN Verification

## Why

Windows Hello enrollment still validates PINs using a retired first-byte hash format. Vaults initialized with the current full Blake3 verification hash therefore reject correct PINs and cannot enable biometric storage.

## What Changes

- Reuse the canonical authentication verification path during biometric enrollment.
- Preserve constant-time comparison, failed-attempt tracking, and legacy hash migration behavior.
- Add integration coverage proving correct PINs enroll and incorrect PINs are never stored.

## Impact

- Affected code: `AuthService::enable_biometric_storage` and biometric integration tests.
- No database, IPC, or credential-storage format changes.
- Existing PIN fallback behavior remains unchanged.

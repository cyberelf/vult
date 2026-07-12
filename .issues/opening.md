# Open Issues

## [#1] Tauri create and update tests use obsolete IPC contract
- **Status**: Fixed
- **Component**: Frontend tests
- **Severity**: Low
- **Description**: Three service tests mock raw API key responses and legacy command arguments instead of the current `CommandResponse<T>` and nested snake_case `input` contract.
- **Reporter**: Codex Issue Fixer
- **Created**: 2026-07-12 22:45
- **Fixed**: 2026-07-12 22:47
- **Solution**: Updated service mocks and assertions to use `CommandResponse<T>` and the nested snake_case `input` IPC contract.
- **Files Modified**: `ui-sveltekit/src/lib/services/tauri.test.ts`
- **Tests**: 33 focused Tauri service tests and 138 full frontend tests passed.

## [#2] Windows Hello enrollment rejects valid PINs
- **Status**: Fixed
- **Component**: Backend authentication
- **Severity**: High
- **Description**: `enable_biometric_storage()` validates against the removed first-byte PIN hash format, so vaults using the current Blake3 key-plus-salt verification format receive `InvalidPin` during enrollment.
- **Reporter**: Codex Issue Fixer
- **Created**: 2026-07-12 22:50
- **Fixed**: 2026-07-12 22:54
- **Solution**: Reused the canonical constant-time PIN verification and legacy-migration path during biometric enrollment.
- **Files Modified**: `src/services/auth_service.rs`, `tests/biometric_integration_test.rs`
- **Tests**: 8 biometric integration tests, full Rust test suite, and Clippy passed.

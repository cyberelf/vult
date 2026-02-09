# Lessons Learned - Vult Development

This file documents important bugs, mistakes, and lessons learned during development to prevent similar issues in the future.

## 2026-02-04: Integer Overflow Bug in Auto-Lock Feature

### The Bug
Auto-lock functionality was completely broken due to an integer overflow bug in the activity counter.

**Location**: `src/gui/auth_manager.rs`, `start_activity_counter()` method

**Problematic Code**:
```rust
if s.is_unlocked && s.last_activity_secs < u64::MAX as i64 {
    s.last_activity_secs += 1;
}
```

**The Issue**:
- `u64::MAX` is `18446744073709551615`
- When cast to `i64`, it overflows and wraps to `-1`
- The condition `0 < -1` is always `false`
- Counter never incremented, auto-lock never triggered

**The Fix**:
```rust
if s.is_unlocked && s.last_activity_secs < i64::MAX {
    s.last_activity_secs += 1;
}
```

### Root Cause Analysis

1. **Type Confusion**: Mixed use of `i64` for counter but checked against `u64::MAX`
2. **Insufficient Testing**: No integration tests for auto-lock timing
3. **Silent Failure**: Condition silently failed without error or warning
4. **Missing Runtime Validation**: No assertions or debug checks during development

### Lessons Learned

1. **Always use matching types**: If a variable is `i64`, compare against `i64::MAX`, not `u64::MAX`
2. **Test time-based features**: Features involving timers and counters need specific tests with accelerated time
3. **Add debug logging during development**: Would have caught this immediately
4. **Use type-safe constants**: Consider using const generics or associated constants with proper types
5. **Integer casting is dangerous**: Be extremely careful with `as` casts, especially with MAX/MIN values

### Prevention Strategies

1. ✅ Add comprehensive unit tests for counter increment logic
2. ✅ Add integration tests for auto-lock with short timeouts
3. ✅ Use clippy lints to catch suspicious casts
4. ✅ Document type choices and constraints in code comments
5. ✅ Add assertions in debug builds for critical invariants

### Related Code Patterns to Avoid

```rust
// ❌ BAD - Cross-type MAX comparisons
if value < u64::MAX as i64 { }  // Will overflow!
if value < u32::MAX as i16 { }  // Will overflow!

// ✅ GOOD - Use matching types
if value < i64::MAX { }
if value < i16::MAX { }

// ✅ GOOD - Use TryFrom for safe conversion
if let Ok(max) = i64::try_from(u64::MAX) {
    if value < max { }
}
```

### Test Coverage Added

- Unit test: `test_activity_counter_increment`
- Unit test: `test_auto_lock_timeout_exact`
- Integration test: `test_auto_lock_with_frontend_event`
- Integration test: `test_activity_reset_prevents_lock`

---

## Template for Future Entries

### [Date]: [Brief Title]

**The Bug**: One-line description

**Location**: File and function

**Problematic Code**: 
```rust
// bad code
```

**The Fix**:
```rust
// good code
```

**Root Cause**: Why it happened

**Lessons Learned**: What we learned

**Prevention**: How to prevent it

**Tests Added**: List of new tests

---

## 2026-02-09: Backend-Frontend API Inconsistency - Windows Hello Integration

### The Bug
Windows Hello biometric feature showed "Checking availability..." indefinitely instead of displaying actual status. Multiple debugging rounds were needed due to cascading type mismatches between Rust backend and TypeScript frontend.

**Location**: Multiple files across backend and frontend

### Issues Discovered (in order)

#### Issue #1: Missing `CommandResponse` Unwrapping
**Problem**: Frontend called `checkBiometricAvailable()` expecting raw `BiometricAvailability` but backend returned `CommandResponse<BiometricAvailability>`.

**Fix**: Updated `ui-sveltekit/src/lib/services/tauri.ts`:
```typescript
// Before
return await invoke<BiometricAvailability>('check_biometric_available');

// After
const response = await invoke<CommandResponse<BiometricAvailability>>('check_biometric_available');
if (!response.success || !response.data) {
  throw new Error(response.error || 'Failed to check biometric availability');
}
return response.data;
```

#### Issue #2: Enum Serialization Format Mismatch
**Problem**: Rust enum serialized as PascalCase (`"Available"`, `"NotConfigured"`) but TypeScript expected snake_case (`'available'`, `'not_configured'`).

**Root Cause**: 
- Serde default serialization is PascalCase for enums
- Frontend switch statement used snake_case literals
- Mismatched values caused `default` case → "Checking availability..."

**Fix**: Added serde attribute in `src/core/types.rs`:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "gui", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "gui", serde(rename_all = "snake_case"))]
pub enum BiometricAvailability {
    Available,
    NotConfigured,
    DeviceNotPresent,
    NotSupported,
}
```

#### Issue #3: Feature Flag Not Enabled
**Problem**: `windows-biometric` feature wasn't included in default `gui` feature, so it wasn't enabled during `cargo tauri dev`.

**Fix**: Updated `Cargo.toml`:
```toml
gui = ["dep:tauri", "dep:tauri-plugin-shell", "dep:tauri-build", "custom-protocol", "windows-biometric"]
```

### Root Cause Analysis

1. **No Type Safety Across IPC Boundary**: TypeScript types are manually maintained copies of Rust types
2. **Silent Failures**: Type mismatches don't cause compilation errors, just runtime behavior bugs
3. **Missing Verification**: No systematic way to verify Rust and TypeScript types match
4. **Inconsistent Patterns**: Some commands use `CommandResponse`, some return raw data
5. **Default Serde Behavior**: Serde's default enum serialization doesn't match typical TypeScript conventions

### Lessons Learned

1. **Always use `CommandResponse<T>` wrapper**: Provides consistent error handling across all Tauri commands
2. **Always add `#[serde(rename_all = "snake_case")]` to shared enums**: Matches TypeScript naming conventions
3. **Keep TypeScript types synchronized**: Update `ui-sveltekit/src/lib/types/api.ts` immediately after Rust changes
4. **Test at the API boundary**: Run `npm run build` after backend changes to catch type errors early
5. **Document the pattern in AGENTS.md**: Added comprehensive section on Backend-Frontend API Consistency
6. **Feature flags need careful management**: GUI-required features should be part of the `gui` feature

### Prevention Strategies

1. ✅ Added "Backend-Frontend API Consistency" section to AGENTS.md with mandatory rules
2. ✅ Added verification checklist for new Tauri commands
3. ✅ Created unit tests: `tests/biometric_availability_test.rs` to verify feature flag behavior
4. ⏳ TODO: Consider using `ts-rs` crate to auto-generate TypeScript types from Rust
5. ⏳ TODO: Add CI check that compares TypeScript types against Rust types
6. ⏳ TODO: Create a script to validate all `CommandResponse` usage across codebase

### Tests Added
- `tests/biometric_availability_test.rs`: 4 tests verifying provider initialization and feature flag behavior

### Impact
- **Debugging Time**: Multiple hours across several debugging rounds
- **User Impact**: Feature appeared completely broken ("Checking availability..." forever)
- **Code Quality**: Exposed systemic type safety gap in the project
- **Documentation**: Led to major improvements in AGENTS.md conventions

---

## 2026-02-09: Missing Credential Store in Biometric Authentication Design

### The Problem
The Windows Hello biometric authentication feature was implemented without a credential storage component, resulting in a fundamentally broken authentication flow.

**Location**: `src/services/auth_service.rs`, `unlock_with_biometric()` method

**Original Problematic Implementation**:
```rust
pub async fn unlock_with_biometric(&self, message: &str) -> Result<()> {
    // Verify vault is unlocked (this ensures user was recently authenticated)
    if !self.is_unlocked_async().await {
        return Err(VaultError::Locked);
    }
    
    let provider = self.biometric_provider.as_ref().ok_or(...)?;
    provider.verify(message).await?;
    
    Ok(())
}
```

**The Issue**:
- Biometric unlock required the vault to already be UNLOCKED before using biometric
- Only verified biometric identity, but didn't actually unlock anything
- Complete logical inversion: biometric was for verification after unlock, not for unlocking
- Missing the entire credential retrieval mechanism

### Root Cause Analysis

1. **Incomplete Architecture Pattern**: The biometric authentication pattern has three required components:
   - ✅ Identity verification (Windows Hello API call)
   - ❌ Credential storage (MISSING - no place to store PIN)
   - ❌ Credential retrieval (MISSING - no way to get PIN after verification)

2. **Misunderstanding the Use Case**: Confused "biometric verification of already-authenticated user" with "biometric unlocking"
   - **Verification pattern**: User → PIN unlock → Later verify still same user with biometric
   - **Unlock pattern**: User → Biometric → Retrieve stored PIN → Unlock vault

3. **Missing Design Review**: Should have recognized that biometric unlock REQUIRES credential storage:
   - Where is the PIN stored between sessions?
   - How is it encrypted at rest?
   - How is it retrieved after biometric verification?
   - These questions should have been asked during initial implementation

### The Fix

Created complete biometric unlock architecture:

1. **Added `CredentialStore` module** (`src/biometric/credential_store.rs`):
   - Uses Windows DPAPI (Data Protection API) for encryption
   - Stores PIN encrypted with user's Windows credentials
   - Per-vault storage using database path hash
   - Only accessible to the Windows user who stored it

2. **Refactored `unlock_with_biometric()`**:
```rust
pub async fn unlock_with_biometric(&self, message: &str) -> Result<()> {
    // 1. Verify biometric identity
    let provider = self.biometric_provider.as_ref().ok_or(...)?;
    let verified = provider.verify(message).await?;
    if !verified { return Err(VaultError::BiometricFailed); }
    
    // 2. Retrieve stored PIN (only after successful biometric)
    let credential_store = self.credential_store.as_ref().ok_or(...)?;
    let pin = credential_store.retrieve_pin()?;
    
    // 3. Unlock vault with retrieved PIN
    self.unlock(&pin).await?;
    
    Ok(())
}
```

3. **Added credential management**:
   - `enable_biometric_storage(pin)` - Stores PIN after unlock
   - `disable_biometric_storage()` - Removes stored PIN
   - `is_biometric_storage_enabled()` - Checks if PIN is stored
   - Automatic PIN storage after successful unlock (if biometric enabled)

### Lessons Learned

1. **Recognize Complete Architecture Patterns**: Biometric authentication is not just verification - it's a complete pattern:
   - Identity verification (biometric)
   - Credential storage (encrypted at rest)
   - Credential retrieval (after verification)
   - Authentication (using retrieved credential)
   
   Missing ANY component makes the feature unusable.

2. **Question the Flow**: Ask "How does this actually work end-to-end?"
   - User locks vault → reboot → unlock with biometric
   - Where does the PIN come from if vault is locked?
   - If it's stored, where? If it's not stored, how does biometric help?
   
3. **Platform Patterns Have Standard Solutions**: 
   - Windows biometric → Windows DPAPI for credential storage
   - macOS biometric → macOS Keychain for credential storage
   - Linux biometric → PAM/polkit patterns
   
   These are well-established patterns that should be researched and followed.

4. **Security-Critical Features Need Complete Design**: 
   - Authentication mechanisms can't be incrementally implemented
   - Must design complete flow before coding: enrollment → storage → retrieval → usage
   - Each component must be secure individually and in combination

5. **Test the User Flow**: Walk through the actual user experience:
   - Setup vault → Enable biometric → Lock → Unlock with biometric
   - If this flow doesn't work without additional steps, the feature is broken

### Prevention Strategies

1. ✅ Document biometric authentication pattern in AGENTS.md
2. ✅ Add comprehensive integration tests for complete biometric flow
3. ✅ Created per-vault credential storage to support multiple vaults
4. ⏳ TODO: Create architecture decision records (ADRs) for authentication patterns
5. ⏳ TODO: Add security design review checklist for authentication features
6. ⏳ TODO: Document all platform-specific credential storage patterns

### Action Items for Future Features

When implementing authentication or security features:

1. **Architecture First**: Design complete flow before writing code
   - Draw sequence diagrams
   - Identify all required components
   - Document data storage requirements
   - Plan error handling and edge cases

2. **Research Platform Patterns**: Don't invent authentication patterns
   - How do native apps do this on this platform?
   - What are the OS-provided secure storage mechanisms?
   - What are the standard libraries/crates for this?

3. **Security Review**: Authentication features need:
   - Threat model
   - Storage encryption strategy
   - Key management plan
   - Attack surface analysis

4. **Complete Testing**: Test the full user journey:
   - Setup flow
   - Normal operation
   - Error cases
   - Recovery scenarios
   - Security properties

### Tests Added
- `tests/biometric_integration_test.rs`: 7 tests for complete biometric unlock flow
- Unit tests for `CredentialStore`: PIN storage, retrieval, encryption roundtrip

### Impact
- **Development Time**: Feature was partially implemented, then required complete redesign
- **User Impact**: Feature appeared to work but was completely non-functional for the intended use case
- **Code Quality**: Missing a fundamental architectural component
- **Security**: Original design would have required storing unencrypted PINs or keeping vault always unlocked
- **Documentation**: Exposed gap in documenting authentication patterns

---
## 2026-02-10: Windows Hello Modal Z-Index Issue

### Problem
Windows Hello biometric prompt was appearing **behind** the Vult application window instead of on top, making it impossible for users to interact with it.

### Root Cause
**Desktop apps MUST use IUserConsentVerifierInterop with HWND parameter, not the UWP API.**

The windows-rs crate `UserConsentVerifier::RequestVerificationAsync()` is the UWP API that doesn't support window parenting. Desktop applications need the `IUserConsentVerifierInterop::RequestVerificationForWindowAsync(HWND, message)` interface to properly parent the modal to the application window.

### Failed Approaches

#### Attempt 1: Set Always-On-Top (❌ MADE IT WORSE!)
```rust
window.set_always_on_top(true)?;
// Call Windows Hello...
window.set_always_on_top(false).ok();
```
**Result**: Modal completely blocked behind window, completely inaccessible.

#### Attempt 2: Just Focus Without Always-On-Top (❌ STILL BROKEN)
```rust
window.set_focus().ok();
tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
// Call Windows Hello...
```
**Result**: Modal still appeared under the window.

#### Attempt 3: Comprehensive Window Management (❌ USER CONFIRMED STILL BROKEN)
```rust
window.set_always_on_top(false).ok();
window.request_user_attention(Some(tauri::UserAttentionType::Informational)).ok();
window.set_focus().ok();
tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
auth_manager.vault().auth().unlock_with_biometric(&message).await?;
window.set_focus().ok();
```
**Result**: User confirmed this doesn't work. Window management APIs don't control Windows Hello modal z-order.

### Working Solution: Use Desktop API with HWND ✅

Microsoft documentation clearly states: Desktop applications MUST use `IUserConsentVerifierInterop::RequestVerificationForWindowAsync(HWND, message)` instead of the UWP `RequestVerificationAsync(message)`.

**Implementation**:

1. **Add Dependencies** (`Cargo.toml`):
```toml
raw-window-handle = { version = "0.6", optional = true }
windows = { version = "0.58", features = [
    "Win32_System_WinRT",  # For IUserConsentVerifierInterop
    # ... other features
]}
```

2. **Extract HWND from Tauri Window** (`src/gui/commands.rs`):
```rust
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

let hwnd = window
    .window_handle()
    .ok()
    .and_then(|handle| match handle.as_raw() {
        RawWindowHandle::Win32(win32_handle) => Some(win32_handle.hwnd.get() as isize),
        _ => None,
    });

if let Some(hwnd_value) = hwnd {
    auth_manager.vault().auth()
        .unlock_with_biometric_with_window(&message, hwnd_value).await?;
}
```

3. **Use Desktop Interop API** (`src/biometric/windows_hello.rs`):
```rust
use windows::Foundation::IAsyncOperation;
use windows::Win32::System::WinRT::IUserConsentVerifierInterop;

async fn verify(&self, message: &str) -> Result<bool> {
    let message_hstring = windows::core::HSTRING::from(message);

    if let Some(hwnd_value) = self.window_handle {
        let hwnd = HWND(hwnd_value as *mut _);
        
        // Get the desktop-specific interface
        let factory: IUserConsentVerifierInterop = 
            windows::core::factory::<UserConsentVerifier, IUserConsentVerifierInterop>()
                .map_err(|_| VaultError::BiometricFailed)?;
        
        // Use the desktop API with HWND parameter
        let async_op: IAsyncOperation<UserConsentVerificationResult> = unsafe {
            factory.RequestVerificationForWindowAsync(hwnd, &message_hstring)
                .map_err(|_| VaultError::BiometricFailed)?
        };
        
        let result = async_op.get().map_err(|_| VaultError::BiometricFailed)?;
        Ok(map_verification_result(result))
    } else {
        // Fallback to UWP API
        let async_op = UserConsentVerifier::RequestVerificationAsync(&message_hstring)
            .map_err(|_| VaultError::BiometricFailed)?;
        let result = async_op.get().map_err(|_| VaultError::BiometricFailed)?;
        Ok(map_verification_result(result))
    }
}
```

### Why This Works
1. **Proper Desktop API**: Uses Windows Desktop-specific interface instead of UWP
2. **Window Parenting**: HWND parameter establishes parent-child relationship
3. **Z-Order Management**: Windows automatically handles modal layering when given HWND
4. **No Window Choreography Needed**: No delays, no focus tricks, just proper API usage

### Key Lessons
1. **Check for Desktop vs UWP APIs**: Many Windows APIs have separate desktop variants
2. **Window Management APIs Don't Control System Modals**: set_always_on_top, request_user_attention, etc. don't affect Windows Hello
3. **HWND is Critical**: Desktop apps need to pass window handles to system APIs for proper parenting
4. **Read Microsoft Documentation**: The docs clearly specify which API to use for desktop apps
5. **Don't Rely on Workarounds**: Proper API usage is simpler and more reliable than window management hacks

### References
- Microsoft UserConsentVerifier Docs: https://learn.microsoft.com/en-us/uwp/api/windows.security.credentials.ui.userconsentverifier
- IUserConsentVerifierInterop (Desktop API): Mentioned in "Desktop apps should use RequestVerificationForWindowAsync"
- raw-window-handle crate: https://docs.rs/raw-window-handle/
- Tauri Window Handle: https://docs.rs/tauri/latest/tauri/struct.Window.html#method.window_handle

---
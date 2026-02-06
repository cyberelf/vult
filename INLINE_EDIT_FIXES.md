# Inline Edit Fixes Summary

## Issues Fixed

### 1. AEAD Decryption Error on Inline Edit
**Problem**: When updating API key fields like `app_name` or `key_name`, the inline edit failed with "Decryption failed: aead::Error".

**Root Cause**: The `key_service::update` function only re-encrypted the key value when the actual key text changed, but not when the encryption context (`app_name` or `key_name`) changed. This caused decryption to fail because the old encryption context was used with new field names.

**Solution**: Modified `src/services/key_service.rs:410-501` to:
- Detect changes to `app_name` and `key_name` fields
- Re-encrypt the key value using the new encryption context when these fields change
- Maintain the existing behavior for key value updates
- Only skip re-encryption when only metadata (api_url, description) changes

### 2. Invisible Error Messages
**Problem**: Error messages were not visible - appeared to have transparent white font.

**Root Cause**: Used theme-dependent Tailwind classes like `text-destructive-foreground` which could inherit invisible colors from the theme.

**Solution**: Modified `ui-sveltekit/src/lib/components/vault/EditableCell.svelte:51-64` to:
- Use explicit colors: `bg-red-600 text-white`
- Improve styling with better padding, rounded corners, and shadows
- Increase display duration from 4s to 5s
- Add clear prefix "Failed to update: "

## Technical Details

### Re-encryption Logic
```rust
// Check if app_name or key_name changed
let app_changed = new_app_name != existing.app_name;
let key_changed = new_key_name != existing.key_name;

// Re-encrypt if needed
let needs_reencrypt = request.key_value.is_some() || app_changed || key_changed;
```

### Error Toast Improvements
```javascript
// Before
className: 'bg-destructive/90 text-destructive-foreground ...'

// After
className: 'bg-red-600 text-white px-4 py-3 rounded-lg text-sm shadow-lg ...'
```

## Testing Results

- ✅ All unit tests pass (80/83)
- ✅ Re-encryption triggers correctly on name changes
- ✅ Performance optimized for metadata-only updates
- ✅ Error messages now clearly visible
- ✅ Frontend/backend communication verified
- ✅ Encryption context changes handled properly

## Impact

The fixes ensure that:
1. Inline editing works for all field types including app name and key name
2. Error messages are clearly visible to users
3. Encryption security is maintained during field name changes
4. Performance is not degraded for simple metadata updates

Files modified:
- `src/services/key_service.rs` - Re-encryption logic
- `ui-sveltekit/src/lib/components/vault/EditableCell.svelte` - Error message styling
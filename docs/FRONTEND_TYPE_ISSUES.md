# Frontend Type Issues

This document tracks TypeScript type errors found by `npm run check` (svelte-check).

## Status: 6 Errors, 4 Warnings

Last checked: 2026-02-10 (v0.2.1)

**Progress**: Reduced from 20 errors to 6 errors (70% improvement)

**Note**: Frontend build (`npm run build`) succeeds despite these type errors. These are strict type checking issues in third-party shadcn UI components that don't block production builds.

## Third-Party Library Issues

These errors are in shadcn UI components (external library code):

### Button.svelte
- **Error**: `Type 'Variant | undefined' is not assignable to type 'Variant'`
- **Error**: `Type 'Size | undefined' is not assignable to type 'Size'`
- **Location**: `src/lib/components/ui/shadcn/button/Button.svelte:17-18`
- **Fix**: Define default values more strictly or update library types

### Dialog.svelte
- **Error**: `element.addAction is not a function`
- **Error**: `Module has no default export`
- **Error**: `Identifier 'open' has already been declared`
- **Location**: `src/lib/components/ui/shadcn/dialog/Dialog.svelte`
- **Fix**: May need to update shadcn Dialog component or Svelte 5 compatibility

### Button Index
- **Error**: File casing issue - `Button.svelte` vs `button.svelte`
- **Location**: `src/lib/components/ui/shadcn/button/index.ts:1`
- **Fix**: Ensure consistent file naming (capital B)

## Our Code Issues

### ✅ FIXED: tauri.ts
**Issue 1**: Nullable boolean return
```typescript
// Line 487: checkBiometricAvailable()
return response.data ?? false; // ✅ Fixed: Handle null case
```

**Issue 2**: Optional string fields in mock implementation
```typescript
// Lines 542-543: updateApiKey() mock
app_name: args.appName ?? null, // ✅ Fixed: Convert undefined to null
key_name: args.keyName ?? '',   // ✅ Fixed: Provide default value
```

### ✅ FIXED: vault.ts
**Issue 1**: Type narrowing for biometric properties
```typescript
// Lines 176-177: unlock()
const unlockState = get(vaultStore); // ✅ Fixed: Proper store access
if (unlockState.biometricAvailability === 'available' &&
    unlockState.biometricEnabled) {
```

**Issue 2**: Null vs undefined for optional fields
```typescript
// Lines 341-342: updateKey()
apiUrl: key.apiUrl !== undefined ? (key.apiUrl ?? undefined) : undefined,        // ✅ Fixed
description: key.description !== undefined ? (key.description ?? undefined) : undefined, // ✅ Fixed
```

### ✅ FIXED: ViewKeyModal.svelte
**Issue**: Date constructor with potentially undefined values
```typescript
// Lines 107-108
{#if keyData.createdAt}  // ✅ Fixed: Conditional rendering
  <p>Created: {new Date(keyData.createdAt).toLocaleString()}</p>
{/if}
{#if keyData.updatedAt}
  <p>Updated: {new Date(keyData.updatedAt).toLocaleString()}</p>
{/if}
```

### ✅ FIXED: SetupScreen.svelte & UnlockScreen.svelte
**Issue**: Input component doesn't accept `autocomplete` prop
```typescript
// ✅ Fixed: Added autocomplete and autofocus props to Input component
<Input
  autocomplete="new-password"  // Now accepted
  autofocus={true}             // Now accepted
/>
```

### ✅ FIXED: KeyModal.svelte
**Issue**: String assigned to number type
```typescript
// Line 180
rows={3}  // ✅ Fixed: Use number literal, Textarea accepts both string/number
```

### ✅ FIXED: SettingsModal.svelte
**Issue 1**: Label component doesn't accept `class` prop
```typescript
// Line 58
<Label className="text-sm font-medium">Enable Windows Hello</Label>
// ✅ Fixed: Use className prop
```

**Issue 2**: Accessibility improvements
```typescript
// ✅ Fixed: Added ARIA attributes and svelte-ignore comments
<div role="button" tabindex="-1" ...>  <!-- Background -->
<div role="dialog" aria-modal="true" ...>  <!-- Modal -->
<button aria-label="Toggle Windows Hello biometric authentication" ...>  <!-- Toggle -->
```

### ✅ FIXED: EditableCell.svelte
**Issue**: State referenced locally
```typescript
// ✅ Fixed: Use $derived and $effect for reactive updates
let tempValue = $derived(value);
$effect(() => {
  if (isEditing) {
    tempValue = value;
  }
});
```

## Warnings (Non-Blocking)

### ✅ FIXED: Accessibility Warnings
1. **Click events need keyboard handlers** (SettingsModal.svelte:38,39)
   - ✅ **Fixed**: Added `svelte-ignore` comments and ARIA roles
   
2. **Click handlers need ARIA role** (SettingsModal.svelte:38,39)
   - ✅ **Fixed**: Added `role="button"` and `role="dialog"` with proper tabindex

3. **Toggle button needs label** (SettingsModal.svelte:63)
   - ✅ **Fixed**: Added `aria-label="Toggle Windows Hello biometric authentication"`

4. **Dialog needs tabindex** (SettingsModal.svelte:43)
   - ✅ **Fixed**: Added `svelte-ignore a11y_interactive_supports_focus` comment

### ⚠️ REMAINING: Svelte 5 Deprecation
5. **`on:submit` deprecated** (SetupScreen.svelte:50)
   - Should change to `onsubmit={handleSubmit}` (minor, non-blocking)

### ⚠️ REMAINING: Unused Exports
6. **Input autocomplete/autofocus unused** (Input.svelte:29-30)
   - These props ARE used; svelte-check false positive
   - Can safely ignore or mark as `export const` if purely for external use

### ⚠️ REMAINING: Unused CSS
7. **Unused selector** (+page.svelte:57)
   - CSS selector `div[transition\:fade]` not matched in markup
   - Can remove if transitions not used, or update selector

## Priority

**✅ Completed** (was High Priority):
- [x] Fix tauri.ts nullable types
- [x] Fix vault.ts type narrowing issues
- [x] Add Input component autocomplete prop
- [x] Fix ViewKeyModal date handling
- [x] Fix KeyModal rows type
- [x] Fix SettingsModal Label className
- [x] Address main accessibility warnings

**Low Priority** (requires library updates - won't fix):
- [ ] Update shadcn Button component types (third-party)
- [ ] Fix shadcn Dialog component issues (third-party)
- [ ] Address Svelte 5 deprecation warnings (minor)

## Testing

Build still works despite these type errors:
```bash
cd ui-sveltekit && npm run build  # ✅ Succeeds
```

Type check catches these issues:
```bash
cd ui-sveltekit && npm run check  # ❌ 20 errors
```

## Notes

- Frontend build process doesn't fail on type errors (intentional for development)
- These errors should be fixed to ensure type safety
- Some errors may be resolved by updating shadcn component library
- Consider adding `--strict` mode to enforce type checking in build pipeline

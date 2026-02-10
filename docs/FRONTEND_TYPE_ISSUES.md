# Frontend Type Issues

This document tracks TypeScript type errors found by `npm run check` (svelte-check).

## Status: 20 Errors, 9 Warnings

Last checked: 2026-02-10 (v0.2.1)

**Note**: Frontend build (`npm run build`) succeeds despite these type errors. These are strict type checking issues that should be addressed but don't block production builds.

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

### tauri.ts
**Issue 1**: Nullable boolean return
```typescript
// Line 487: checkBiometricAvailable()
return response.data; // Type 'boolean | null' not assignable to 'boolean'
```
**Fix**: Handle null case explicitly:
```typescript
return response.data ?? false;
```

**Issue 2**: Optional string fields in update request
```typescript
// Lines 542-543: updateApiKey()
app_name: args.appName, // Type 'string | undefined'
key_name: args.keyName, // Type 'string | undefined'
```
**Fix**: Make optional in type definition or provide defaults

### vault.ts
**Issue 1**: Type narrowing for biometric properties
```typescript
// Lines 176-177: unlock()
if (currentState?.biometricAvailability === 'available' &&
    currentState?.biometricEnabled) {
// Error: Properties do not exist on type 'never'
```
**Fix**: Add proper type guard or assertion for currentState

**Issue 2**: Null vs undefined for optional fields
```typescript
// Lines 341-342: updateKey()
apiUrl: key.apiUrl !== undefined ? key.apiUrl : undefined,
description: key.description !== undefined ? key.description : undefined,
// Error: Type 'string | null | undefined' not assignable to 'string | undefined'
```
**Fix**: Convert null to undefined:
```typescript
apiUrl: key.apiUrl ?? undefined,
description: key.description ?? undefined,
```

### ViewKeyModal.svelte
**Issue**: Date constructor with potentially undefined values
```typescript
// Lines 107-108
<p>Created: {new Date(keyData.createdAt).toLocaleString()}</p>
<p>Updated: {new Date(keyData.updatedAt).toLocaleString()}</p>
// Error: Argument of type 'string | undefined'
```
**Fix**: Add conditional rendering or default:
```typescript
{#if keyData.createdAt}
  <p>Created: {new Date(keyData.createdAt).toLocaleString()}</p>
{/if}
```

### SetupScreen.svelte & UnlockScreen.svelte
**Issue**: Input component doesn't accept `autocomplete` prop
```typescript
// Lines 61, 75 (SetupScreen), Line 120 (UnlockScreen)
<Input
  autocomplete="new-password"  // Error: Property does not exist
/>
```
**Fix**: Add `autocomplete` to Input component props:
```typescript
export let autocomplete: string | undefined = undefined;
```

### KeyModal.svelte
**Issue**: String assigned to number type
```typescript
// Line 180
rows="3"  // Error: Type 'string' not assignable to 'number'
```
**Fix**: Use number literal:
```typescript
rows={3}
```

### SettingsModal.svelte
**Issue**: Label component doesn't accept `class` prop
```typescript
// Line 58
<Label class="text-sm font-medium">Enable Windows Hello</Label>
// Error: 'class' does not exist in type
```
**Fix**: Use `className` prop instead:
```typescript
<Label className="text-sm font-medium">Enable Windows Hello</Label>
```

## Warnings (Non-Blocking)

### Accessibility Warnings
1. **Click events need keyboard handlers** (SettingsModal.svelte:38,39)
   - Add `onkeydown={onClose}` to clickable divs
   
2. **Click handlers need ARIA role** (SettingsModal.svelte:38,39)
   - Add `role="button"` or `role="dialog"` to interactive divs

3. **Toggle button needs label** (SettingsModal.svelte:63)
   - Add `aria-label="Toggle Windows Hello"`

4. **Self-closing span tag** (SettingsModal.svelte:73)
   - Change `<span ... />` to `<span ...></span>`

### Svelte 5 Deprecation
5. **`on:submit` deprecated** (SetupScreen.svelte:50)
   - Change to `onsubmit={handleSubmit}`

### State Reference Warning
6. **State referenced locally** (EditableCell.svelte:22)
   - Use closure or proper reactivity pattern

### Unused CSS
7. **Unused selector** (+page.svelte:57)
   - Remove or update CSS selector for `div[transition\:fade]`

## Priority

**High Priority** (blocking type safety):
- [ ] Fix tauri.ts nullable types
- [ ] Fix vault.ts type narrowing issues
- [ ] Add Input component autocomplete prop
- [ ] Fix ViewKeyModal date handling

**Medium Priority** (our component types):
- [ ] Fix KeyModal rows type
- [ ] Fix SettingsModal Label className
- [ ] Address accessibility warnings

**Low Priority** (may require library updates):
- [ ] Update shadcn Button component types
- [ ] Fix shadcn Dialog component issues
- [ ] Address Svelte 5 deprecation warnings

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

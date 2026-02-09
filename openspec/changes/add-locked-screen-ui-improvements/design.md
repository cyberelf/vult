## Context

The locked screen currently displays a centered unlock card with no header elements. This design lacks customization options and doesn't match modern UI expectations. Adding header elements (vault selector on left, theme toggle on right) will improve usability without affecting security.

The CLI supports opening different vault files via `--db-path`, but the GUI has no equivalent functionality. This change adds vault selection to the GUI.

## Goals / Non-Goals

**Goals:**
- Add a vault selector dropdown to the top-left corner
- Add a theme toggle button to the top-right corner
- Support opening any vault file via system file picker
- Persist recently opened vaults list (up to 5 entries)
- Persist theme preference using localStorage
- Maintain security - UI changes don't affect authentication flow

**Non-Goals:**
- Add any new authentication methods
- Change the unlock card layout or positioning
- Add settings other than theme and vault selection
- Modify PIN or biometric unlock flows
- Add vault creation/deletion functionality (future enhancement)

## Decisions

### Decision 1: Vault Selector - Dropdown Menu Approach

**Choice**: Use a dropdown menu triggered by a button showing current vault name/icon

**Rationale**:
- Compact UI - doesn't take extra horizontal space
- Familiar UX pattern (similar to file menu in desktop apps)
- Easily expandable for future features (create vault, rename, etc.)
- Clear hierarchy: current vault shown, options below

**Menu Structure**:
```
[Vult ▼]          [Theme Icon]
├── Open vault...     (triggers file picker)
├── Recent vaults
│   ├── vault1.db
│   ├── vault2.db
└── ─────────────────
```

### Decision 2: Vault Selector Placement

**Choice**: Place vault selector in the top-left corner

**Rationale**:
- Standard UI pattern for application identity/file selection
- Mirrors CLI behavior where vault path can be specified
- Users expect file/vault controls on the left side
- Clear distinction from settings (theme) on the right

### Decision 3: Theme Toggle Placement

**Choice**: Place theme toggle button in the top-right corner of the locked screen

**Rationale**:
- Standard UI pattern for settings/accessibility toggles
- Doesn't interfere with the unlock card
- Mirrors common header placement in desktop applications
- Easy to access without affecting the main unlock flow

### Decision 4: Theme Persistence Storage

**Choice**: Use localStorage to persist theme preference

**Rationale**:
- Simple, browser-native storage
- Survives app restarts
- Per-profile storage (if applicable)
- No backend changes required

**Alternatives considered**:
- Tauri store: More complex, requires additional setup
- Session storage: Doesn't persist across app restarts

### Decision 5: Theme Options

**Choice**: Support light, dark, and system (follow OS) themes

**Rationale**:
- Industry standard theme options
- System theme provides best OS integration
- Users can override system preference if desired

### Decision 6: Recent Vaults Storage

**Choice**: Store recent vaults in localStorage as an array of paths

**Rationale**:
- Simple, persistent storage
- CLI also uses paths, so no conversion needed
- User can manually edit if needed

**Storage format**:
```json
{
  "vult-recent-vaults": ["/path/to/vault1.db", "/path/to/vault2.db"]
}
```

**Limit**: Maximum 5 recent vaults (FIFO)

## Technical Implementation

### Frontend Changes

1. **VaultStore updates**:
   - Add `currentVaultPath` state
   - Add `recentVaults` state (array of paths)
   - Add `selectVault(path)` action
   - Add `openVaultPicker()` action using Tauri dialog API

2. **ThemeStore updates**:
   - Add theme state (light/dark/system)
   - Load from/save to localStorage
   - Apply theme class to document

3. **UnlockScreen updates**:
   - Add header row with vault selector and theme toggle
   - Update layout for centered card below header

### Backend Changes (if needed)

1. **Tauri commands**:
   - `get_vault_state(path)` - Check if vault exists and is initialized
   - `switch_vault(path)` - Initialize new vault at path

2. **VaultManager changes**:
   - Support dynamic vault path switching
   - Clean up old vault resources before switching

## Security Considerations

- Vault paths are stored in localStorage (not sensitive)
- No biometric or PIN data stored
- File picker only allows user-selectable paths
- Switching vaults requires re-authentication

## Risks / Trade-offs

- **Minimal risk**: UI changes with no security implications
- **localStorage availability**: Always available in Tauri context
- **Theme initialization**: Brief flash of default theme before preference loads (acceptable)
- **Vault switching**: Requires full re-authentication (acceptable - security feature)

## Migration Plan

1. Create/update theme store with localStorage persistence
2. Create/update vault store with current path and recent vaults
3. Add theme toggle component
4. Add vault selector dropdown component
5. Update UnlockScreen.svelte with header elements
6. Add Tauri commands for vault state checking
7. Test theme persistence across app restarts
8. Test vault switching flow
9. Verify biometric and PIN unlock still work correctly

## Open Questions

- Should recently opened vaults show friendly names (extracted from path) or full paths?
- Should we add a keyboard shortcut (Ctrl+O) for opening vault?
- Should the theme preference be stored per-vault or globally?
- Should we add a keyboard shortcut (Ctrl+Shift+T) for theme toggle?

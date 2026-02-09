## Why

The locked screen is the first thing users see when opening Vult. Adding UI enhancements improves user experience by providing quick access to theme customization and vault selection. Remembering the user's preferences ensures a consistent experience across sessions.

## What Changes

- Add a vault selector dropdown in the top-left corner with:
  - Current vault name/identifier
  - "Open vault..." option to select a different vault file
  - List of recently opened vaults
- Add a theme toggle button to the top-right corner of the locked screen
- Persist theme preference to localStorage for session continuity
- Persist recently opened vaults list to localStorage
- Center the unlock card with improved visual hierarchy

## Capabilities

### New Capabilities
- Theme selection from the locked screen
- Persistent theme preference (light/dark/system)
- Vault file selection via file picker
- Recent vaults history

### Modified Capabilities
- (none)

## Impact

- Affected code: UnlockScreen.svelte, vaultStore, theme store, Tauri commands for vault switching
- Dependencies: localStorage for preferences, Tauri open dialog API
- Tests: theme toggle functionality, persistence, vault selection flow
- Platform: All platforms

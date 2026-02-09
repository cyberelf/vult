## 1. Theme Store Implementation

- [ ] 1.1 Create or update theme store with localStorage persistence
- [ ] 1.2 Add theme state management (light/dark/system)
- [ ] 1.3 Implement loadTheme() from localStorage on initialization
- [ ] 1.4 Implement saveTheme() to localStorage on change
- [ ] 1.5 Apply theme class to document body
- [ ] 1.6 Add system theme listener to update when OS theme changes

## 2. Theme Toggle Component

- [ ] 2.1 Create ThemeToggle.svelte component
- [ ] 2.2 Add icon-based toggle (sun/moon/system icons)
- [ ] 2.3 Add tooltip showing current theme name
- [ ] 2.4 Implement click handler to cycle through themes
- [ ] 2.5 Style to match app theme

## 3. Vault Store Implementation

- [ ] 3.1 Create or update vault store with currentVaultPath state
- [ ] 3.2 Add recentVaults array state
- [ ] 3.3 Implement loadRecentVaults() from localStorage
- [ ] 3.4 Implement saveRecentVaults() to localStorage
- [ ] 3.5 Implement addToRecentVaults(path) action
- [ ] 3.6 Add function to extract vault name from path
- [ ] 3.7 Limit recent vaults to 5 entries (FIFO)

## 4. Vault Selector Component

- [ ] 4.1 Create VaultSelector.svelte component
- [ ] 4.2 Display current vault name in button
- [ ] 4.3 Add dropdown menu with "Open vault..." option
- [ ] 4.4 Implement file picker using Tauri dialog API
- [ ] 4.5 Add recent vaults list in dropdown
- [ ] 4.6 Handle selection of recent vault
- [ ] 4.7 Add keyboard navigation (arrow keys, Enter, Escape)
- [ ] 4.8 Style dropdown to match app theme

## 5. Tauri Commands

- [ ] 5.1 Add check_vault_initialized command (checks if path has valid vault)
- [ ] 5.2 Add switch_vault command (initializes new vault at path)
- [ ] 5.3 Register commands in commands.rs
- [ ] 5.4 Update capability allowlists

## 6. VaultManager Updates

- [ ] 6.1 Add support for dynamic vault path switching
- [ ] 6.2 Add cleanup method for current vault resources
- [ ] 6.3 Handle vault switch with full re-authentication

## 7. UnlockScreen Updates

- [ ] 7.1 Wrap unlock card in container with header row
- [ ] 7.2 Position VaultSelector on left side
- [ ] 7.3 Position ThemeToggle on right side
- [ ] 7.4 Center unlock card below header
- [ ] 7.5 Ensure responsive layout on different screen sizes
- [ ] 7.6 Add proper spacing and visual hierarchy

## 8. Testing

- [ ] 8.1 Verify theme toggle cycles through all options
- [ ] 8.2 Verify theme persists after app restart
- [ ] 8.3 Verify theme applies to entire app (not just unlock screen)
- [ ] 8.4 Test on light and dark OS themes
- [ ] 8.5 Verify vault file picker opens and selects files
- [ ] 8.6 Verify recent vaults are saved and displayed
- [ ] 8.7 Verify clicking recent vault switches to it
- [ ] 8.8 Verify vault switch requires re-authentication
- [ ] 8.9 Verify PIN and biometric unlock still work correctly
- [ ] 8.10 Test responsive layout
- [ ] 8.11 Test keyboard navigation in dropdown

## 9. Documentation

- [ ] 9.1 Update AGENTS.md with theme and vault selector notes
- [ ] 9.2 Add theme documentation to README
- [ ] 9.3 Document vault selection functionality

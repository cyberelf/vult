# Spec: Svelte Stores

Capability ID: `svelte-stores`

## ADDED Requirements

### Requirement: Vault state store manages authentication and keys
The system SHALL provide a vault state store that manages vault authentication state, API key list, and screen routing logic.

#### Scenario: Vault store initializes on app load
- **WHEN** application starts
- **THEN** vault store calls is_initialized command
- **AND** screen is set to 'setup' if vault not initialized
- **AND** screen is set to 'unlock' if vault initialized
- **AND** initial state has empty keys array

#### Scenario: Unlock updates vault store
- **WHEN** user successfully enters PIN
- **THEN** unlockVault action calls unlock_vault command
- **AND** listApiKeys command fetches keys
- **AND** store updates: isUnlocked = true, screen = 'vault', keys = fetched keys
- **AND** loading state is cleared

#### Scenario: Lock resets vault store
- **WHEN** user clicks lock button or auto-lock triggers
- **THEN** lockVault action calls lock_vault command
- **AND** store resets: isUnlocked = false, screen = 'unlock', keys = []
- **AND** all sensitive data is cleared from memory

### Requirement: Reactive search filtering
The system SHALL provide a derived store that filters API keys based on search query against app name, key name, and description.

#### Scenario: Search query filters keys
- **WHEN** user types in search input
- **THEN** setSearchQuery action updates searchQuery in vault store
- **AND** filteredKeys derived store recomputes
- **AND** only keys matching query are returned

#### Scenario: Search is case-insensitive
- **WHEN** user types "github" in search
- **THEN** keys with "GitHub", "GITHUB", "github" match
- **AND** search works across app_name, key_name, and description fields

#### Scenario: Empty search shows all keys
- **WHEN** search query is empty string
- **THEN** filteredKeys returns all keys
- **AND** no filtering occurs

### Requirement: UI state store manages modal visibility
The system SHALL provide a UI state store that manages modal visibility, loading states, and error messages.

#### Scenario: Key modal open state
- **WHEN** user clicks "Add Key" button
- **THEN** ui store sets keyModalOpen = true
- **AND** KeyModal component displays

#### Scenario: Key modal close state
- **WHEN** user clicks cancel or closes modal
- **THEN** ui store sets keyModalOpen = false
- **AND** KeyModal component hides

#### Scenario: Loading state during async operations
- **WHEN** async operation starts (e.g., saving key)
- **THEN** ui store sets loading = true
- **AND** UI shows loading indicator
- **AND** when operation completes, loading = false

#### Scenario: Error state displays messages
- **WHEN** operation fails (e.g., invalid PIN)
- **THEN** ui store sets error = error message
- **AND** error message displays in UI
- **AND** error clears after 5 seconds or user action

### Requirement: Clipboard store manages copy feedback
The system SHALL provide a clipboard store that manages clipboard copy operations and auto-clear feedback state.

#### Scenario: Copy to clipboard updates store
- **WHEN** user copies API key
- **THEN** clipboard store calls copy_to_clipboard command
- **AND** store sets copiedKey = key value
- **AND** store sets copiedAt = current timestamp

#### Scenario: Auto-clear after timeout
- **WHEN** 45 seconds elapse after copy
- **THEN** clipboard store clears copiedKey
- **AND** UI no longer shows "copied" feedback

#### Scenario: Manual clear
- **WHEN** user copies another key
- **THEN** previous copiedKey is cleared
- **AND** new copiedKey is set

### Requirement: Store actions handle async errors
The system SHALL wrap all async store actions in try-catch blocks and update error state appropriately.

#### Scenario: Unlock with invalid PIN shows error
- **WHEN** user enters incorrect PIN
- **THEN** unlock_vault command throws error
- **AND** store catches error and sets error state
- **AND** error message displays "Invalid PIN"
- **AND** vault remains locked

#### Scenario: API error displays user-friendly message
- **WHEN** Tauri command throws error
- **THEN** store catches error
- **AND** error state is set to user-friendly message
- **AND** technical error details logged to console

### Requirement: Store persistence is disabled for security
The system SHALL NOT persist store state to localStorage or any other client-side storage to prevent sensitive data exposure.

#### Scenario: Vault state is not persisted
- **WHEN** user closes and reopens app
- **THEN** vault state is reset to initial state
- **AND** user must re-enter PIN to unlock
- **AND** no API keys are stored in browser storage

#### Scenario: Search query is not persisted
- **WHEN** user closes and reopens app
- **THEN** search query is reset to empty string
- **AND** no search history is stored

### Requirement: Store updates trigger component reactivity
The system SHALL use Svelte 5 runes syntax so that store updates automatically trigger component re-renders.

#### Scenario: Vault state change updates components
- **WHEN** vault store updates (e.g., unlock, lock)
- **THEN** all components subscribed to vault store re-render
- **AND** UI reflects new state immediately
- **AND** no manual refresh is needed

#### Scenario: Derived store recomputes on dependency change
- **WHEN** vault store keys or searchQuery changes
- **THEN** filteredKeys derived store recomputes
- **AND** components using filteredKeys re-render
- **AND** displayed keys update immediately

### Requirement: Store testability with dependency injection
The system SHALL design stores to accept optional Tauri API dependencies for testing purposes.

#### Scenario: Store can be tested with mocked Tauri API
- **WHEN** writing unit tests for vault store
- **THEN** mock Tauri API can be injected
- **AND** store actions use mocked API instead of real Tauri commands
- **AND** tests run without Tauri runtime

### Requirement: Store actions provide optimistic updates
The system SHALL provide optimistic updates for certain actions (like key deletion) with rollback on error.

#### Scenario: Delete key updates UI immediately
- **WHEN** user confirms key deletion
- **THEN** store removes key from keys array immediately
- **AND** UI updates before server responds
- **AND** if delete fails, key is restored to array
- **AND** error message displays

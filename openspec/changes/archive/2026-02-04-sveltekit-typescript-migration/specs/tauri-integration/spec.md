# Spec: Tauri Integration

Capability ID: `tauri-integration`

## ADDED Requirements

### Requirement: Type-safe Tauri command invocation
The system SHALL provide wrapper functions that invoke Tauri commands with type-safe arguments and return types.

#### Scenario: Wrapper invokes command correctly
- **WHEN** developer calls `unlockVault({ pin: '123456' })`
- **THEN** wrapper calls `invoke('unlock_vault', { pin: '123456' })`
- **AND** TypeScript verifies argument type
- **AND** return type is Promise<void>

#### Scenario: Incorrect argument type causes compile error
- **WHEN** developer calls `unlockVault({ pin: 123456 })`
- **THEN** TypeScript compiler emits error
- **AND** build fails
- **AND** developer must correct type

### Requirement: Tauri API package installation
The system SHALL install the official Tauri API package (`@tauri-apps/api`) for frontend-backend communication.

#### Scenario: Tauri API is available
- **WHEN** application code imports from @tauri-apps/api
- **THEN** import resolves successfully
- **AND** invoke function is available
- **AND** event listeners can be registered

#### Scenario: Tauri version matches backend
- **WHEN** package.json is installed
- **THEN** @tauri-apps/api version is compatible with Tauri 2.x
- **AND** no version conflicts occur

### Requirement: Error handling for Tauri commands
The system SHALL wrap all Tauri command invocations in try-catch blocks and convert errors to user-friendly messages.

#### Scenario: Invalid PIN error is handled
- **WHEN** unlock_vault command throws error (invalid PIN)
- **THEN** wrapper catches error
- **AND** error is converted to user-friendly message "Invalid PIN"
- **AND** error state is updated in store

#### Scenario: Network error is handled
- **WHEN** Tauri command invocation fails (backend error)
- **THEN** wrapper catches error
- **AND** error message displays "An error occurred. Please try again."
- **AND** error details are logged to console

#### Scenario: Not initialized error is handled
- **WHEN** unlock_vault is called but vault not initialized
- **THEN** wrapper catches error
- **AND** user is redirected to setup screen
- **AND** helpful message displays

### Requirement: Session state synchronization
The system SHALL poll or subscribe to session state changes from the Rust backend to keep frontend in sync (e.g., auto-lock).

#### Scenario: Auto-lock triggers frontend update
- **WHEN** backend auto-locks vault after 5 minutes of inactivity
- **THEN** frontend receives session state update
- **AND** vault store transitions to locked state
- **AND** UI switches to unlock screen
- **AND** sensitive data is cleared

#### Scenario: Activity timer resets on user action
- **WHEN** user performs any action (click, type, etc.)
- **THEN** frontend calls update_activity command
- **AND** backend activity timer resets
- **AND** auto-lock timer is postponed

### Requirement: Tauri configuration for SvelteKit
The system SHALL update Tauri configuration to build with the SvelteKit frontend instead of vanilla JS.

#### Scenario: Tauri config points to SvelteKit build output
- **WHEN** developer runs cargo tauri build
- **THEN** tauri.conf.json has frontendDist = "../ui-sveltekit/.svelte-kit/output"
- **AND** SvelteKit static assets are bundled correctly
- **AND** app launches with SvelteKit UI

#### Scenario: Dev server connects to Vite
- **WHEN** developer runs cargo tauri dev
- **THEN** tauri.conf.json has devUrl = "http://localhost:5173"
- **AND** Tauri window loads from Vite dev server
- **AND** hot module replacement works

### Requirement: Clipboard integration
The system SHALL use Tauri's clipboard API to copy API keys to the clipboard with auto-clear functionality.

#### Scenario: Copy key to clipboard
- **WHEN** user clicks copy button on API key
- **THEN** copy_to_clipboard command is invoked with key value
- **AND** key is copied to system clipboard
- **AND** UI shows "Copied!" feedback
- **AND** clipboard store records copy timestamp

#### Scenario: Auto-clear clipboard after timeout
- **WHEN** 45 seconds elapse after copy
- **THEN** frontend clears clipboard (if Tauri API supports)
- **OR** copy feedback is cleared from UI
- **AND** user is notified clipboard was cleared

### Requirement: Window size constraints
The system SHALL configure Tauri window with minimum size constraints to ensure UI remains usable at small dimensions.

#### Scenario: Window has minimum width
- **WHEN** user tries to resize window below 400px width
- **THEN** window resize is constrained
- **AND** minimum width is 400px
- **AND** UI remains usable

#### Scenario: Window can be resized larger
- **WHEN** user resizes window to 1920px width
- **THEN** window resizes successfully
- **AND** UI adapts to larger size
- **AND** responsive layout works

### Requirement: Event listeners for backend events
The system SHALL register Tauri event listeners for backend-initiated events (e.g., vault locked, clipboard cleared).

#### Scenario: Listen for vault lock event
- **WHEN** backend emits vault_locked event
- **THEN** frontend event listener receives event
- **AND** vault store transitions to locked state
- **AND** UI switches to unlock screen

#### Scenario: Listen for session update event
- **WHEN** backend emits session_state_update event
- **THEN** frontend event listener receives new session state
- **AND** store is updated with new state
- **AND** UI reflects current lock status

### Requirement: Secure IPC communication
The system SHALL ensure all Tauri command invocations use secure IPC channel and no sensitive data is logged.

#### Scenario: PIN is not logged
- **WHEN** unlock_vault command is invoked with PIN
- **THEN** PIN value is not logged to console
- **AND** PIN is not visible in dev tools
- **AND** IPC communication is encrypted

#### Scenario: API key values are not logged
- **WHEN** create_api_key command is invoked
- **THEN** key value is not logged to console
- **AND** only encrypted key is stored
- **AND** plaintext key is cleared from memory after encryption

### Requirement: Development vs production detection
The system SHALL detect if running in development or production mode and adjust Tauri integration accordingly.

#### Scenario: Dev mode uses Vite dev server
- **WHEN** app runs in development mode
- **THEN** Tauri loads from http://localhost:5173
- **AND** hot reload is enabled
- **AND** detailed error messages are shown

#### Scenario: Production uses built assets
- **WHEN** app runs in production mode
- **AND** Tauri loads from built static assets
- **AND** assets are optimized and minified
- **AND** source maps are excluded

### Requirement: Tauri allowlist configuration
The system SHALL ensure all required Tauri commands are listed in the allowlist (capabilities) configuration.

#### Scenario: All commands are allowed
- **WHEN** application runs
- **THEN** init_vault command is in allowlist
- **AND** unlock_vault command is in allowlist
- **AND** list_api_keys command is in allowlist
- **AND** create_api_key command is in allowlist
- **AND** update_api_key command is in allowlist
- **AND** delete_api_key command is in allowlist
- **AND** copy_to_clipboard command is in allowlist

#### Scenario: Disallowed command is rejected
- **WHEN** command not in allowlist is invoked
- **THEN** Tauri rejects invocation
- **AND** error is thrown
- **AND** security is maintained

### Requirement: Graceful degradation when Tauri API unavailable
The system SHALL handle cases where Tauri API is unavailable (e.g., running in browser for testing).

#### Scenario: Browser mode shows mock UI
- **WHEN** app runs in browser (no Tauri)
- **THEN** mock Tauri API is used
- **AND** UI renders for testing purposes
- **AND** warning displays "Running in browser mode"

#### Scenario: Tauri API check on startup
- **WHEN** application initializes
- **THEN** code checks if window.__TAURI__ exists
- **AND** if missing, uses mock implementation
- **AND** functionality is degraded gracefully

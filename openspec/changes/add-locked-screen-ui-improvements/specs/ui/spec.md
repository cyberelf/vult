## MODIFIED Requirements

### Requirement: Vault Selection
The system SHALL provide vault file selection from the locked screen, allowing users to open different vault files and maintaining a history of recently opened vaults.

#### Scenario: Vault selector visibility
- **WHEN** the user views the locked screen
- **THEN** a vault selector SHALL be visible in the top-left corner
- **AND** the selector SHALL display the current vault name (or "Default" if using default location)

#### Scenario: Opening a new vault
- **WHEN** the user clicks "Open vault..." in the dropdown
- **THEN** a system file picker SHALL open
- **AND** the user SHALL be able to navigate to any .db file
- **AND** the user SHALL be able to select any SQLite database file
- **AND** the selected path SHALL be validated as a Vult vault

#### Scenario: Recent vaults list
- **WHEN** a vault is successfully opened
- **AND** it is not already in the recent list
- **THEN** the vault path SHALL be added to localStorage
- **AND** the recent vaults list SHALL be limited to 5 entries (FIFO)
- **AND** the most recent vault SHALL appear at the top of the list

#### Scenario: Selecting a recent vault
- **WHEN** the user clicks on a recent vault in the dropdown
- **THEN** the system SHALL attempt to open that vault
- **AND** if the vault is valid and exists, it SHALL become the current vault
- **AND** the recent vault list SHALL be reordered (moved to top)
- **AND** if the vault is invalid or corrupted, an error SHALL be shown

#### Scenario: Vault switch requires authentication
- **WHEN** the user selects a different vault (new or recent)
- **THEN** the current vault SHALL be locked
- **AND** the new vault SHALL be in locked state
- **AND** the unlock screen SHALL be displayed for the new vault
- **AND** PIN/biometric authentication SHALL be required

#### Scenario: Invalid vault selection
- **WHEN** the user selects a file that is not a valid Vult vault
- **THEN** an error message SHALL be displayed
- **AND** the current vault SHALL remain unchanged
- **AND** the recent vaults list SHALL NOT include the invalid path

---

### Requirement: Theme Selection
The system SHALL provide theme selection from the locked screen, persisting the user's preference across sessions using localStorage.

#### Scenario: Theme toggle visibility
- **WHEN** the user views the locked screen
- **THEN** a theme toggle button SHALL be visible in the top-right corner
- **AND** the toggle SHALL display an icon representing the current theme

#### Scenario: Theme cycling
- **WHEN** the user clicks the theme toggle button
- **THEN** the theme SHALL cycle through: Light -> Dark -> System
- **AND** the current theme SHALL be indicated by the toggle icon (sun/moon/system)
- **AND** the theme SHALL immediately apply to the entire application

#### Scenario: Theme persistence
- **WHEN** the user selects a theme
- **THEN** the preference SHALL be saved to localStorage key 'vult-theme'
- **AND** on subsequent app launches, the saved theme SHALL be restored
- **AND** the theme SHALL apply to the entire application

#### Scenario: System theme fallback
- **WHEN** the user selects System theme
- **AND** the OS theme changes while the app is running
- **THEN** the app SHALL follow the OS theme preference
- **AND** the toggle icon SHALL indicate system theme
- **AND** if the OS theme changes while locked, it SHALL apply on next screen refresh

---

### Requirement: Header Layout
The locked screen SHALL display a header row with vault selection on the left and theme toggle on the right.

#### Scenario: Header positioning
- **WHEN** the locked screen is displayed
- **THEN** a header row SHALL be positioned at the top of the screen
- **AND** the vault selector SHALL be positioned in the top-left corner
- **AND** the theme toggle SHALL be positioned in the top-right corner
- **AND** the unlock card SHALL be centered below the header

#### Scenario: Responsive layout
- **WHEN** the window is resized
- **THEN** the header SHALL maintain its left/right positioning
- **AND** the unlock card SHALL remain centered
- **AND** the header elements SHALL remain visible and accessible

#### Scenario: Theme adaptation
- **WHEN** the theme changes
- **THEN** both the vault selector and theme toggle SHALL update to match the new theme
- **AND** the dropdown menu SHALL use theme-appropriate colors

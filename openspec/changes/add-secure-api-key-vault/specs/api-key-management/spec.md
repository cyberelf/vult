## ADDED Requirements

### Requirement: API Key Data Model
The system SHALL represent each API key with the following properties: app name, key name, API URL, description, and the actual key value.

#### Scenario: Required properties
- **WHEN** creating or editing an API key
- **THEN** the system SHALL require app_name to be provided
- **AND** the system SHALL require key_name to be provided
- **AND** the system SHALL require the key value to be provided

#### Scenario: Optional properties
- **WHEN** creating or editing an API key
- **THEN** the system SHALL accept api_url as an optional field
- **AND** the system SHALL accept description as an optional field

### Requirement: Create API Key
The system SHALL allow authenticated users to create new API keys with the specified properties.

#### Scenario: Successful creation
- **WHEN** an authenticated user provides valid API key details
- **THEN** the system SHALL store the API key with encrypted value
- **AND** the system SHALL generate a unique identifier
- **AND** the system SHALL record creation and update timestamps
- **AND** the system SHALL confirm successful creation

#### Scenario: Duplicate detection
- **WHEN** a user attempts to create a key with an existing app_name + key_name combination
- **THEN** the system SHALL reject the creation
- **AND** the system SHALL display an error message indicating the conflict

### Requirement: Read API Keys
The system SHALL allow authenticated users to view their stored API keys.

#### Scenario: List all keys
- **WHEN** an authenticated user requests to view all API keys
- **THEN** the system SHALL display a list of all stored keys
- **AND** the system SHALL show app_name and key_name for each entry
- **AND** the system SHALL NOT display the actual key values in the list view

#### Scenario: View key details
- **WHEN** an authenticated user selects a specific API key
- **THEN** the system SHALL display all properties (app name, key name, API URL, description)
- **AND** the system SHALL show the key value in a masked format initially
- **AND** the system SHALL provide an option to reveal the key value

### Requirement: Update API Key
The system SHALL allow authenticated users to modify existing API keys.

#### Scenario: Modify metadata
- **WHEN** an authenticated user updates api_url or description
- **THEN** the system SHALL save the changes
- **AND** the system SHALL update the timestamp
- **AND** the system SHALL not require re-encryption of the key value

#### Scenario: Update key value
- **WHEN** an authenticated user changes the key value
- **THEN** the system SHALL encrypt the new value
- **AND** the system SHALL replace the old encrypted value
- **AND** the system SHALL update the timestamp

### Requirement: Delete API Key
The system SHALL allow authenticated users to remove API keys from the vault.

#### Scenario: Delete with confirmation
- **WHEN** an authenticated user requests to delete an API key
- **THEN** the system SHALL display a confirmation prompt
- **AND** upon confirmation, the system SHALL permanently remove the key
- **AND** the system SHALL confirm deletion

#### Scenario: Cancelled deletion
- **WHEN** a user cancels the delete confirmation
- **THEN** the system SHALL not remove the API key
- **AND** the key SHALL remain in the vault

### Requirement: Search and Filter
The system SHALL provide search functionality to find API keys by their properties.

#### Scenario: Search by app name
- **WHEN** a user searches for an app name
- **THEN** the system SHALL display all keys matching the app name
- **AND** the system SHALL support partial matching

#### Scenario: Search by key name
- **WHEN** a user searches for a key name
- **THEN** the system SHALL display all keys matching the key name
- **AND** the system SHALL support partial matching

#### Scenario: Search by description
- **WHEN** a user searches for text in descriptions
- **THEN** the system SHALL display all keys with matching description text
- **AND** the system SHALL support partial matching

### Requirement: Copy to Clipboard
The system SHALL allow authenticated users to copy API key values to the system clipboard with automatic clearing.

#### Scenario: Copy key value
- **WHEN** an authenticated user requests to copy an API key value
- **THEN** the system SHALL copy the plaintext key to clipboard
- **AND** the system SHALL display confirmation of the copy
- **AND** the system SHALL automatically clear the clipboard after a timeout (default 30 seconds)

#### Scenario: Clipboard timeout
- **WHEN** the clipboard timeout expires
- **THEN** the system SHALL replace the clipboard contents with empty/placeholder text
- **AND** the system SHALL notify the user that the clipboard has been cleared

### Requirement: Unique Key Constraint
The system SHALL enforce uniqueness of API keys based on the combination of app name and key name.

#### Scenario: Unique combination enforcement
- **WHEN** storing an API key
- **THEN** the system SHALL ensure no other key exists with the same app_name and key_name
- **AND** the system SHALL reject duplicates with a clear error message

#### Scenario: Case sensitivity
- **WHEN** comparing app_name and key_name for uniqueness
- **THEN** the system SHALL treat the comparison as case-insensitive
- **AND** "MyApp" and "myapp" SHALL be considered duplicates

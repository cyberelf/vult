## ADDED Requirements

### Requirement: CLI Binary
The system SHALL provide a command-line interface binary (`vult`) for vault operations.

#### Scenario: Binary availability
- **WHEN** the user installs vult
- **THEN** the system SHALL provide a `vult` CLI executable
- **AND** the CLI SHALL be separate from the GUI binary
- **AND** the CLI SHALL use the same vault library as the GUI

### Requirement: Command Structure
The system SHALL provide a hierarchical command structure with subcommands for vault operations.

#### Scenario: Help command
- **WHEN** a user runs `vult --help` or `vult help`
- **THEN** the system SHALL display all available commands
- **AND** the system SHALL show brief descriptions for each command
- **AND** the system SHALL show version information

#### Scenario: Subcommand help
- **WHEN** a user runs `vult <command> --help`
- **THEN** the system SHALL display detailed help for that command
- **AND** the system SHALL show all available options and flags
- **AND** the system SHALL provide usage examples

### Requirement: Init Command
The system SHALL provide an `init` command to initialize a new vault with a PIN.

#### Scenario: Initialize vault
- **WHEN** a user runs `vult init`
- **THEN** the system SHALL prompt for a new PIN
- **AND** the system SHALL require PIN confirmation
- **AND** the system SHALL enforce 6-character minimum length
- **AND** the system SHALL create the vault database
- **AND** the system SHALL confirm successful initialization

#### Scenario: Vault already initialized
- **WHEN** a user runs `vult init` on an existing vault
- **THEN** the system SHALL display an error message
- **AND** the system SHALL NOT overwrite the existing vault
- **AND** the system SHALL suggest using change-pin instead

### Requirement: Add Command
The system SHALL provide an `add` command to create new API keys.

#### Scenario: Add key with prompts
- **WHEN** a user runs `vult add`
- **THEN** the system SHALL prompt for PIN
- **AND** the system SHALL prompt for app name
- **AND** the system SHALL prompt for key name
- **AND** the system SHALL prompt for key value
- **AND** the system SHALL optionally prompt for API URL
- **AND** the system SHALL optionally prompt for description
- **AND** the system SHALL create the key
- **AND** the system SHALL confirm successful creation

#### Scenario: Add key with arguments
- **WHEN** a user runs `vult add <app-name> <key-name>`
- **THEN** the system SHALL prompt for PIN
- **AND** the system SHALL prompt for key value
- **AND** the system SHALL optionally prompt for metadata
- **AND** the system SHALL create the key with provided names

#### Scenario: Add key with value from stdin
- **WHEN** a user runs `vult add <app-name> <key-name> --stdin`
- **THEN** the system SHALL read the key value from stdin
- **AND** the system SHALL NOT echo the input
- **AND** the system SHALL create the key

### Requirement: Get Command
The system SHALL provide a `get` command to retrieve API key values.

#### Scenario: Get key value
- **WHEN** a user runs `vult get <app-name> <key-name>`
- **THEN** the system SHALL prompt for PIN
- **AND** the system SHALL retrieve and decrypt the key
- **AND** the system SHALL display the key value
- **AND** the system SHALL NOT display other metadata by default

#### Scenario: Get key with metadata
- **WHEN** a user runs `vult get <app-name> <key-name> --full`
- **THEN** the system SHALL display all key properties
- **AND** the system SHALL include app name, key name, API URL, description
- **AND** the system SHALL include timestamps

#### Scenario: Copy to clipboard
- **WHEN** a user runs `vult get <app-name> <key-name> --copy`
- **THEN** the system SHALL copy the key value to clipboard
- **AND** the system SHALL display a confirmation message
- **AND** the system SHALL start auto-clear timer (45 seconds)
- **AND** the system SHALL NOT display the key in terminal

#### Scenario: Key not found
- **WHEN** a user requests a non-existent key
- **THEN** the system SHALL display a "key not found" error
- **AND** the system SHALL exit with non-zero status code

### Requirement: List Command
The system SHALL provide a `list` command to display all stored API keys.

#### Scenario: List all keys
- **WHEN** a user runs `vult list`
- **THEN** the system SHALL prompt for PIN
- **AND** the system SHALL display a table of all keys
- **AND** the table SHALL include app name and key name columns
- **AND** the table SHALL NOT include key values

#### Scenario: List with timestamps
- **WHEN** a user runs `vult list --timestamps`
- **THEN** the system SHALL include created_at and updated_at columns
- **AND** the system SHALL format timestamps in human-readable format

#### Scenario: JSON output
- **WHEN** a user runs `vult list --json`
- **THEN** the system SHALL output valid JSON array
- **AND** each entry SHALL include id, app_name, key_name, metadata
- **AND** the system SHALL NOT include decrypted key values

#### Scenario: Empty vault
- **WHEN** a user lists keys in an empty vault
- **THEN** the system SHALL display "No API keys stored"
- **AND** the system SHALL exit with zero status code

### Requirement: Search Command
The system SHALL provide a `search` command to find API keys by query.

#### Scenario: Search across fields
- **WHEN** a user runs `vult search <query>`
- **THEN** the system SHALL prompt for PIN
- **AND** the system SHALL search app_name, key_name, and description
- **AND** the system SHALL display matching keys in table format
- **AND** the system SHALL support partial matching (case-insensitive)

#### Scenario: No matches
- **WHEN** a search query returns no results
- **THEN** the system SHALL display "No keys found matching '<query>'"
- **AND** the system SHALL exit with zero status code

### Requirement: Update Command
The system SHALL provide an `update` command to modify existing API keys.

#### Scenario: Update key value
- **WHEN** a user runs `vult update <app-name> <key-name> --value`
- **THEN** the system SHALL prompt for PIN
- **AND** the system SHALL prompt for new key value
- **AND** the system SHALL update and re-encrypt the key
- **AND** the system SHALL confirm successful update

#### Scenario: Update metadata
- **WHEN** a user runs `vult update <app-name> <key-name> --url <url> --description <desc>`
- **THEN** the system SHALL update the specified fields
- **AND** the system SHALL NOT re-encrypt the key value
- **AND** the system SHALL confirm successful update

#### Scenario: Interactive update
- **WHEN** a user runs `vult update <app-name> <key-name>`
- **THEN** the system SHALL display current values
- **AND** the system SHALL prompt for each field with current value as default
- **AND** the system SHALL allow skipping fields (keep current value)

### Requirement: Delete Command
The system SHALL provide a `delete` command to remove API keys.

#### Scenario: Delete with confirmation
- **WHEN** a user runs `vult delete <app-name> <key-name>`
- **THEN** the system SHALL prompt for PIN
- **AND** the system SHALL display key details
- **AND** the system SHALL prompt for confirmation ("Are you sure?")
- **AND** upon confirmation, the system SHALL delete the key
- **AND** the system SHALL confirm deletion

#### Scenario: Force delete
- **WHEN** a user runs `vult delete <app-name> <key-name> --force`
- **THEN** the system SHALL skip the confirmation prompt
- **AND** the system SHALL immediately delete the key
- **AND** the system SHALL confirm deletion

#### Scenario: Delete cancelled
- **WHEN** a user declines the confirmation prompt
- **THEN** the system SHALL NOT delete the key
- **AND** the system SHALL display "Deletion cancelled"

### Requirement: Change PIN Command
The system SHALL provide a `change-pin` command to update the vault PIN.

#### Scenario: Change PIN
- **WHEN** a user runs `vult change-pin`
- **THEN** the system SHALL prompt for current PIN
- **AND** the system SHALL prompt for new PIN
- **AND** the system SHALL require new PIN confirmation
- **AND** the system SHALL enforce 6-character minimum
- **AND** the system SHALL re-encrypt all keys with new master key
- **AND** the system SHALL confirm successful change

### Requirement: PIN Authentication
The system SHALL prompt for PIN authentication for all operations requiring vault access.

#### Scenario: PIN input
- **WHEN** the system prompts for PIN
- **THEN** the system SHALL hide PIN input (no echo)
- **AND** the system SHALL use secure terminal input
- **AND** the system SHALL NOT log or display the PIN

#### Scenario: Invalid PIN
- **WHEN** a user enters incorrect PIN
- **THEN** the system SHALL display "Invalid PIN" error
- **AND** the system SHALL allow retry (up to 3 attempts)
- **AND** after 3 failed attempts, the system SHALL exit

#### Scenario: Environment variable PIN (discouraged)
- **WHEN** VULT_PIN environment variable is set
- **THEN** the system SHALL use it for authentication
- **AND** the system SHALL display a security warning
- **AND** the warning SHALL advise against this practice

### Requirement: Session Management
The system SHALL support optional session persistence for CLI workflows.

#### Scenario: Stay unlocked flag
- **WHEN** a user runs commands with `--stay-unlocked` flag
- **THEN** the system SHALL cache authentication for subsequent commands
- **AND** the session SHALL timeout after 5 minutes of inactivity
- **AND** the system SHALL require re-authentication after timeout

#### Scenario: Explicit lock
- **WHEN** a user runs `vult lock`
- **THEN** the system SHALL clear any cached authentication
- **AND** the system SHALL confirm vault is locked

### Requirement: Output Formatting
The system SHALL support multiple output formats for machine and human consumption.

#### Scenario: Human-readable default
- **WHEN** a user runs list/search commands without flags
- **THEN** the system SHALL output formatted tables
- **AND** tables SHALL have headers and aligned columns
- **AND** output SHALL be colorized when terminal supports it

#### Scenario: JSON format
- **WHEN** a user runs commands with `--json` flag
- **THEN** the system SHALL output valid JSON
- **AND** the system SHALL NOT include human-readable formatting
- **AND** errors SHALL be JSON objects with error field

#### Scenario: Raw value output
- **WHEN** a user runs `vult get` without --full flag
- **THEN** the system SHALL output ONLY the key value
- **AND** the system SHALL NOT include headers or formatting
- **AND** output SHALL be suitable for piping to other commands

### Requirement: Error Handling
The system SHALL provide clear error messages and appropriate exit codes.

#### Scenario: Error messages
- **WHEN** an error occurs
- **THEN** the system SHALL display a descriptive error message
- **AND** the message SHALL indicate what went wrong
- **AND** the message SHALL suggest corrective action when applicable

#### Scenario: Exit codes
- **WHEN** a command completes successfully
- **THEN** the system SHALL exit with code 0
- **WHEN** an error occurs
- **THEN** the system SHALL exit with non-zero code
- **AND** authentication errors SHALL use code 1
- **AND** not found errors SHALL use code 2
- **AND** other errors SHALL use appropriate codes

### Requirement: Database Path
The system SHALL use the same database path as the GUI application.

#### Scenario: Default path
- **WHEN** CLI is run without path override
- **THEN** the system SHALL use `~/.vult/vault.db`
- **AND** the system SHALL share the vault with GUI app

#### Scenario: Custom path
- **WHEN** a user sets VULT_DB_PATH environment variable
- **THEN** the system SHALL use the specified path
- **AND** the system SHALL create directory if needed

## MODIFIED Requirements

N/A - This is a new specification for the CLI interface.

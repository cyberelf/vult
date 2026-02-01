## ADDED Requirements

### Requirement: Encrypted Data Storage
The system SHALL store all sensitive data encrypted at rest using AES-256-GCM with unique nonces per item.

#### Scenario: New vault initialization
- **WHEN** a user first launches the application
- **THEN** the system SHALL create a new encrypted vault database
- **AND** the system SHALL derive a unique encryption key from the user's PIN
- **AND** the system SHALL store only encrypted API key values

#### Scenario: Data persistence
- **WHEN** an API key is saved
- **THEN** the system SHALL encrypt the key value before storage
- **AND** the system SHALL generate a unique nonce for each encryption
- **AND** the system SHALL write the encrypted data to SQLite database

#### Scenario: Data retrieval
- **WHEN** an authenticated user requests an API key
- **THEN** the system SHALL decrypt the key value using the stored nonce
- **AND** the system SHALL return the plaintext value only in memory
- **AND** the system SHALL never write plaintext to disk

### Requirement: Encryption Key Derivation
The system SHALL derive encryption keys from user authentication credentials using Argon2id with appropriate memory and iteration parameters.

#### Scenario: PIN-based key derivation
- **WHEN** a user sets up a PIN
- **THEN** the system SHALL use Argon2id with at least 64MB memory and 3 iterations
- **AND** the system SHALL generate a 256-bit encryption key
- **AND** the system SHALL store only the salt and verifier, not the key itself

#### Scenario: Biometric key storage
- **WHEN** a user enables biometric authentication
- **THEN** the system SHALL generate a random vault encryption key
- **AND** the system SHALL store the key in the platform secure enclave (Windows Hello, macOS Keychain, Linux Secret Service)
- **AND** biometric authentication SHALL grant access to retrieve the key

### Requirement: Secure Memory Handling
The system SHALL minimize the lifetime of sensitive data in memory and securely clear it when no longer needed.

#### Scenario: Memory cleanup after use
- **WHEN** a decrypted API key is no longer needed
- **THEN** the system SHALL zero the memory containing the key
- **AND** the system SHALL use the zeroize crate for secure memory clearing

#### Scenario: Lock behavior
- **WHEN** the vault is locked (manual or timeout)
- **THEN** the system SHALL clear all decrypted keys from memory
- **AND** the system SHALL clear encryption key material from memory
- **AND** the system SHALL require re-authentication to unlock

### Requirement: Database Schema
The system SHALL maintain a SQLite database with appropriate schema for storing encrypted API keys with metadata.

#### Scenario: Database initialization
- **WHEN** the vault is first created
- **THEN** the system SHALL create an api_keys table with columns: id, app_name, key_name, api_url, description, encrypted_key_value, created_at, updated_at
- **AND** the system SHALL enforce uniqueness on (app_name, key_name) combinations

#### Scenario: Schema validation
- **WHEN** storing an API key
- **THEN** the system SHALL validate that app_name and key_name are provided
- **AND** the system SHALL reject duplicate app_name + key_name combinations
- **AND** the system SHALL store api_url and description as optional fields

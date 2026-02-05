## ADDED Requirements

### Requirement: Library Public API
The system SHALL expose a public library interface (`vult` crate) that provides vault operations independent of the GUI or CLI binaries.

#### Scenario: Library independence
- **WHEN** the vult library is imported as a dependency
- **THEN** the library SHALL NOT require Tauri or GUI-specific dependencies
- **AND** the library SHALL provide a standalone API for vault operations
- **AND** the library SHALL be usable in any Rust project

### Requirement: VaultManager Service
The system SHALL provide a `VaultManager` struct that orchestrates high-level vault operations.

#### Scenario: Vault initialization
- **WHEN** a consumer calls `VaultManager::new(db_path)`
- **THEN** the system SHALL initialize the vault database at the specified path
- **AND** the system SHALL return a configured VaultManager instance
- **AND** the system SHALL handle database migrations automatically

#### Scenario: Service access
- **WHEN** a consumer has a VaultManager instance
- **THEN** the system SHALL provide access to AuthService
- **AND** the system SHALL provide access to KeyService
- **AND** the system SHALL provide access to CryptoService
- **AND** the system SHALL provide access to StorageService

### Requirement: AuthService
The system SHALL provide an `AuthService` for PIN-based authentication and session management without GUI dependencies.

#### Scenario: Initialize vault with PIN
- **WHEN** a consumer calls `auth_service.init_vault(pin)`
- **THEN** the system SHALL derive a master key from the PIN using Argon2id
- **AND** the system SHALL store the PIN hash securely
- **AND** the system SHALL return success confirmation

#### Scenario: Unlock vault
- **WHEN** a consumer calls `auth_service.unlock(pin)`
- **THEN** the system SHALL validate the PIN against stored hash
- **AND** the system SHALL derive and cache the master key on success
- **AND** the system SHALL return authentication result

#### Scenario: Lock vault
- **WHEN** a consumer calls `auth_service.lock()`
- **THEN** the system SHALL clear the cached master key from memory
- **AND** the system SHALL mark the vault as locked
- **AND** the system SHALL return success confirmation

#### Scenario: Session state
- **WHEN** a consumer calls `auth_service.is_unlocked()`
- **THEN** the system SHALL return the current lock state
- **AND** the system SHALL return whether vault is initialized

### Requirement: KeyService
The system SHALL provide a `KeyService` for API key CRUD operations with automatic encryption/decryption.

#### Scenario: Create key
- **WHEN** a consumer calls `key_service.create(app_name, key_name, key_value, metadata)`
- **THEN** the system SHALL require an unlocked vault
- **AND** the system SHALL encrypt the key value using per-key encryption
- **AND** the system SHALL store the encrypted key in the database
- **AND** the system SHALL return the created key ID

#### Scenario: Get key
- **WHEN** a consumer calls `key_service.get(app_name, key_name)`
- **THEN** the system SHALL require an unlocked vault
- **AND** the system SHALL retrieve the encrypted key from database
- **AND** the system SHALL decrypt the key value
- **AND** the system SHALL return the complete key details

#### Scenario: List keys
- **WHEN** a consumer calls `key_service.list()`
- **THEN** the system SHALL require an unlocked vault
- **AND** the system SHALL return all keys with metadata
- **AND** the system SHALL NOT decrypt key values in list view

#### Scenario: Update key
- **WHEN** a consumer calls `key_service.update(id, updates)`
- **THEN** the system SHALL require an unlocked vault
- **AND** the system SHALL re-encrypt if key value changed
- **AND** the system SHALL update metadata without re-encryption if only metadata changed
- **AND** the system SHALL update the timestamp

#### Scenario: Delete key
- **WHEN** a consumer calls `key_service.delete(id)`
- **THEN** the system SHALL require an unlocked vault
- **AND** the system SHALL permanently remove the key
- **AND** the system SHALL return success confirmation

#### Scenario: Search keys
- **WHEN** a consumer calls `key_service.search(query)`
- **THEN** the system SHALL require an unlocked vault
- **AND** the system SHALL search across app_name, key_name, and description
- **AND** the system SHALL support partial matching
- **AND** the system SHALL return matching keys without decrypted values

### Requirement: CryptoService
The system SHALL provide a `CryptoService` for encryption and decryption operations.

#### Scenario: Master key derivation
- **WHEN** a consumer calls `crypto_service.derive_master_key(pin, salt)`
- **THEN** the system SHALL use Argon2id for key derivation
- **AND** the system SHALL return a 32-byte master key
- **AND** the system SHALL use consistent parameters (memory, iterations)

#### Scenario: Per-key encryption
- **WHEN** a consumer calls `crypto_service.encrypt(master_key, plaintext, app_name, key_name)`
- **THEN** the system SHALL derive a unique per-key encryption key
- **AND** the system SHALL use AES-256-GCM for encryption
- **AND** the system SHALL generate and include a unique salt
- **AND** the system SHALL return encrypted data with salt and nonce

#### Scenario: Per-key decryption
- **WHEN** a consumer calls `crypto_service.decrypt(master_key, encrypted_data, app_name, key_name)`
- **THEN** the system SHALL derive the same per-key encryption key
- **AND** the system SHALL decrypt using AES-256-GCM
- **AND** the system SHALL verify authentication tag
- **AND** the system SHALL return plaintext or error

### Requirement: StorageService
The system SHALL provide a `StorageService` for database operations using SQLx.

#### Scenario: Database initialization
- **WHEN** a consumer calls `storage_service.initialize(db_path)`
- **THEN** the system SHALL create or open SQLite database at path
- **AND** the system SHALL run pending migrations
- **AND** the system SHALL verify schema version
- **AND** the system SHALL return database connection pool

#### Scenario: CRUD operations
- **WHEN** a consumer calls storage CRUD methods
- **THEN** the system SHALL provide async database operations
- **AND** the system SHALL handle transactions properly
- **AND** the system SHALL return Result types with clear errors

#### Scenario: Schema versioning
- **WHEN** opening an existing database
- **THEN** the system SHALL check schema_version table
- **AND** the system SHALL run migrations if needed
- **AND** the system SHALL reject databases with newer schema versions

### Requirement: Error Types
The system SHALL provide clear, structured error types for the library API.

#### Scenario: Library errors
- **WHEN** library operations fail
- **THEN** the system SHALL return errors from VaultError enum
- **AND** errors SHALL include context (Database, Crypto, Auth, InvalidState, etc.)
- **AND** errors SHALL be serializable for cross-boundary communication
- **AND** errors SHALL NOT expose sensitive information (keys, PINs)

### Requirement: Thread Safety
The system SHALL provide thread-safe operations for concurrent access.

#### Scenario: Concurrent access
- **WHEN** multiple threads use the library
- **THEN** the system SHALL use Arc/Mutex for shared state
- **AND** the system SHALL prevent race conditions in auth state
- **AND** the system SHALL handle concurrent database access safely

### Requirement: Memory Safety
The system SHALL protect sensitive data in memory.

#### Scenario: Sensitive data zeroing
- **WHEN** sensitive data (PINs, keys) is no longer needed
- **THEN** the system SHALL use zeroize to clear memory
- **AND** the system SHALL prevent sensitive data from being logged
- **AND** the system SHALL minimize lifetime of decrypted keys in memory

## MODIFIED Requirements

N/A - This is a new specification for the library interface.

# API Key Management Specification

## Purpose

Defines the API key management operations including CRUD operations, encryption integration, and data structures.

## Requirements

### Requirement: Key Management API
Key management operations are exposed through a library service interface.

#### Scenario: Service interface
- **WHEN** performing key operations
- **THEN** library consumers SHALL use KeyService methods
- **AND** GUI SHALL adapt KeyService to Tauri commands
- **AND** CLI SHALL call KeyService methods directly

#### Scenario: Method signatures
- **WHEN** calling key operations
- **THEN** methods SHALL accept plain parameters (no Tauri types)
- **AND** methods SHALL return standard Result types
- **AND** methods SHALL be async where appropriate

### Requirement: Create API Key
The create operation is library-accessible.

#### Scenario: Create method
- **WHEN** creating a key
- **THEN** `KeyService::create(app_name, key_name, value, metadata) -> Result<KeyId, VaultError>`
- **AND** method SHALL require unlocked vault
- **AND** method SHALL encrypt value using per-key encryption
- **AND** method SHALL return key ID on success
- **AND** method SHALL return VaultError::DuplicateKey if exists

### Requirement: Read/Get API Key
The read operation supports both list and individual get operations.

#### Scenario: Get single key
- **WHEN** retrieving a key
- **THEN** `KeyService::get(app_name, key_name) -> Result<ApiKey, VaultError>`
- **AND** method SHALL require unlocked vault
- **AND** method SHALL decrypt key value
- **AND** method SHALL return complete ApiKey struct
- **AND** method SHALL return VaultError::NotFound if key doesn't exist

#### Scenario: List all keys
- **WHEN** listing keys
- **THEN** `KeyService::list() -> Result<Vec<ApiKeyMetadata>, VaultError>`
- **AND** method SHALL require unlocked vault
- **AND** method SHALL return metadata without decrypted values
- **AND** method SHALL return empty vector if no keys exist

#### Scenario: Search keys
- **WHEN** searching keys
- **THEN** `KeyService::search(query) -> Result<Vec<ApiKeyMetadata>, VaultError>`
- **AND** method SHALL require unlocked vault
- **AND** method SHALL search across app_name, key_name, description
- **AND** method SHALL support partial matching
- **AND** method SHALL return metadata without decrypted values

### Requirement: Update API Key
The update operation is flexible in the library API.

#### Scenario: Update method
- **WHEN** updating a key
- **THEN** `KeyService::update(id, updates) -> Result<(), VaultError>`
- **AND** method SHALL require unlocked vault
- **AND** method SHALL accept UpdateKeyRequest with optional fields
- **AND** method SHALL re-encrypt if value changed
- **AND** method SHALL update only provided fields
- **AND** method SHALL update timestamp

#### Scenario: Selective updates
- **WHEN** updating only metadata
- **THEN** method SHALL NOT re-encrypt key value
- **WHEN** updating key value
- **THEN** method SHALL re-encrypt with fresh salt
- **AND** method SHALL maintain other fields unchanged

### Requirement: Delete API Key
The delete operation returns context for confirmation.

#### Scenario: Delete method
- **WHEN** deleting a key
- **THEN** `KeyService::delete(id) -> Result<DeletedKey, VaultError>`
- **AND** method SHALL require unlocked vault
- **AND** method SHALL return deleted key metadata for confirmation
- **AND** method SHALL permanently remove from database
- **AND** method SHALL return VaultError::NotFound if doesn't exist

### Requirement: Data Structures
Data structures are library-appropriate rather than GUI-specific.

#### Scenario: ApiKey struct
- **WHEN** working with keys
- **THEN** struct SHALL have fields: id, app_name, key_name, value, api_url, description, created_at, updated_at
- **AND** struct SHALL derive Clone, Debug, Serialize, Deserialize
- **AND** struct SHALL NOT depend on Tauri types

#### Scenario: Metadata struct
- **WHEN** listing or searching
- **THEN** library SHALL return ApiKeyMetadata
- **AND** struct SHALL omit the decrypted value field
- **AND** this SHALL improve performance and security

#### Scenario: Update request
- **WHEN** updating keys
- **THEN** UpdateKeyRequest struct SHALL have: value, api_url, description all as Option<T>
- **AND** only provided fields SHALL be updated
- **AND** None SHALL mean "keep existing value"

### Requirement: Encryption Integration
Encryption integration uses the library's CryptoService.

#### Scenario: Per-key encryption
- **WHEN** creating or updating keys
- **THEN** KeyService SHALL call CryptoService::encrypt
- **AND** decryption SHALL use CryptoService::decrypt
- **AND** master key SHALL be obtained from AuthService
- **AND** per-key derivation SHALL use app_name + key_name

#### Scenario: Encryption error handling
- **WHEN** encryption/decryption fails
- **THEN** library SHALL return VaultError::Crypto with context
- **AND** error SHALL NOT expose sensitive data
- **AND** binaries SHALL handle errors appropriately

### Requirement: Authentication Requirements
Authentication requirements are enforced at the library level.

#### Scenario: Unlocked vault check
- **WHEN** key operation is called
- **THEN** method SHALL verify vault is unlocked
- **AND** method SHALL return VaultError::Locked if not unlocked
- **AND** method SHALL NOT proceed without authentication

#### Scenario: Activity tracking
- **WHEN** GUI performs key operations
- **THEN** GUI SHALL update activity counter
- **WHEN** CLI performs operations
- **THEN** CLI MAY skip activity tracking (immediate lock after command)
- **AND** library SHALL NOT assume activity tracking model

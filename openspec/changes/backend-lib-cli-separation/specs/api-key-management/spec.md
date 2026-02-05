## ADDED Requirements

N/A - No new key management requirements.

## MODIFIED Requirements

### MODIFIED: Key Management API
Key management operations are modified to be exposed through a library service interface.

#### Scenario: Service interface (was: Tauri commands)
- **BEFORE**: Key operations exposed as Tauri commands in commands.rs
- **AFTER**: Key operations exposed through KeyService public API
- **WHEN** performing key operations
- **THEN** library consumers SHALL use KeyService methods
- **AND** GUI SHALL adapt KeyService to Tauri commands
- **AND** CLI SHALL call KeyService methods directly

#### Scenario: Method signatures (cleaned up)
- **BEFORE**: Commands used Tauri State<T> and returned Tauri Result
- **AFTER**: Service methods use standard Rust types and return Result<T, VaultError>
- **WHEN** calling key operations
- **THEN** methods SHALL accept plain parameters (no Tauri types)
- **AND** methods SHALL return standard Result types
- **AND** methods SHALL be async where appropriate

### MODIFIED: Create API Key
The create operation is modified to be library-accessible.

#### Scenario: Create method (was: create_api_key command)
- **BEFORE**: `create_api_key(State, app_name, key_name, ...) -> Result<(), String>`
- **AFTER**: `KeyService::create(app_name, key_name, value, metadata) -> Result<KeyId, VaultError>`
- **WHEN** creating a key
- **THEN** method SHALL require unlocked vault
- **AND** method SHALL encrypt value using per-key encryption
- **AND** method SHALL return key ID on success
- **AND** method SHALL return VaultError::DuplicateKey if exists

### MODIFIED: Read/Get API Key
The read operation is modified to support both list and individual get operations.

#### Scenario: Get single key (was: get_api_key command)
- **BEFORE**: `get_api_key(State, id) -> Result<ApiKeyResponse, String>`
- **AFTER**: `KeyService::get(app_name, key_name) -> Result<ApiKey, VaultError>`
- **WHEN** retrieving a key
- **THEN** method SHALL require unlocked vault
- **AND** method SHALL decrypt key value
- **AND** method SHALL return complete ApiKey struct
- **AND** method SHALL return VaultError::NotFound if key doesn't exist

#### Scenario: List all keys (was: list_api_keys command)
- **BEFORE**: `list_api_keys(State) -> Result<Vec<ApiKeyResponse>, String>`
- **AFTER**: `KeyService::list() -> Result<Vec<ApiKeyMetadata>, VaultError>`
- **WHEN** listing keys
- **THEN** method SHALL require unlocked vault
- **AND** method SHALL return metadata without decrypted values
- **AND** method SHALL return empty vector if no keys exist

#### Scenario: Search keys (was: search_api_keys command)
- **BEFORE**: `search_api_keys(State, query) -> Result<Vec<ApiKeyResponse>, String>`
- **AFTER**: `KeyService::search(query) -> Result<Vec<ApiKeyMetadata>, VaultError>`
- **WHEN** searching keys
- **THEN** method SHALL require unlocked vault
- **AND** method SHALL search across app_name, key_name, description
- **AND** method SHALL support partial matching
- **AND** method SHALL return metadata without decrypted values

### MODIFIED: Update API Key
The update operation is modified to be more flexible in the library API.

#### Scenario: Update method (was: update_api_key command)
- **BEFORE**: `update_api_key(State, id, updates) -> Result<(), String>`
- **AFTER**: `KeyService::update(id, updates) -> Result<(), VaultError>`
- **WHEN** updating a key
- **THEN** method SHALL require unlocked vault
- **AND** method SHALL accept UpdateKeyRequest with optional fields
- **AND** method SHALL re-encrypt if value changed
- **AND** method SHALL update only provided fields
- **AND** method SHALL update timestamp

#### Scenario: Selective updates (enhanced)
- **AFTER**: Library supports atomic field updates
- **WHEN** updating only metadata
- **THEN** method SHALL NOT re-encrypt key value
- **WHEN** updating key value
- **THEN** method SHALL re-encrypt with fresh salt
- **AND** method SHALL maintain other fields unchanged

### MODIFIED: Delete API Key
The delete operation is modified to return more context.

#### Scenario: Delete method (was: delete_api_key command)
- **BEFORE**: `delete_api_key(State, id) -> Result<(), String>`
- **AFTER**: `KeyService::delete(id) -> Result<DeletedKey, VaultError>`
- **WHEN** deleting a key
- **THEN** method SHALL require unlocked vault
- **AND** method SHALL return deleted key metadata for confirmation
- **AND** method SHALL permanently remove from database
- **AND** method SHALL return VaultError::NotFound if doesn't exist

### MODIFIED: Data Structures
Data structures are modified to be library-appropriate rather than GUI-specific.

#### Scenario: ApiKey struct (was: ApiKeyResponse)
- **BEFORE**: ApiKeyResponse was JSON-serializable for Tauri
- **AFTER**: ApiKey is a library struct with proper types
- **WHEN** working with keys
- **THEN** struct SHALL have fields: id, app_name, key_name, value, api_url, description, created_at, updated_at
- **AND** struct SHALL derive Clone, Debug, Serialize, Deserialize
- **AND** struct SHALL NOT depend on Tauri types

#### Scenario: Metadata struct (new)
- **AFTER**: ApiKeyMetadata for operations that don't need decrypted value
- **WHEN** listing or searching
- **THEN** library SHALL return ApiKeyMetadata
- **AND** struct SHALL omit the decrypted value field
- **AND** this SHALL improve performance and security

#### Scenario: Update request (enhanced)
- **AFTER**: UpdateKeyRequest struct with all fields optional
- **WHEN** updating keys
- **THEN** struct SHALL have: value, api_url, description all as Option<T>
- **AND** only provided fields SHALL be updated
- **AND** None SHALL mean "keep existing value"

### MODIFIED: Encryption Integration
Encryption integration is modified to use the library's CryptoService.

#### Scenario: Per-key encryption (was: crypto module direct)
- **BEFORE**: Commands called crypto functions directly
- **AFTER**: KeyService uses CryptoService internally
- **WHEN** creating or updating keys
- **THEN** KeyService SHALL call CryptoService::encrypt
- **AND** decryption SHALL use CryptoService::decrypt
- **AND** master key SHALL be obtained from AuthService
- **AND** per-key derivation SHALL use app_name + key_name

#### Scenario: Encryption error handling (improved)
- **AFTER**: Crypto errors properly typed and propagated
- **WHEN** encryption/decryption fails
- **THEN** library SHALL return VaultError::Crypto with context
- **AND** error SHALL NOT expose sensitive data
- **AND** binaries SHALL handle errors appropriately

### MODIFIED: Authentication Requirements
Authentication requirements are enforced at the library level.

#### Scenario: Unlocked vault check (was: Tauri State check)
- **BEFORE**: Commands checked if vault was unlocked via AuthManager
- **AFTER**: KeyService methods check AuthService state
- **WHEN** key operation is called
- **THEN** method SHALL verify vault is unlocked
- **AND** method SHALL return VaultError::Locked if not unlocked
- **AND** method SHALL NOT proceed without authentication

#### Scenario: Activity tracking (responsibility shift)
- **BEFORE**: Commands updated activity counter for auto-lock
- **AFTER**: Activity tracking is binary responsibility
- **WHEN** GUI performs key operations
- **THEN** GUI SHALL update activity counter
- **WHEN** CLI performs operations
- **THEN** CLI MAY skip activity tracking (immediate lock after command)
- **AND** library SHALL NOT assume activity tracking model

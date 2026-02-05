## ADDED Requirements

N/A - No new storage requirements.

## MODIFIED Requirements

### MODIFIED: Storage Layer API
The storage layer is modified to be library-accessible with a clean public interface.

#### Scenario: Public API exposure (was: internal module)
- **BEFORE**: `database.rs` was internal to Tauri application
- **AFTER**: Storage functionality exposed through `StorageService`
- **WHEN** library users need database access
- **THEN** they SHALL use StorageService public API
- **AND** internal SQLx details SHALL remain encapsulated
- **AND** API SHALL provide async database operations

#### Scenario: Database initialization (enhanced)
- **BEFORE**: Database initialized in main.rs with hardcoded path
- **AFTER**: Library accepts database path as parameter
- **WHEN** creating VaultManager or StorageService
- **THEN** caller SHALL provide database path
- **AND** library SHALL NOT assume fixed ~/.vult location
- **AND** binaries SHALL provide appropriate paths for their context

### MODIFIED: Storage Abstraction
Storage abstraction is modified to separate SQLx implementation from public interface.

#### Scenario: Public storage interface (new)
- **AFTER**: Library exposes storage operations through service methods
- **WHEN** consumers perform storage operations
- **THEN** they SHALL call service methods like `store_key()`, `retrieve_key()`
- **AND** they SHALL NOT directly access SQLx pool or queries
- **AND** service SHALL handle connection pooling internally

#### Scenario: Internal implementation (unchanged)
- **BEFORE**: Used SQLx with SQLite backend
- **AFTER**: Still uses SQLx with SQLite internally
- **WHEN** storage operations execute
- **THEN** implementation SHALL use same SQLx patterns
- **AND** database schema SHALL remain the same
- **AND** migrations SHALL work identically

### MODIFIED: Database Path Management
Database path management is modified to support different contexts and configurations.

#### Scenario: Path configuration (was: hardcoded)
- **BEFORE**: Path was computed in main.rs as `~/.vult/vault.db`
- **AFTER**: Path is provided by caller (binary layer)
- **WHEN** GUI binary starts
- **THEN** it SHALL provide `~/.vult/vault.db` path
- **WHEN** CLI binary starts
- **THEN** it SHALL use VULT_DB_PATH env var or default to `~/.vult/vault.db`
- **AND** both SHALL share same database when using default path

#### Scenario: Directory creation (responsibility shift)
- **BEFORE**: main.rs created .vult directory
- **AFTER**: Library creates parent directories if needed
- **WHEN** library receives database path
- **THEN** library SHALL create parent directories
- **AND** library SHALL return error if path is invalid or inaccessible

### MODIFIED: Connection Management
Connection management is modified to support both long-lived GUI and short-lived CLI sessions.

#### Scenario: GUI connection pooling (unchanged)
- **BEFORE**: Single connection pool for application lifetime
- **AFTER**: GUI binary maintains connection pool for application lifetime
- **WHEN** GUI is running
- **THEN** connection pool SHALL remain active
- **AND** pool size SHALL be appropriate for GUI workload

#### Scenario: CLI connection handling (new)
- **AFTER**: CLI creates connection per command or per session
- **WHEN** CLI command executes
- **THEN** library SHALL create connection as needed
- **AND** connection SHALL be closed after command completes (unless session mode)
- **AND** connection overhead SHALL be minimal for single operations

### MODIFIED: Transaction Handling
Transaction handling is modified to be exposed through the library API.

#### Scenario: Transaction support (enhanced API)
- **BEFORE**: Transactions used internally but not exposed
- **AFTER**: Library provides transaction support in API
- **WHEN** operations need atomic execution
- **THEN** library SHALL provide transaction methods
- **AND** API SHALL support nested operations within transaction
- **AND** rollback SHALL occur on error

### MODIFIED: Schema Versioning
Schema versioning remains the same but is documented as library responsibility.

#### Scenario: Migration execution (clarified ownership)
- **BEFORE**: Migrations ran during VaultDb::new() in main.rs
- **AFTER**: Migrations run during StorageService initialization
- **WHEN** library initializes storage
- **THEN** library SHALL check schema version
- **AND** library SHALL run pending migrations
- **AND** library SHALL reject databases with newer schema versions
- **AND** behavior SHALL be identical to current implementation

#### Scenario: Version guard (unchanged)
- **BEFORE**: Protected against opening newer database versions
- **AFTER**: Same protection, but as library feature
- **WHEN** opening database with schema_version > current
- **THEN** library SHALL return error
- **AND** error SHALL indicate version mismatch
- **AND** error SHALL prevent data corruption

### MODIFIED: Error Handling
Storage error handling is modified to use library error types.

#### Scenario: Storage errors (was: anyhow/Tauri errors)
- **BEFORE**: Errors propagated as anyhow::Error or Tauri errors
- **AFTER**: Errors use VaultError::Database variant
- **WHEN** storage operation fails
- **THEN** library SHALL return VaultError::Database with context
- **AND** error SHALL include underlying SQLx error details
- **AND** binaries SHALL map errors to appropriate responses

#### Scenario: Database constraints (unchanged behavior)
- **BEFORE**: Unique constraint violations returned as database errors
- **AFTER**: Still detected and returned as errors, but typed differently
- **WHEN** duplicate key insertion attempted
- **THEN** library SHALL return VaultError::DuplicateKey
- **AND** error SHALL include conflicting app_name and key_name
- **AND** caller SHALL handle appropriately

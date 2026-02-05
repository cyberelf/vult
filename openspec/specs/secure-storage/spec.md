# Secure Storage Specification

## Purpose

Defines the secure storage layer for Vult, including database abstraction, connection management, and schema versioning.

## Requirements

### Requirement: Storage Layer API
The storage layer is library-accessible with a clean public interface.

#### Scenario: Public API exposure
- **WHEN** library users need database access
- **THEN** they SHALL use StorageService public API
- **AND** internal SQLx details SHALL remain encapsulated
- **AND** API SHALL provide async database operations

#### Scenario: Database initialization
- **WHEN** creating VaultManager or StorageService
- **THEN** caller SHALL provide database path
- **AND** library SHALL NOT assume fixed ~/.vult location
- **AND** binaries SHALL provide appropriate paths for their context

### Requirement: Storage Abstraction
Storage abstraction separates SQLx implementation from public interface.

#### Scenario: Public storage interface
- **WHEN** consumers perform storage operations
- **THEN** they SHALL call service methods like `store_key()`, `retrieve_key()`
- **AND** they SHALL NOT directly access SQLx pool or queries
- **AND** service SHALL handle connection pooling internally

#### Scenario: Internal implementation
- **WHEN** storage operations execute
- **THEN** implementation SHALL use SQLx with SQLite backend
- **AND** database schema SHALL remain the same
- **AND** migrations SHALL work identically

### Requirement: Database Path Management
Database path management supports different contexts and configurations.

#### Scenario: Path configuration
- **WHEN** GUI binary starts
- **THEN** it SHALL provide `~/.vult/vault.db` path
- **WHEN** CLI binary starts
- **THEN** it SHALL use VULT_DB_PATH env var or default to `~/.vult/vault.db`
- **AND** both SHALL share same database when using default path

#### Scenario: Directory creation
- **WHEN** library receives database path
- **THEN** library SHALL create parent directories if needed
- **AND** library SHALL return error if path is invalid or inaccessible

### Requirement: Connection Management
Connection management supports both long-lived GUI and short-lived CLI sessions.

#### Scenario: GUI connection pooling
- **WHEN** GUI is running
- **THEN** connection pool SHALL remain active for application lifetime
- **AND** pool size SHALL be appropriate for GUI workload

#### Scenario: CLI connection handling
- **WHEN** CLI command executes
- **THEN** library SHALL create connection as needed
- **AND** connection SHALL be closed after command completes (unless session mode)
- **AND** connection overhead SHALL be minimal for single operations

### Requirement: Transaction Handling
Transaction handling is exposed through the library API.

#### Scenario: Transaction support
- **WHEN** operations need atomic execution
- **THEN** library SHALL provide transaction methods
- **AND** API SHALL support nested operations within transaction
- **AND** rollback SHALL occur on error

### Requirement: Schema Versioning
Schema versioning is a library responsibility.

#### Scenario: Migration execution
- **WHEN** library initializes storage
- **THEN** library SHALL check schema version
- **AND** library SHALL run pending migrations
- **AND** library SHALL reject databases with newer schema versions
- **AND** behavior SHALL be identical to current implementation

#### Scenario: Version guard
- **WHEN** opening database with schema_version > current
- **THEN** library SHALL return error
- **AND** error SHALL indicate version mismatch
- **AND** error SHALL prevent data corruption

### Requirement: Error Handling
Storage error handling uses library error types.

#### Scenario: Storage errors
- **WHEN** storage operation fails
- **THEN** library SHALL return VaultError::Database with context
- **AND** error SHALL include underlying SQLx error details
- **AND** binaries SHALL map errors to appropriate responses

#### Scenario: Database constraints
- **WHEN** duplicate key insertion attempted
- **THEN** library SHALL return VaultError::DuplicateKey
- **AND** error SHALL include conflicting app_name and key_name
- **AND** caller SHALL handle appropriately

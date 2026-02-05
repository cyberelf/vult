# Vault Architecture Specification

## Purpose

Defines the high-level architecture of Vult, including library-binary separation, module organization, and public API surface.

## Requirements

### Requirement: Library-Binary Separation
The system SHALL separate core vault logic into a reusable library with multiple binary frontends.

#### Scenario: Library independence
- **WHEN** the vult library is compiled
- **THEN** the library SHALL NOT depend on Tauri or GUI frameworks
- **AND** the library SHALL be usable by any Rust application
- **AND** the library SHALL provide a stable public API

#### Scenario: Multiple binaries
- **WHEN** building the project
- **THEN** the system SHALL compile two separate binaries
- **AND** `vult-gui` SHALL be the Tauri GUI application
- **AND** `vult` SHALL be the CLI application
- **AND** both binaries SHALL use the same library

### Requirement: Module Organization
The system SHALL organize code into clear layers: library core, service layer, and binary adapters.

#### Scenario: Library core modules
- **WHEN** inspecting `src/` directory
- **THEN** crypto, database modules SHALL be in the library
- **AND** these modules SHALL be framework-agnostic
- **AND** these modules SHALL export public APIs

#### Scenario: Service layer
- **WHEN** accessing high-level operations
- **THEN** the system SHALL provide service layer abstractions
- **AND** VaultManager SHALL orchestrate operations
- **AND** AuthService, KeyService SHALL provide business logic
- **AND** services SHALL use core modules internally

#### Scenario: Binary layer
- **WHEN** implementing binaries
- **THEN** GUI binary SHALL adapt library to Tauri commands
- **AND** CLI binary SHALL adapt library to command-line interface
- **AND** binaries SHALL contain minimal logic (thin adapters)

### Requirement: Dependency Management
The system SHALL manage dependencies appropriately for library vs binary usage.

#### Scenario: Library dependencies
- **WHEN** compiling the library
- **THEN** the library SHALL depend only on: sqlx, argon2, aes-gcm, zeroize, chrono, thiserror
- **AND** the library SHALL NOT depend on: tauri, clap, dialoguer, tabled
- **AND** library dependencies SHALL be minimal and focused

#### Scenario: Binary dependencies
- **WHEN** compiling binaries
- **THEN** GUI binary SHALL depend on tauri and related plugins
- **AND** CLI binary SHALL depend on clap, dialoguer, tabled
- **AND** both binaries SHALL depend on the vult library

### Requirement: Public API Surface
The system SHALL expose a clean, well-documented public API from the library.

#### Scenario: Public exports
- **WHEN** importing the vult library
- **THEN** the system SHALL export VaultManager, AuthService, KeyService
- **AND** the system SHALL export CryptoService, StorageService
- **AND** the system SHALL export error types and data structures
- **AND** the system SHALL NOT export internal implementation details

#### Scenario: API documentation
- **WHEN** generating library documentation
- **THEN** all public APIs SHALL have rustdoc comments
- **AND** documentation SHALL include usage examples
- **AND** documentation SHALL specify async/sync behavior

### Requirement: Application Structure
The system's application structure supports library-binary separation.

#### Scenario: Entry points
- **WHEN** starting the application
- **THEN** users can choose GUI or CLI binary
- **AND** GUI binary is located at `src/bin/vult-gui.rs`
- **AND** CLI binary is located at `src/bin/vult.rs`
- **AND** both share the same vault database

#### Scenario: Code organization
- **WHEN** external code imports vult
- **THEN** it can access core, services, crypto, database through public API
- **AND** internal implementation details remain private

### Requirement: State Management
State management supports both GUI and CLI contexts.

#### Scenario: GUI state
- **WHEN** GUI commands execute
- **THEN** they access library services through Tauri State
- **AND** state lifecycle is managed by Tauri
- **AND** GUI adapter wraps library services in Tauri State

#### Scenario: CLI state
- **WHEN** CLI commands execute
- **THEN** they create service instances as needed
- **AND** state is scoped to command lifetime or session

### Requirement: Build Configuration
Build configuration supports multiple binaries.

#### Scenario: Cargo configuration
- **WHEN** running `cargo build`
- **THEN** both binaries are compiled with appropriate feature flags
- **AND** Cargo.toml defines [[bin]] entries for vult-gui and vult
- **AND** each binary has its own dependencies feature-flagged appropriately

#### Scenario: Library target
- **WHEN** other projects depend on vult
- **THEN** they can use it as a library dependency
- **AND** Cargo.toml has [lib] section with clear public API
- **AND** binary code is not included in library builds

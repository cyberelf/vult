# Spec: TypeScript Types

Capability ID: `typescript-types`

## ADDED Requirements

### Requirement: Type definitions mirror Rust data structures
The system SHALL provide TypeScript type definitions that exactly match the Rust backend structs for API keys, session state, and command arguments/responses.

#### Scenario: ApiKey type matches Rust struct
- **WHEN** TypeScript code references ApiKey type
- **THEN** type includes all fields: `id`, `app_name`, `key_name`, `api_url`, `description`, `key_value`, `created_at`, `updated_at`
- **AND** field types match Rust types (number for i64, string for String, null for Option)

#### Scenario: SessionState type matches Rust struct
- **WHEN** TypeScript code references SessionState type
- **THEN** type includes fields: `is_unlocked: boolean`, `last_activity_secs: number`
- **AND** field names use camelCase (converted from Rust snake_case)

### Requirement: Tauri command argument types
The system SHALL provide TypeScript interfaces for all Tauri command arguments, ensuring type safety when invoking commands.

#### Scenario: InitVaultArgs type defines PIN field
- **WHEN** code invokes init_vault command
- **THEN** InitVaultArgs type requires `pin: string` field
- **AND** TypeScript compiler verifies argument type at compile time

#### Scenario: CreateApiKeyArgs type defines all required fields
- **WHEN** code invokes create_api_key command
- **THEN** CreateApiKeyArgs type includes `appName`, `keyName`, `keyValue` as required
- **AND** `apiUrl` and `description` are optional (string | null)
- **AND** TypeScript compiler validates all fields

#### Scenario: UpdateApiKeyArgs extends CreateApiKeyArgs with id
- **WHEN** code invokes update_api_key command
- **THEN** UpdateApiKeyArgs type includes all CreateApiKeyArgs fields
- **AND** `id: number` field is required
- **AND** TypeScript ensures id is present

### Requirement: Tauri command return types
The system SHALL provide TypeScript return type annotations for all Tauri commands, ensuring type-safe response handling.

#### Scenario: list_api_keys returns ApiKey array
- **WHEN** code invokes list_api_keys command
- **THEN** return type is `Promise<ApiKey[]>`
- **AND** TypeScript knows array contains ApiKey objects
- **AND** autocomplete works for ApiKey properties

#### Scenario: get_session_state returns SessionState
- **WHEN** code invokes get_session_state command
- **THEN** return type is `Promise<SessionState>`
- **AND** TypeScript knows response structure
- **AND** invalid property access causes compile error

### Requirement: Type-safe command wrappers
The system SHALL provide wrapper functions that enforce type safety for all Tauri command invocations.

#### Scenario: Wrapper function enforces argument types
- **WHEN** developer calls wrapper function like `unlockVault({ pin: '123456' })`
- **THEN** TypeScript validates argument matches UnlockVaultArgs interface
- **AND** incorrect argument type causes compile error
- **AND** missing required field causes compile error

#### Scenario: Wrapper function provides return type
- **WHEN** developer calls wrapper function like `listApiKeys()`
- **THEN** return type is `Promise<ApiKey[]>`
- **AND** TypeScript infers response type correctly
- **AND** chaining works with type safety

### Requirement: Component prop types
The system SHALL define TypeScript interfaces for all Svelte component props, ensuring type safety within component hierarchies.

#### Scenario: Modal component props are typed
- **WHEN** KeyModal component is used
- **THEN** props interface defines `open: boolean`, `onSave: (data: CreateKeyData) => Promise<void>`
- **AND** TypeScript validates prop types at compile time
- **AND** incorrect prop type causes compile error

#### Scenario: Table component props are typed
- **WHEN** KeyTable component is used
- **THEN** props interface defines `keys: ApiKey[]`, `onEdit: (key: ApiKey) => void`, `onDelete: (id: number) => void`
- **AND** TypeScript validates array elements are ApiKey objects
- **AND** callback function signatures are enforced

### Requirement: Store types with Svelte 5 runes
The system SHALL use TypeScript with Svelte 5 stores to ensure type-safe reactive state management.

#### Scenario: Vault store type is defined
- **WHEN** vaultState store is created
- **THEN** store type is `Writable<VaultState>` where VaultState interface defines all state properties
- **AND** TypeScript validates state updates
- **AND** derived stores have proper type inference

#### Scenario: Derived store type is inferred
- **WHEN** filteredKeys derived store is created from vaultState
- **THEN** return type is `Readable<ApiKey[]>`
- **AND** TypeScript knows derived store contains filtered ApiKey array
- **AND** type errors occur if store returns wrong type

### Requirement: No implicit any types
The system SHALL compile TypeScript with strict mode enabled, prohibiting implicit `any` types and requiring explicit type annotations.

#### Scenario: Implicit any causes compile error
- **WHEN** developer omits type annotation without initializer
- **THEN** TypeScript compiler emits error
- **AND** build fails
- **AND** developer must provide explicit type or initializer

#### Scenario: Strict null checks enforce null safety
- **WHEN** code accesses potentially null value
- **THEN** TypeScript requires null check or type narrowing
- **AND** accidental null access causes compile error
- **AND** optional chaining (`?.`) is used appropriately

### Requirement: Type definitions export barrel
The system SHALL provide a barrel export file (`types/index.ts`) that re-exports all type definitions for convenient importing.

#### Scenario: Types can be imported from barrel
- **WHEN** developer imports types
- **THEN** `import type { ApiKey, SessionState } from '$lib/types'` works
- **AND** all types are available from single import path
- **AND** circular dependencies are avoided

### Requirement: JSDoc comments for type documentation
The system SHALL include JSDoc comments on all public type definitions and wrapper functions for IDE documentation and hover tooltips.

#### Scenario: Type documentation displays in IDE
- **WHEN** developer hovers over ApiKey type
- **THEN** JSDoc comment describes what ApiKey represents
- **AND** field descriptions display in autocomplete
- **AND** usage examples are available

#### Scenario: Function documentation explains parameters
- **WHEN** developer hovers over unlockVault function
- **THEN** JSDoc comment explains purpose and parameters
- **AND** parameter types are documented
- **AND** return type is documented

/**
 * API type definitions matching Rust backend structures
 * @module api
 */

/**
 * Represents an API key stored in the vault.
 * Matches the Rust ApiKey struct from src/commands.rs
 */

/**
 * Standard response wrapper for all Tauri commands.
 * Matches the Rust CommandResponse struct from src/commands.rs
 */
export interface CommandResponse<T> {
  /** Whether the command succeeded */
  success: boolean;
  /** The response data if successful */
  data: T | null;
  /** Error message if the command failed */
  error: string | null;
}

export interface ApiKey {
export interface ApiKey {
  /** Unique identifier for the key */
  id: number;
  /** Application name (e.g., "GitHub") */
  app_name: string;
  /** Display name for the key (e.g., "Personal Access Token") */
  key_name: string;
  /** Optional API endpoint URL */
  api_url: string | null;
  /** Optional description of the key's purpose */
  description: string | null;
  /** The encrypted API key value */
  key_value: string;
  /** ISO 8601 timestamp when key was created */
  created_at: string;
  /** ISO 8601 timestamp when key was last updated */
  updated_at: string;
}

/**
 * Represents the current authentication session state.
 * Matches the Rust SessionState struct from src/auth.rs
 */
export interface SessionState {
  /** Whether the vault is currently unlocked */
  is_unlocked: boolean;
  /** Seconds since last user activity (0 = just now) */
  last_activity_secs: number;
}

/**
 * Arguments for initializing the vault with a new PIN.
 * Used with init_vault Tauri command.
 */
export interface InitVaultArgs {
  /** The PIN to set for vault access (min 6 characters) */
  pin: string;
}

/**
 * Arguments for unlocking the vault.
 * Used with unlock_vault Tauri command.
 */
export interface UnlockVaultArgs {
  /** The PIN to unlock the vault */
  pin: string;
}

/**
 * Arguments for changing the vault PIN.
 * Used with change_pin Tauri command.
 */
export interface ChangePinArgs {
  /** The current PIN for verification */
  oldPin: string;
  /** The new PIN to set */
  newPin: string;
}

/**
 * Arguments for creating a new API key.
 * Used with create_api_key Tauri command.
 */
export interface CreateApiKeyArgs {
  /** Optional application name (e.g., "GitHub") */
  appName: string;
  /** Display name for the key (required) */
  keyName: string;
  /** The plaintext API key value to encrypt and store */
  keyValue: string;
  /** Optional API endpoint URL */
  apiUrl?: string;
  /** Optional description of the key's purpose */
  description?: string;
}

/**
 * Arguments for updating an existing API key.
 * Used with update_api_key Tauri command.
 */
export interface UpdateApiKeyArgs extends CreateApiKeyArgs {
  /** The ID of the key to update */
  id: number;
}

/**
 * Return type for listing API keys.
 */
export type ListApiKeysResult = ApiKey[];

/**
 * Return type for checking if vault is initialized.
 */
export type IsInitializedResult = boolean;

/**
 * Error types that can be returned from Tauri commands.
 */
export type TauriError =
  | { type: 'Database'; message: string }
  | { type: 'Crypto'; message: string }
  | { type: 'InvalidPin' }
  | { type: 'PinTooShort' }
  | { type: 'NotInitialized' }
  | { type: 'AlreadyInitialized' }
  | { type: 'TooManyAttempts' }
  | { type: 'Unknown'; message: string };

/**
 * Union type for all Tauri command results.
 */
export type CommandResult<T> = Result<T, TauriError>;

/**
 * Screen states for UI routing.
 */
export type ScreenState = 'setup' | 'unlock' | 'vault';

/**
 * Loading state for async operations.
 */
export type LoadingState = 'idle' | 'loading' | 'success' | 'error';

/**
 * UI modal states.
 */
export type ModalState = 'key' | 'view' | 'delete' | null;

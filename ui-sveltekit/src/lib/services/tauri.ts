/**
 * Type-safe Tauri command wrapper functions
 * @module tauri
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  ApiKey,
  SessionState,
  InitVaultArgs,
  UnlockVaultArgs,
  ChangePinArgs,
  CreateApiKeyArgs,
  UpdateApiKeyArgs,
  ListApiKeysResult,
  IsInitializedResult,
  CommandResponse,
} from '$lib/types';

/**
 * Initializes a new vault with the given PIN.
 * Creates the vault configuration and sets up encryption keys.
 *
 * @param args - The PIN and initialization parameters
 * @throws {Error} If vault is already initialized or PIN is invalid
 *
 * @example
 * ```ts
 * await initVault({ pin: 'secure123' });
 * ```
 */
export async function initVault(args: InitVaultArgs): Promise<void> {
  try {
    await invoke('init_vault', { pin: args.pin });
  } catch (error) {
    throw new Error(`Failed to initialize vault: ${error}`);
  }
}

/**
 * Unlocks the vault using the provided PIN.
 * Derives the encryption key and fetches all stored API keys.
 *
 * @param args - The PIN to unlock the vault
 * @throws {Error} If PIN is invalid or vault is not initialized
 *
 * @example
 * ```ts
 * await unlockVault({ pin: 'secure123' });
 * ```
 */
export async function unlockVault(args: UnlockVaultArgs): Promise<void> {
  try {
    await invoke('unlock_vault', { pin: args.pin });
  } catch (error) {
    throw new Error(`Failed to unlock vault: ${error}`);
  }
}

/**
 * Locks the vault and clears sensitive data from memory.
 *
 * @throws {Error} If vault is not unlocked
 *
 * @example
 * ```ts
 * await lockVault();
 * ```
 */
export async function lockVault(): Promise<void> {
  try {
    await invoke('lock_vault');
  } catch (error) {
    throw new Error(`Failed to lock vault: ${error}`);
  }
}

/**
 * Lists all API keys in the vault.
 * Requires the vault to be unlocked.
 *
 * @returns Array of all stored API keys
 * @throws {Error} If vault is locked
 *
 * @example
 * ```ts
 * const keys = await listApiKeys();
 * console.log(`Found ${keys.length} keys`);
 * ```
 */
export async function listApiKeys(): Promise<ListApiKeysResult> {
  try {
    const response = await invoke<CommandResponse<ApiKey[]>>('list_api_keys');
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to list API keys');
    }
    return response.data.map((key) => ({
      ...key,
      // Convert snake_case from Rust to camelCase for TypeScript
      appName: key.app_name,
      keyName: key.key_name,
      keyValue: key.key_value,
      apiUrl: key.api_url,
      createdAt: key.created_at,
      updatedAt: key.updated_at,
    }));
  } catch (error) {
    throw new Error(`Failed to list API keys: ${error}`);
  }
}

/**
 * Creates a new API key in the vault.
 * The key value is encrypted before storage.
 *
 * @param args - The key details to store
 * @returns The created API key with assigned ID
 * @throws {Error} If vault is locked or validation fails
 *
 * @example
 * ```ts
 * const newKey = await createApiKey({
 *   appName: 'GitHub',
 *   keyName: 'Personal Token',
 *   keyValue: 'ghp_xxxxxxxxxxxx',
 *   apiUrl: 'https://api.github.com',
 *   description: 'For GitHub API access'
 * });
 * ```
 */
export async function createApiKey(args: CreateApiKeyArgs): Promise<ApiKey> {
  try {
    const result = await invoke<ApiKey>('create_api_key', {
      appName: args.appName,
      keyName: args.keyName,
      apiKey: args.keyValue,
      apiUrl: args.apiUrl ?? null,
      description: args.description ?? null,
    });
    return result;
  } catch (error) {
    throw new Error(`Failed to create API key: ${error}`);
  }
}

/**
 * Updates an existing API key.
 * The key value is re-encrypted with the current vault key.
 *
 * @param args - The key details including ID for updating
 * @returns The updated API key
 * @throws {Error} If vault is locked or key doesn't exist
 *
 * @example
 * ```ts
 * await updateApiKey({
 *   id: 1,
 *   appName: 'GitHub',
 *   keyName: 'Updated Token Name',
 *   keyValue: 'ghp_newtoken'
 * });
 * ```
 */
export async function updateApiKey(args: UpdateApiKeyArgs): Promise<ApiKey> {
  try {
    // Construct the input object matching the Rust UpdateApiKey struct
    const input = {
      id: String(args.id),
      app_name: args.appName || null,
      key_name: args.keyName || null,
      key_value: args.keyValue || null, // Optional, only if re-keying
      api_url: args.apiUrl ? args.apiUrl : args.apiUrl === '' ? null : undefined, // Handle empty strings as null? Rust Option<Option<String>> vs Option<String>
      description: args.description ? args.description : args.description === '' ? null : undefined,
    };
    
    // Rust UpdateApiKey:
    // pub api_url: Option<Option<String>>
    // This allows: None (no change), Some(None) (clear), Some(Some("...")) (set)
    // My construction above:
    // If args.apiUrl is undefined -> api_url: undefined (omitted) -> None?
    // In JS object, undefined keys are omitted. 
    // However, if I want to set it, I need `api_url: Some(...)`.
    // The frontend args are simplified.
    // Logic:
    // If `args.apiUrl` is passed, we update it.
    // If `args.apiUrl` is undefined, we don't change it.
    // If we want to CLEAR it, we might pass "" (empty string).
    
    // Simpler mapping for now:
    const rustInput: any = {
        id: String(args.id),
    };
    if (args.appName !== undefined) rustInput.app_name = args.appName;
    if (args.keyName !== undefined) rustInput.key_name = args.keyName;
    if (args.keyValue !== undefined) rustInput.key_value = args.keyValue;
    if (args.apiUrl !== undefined) rustInput.api_url = args.apiUrl || null;
    if (args.description !== undefined) rustInput.description = args.description || null;

    const result = await invoke<ApiKey>('update_api_key', {
      input: rustInput
    });
    return result;
  } catch (error) {
    throw new Error(`Failed to update API key: ${error}`);
  }
}

/**
 * Deletes an API key from the vault.
 *
 * @param id - The ID of the key to delete
 * @throws {Error} If vault is locked or key doesn't exist
 *
 * @example
 * ```ts
 * await deleteApiKey(1);
 * ```
 */
export async function deleteApiKey(id: number): Promise<void> {
  try {
    await invoke('delete_api_key', { id });
  } catch (error) {
    throw new Error(`Failed to delete API key: ${error}`);
  }
}

/**
 * Changes the vault PIN.
 * Requires the current PIN for verification.
 *
 * @param args - The old PIN for verification and new PIN to set
 * @throws {Error} If old PIN is invalid or new PIN is too short
 *
 * @example
 * ```ts
 * await changePin({
 *   oldPin: 'old123',
 *   newPin: 'new456'
 * });
 * ```
 */
export async function changePin(args: ChangePinArgs): Promise<void> {
  try {
    await invoke('change_pin', {
      oldPin: args.oldPin,
      newPin: args.newPin,
    });
  } catch (error) {
    throw new Error(`Failed to change PIN: ${error}`);
  }
}

/**
 * Gets the current session state including lock status and activity.
 *
 * @returns The current session state
 * @throws {Error} If session state cannot be retrieved
 *
 * @example
 * ```ts
 * const session = await getSessionState();
 * console.log('Unlocked:', session.is_unlocked);
 * ```
 */
export async function getSessionState(): Promise<SessionState> {
  try {
    return await invoke<SessionState>('get_session_state');
  } catch (error) {
    throw new Error(`Failed to get session state: ${error}`);
  }
}

/**
 * Checks if the vault has been initialized.
 *
 * @returns true if vault is initialized, false otherwise
 * @throws {Error} If initialization status cannot be checked
 *
 * @example
 * ```ts
 * const isInit = await isInitialized();
 * if (!isInit) {
 *   // Show setup screen
 * }
 * ```
 */
export async function isInitialized(): Promise<IsInitializedResult> {
  try {
    return await invoke<boolean>('is_initialized');
  } catch (error) {
    throw new Error(`Failed to check initialization: ${error}`);
  }
}

/**
 * Copies an API key to clipboard by ID using the backend's secure copy.
 * The backend will look up the encrypted key, decrypt it, and copy with auto-clear.
 *
 * @param id - The ID of the API key to copy
 * @returns The decrypted key value that was copied
 * @throws {Error} If clipboard access fails
 *
 * @example
 * ```ts
 * const keyValue = await copyApiKeyById(123);
 * console.log(`Copied: ${keyValue}`);
 * ```
 */
export async function copyApiKeyById(id: number): Promise<string> {
  try {
    const response = await invoke<CommandResponse<string>>('copy_to_clipboard', { id: String(id) });
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to copy to clipboard');
    }
    return response.data;
  } catch (error) {
    throw new Error(`Failed to copy to clipboard: ${error}`);
  }
}

/**
 * Updates the activity timer to prevent auto-lock.
 * Call this when the user performs any action.
 *
 * @throws {Error} If activity cannot be updated
 *
 * @example
 * ```ts
 * // Call on user interaction
 * document.addEventListener('click', async () => {
 *   await updateActivity();
 * });
 * ```
 */
export async function updateActivity(): Promise<void> {
  try {
    await invoke('update_activity');
  } catch (error) {
    throw new Error(`Failed to update activity: ${error}`);
  }
}

/**
 * Checks if the Tauri API is available.
 * Useful for detecting if running in Tauri vs browser.
 *
 * @returns true if Tauri API is available
 */
export function isTauriAvailable(): boolean {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}

/**
 * Mock Tauri API for browser testing.
 * Returns mock implementations of all commands.
 */
export function createMockTauriApi() {
  return {
    initVault: async (args: InitVaultArgs) => {
      console.log('[MOCK] initVault', args);
      await new Promise((resolve) => setTimeout(resolve, 500));
    },
    unlockVault: async (args: UnlockVaultArgs) => {
      console.log('[MOCK] unlockVault', args);
      await new Promise((resolve) => setTimeout(resolve, 500));
    },
    lockVault: async () => {
      console.log('[MOCK] lockVault');
      await new Promise((resolve) => setTimeout(resolve, 200));
    },
    listApiKeys: async (): Promise<ApiKey[]> => {
      console.log('[MOCK] listApiKeys');
      return [];
    },
    createApiKey: async (args: CreateApiKeyArgs): Promise<ApiKey> => {
      console.log('[MOCK] createApiKey', args);
      return {
        id: 1,
        app_name: args.appName,
        key_name: args.keyName,
        api_url: args.apiUrl ?? null,
        description: args.description ?? null,
        key_value: 'encrypted',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };
    },
    updateApiKey: async (args: UpdateApiKeyArgs): Promise<ApiKey> => {
      console.log('[MOCK] updateApiKey', args);
      return {
        id: args.id,
        app_name: args.appName,
        key_name: args.keyName,
        api_url: args.apiUrl ?? null,
        description: args.description ?? null,
        key_value: 'encrypted',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };
    },
    deleteApiKey: async (id: number) => {
      console.log('[MOCK] deleteApiKey', id);
    },
    changePin: async (args: ChangePinArgs) => {
      console.log('[MOCK] changePin', args);
      await new Promise((resolve) => setTimeout(resolve, 500));
    },
    getSessionState: async (): Promise<SessionState> => {
      console.log('[MOCK] getSessionState');
      return { is_unlocked: false, last_activity_secs: 0 };
    },
    isInitialized: async (): Promise<boolean> => {
      console.log('[MOCK] isInitialized');
      return false;
    },
    copyToClipboard: async (text: string) => {
      console.log('[MOCK] copyToClipboard', text);
      await navigator.clipboard.writeText(text);
    },
    updateActivity: async () => {
      console.log('[MOCK] updateActivity');
    },
  };
}

/**
 * Vault state management store
 * Handles authentication, API keys, and screen routing
 */

import { writable, derived } from 'svelte/store';
import type { ApiKey, ScreenState, BiometricAvailability } from '$lib/types';
import * as tauri from '$lib/services/tauri';

/**
 * Vault state interface
 */
export interface VaultState {
  /** Current screen to display */
  screen: ScreenState;
  /** Whether vault is unlocked */
  isUnlocked: boolean;
  /** All API keys in the vault */
  keys: ApiKey[];
  /** Current search query */
  searchQuery: string;
  /** Loading state for async operations */
  loading: boolean;
  /** Error message if any */
  error: string | null;
  /** Biometric availability status */
  biometricAvailability: BiometricAvailability | null;
  /** Whether biometric unlock is enabled in settings */
  biometricEnabled: boolean;
}

/**
 * Load biometric setting from localStorage
 */
function loadBiometricEnabled(): boolean {
  if (typeof window === 'undefined') return true;
  const stored = localStorage.getItem('vult_biometric_enabled');
  return stored === null ? true : stored === 'true'; // Default enabled
}

/**
 * Save biometric setting to localStorage
 */
function saveBiometricEnabled(enabled: boolean): void {
  if (typeof window !== 'undefined') {
    localStorage.setItem('vult_biometric_enabled', String(enabled));
  }
}

/**
 * Initial vault state
 */
const initialState: VaultState = {
  screen: 'setup',
  isUnlocked: false,
  keys: [],
  searchQuery: '',
  loading: false,
  error: null,
  biometricAvailability: null,
  biometricEnabled: loadBiometricEnabled(),
};

/**
 * Creates the vault state store with actions
 */
function createVaultStore() {
  const { subscribe, set, update } = writable<VaultState>(initialState);

  return {
    subscribe,

    /**
     * Initialize vault state - check if vault exists
     */
    initialize: async () => {
      update((s) => ({ ...s, loading: true, error: null }));
      try {
        const isInit = await tauri.isInitialized();
        // Check biometric availability if on Windows
        let biometricAvailability: BiometricAvailability | null = null;
        try {
          biometricAvailability = await tauri.checkBiometricAvailable();
        } catch {
          // If check fails, assume not supported
          biometricAvailability = 'not_supported';
        }
        update((s) => ({
          ...s,
          screen: isInit ? 'unlock' : 'setup',
          loading: false,
          biometricAvailability,
        }));
      } catch (error) {
        update((s) => ({
          ...s,
          loading: false,
          error: error instanceof Error ? error.message : 'Failed to check vault status',
        }));
      }
    },

    /**
     * Initialize new vault with PIN (setup screen)
     */
    setupVault: async (pin: string, pinConfirm: string) => {
      if (pin.length < 6) {
        update((s) => ({ ...s, error: 'PIN must be at least 6 characters' }));
        return;
      }
      if (pin !== pinConfirm) {
        update((s) => ({ ...s, error: 'PINs do not match' }));
        return;
      }

      update((s) => ({ ...s, loading: true, error: null }));
      try {
        await tauri.initVault({ pin });
        // After setup, unlock automatically
        const keys = await tauri.listApiKeys();
        update((s) => ({
          ...s,
          screen: 'vault',
          isUnlocked: true,
          keys,
          searchQuery: '',
          loading: false,
          error: null,
        }));
      } catch (error) {
        update((s) => ({
          ...s,
          loading: false,
          error: error instanceof Error ? error.message : 'Failed to initialize vault',
        }));
      }
    },

    /**
     * Unlock vault with PIN
     */
    unlock: async (pin: string) => {
      update((s) => ({ ...s, loading: true, error: null }));
      try {
        await tauri.unlockVault({ pin });
        const keys = await tauri.listApiKeys();
        update((s) => ({
          ...s,
          screen: 'vault',
          isUnlocked: true,
          keys,
          searchQuery: '',
          loading: false,
          error: null,
        }));
      } catch (error) {
        update((s) => ({
          ...s,
          loading: false,
          error: error instanceof Error ? error.message : 'Failed to unlock vault',
        }));
      }
    },

    /**
     * Unlock vault with biometric authentication (Windows Hello)
     */
    unlockWithBiometric: async () => {
      update((s) => ({ ...s, loading: true, error: null }));
      try {
        await tauri.unlockWithBiometric({ message: 'Unlock Vult API Vault' });
        const keys = await tauri.listApiKeys();
        update((s) => ({
          ...s,
          screen: 'vault',
          isUnlocked: true,
          keys,
          searchQuery: '',
          loading: false,
          error: null,
        }));
      } catch (error) {
        // On biometric failure, show error but keep unlock screen visible
        // User can then try PIN as fallback
        update((s) => ({
          ...s,
          loading: false,
          error: error instanceof Error ? error.message : 'Biometric unlock failed',
        }));
        throw error; // Let caller handle fallback
      }
    },

    /**
     * Lock vault and clear sensitive data
     */
    lock: async () => {
      update((s) => ({ ...s, loading: true, error: null }));
      try {
        await tauri.lockVault();
        update((s) => ({
          ...s,
          screen: 'unlock',
          isUnlocked: false,
          keys: [],
          searchQuery: '',
          loading: false,
          error: null,
        }));
      } catch (error) {
        update((s) => ({
          ...s,
          loading: false,
          error: error instanceof Error ? error.message : 'Failed to lock vault',
        }));
      }
    },

    /**
     * Set search query for filtering keys
     */
    setSearchQuery: (query: string) => {
      update((s) => ({ ...s, searchQuery: query }));
    },

    /**
     * Clear error message
     */
    clearError: () => {
      update((s) => ({ ...s, error: null }));
    },

    /**
     * Toggle biometric unlock setting
     */
    toggleBiometric: (enabled: boolean) => {
      saveBiometricEnabled(enabled);
      update((s) => ({ ...s, biometricEnabled: enabled }));
    },

    /**
     * Check biometric availability (can be called to refresh status)
     */
    checkBiometric: async () => {
      try {
        const availability = await tauri.checkBiometricAvailable();
        update((s) => ({ ...s, biometricAvailability: availability }));
        return availability;
      } catch {
        update((s) => ({ ...s, biometricAvailability: 'not_supported' }));
        return 'not_supported';
      }
    },

    /**
     * Add a new key to the store
     */
    addKey: (key: ApiKey) => {
      update((s) => ({ ...s, keys: [...s.keys, key] }));
    },

    /**
     * Update an existing key in the store
     */
    updateKey: (key: ApiKey) => {
      update((s) => ({
        ...s,
        keys: s.keys.map((k) => (k.id === key.id ? key : k)),
      }));
    },

    /**
     * Persist key updates to backend and update store
     */
    saveKey: async (key: Partial<ApiKey> & { id: number }) => {
       update((s) => ({ ...s, loading: true, error: null }));
       try {
         const result = await tauri.updateApiKey({
           id: key.id,
           appName: key.appName !== undefined ? key.appName : undefined,
           keyName: key.keyName !== undefined ? key.keyName : undefined,
           // Do not send keyValue for inline edits (only for full form updates)
           apiUrl: key.apiUrl !== undefined ? key.apiUrl : undefined,
           description: key.description !== undefined ? key.description : undefined,
         });
         
         // Update local state with the returned (fresh) key
         update((s) => ({
           ...s,
           keys: s.keys.map((k) => (k.id === key.id ? { ...k, ...result } : k)),
           loading: false
         }));
       } catch (error) {
         update((s) => ({
           ...s,
           loading: false,
           error: error instanceof Error ? error.message : 'Failed to update key',
         }));
         throw error;
       }
    },

    /**
     * Remove a key from the store
     */
    removeKey: (id: number) => {
      update((s) => ({ ...s, keys: s.keys.filter((k) => k.id !== id) }));
    },

    /**
     * Get current state snapshot
     * Useful for external services that need to check state
     */
    getCurrentState: (): VaultState | null => {
      let state: VaultState | null = null;
      const unsubscribe = subscribe((s) => {
        state = s;
      });
      unsubscribe();
      return state;
    },
  };
}

/**
 * Vault state store
 */
export const vaultStore = createVaultStore();

/**
 * Derived store for filtered keys based on search query
 */
export const filteredKeys = derived(vaultStore, ($vault) => {
  if (!$vault.searchQuery) {
    return $vault.keys;
  }
  const query = $vault.searchQuery.toLowerCase();
  return $vault.keys.filter(
    (key) =>
      key.app_name?.toLowerCase().includes(query) ||
      key.key_name.toLowerCase().includes(query) ||
      key.description?.toLowerCase().includes(query)
  );
});

/**
 * Derived store for loading state
 */
export const isLoading = derived(vaultStore, ($vault) => $vault.loading);

/**
 * Derived store for error state
 */
export const error = derived(vaultStore, ($vault) => $vault.error);

/**
 * Derived store for current screen
 */
export const currentScreen = derived(vaultStore, ($vault) => $vault.screen);

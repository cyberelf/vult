/**
 * Tests for Vault Store
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { vaultStore, filteredKeys, isLoading, error, currentScreen } from '$lib/stores/vault';
import * as tauri from '$lib/services/tauri';
import type { ApiKey } from '$lib/types';

// Mock Tauri service
vi.mock('$lib/services/tauri', () => ({
  initVault: vi.fn(),
  unlockVault: vi.fn(),
  lockVault: vi.fn(),
  listApiKeys: vi.fn(),
  isInitialized: vi.fn(),
}));

// Helper to create mock ApiKey
const createMockKey = (overrides?: Partial<ApiKey>): ApiKey => ({
  id: 1,
  app_name: 'TestApp',
  key_name: 'TestKey',
  api_url: null,
  description: null,
  key_value: 'encrypted_value',
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-01T00:00:00Z',
  ...overrides,
});

describe('Vault Store', () => {
  beforeEach(() => {
    // Reset store to initial state before each test
    vaultStore.lock();
  });

  describe('initialize', () => {
    it('should set screen to unlock when vault is initialized', async () => {
      vi.mocked(tauri.isInitialized).mockResolvedValue(true);

      await vaultStore.initialize();

      let screen = '';
      currentScreen.subscribe((s) => (screen = s))();

      expect(screen).toBe('unlock');
    });

    it('should set screen to setup when vault is not initialized', async () => {
      vi.mocked(tauri.isInitialized).mockResolvedValue(false);

      await vaultStore.initialize();

      let screen = '';
      currentScreen.subscribe((s) => (screen = s))();

      expect(screen).toBe('setup');
    });

    it('should handle initialization errors', async () => {
      vi.mocked(tauri.isInitialized).mockRejectedValue(new Error('Database error'));

      await vaultStore.initialize();

      let errorMessage: string | null = '';
      error.subscribe((e) => (errorMessage = e))();

      expect(errorMessage).toBe('Database error');
    });
  });

  describe('setupVault', () => {
    it('should reject PINs shorter than 6 characters', async () => {
      await vaultStore.setupVault('12345', '12345');

      let errorMessage: string | null = '';
      error.subscribe((e) => (errorMessage = e))();

      expect(errorMessage).toBe('PIN must be at least 6 characters');
    });

    it('should reject mismatched PINs', async () => {
      await vaultStore.setupVault('123456', '654321');

      let errorMessage: string | null = '';
      error.subscribe((e) => (errorMessage = e))();

      expect(errorMessage).toBe('PINs do not match');
    });

    it('should call initVault and unlock on success', async () => {
      vi.mocked(tauri.initVault).mockResolvedValue(undefined);
      vi.mocked(tauri.listApiKeys).mockResolvedValue([]);

      await vaultStore.setupVault('test123', 'test123');

      expect(tauri.initVault).toHaveBeenCalledWith({ pin: 'test123' });
      expect(tauri.listApiKeys).toHaveBeenCalled();
    });
  });

  describe('unlock', () => {
    it('should call unlockVault and fetch keys', async () => {
      const mockKeys: ApiKey[] = [
        {
          id: 1,
          app_name: 'GitHub',
          key_name: 'Token',
          api_url: null,
          description: null,
          key_value: 'encrypted',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
      ];

      vi.mocked(tauri.unlockVault).mockResolvedValue(undefined);
      vi.mocked(tauri.listApiKeys).mockResolvedValue(mockKeys);

      await vaultStore.unlock('test1234');

      expect(tauri.unlockVault).toHaveBeenCalledWith({ pin: 'test1234' });
      expect(tauri.listApiKeys).toHaveBeenCalled();
    });

    it('should handle invalid PIN', async () => {
      vi.mocked(tauri.unlockVault).mockRejectedValue(new Error('Invalid PIN'));

      await vaultStore.unlock('wrong');

      let errorMessage: string | null = '';
      error.subscribe((e) => (errorMessage = e))();

      expect(errorMessage).toBe('Invalid PIN');
    });
  });

  describe('lock', () => {
    it('should call lockVault and clear keys', async () => {
      vi.mocked(tauri.lockVault).mockResolvedValue(undefined);

      await vaultStore.lock();

      expect(tauri.lockVault).toHaveBeenCalled();

      let screen = '';
      currentScreen.subscribe((s) => (screen = s))();

      expect(screen).toBe('unlock');
    });
  });

  describe('setSearchQuery', () => {
    it('should update search query', () => {
      vaultStore.setSearchQuery('github');

      let query = '';
      vaultStore.subscribe((s) => (query = s.searchQuery))();

      expect(query).toBe('github');
    });

    it('should filter keys by search query', async () => {
      const mockKeys: ApiKey[] = [
        {
          id: 1,
          app_name: 'GitHub',
          key_name: 'Personal Token',
          api_url: 'https://api.github.com',
          description: 'For GitHub API',
          key_value: 'encrypted',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
        {
          id: 2,
          app_name: 'GitLab',
          key_name: 'Deploy Token',
          api_url: 'https://gitlab.com',
          description: 'For deployments',
          key_value: 'encrypted',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
      ];

      // Add keys to store
      vaultStore.addKey(mockKeys[0]);
      vaultStore.addKey(mockKeys[1]);

      // Search for 'github'
      vaultStore.setSearchQuery('github');

      let filtered: typeof mockKeys = [];
      filteredKeys.subscribe((f) => (filtered = f))();

      expect(filtered).toHaveLength(1);
      expect(filtered[0].app_name).toBe('GitHub');
    });

    it('should be case-insensitive', async () => {
      const mockKeys: ApiKey[] = [
        {
          id: 1,
          app_name: 'GitHub',
          key_name: 'Personal Token',
          api_url: null,
          description: 'For GitHub API',
          key_value: 'encrypted',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
      ];

      vaultStore.addKey(mockKeys[0]);
      vaultStore.setSearchQuery('GITHUB');

      let filtered: typeof mockKeys = [];
      filteredKeys.subscribe((f) => (filtered = f))();

      expect(filtered).toHaveLength(1);
    });

    it('should search across app_name, key_name, and description', async () => {
      const mockKeys: ApiKey[] = [
        {
          id: 1,
          app_name: 'API Service',
          key_name: 'Test Key',
          api_url: null,
          description: 'Production environment key',
          key_value: 'encrypted',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
      ];

      vaultStore.addKey(mockKeys[0]);

      // Search by description
      vaultStore.setSearchQuery('production');

      let filtered: typeof mockKeys = [];
      filteredKeys.subscribe((f) => (filtered = f))();

      expect(filtered).toHaveLength(1);
    });
  });

  describe('clearError', () => {
    it('should clear error message', () => {
      // Set an error first
      vaultStore.setSearchQuery('x');
      vaultStore.setupVault('12345', '12345');

      // Clear it
      vaultStore.clearError();

      let errorMessage: string | null = '';
      error.subscribe((e) => (errorMessage = e))();

      expect(errorMessage).toBeNull();
    });
  });

  describe('addKey', () => {
    it('should add key to store', () => {
      const mockKey = {
        id: 1,
        app_name: 'GitHub',
        key_name: 'Token',
        api_url: null,
        description: null,
        key_value: 'encrypted',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      };

      vaultStore.addKey(mockKey);

      let keys: ApiKey[] = [];
      vaultStore.subscribe((s) => (keys = s.keys))();

      expect(keys).toHaveLength(1);
      expect(keys[0]).toEqual(mockKey);
    });
  });

  describe('updateKey', () => {
    it('should update existing key in store', () => {
      const originalKey = {
        id: 1,
        app_name: 'GitHub',
        key_name: 'Old Name',
        api_url: null,
        description: null,
        key_value: 'encrypted',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      };

      const updatedKey = {
        ...originalKey,
        key_name: 'New Name',
      };

      vaultStore.addKey(originalKey);
      vaultStore.updateKey(updatedKey);

      let keys: ApiKey[] = [];
      vaultStore.subscribe((s) => (keys = s.keys))();

      expect(keys).toHaveLength(1);
      expect(keys[0].key_name).toBe('New Name');
    });
  });

  describe('removeKey', () => {
    it('should remove key from store', () => {
      const mockKey = {
        id: 1,
        app_name: 'GitHub',
        key_name: 'Token',
        api_url: null,
        description: null,
        key_value: 'encrypted',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      };

      vaultStore.addKey(mockKey);
      vaultStore.removeKey(1);

      let keys: ApiKey[] = [];
      vaultStore.subscribe((s) => (keys = s.keys))();

      expect(keys).toHaveLength(0);
    });
  });

  describe('isLoading derived store', () => {
    it('should reflect loading state', async () => {
      // Create a never-resolving promise to keep loading state
      const neverResolving = new Promise<boolean>(() => {});
      vi.mocked(tauri.isInitialized).mockReturnValue(neverResolving);

      const initPromise = vaultStore.initialize();

      let loading = false;
      isLoading.subscribe((l) => (loading = l))();

      // Should be loading during initialization
      expect(loading).toBe(true);

      // Wait for init to complete or timeout
      await Promise.race([
        initPromise,
        new Promise((resolve) => setTimeout(resolve, 100)),
      ]);
    });
  });

  describe('getCurrentState', () => {
    it('should return current vault state snapshot', () => {
      vaultStore.setSearchQuery('test');

      const state = vaultStore.getCurrentState();

      expect(state).not.toBeNull();
      expect(state?.searchQuery).toBe('test');
    });

    it('should return isUnlocked flag correctly', async () => {
      const mockKeys: ApiKey[] = [
        {
          id: 1,
          app_name: 'GitHub',
          key_name: 'Token',
          api_url: null,
          description: null,
          key_value: 'encrypted',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
      ];

      vi.mocked(tauri.unlockVault).mockResolvedValue(undefined);
      vi.mocked(tauri.listApiKeys).mockResolvedValue(mockKeys);

      await vaultStore.unlock('test1234');

      const state = vaultStore.getCurrentState();

      expect(state?.isUnlocked).toBe(true);
    });

    it('should return empty keys array when locked', () => {
      const state = vaultStore.getCurrentState();

      expect(state?.keys).toEqual([]);
      expect(state?.isUnlocked).toBe(false);
    });

    it('should return correct screen state', async () => {
      vi.mocked(tauri.isInitialized).mockResolvedValue(true);

      await vaultStore.initialize();

      const state = vaultStore.getCurrentState();

      expect(state?.screen).toBe('unlock');
    });
  });

  describe('camelCase conversion from listApiKeys', () => {
    it('should handle keys with null optional fields', async () => {
      const mockKeys: ApiKey[] = [
        {
          id: 1,
          app_name: 'Service',
          key_name: 'API Key',
          api_url: null,
          description: null,
          key_value: 'encrypted',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
          // Add camelCase properties that listApiKeys adds
          appName: 'Service',
          keyName: 'API Key',
          keyValue: 'encrypted',
          apiUrl: null,
          createdAt: '2024-01-01T00:00:00Z',
          updatedAt: '2024-01-01T00:00:00Z',
        },
      ];

      vi.mocked(tauri.unlockVault).mockResolvedValue(undefined);
      vi.mocked(tauri.listApiKeys).mockResolvedValue(mockKeys);

      await vaultStore.unlock('test1234');

      let keys: typeof mockKeys = [];
      vaultStore.subscribe((s) => (keys = s.keys))();

      // Keys should have both snake_case and camelCase
      expect(keys[0].app_name).toBeDefined();
      expect(keys[0].appName).toBeDefined();
      expect(keys[0].apiUrl).toBeNull();
    });

    it('should handle keys with all fields present', async () => {
      const mockKeys: ApiKey[] = [
        {
          id: 1,
          app_name: 'GitHub',
          key_name: 'Personal Token',
          api_url: 'https://api.github.com',
          description: 'For API access',
          key_value: 'encrypted',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
          // Add camelCase properties that listApiKeys adds
          appName: 'GitHub',
          keyName: 'Personal Token',
          keyValue: 'encrypted',
          apiUrl: 'https://api.github.com',
          createdAt: '2024-01-01T00:00:00Z',
          updatedAt: '2024-01-01T00:00:00Z',
        },
      ];

      vi.mocked(tauri.unlockVault).mockResolvedValue(undefined);
      vi.mocked(tauri.listApiKeys).mockResolvedValue(mockKeys);

      await vaultStore.unlock('test1234');

      let keys: typeof mockKeys = [];
      vaultStore.subscribe((s) => (keys = s.keys))();

      expect(keys[0].appName).toBe('GitHub');
      expect(keys[0].keyName).toBe('Personal Token');
      expect(keys[0].apiUrl).toBe('https://api.github.com');
      expect(keys[0].description).toBe('For API access');
    });
  });

  describe('error edge cases', () => {
    it('should handle unlock with empty response', async () => {
      vi.mocked(tauri.unlockVault).mockResolvedValue(undefined);
      vi.mocked(tauri.listApiKeys).mockResolvedValue([]);

      await expect(vaultStore.unlock('test1234')).resolves.not.toThrow();
    });

    it('should handle setupVault with error from Tauri', async () => {
      vi.mocked(tauri.initVault).mockRejectedValue(new Error('Database error'));

      await vaultStore.setupVault('test123', 'test123');

      let errorMessage: string | null = '';
      error.subscribe((e) => (errorMessage = e))();

      expect(errorMessage).toBe('Database error');
    });

    it('should handle updateKey for non-existent key', () => {
      const nonExistentKey = {
        id: 999,
        app_name: 'Ghost',
        key_name: 'Non-existent',
        api_url: null,
        description: null,
        key_value: 'encrypted',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      };

      // Should not throw even if key doesn't exist
      expect(() => vaultStore.updateKey(nonExistentKey)).not.toThrow();
    });

    it('should handle removeKey for non-existent key', () => {
      // Should not throw even if key doesn't exist
      expect(() => vaultStore.removeKey(999)).not.toThrow();
    });
  });
});

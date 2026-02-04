/**
 * Tests for Tauri service layer
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
  initVault,
  unlockVault,
  lockVault,
  listApiKeys,
  createApiKey,
  updateApiKey,
  deleteApiKey,
  changePin,
  getSessionState,
  isInitialized,
  copyApiKeyById,
  updateActivity,
  isTauriAvailable,
  createMockTauriApi,
} from '$lib/services/tauri';
import type { ApiKey } from '$lib/types';

// Mock @tauri-apps/api/core module
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

// Mock Tauri invoke function
const mockInvoke = vi.mocked(invoke);

// Setup global window.__TAURI__
beforeEach(() => {
  mockInvoke.mockResolvedValue(undefined);
  Object.defineProperty(window, '__TAURI__', {
    value: {
      tauri: { invoke: mockInvoke },
    },
    writable: true,
  });
});

afterEach(() => {
  vi.clearAllMocks();
});

describe('Tauri Service Layer', () => {
  describe('initVault', () => {
    it('should call init_vault command with PIN', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await initVault({ pin: 'test1234' });

      expect(mockInvoke).toHaveBeenCalledWith('init_vault', { pin: 'test1234' });
    });

    it('should throw error on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('Already initialized'));

      await expect(initVault({ pin: 'test1234' })).rejects.toThrow(
        'Failed to initialize vault'
      );
    });
  });

  describe('unlockVault', () => {
    it('should call unlock_vault command with PIN', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await unlockVault({ pin: 'test1234' });

      expect(mockInvoke).toHaveBeenCalledWith('unlock_vault', { pin: 'test1234' });
    });

    it('should throw error on invalid PIN', async () => {
      mockInvoke.mockRejectedValue(new Error('Invalid PIN'));

      await expect(unlockVault({ pin: 'wrong' })).rejects.toThrow('Failed to unlock vault');
    });
  });

  describe('lockVault', () => {
    it('should call lock_vault command', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await lockVault();

      expect(mockInvoke).toHaveBeenCalledWith('lock_vault');
    });

    it('should throw error on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('Not unlocked'));

      await expect(lockVault()).rejects.toThrow('Failed to lock vault');
    });
  });

  describe('listApiKeys', () => {
    const mockKeys: ApiKey[] = [
      {
        id: 1,
        app_name: 'GitHub',
        key_name: 'Personal Token',
        api_url: 'https://api.github.com',
        description: 'For GitHub API access',
        key_value: 'encrypted_value',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      },
    ];

    it('should return list of API keys', async () => {
      mockInvoke.mockResolvedValue({ success: true, data: mockKeys, error: null });

      const keys = await listApiKeys();

      expect(mockInvoke).toHaveBeenCalledWith('list_api_keys');
      expect(keys).toHaveLength(1);
      expect(keys[0].appName).toBe('GitHub');
    });

    it('should convert snake_case to camelCase', async () => {
      mockInvoke.mockResolvedValue({ success: true, data: mockKeys, error: null });

      const keys = await listApiKeys();

      expect(keys[0]).toHaveProperty('appName');
      expect(keys[0]).toHaveProperty('keyName');
      expect(keys[0]).toHaveProperty('keyValue');
      expect(keys[0]).toHaveProperty('apiUrl');
      expect(keys[0]).toHaveProperty('createdAt');
      expect(keys[0]).toHaveProperty('updatedAt');
    });

    it('should return empty array when no keys exist', async () => {
      mockInvoke.mockResolvedValue({ success: true, data: [], error: null });

      const keys = await listApiKeys();

      expect(keys).toEqual([]);
    });

    it('should throw error when command fails', async () => {
      mockInvoke.mockResolvedValue({
        success: false,
        data: null,
        error: 'Vault is locked'
      });

      await expect(listApiKeys()).rejects.toThrow('Failed to list API keys');
    });

    it('should throw error when data is null', async () => {
      mockInvoke.mockResolvedValue({
        success: true,
        data: null,
        error: null
      });

      await expect(listApiKeys()).rejects.toThrow('Failed to list API keys');
    });

    it('should handle network errors', async () => {
      mockInvoke.mockRejectedValue(new Error('Network error'));

      await expect(listApiKeys()).rejects.toThrow('Failed to list API keys');
    });
  });

  describe('createApiKey', () => {
    it('should call create_api_key command with correct params', async () => {
      const newKey: ApiKey = {
        id: 1,
        app_name: 'TestApp',
        key_name: 'Test Key',
        api_url: 'https://api.test.com',
        description: 'Test description',
        key_value: 'encrypted',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      };
      mockInvoke.mockResolvedValue(newKey);

      const result = await createApiKey({
        appName: 'TestApp',
        keyName: 'Test Key',
        keyValue: 'test_key_value',
        apiUrl: 'https://api.test.com',
        description: 'Test description',
      });

      expect(mockInvoke).toHaveBeenCalledWith('create_api_key', {
        appName: 'TestApp',
        keyName: 'Test Key',
        apiKey: 'test_key_value',
        apiUrl: 'https://api.test.com',
        description: 'Test description',
      });
      expect(result).toEqual(newKey);
    });

    it('should handle optional parameters', async () => {
      mockInvoke.mockResolvedValue({
        id: 1,
        app_name: 'TestApp',
        key_name: 'Test Key',
        api_url: null,
        description: null,
        key_value: 'encrypted',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      });

      await createApiKey({
        appName: 'TestApp',
        keyName: 'Test Key',
        keyValue: 'test_key_value',
      });

      expect(mockInvoke).toHaveBeenCalledWith('create_api_key', {
        appName: 'TestApp',
        keyName: 'Test Key',
        apiKey: 'test_key_value',
        apiUrl: null,
        description: null,
      });
    });

    it('should handle errors from command', async () => {
      mockInvoke.mockRejectedValue(new Error('Failed to create key'));

      await expect(
        createApiKey({
          appName: 'Test',
          keyName: 'Test Key',
          keyValue: 'secret',
        })
      ).rejects.toThrow('Failed to create API key');
    });
  });

  describe('updateApiKey', () => {
    it('should call update_api_key command with id and params', async () => {
      mockInvoke.mockResolvedValue({
        id: 1,
        app_name: 'UpdatedApp',
        key_name: 'Updated Key',
        api_url: null,
        description: null,
        key_value: 'encrypted',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-02T00:00:00Z',
      });

      await updateApiKey({
        id: 1,
        appName: 'UpdatedApp',
        keyName: 'Updated Key',
        keyValue: 'new_key_value',
      });

      expect(mockInvoke).toHaveBeenCalledWith('update_api_key', {
        id: 1,
        appName: 'UpdatedApp',
        keyName: 'Updated Key',
        apiKey: 'new_key_value',
        apiUrl: null,
        description: null,
      });
    });
  });

  describe('deleteApiKey', () => {
    it('should call delete_api_key command with id', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await deleteApiKey(1);

      expect(mockInvoke).toHaveBeenCalledWith('delete_api_key', { id: 1 });
    });
  });

  describe('changePin', () => {
    it('should call change_pin command with old and new PIN', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await changePin({ oldPin: 'old123', newPin: 'new456' });

      expect(mockInvoke).toHaveBeenCalledWith('change_pin', {
        oldPin: 'old123',
        newPin: 'new456',
      });
    });

    it('should throw error on incorrect old PIN', async () => {
      mockInvoke.mockRejectedValue(new Error('Invalid PIN'));

      await expect(
        changePin({ oldPin: 'wrong', newPin: 'new456' })
      ).rejects.toThrow('Failed to change PIN');
    });
  });

  describe('getSessionState', () => {
    it('should return session state', async () => {
      const mockState = { is_unlocked: true, last_activity_secs: 30 };
      mockInvoke.mockResolvedValue(mockState);

      const state = await getSessionState();

      expect(mockInvoke).toHaveBeenCalledWith('get_session_state');
      expect(state).toEqual(mockState);
    });
  });

  describe('isInitialized', () => {
    it('should return true when vault is initialized', async () => {
      mockInvoke.mockResolvedValue(true);

      const result = await isInitialized();

      expect(result).toBe(true);
    });

    it('should return false when vault is not initialized', async () => {
      mockInvoke.mockResolvedValue(false);

      const result = await isInitialized();

      expect(result).toBe(false);
    });
  });

  describe('copyApiKeyById', () => {
    it('should call copy_to_clipboard command with id', async () => {
      mockInvoke.mockResolvedValue({
        success: true,
        data: 'secret_key',
        error: null
      });

      const result = await copyApiKeyById(123);

      expect(mockInvoke).toHaveBeenCalledWith('copy_to_clipboard', {
        id: '123',
      });
      expect(result).toBe('secret_key');
    });

    it('should throw error when command fails', async () => {
      mockInvoke.mockResolvedValue({
        success: false,
        data: null,
        error: 'Key not found'
      });

      await expect(copyApiKeyById(999)).rejects.toThrow('Failed to copy to clipboard');
    });

    it('should throw error when command returns null data', async () => {
      mockInvoke.mockResolvedValue({
        success: true,
        data: null,
        error: null
      });

      await expect(copyApiKeyById(123)).rejects.toThrow('Failed to copy to clipboard');
    });

    it('should handle network errors', async () => {
      mockInvoke.mockRejectedValue(new Error('Network error'));

      await expect(copyApiKeyById(123)).rejects.toThrow('Failed to copy to clipboard');
    });
  });

  describe('updateActivity', () => {
    it('should call update_activity command', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await updateActivity();

      expect(mockInvoke).toHaveBeenCalledWith('update_activity');
    });
  });

  describe('isTauriAvailable', () => {
    it('should return true when __TAURI__ exists', () => {
      expect(isTauriAvailable()).toBe(true);
    });

    it('should return false when __TAURI__ does not exist', () => {
      delete window.__TAURI__;

      expect(isTauriAvailable()).toBe(false);

      // Restore for other tests
      Object.defineProperty(window, '__TAURI__', {
        value: { tauri: { invoke: mockInvoke } },
        writable: true,
      });
    });
  });

  describe('createMockTauriApi', () => {
    it('should return mock functions for all commands', () => {
      const mock = createMockTauriApi();

      expect(mock).toHaveProperty('initVault');
      expect(mock).toHaveProperty('unlockVault');
      expect(mock).toHaveProperty('lockVault');
      expect(mock).toHaveProperty('listApiKeys');
      expect(mock).toHaveProperty('createApiKey');
      expect(mock).toHaveProperty('updateApiKey');
      expect(mock).toHaveProperty('deleteApiKey');
      expect(mock).toHaveProperty('changePin');
      expect(mock).toHaveProperty('getSessionState');
      expect(mock).toHaveProperty('isInitialized');
      expect(mock).toHaveProperty('copyToClipboard');
      expect(mock).toHaveProperty('updateActivity');
    });

    it('should mock initVault', async () => {
      const mock = createMockTauriApi();

      await mock.initVault({ pin: 'test1234' });

      // Should not throw and should complete
      expect(true).toBe(true);
    });

    it('should mock listApiKeys with empty array', async () => {
      const mock = createMockTauriApi();

      const keys = await mock.listApiKeys();

      expect(keys).toEqual([]);
    });

    it('should mock createApiKey and return key with id', async () => {
      const mock = createMockTauriApi();

      const key = await mock.createApiKey({
        appName: 'Test',
        keyName: 'Test Key',
        keyValue: 'secret',
      });

      expect(key).toHaveProperty('id', 1);
      expect(key.app_name).toBe('Test');
    });
  });
});

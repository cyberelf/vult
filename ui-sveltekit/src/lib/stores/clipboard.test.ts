/**
 * Tests for Clipboard Store
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { clipboardStore, hasCopiedKey, copiedKeyDisplay, countdownPercent } from '$lib/stores/clipboard';

describe('Clipboard Store', () => {
  beforeEach(() => {
    clipboardStore.reset();
  });

  afterEach(() => {
    clipboardStore.reset();
  });

  describe('copy', () => {
    it('should copy text and update state', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('test_key_value', mockTauriCopy);

      expect(mockTauriCopy).toHaveBeenCalledWith('test_key_value');
    });

    it('should set copiedKey when copy succeeds', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('secret_key', mockTauriCopy);

      let copiedKey: string | null = null;
      clipboardStore.subscribe((s) => (copiedKey = s.copiedKey))();

      expect(copiedKey).toBe('secret_key');
    });

    it('should set isCountingDown to true when copy succeeds', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('secret', mockTauriCopy);

      let isCountingDown = false;
      clipboardStore.subscribe((s) => (isCountingDown = s.isCountingDown))();

      expect(isCountingDown).toBe(true);
    });

    it('should set secondsRemaining to 45 on copy', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('secret', mockTauriCopy);

      let secondsRemaining = 0;
      clipboardStore.subscribe((s) => (secondsRemaining = s.secondsRemaining))();

      expect(secondsRemaining).toBe(45);
    });

    it('should handle copy errors gracefully', async () => {
      const mockTauriCopy = vi.fn().mockRejectedValue(new Error('Clipboard error'));
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await clipboardStore.copy('secret', mockTauriCopy);

      let copiedKey: string | null = null;
      clipboardStore.subscribe((s) => (copiedKey = s.copiedKey))();

      // Should not set copiedKey on error
      expect(copiedKey).toBeNull();

      consoleSpy.mockRestore();
    });

    it('should start countdown timer after copy', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('secret', mockTauriCopy);

      let secondsRemaining = 45;
      clipboardStore.subscribe((s) => (secondsRemaining = s.secondsRemaining))();

      // Should set initial countdown to 45
      expect(secondsRemaining).toBe(45);
      expect(secondsRemaining).toBeGreaterThan(0);
    });

    it('should clear clipboard after countdown reaches zero', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);
      vi.useFakeTimers();

      await clipboardStore.copy('secret', mockTauriCopy);

      let copiedKey: string | null = 'secret';
      clipboardStore.subscribe((s) => (copiedKey = s.copiedKey))();

      expect(copiedKey).toBe('secret');

      // Advance time by 45 seconds
      vi.advanceTimersByTime(45000);
      vi.runOnlyPendingTimers();

      clipboardStore.subscribe((s) => (copiedKey = s.copiedKey))();
      expect(copiedKey).toBeNull();

      vi.useRealTimers();
    }, 10000);

    it('should handle multiple copies by resetting countdown', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);
      vi.useFakeTimers();

      await clipboardStore.copy('first', mockTauriCopy);

      let secondsRemaining = 0;
      clipboardStore.subscribe((s) => (secondsRemaining = s.secondsRemaining))();

      expect(secondsRemaining).toBe(45);

      // Advance by 10 seconds
      vi.advanceTimersByTime(10000);
      vi.runAllTimers();

      clipboardStore.subscribe((s) => (secondsRemaining = s.secondsRemaining))();
      // Should be less than 45 due to countdown
      expect(secondsRemaining).toBeLessThan(45);

      // Copy again - should reset to 45
      await clipboardStore.copy('second', mockTauriCopy);

      clipboardStore.subscribe((s) => (secondsRemaining = s.secondsRemaining))();
      expect(secondsRemaining).toBe(45);

      vi.useRealTimers();
    });
  });

  describe('clear', () => {
    it('should clear clipboard state', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('secret', mockTauriCopy);
      clipboardStore.clear();

      let copiedKey: string | null = 'secret';
      clipboardStore.subscribe((s) => (copiedKey = s.copiedKey))();

      expect(copiedKey).toBeNull();
    });

    it('should stop countdown timer when cleared', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);
      vi.useFakeTimers();

      await clipboardStore.copy('secret', mockTauriCopy);
      clipboardStore.clear();

      // Advance time - state should not change
      vi.advanceTimersByTime(5000);
      vi.runOnlyPendingTimers();

      let secondsRemaining = 0;
      clipboardStore.subscribe((s) => (secondsRemaining = s.secondsRemaining))();
      expect(secondsRemaining).toBe(0);

      vi.useRealTimers();
    });
  });

  describe('hasCopiedKey derived store', () => {
    it('should return false when nothing is copied', () => {
      let hasCopied = false;
      hasCopiedKey.subscribe((h) => (hasCopied = h))();

      expect(hasCopied).toBe(false);
    });

    it('should return true when something is copied', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('secret', mockTauriCopy);

      let hasCopied = false;
      hasCopiedKey.subscribe((h) => (hasCopied = h))();

      expect(hasCopied).toBe(true);
    });

    it('should return false after countdown completes', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);
      vi.useFakeTimers();

      await clipboardStore.copy('secret', mockTauriCopy);

      vi.advanceTimersByTime(45000);
      vi.runOnlyPendingTimers();

      let hasCopied = true;
      hasCopiedKey.subscribe((h) => (hasCopied = h))();
      expect(hasCopied).toBe(false);

      vi.useRealTimers();
    }, 10000);
  });

  describe('copiedKeyDisplay derived store', () => {
    it('should return empty string when nothing is copied', () => {
      let display = '';
      copiedKeyDisplay.subscribe((d) => (display = d))();

      expect(display).toBe('');
    });

    it('should return full key when length <= 20', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('short_key', mockTauriCopy);

      let display = '';
      copiedKeyDisplay.subscribe((d) => (display = d))();

      expect(display).toBe('short_key');
    });

    it('should truncate key when length > 20', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      const longKey = 'very_long_api_key_that_exceeds_twenty_characters';
      await clipboardStore.copy(longKey, mockTauriCopy);

      let display = '';
      copiedKeyDisplay.subscribe((d) => (display = d))();

      // Should be first 20 chars + '...'
      expect(display).toBe('very_long_api_key_th...');
      expect(display).toHaveLength(23); // 20 chars + '...'
    });
  });

  describe('countdownPercent derived store', () => {
    it('should return 0 when nothing is copied', () => {
      let percent = 0;
      countdownPercent.subscribe((p) => (percent = p))();

      expect(percent).toBe(0);
    });

    it('should return 100 when countdown just started', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('secret', mockTauriCopy);

      let percent = 0;
      countdownPercent.subscribe((p) => (percent = p))();

      expect(percent).toBe(100);
    });

    it('should decrease as countdown progresses', async () => {
      const mockTauriCopy = vi.fn().mockResolvedValue(undefined);

      await clipboardStore.copy('secret', mockTauriCopy);

      let percent = 100;
      countdownPercent.subscribe((p) => (percent = p))();

      // Should start at 100%
      expect(percent).toBe(100);
    });
  });
});

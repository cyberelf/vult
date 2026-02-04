/**
 * Clipboard state management store
 * Handles clipboard copy operations and auto-clear functionality
 */

import { writable, derived } from 'svelte/store';

/**
 * Clipboard state interface
 */
export interface ClipboardState {
  /** The key/value that was copied */
  copiedKey: string | null;
  /** Timestamp when the copy occurred */
  copiedAt: Date | null;
  /** Seconds remaining until auto-clear */
  secondsRemaining: number;
  /** Whether the countdown is active */
  isCountingDown: boolean;
}

/**
 * Auto-clear duration in seconds (45 seconds as per requirements)
 */
const AUTO_CLEAR_SECONDS = 45;

/**
 * Initial clipboard state
 */
const initialState: ClipboardState = {
  copiedKey: null,
  copiedAt: null,
  secondsRemaining: 0,
  isCountingDown: false,
};

/**
 * Creates the clipboard state store with actions
 */
function createClipboardStore() {
  const { subscribe, set, update } = writable<ClipboardState>(initialState);

  let countdownInterval: number | null = null;

  return {
    subscribe,

    /**
     * Copy text to clipboard and start countdown
     */
    copy: async (text: string, tauriCopy: (text: string) => Promise<void>) => {
      try {
        await tauriCopy(text);
        const now = new Date();
        update((s) => ({
          ...s,
          copiedKey: text,
          copiedAt: now,
          secondsRemaining: AUTO_CLEAR_SECONDS,
          isCountingDown: true,
        }));

        // Start countdown
        if (countdownInterval) clearInterval(countdownInterval);
        countdownInterval = window.setInterval(() => {
          update((s) => {
            const remaining = s.secondsRemaining - 1;
            if (remaining <= 0) {
              if (countdownInterval) clearInterval(countdownInterval);
              return {
                ...s,
                copiedKey: null,
                copiedAt: null,
                secondsRemaining: 0,
                isCountingDown: false,
              };
            }
            return { ...s, secondsRemaining: remaining };
          });
        }, 1000);
      } catch (error) {
        console.error('Failed to copy to clipboard:', error);
      }
    },

    /**
     * Manually clear the clipboard
     */
    clear: () => {
      if (countdownInterval) clearInterval(countdownInterval);
      set(initialState);
    },

    /**
     * Reset the store to initial state
     */
    reset: () => {
      if (countdownInterval) clearInterval(countdownInterval);
      set(initialState);
    },
  };
}

/**
 * Clipboard state store
 */
export const clipboardStore = createClipboardStore();

/**
 * Derived store for whether something is copied
 */
export const hasCopiedKey = derived(clipboardStore, ($clipboard) => $clipboard.copiedKey !== null);

/**
 * Derived store for display text of what was copied (truncated)
 */
export const copiedKeyDisplay = derived(clipboardStore, ($clipboard) => {
  if (!$clipboard.copiedKey) return '';
  const key = $clipboard.copiedKey;
  return key.length > 20 ? `${key.substring(0, 20)}...` : key;
});

/**
 * Derived store for countdown percentage (for progress bar)
 */
export const countdownPercent = derived(clipboardStore, ($clipboard) => {
  if ($clipboard.secondsRemaining === 0) return 0;
  return ($clipboard.secondsRemaining / AUTO_CLEAR_SECONDS) * 100;
});

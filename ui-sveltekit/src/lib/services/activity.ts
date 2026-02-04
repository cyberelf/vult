/**
 * Activity tracking service
 * Tracks user activity (clicks, keypresses) to enable auto-lock after inactivity
 */

import * as tauri from '$lib/services/tauri';
import { vaultStore } from '$lib/stores/vault';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

/**
 * Activity tracker configuration
 */
const ACTIVITY_UPDATE_DELAY = 500; // Debounce activity updates to 500ms

/**
 * Activity tracker state
 */
let activityTimeout: number | null = null;
let vaultLockedUnlisten: UnlistenFn | null = null;
let isInitialized = false;

/**
 * Debounced activity update function
 */
function updateActivity() {
  if (activityTimeout) {
    clearTimeout(activityTimeout);
  }

  activityTimeout = window.setTimeout(async () => {
    try {
      // Only update activity if vault is unlocked
      const state = vaultStore.getCurrentState();
      if (state.isUnlocked) {
        await tauri.updateActivity();
      }
    } catch (error) {
      console.error('Failed to update activity:', error);
    }
  }, ACTIVITY_UPDATE_DELAY);
}

/**
 * Activity tracker service
 */
export const activityTracker = {
  /**
   * Initialize activity tracking
   * Sets up event listeners for clicks and keypresses
   */
  async init() {
    if (isInitialized) {
      return;
    }

    // Track click events
    document.addEventListener('click', updateActivity);

    // Track keypress events (excluding modifier keys alone)
    document.addEventListener('keydown', (e) => {
      // Ignore modifier keys pressed alone
      if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) {
        return;
      }
      updateActivity();
    });

    // Listen for vault_locked event from Tauri backend
    try {
      vaultLockedUnlisten = await listen('vault_locked', () => {
        vaultStore.lock();
      });
    } catch (error) {
      console.error('Failed to listen for vault_locked event:', error);
    }

    isInitialized = true;
  },

  /**
   * Stop activity tracking
   * Removes event listeners and unsubsribes from Tauri events
   */
  async destroy() {
    if (!isInitialized) {
      return;
    }

    document.removeEventListener('click', updateActivity);

    if (activityTimeout) {
      clearTimeout(activityTimeout);
      activityTimeout = null;
    }

    if (vaultLockedUnlisten) {
      await vaultLockedUnlisten();
      vaultLockedUnlisten = null;
    }

    isInitialized = false;
  },

  /**
   * Manually trigger activity update
   */
  update: () => {
    updateActivity();
  },
};

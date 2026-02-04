/**
 * UI state management store
 * Handles modal visibility, loading states, and UI-specific state
 */

import { writable, derived } from 'svelte/store';
import type { ModalState, LoadingState } from '$lib/types';

/**
 * UI state interface
 */
export interface UIState {
  /** Currently open modal */
  modal: ModalState;
  /** ID of the key being viewed/edited */
  activeKeyId: number | null;
  /** Form mode for key modal */
  keyModalMode: 'add' | 'edit';
  /** Toast notification message */
  toast: string | null;
  /** Toast type */
  toastType: 'success' | 'error' | 'info';
}

/**
 * Initial UI state
 */
const initialState: UIState = {
  modal: null,
  activeKeyId: null,
  keyModalMode: 'add',
  toast: null,
  toastType: 'info',
};

/**
 * Creates the UI state store with actions
 */
function createUIStore() {
  const { subscribe, set, update } = writable<UIState>(initialState);

  return {
    subscribe,

    /**
     * Open a modal
     */
    openModal: (modal: ModalState, keyId: number | null = null) => {
      update((s) => ({
        ...s,
        modal,
        activeKeyId: keyId,
        keyModalMode: keyId ? 'edit' : 'add',
      }));
    },

    /**
     * Close the current modal
     */
    closeModal: () => {
      update((s) => ({
        ...s,
        modal: null,
        activeKeyId: null,
      }));
    },

    /**
     * Show a toast notification
     */
    showToast: (message: string, type: 'success' | 'error' | 'info' = 'info') => {
      update((s) => ({ ...s, toast: message, toastType: type }));

      // Auto-hide after 3 seconds
      setTimeout(() => {
        update((s) => ({ ...s, toast: null }));
      }, 3000);
    },

    /**
     * Hide the toast notification
     */
    hideToast: () => {
      update((s) => ({ ...s, toast: null }));
    },

    /**
     * Set key modal mode
     */
    setKeyModalMode: (mode: 'add' | 'edit') => {
      update((s) => ({ ...s, keyModalMode: mode }));
    },

    /**
     * Reset UI state to initial
     */
    reset: () => {
      set(initialState);
    },
  };
}

/**
 * UI state store
 */
export const uiStore = createUIStore();

/**
 * Derived store for modal open state
 */
export const isModalOpen = derived(uiStore, ($ui) => $ui.modal !== null);

/**
 * Derived store for key modal open state
 */
export const isKeyModalOpen = derived(uiStore, ($ui) => $ui.modal === 'key');

/**
 * Derived store for view key modal open state
 */
export const isViewKeyModalOpen = derived(uiStore, ($ui) => $ui.modal === 'view');

/**
 * Derived store for delete modal open state
 */
export const isDeleteModalOpen = derived(uiStore, ($ui) => $ui.modal === 'delete');

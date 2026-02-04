/**
 * Tests for UI Store
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { uiStore, isModalOpen, isKeyModalOpen, isViewKeyModalOpen, isDeleteModalOpen } from '$lib/stores/ui';

describe('UI Store', () => {
  beforeEach(() => {
    uiStore.reset();
  });

  describe('openModal', () => {
    it('should open key modal with add mode', () => {
      uiStore.openModal('key', null);

      let modal: string | null = null;
      let mode: string = '';
      let keyId: number | null = null;

      uiStore.subscribe((s) => {
        modal = s.modal;
        mode = s.keyModalMode;
        keyId = s.activeKeyId;
      })();

      expect(modal).toBe('key');
      expect(mode).toBe('add');
      expect(keyId).toBeNull();
    });

    it('should open key modal with edit mode when keyId is provided', () => {
      uiStore.openModal('key', 123);

      let modal: string | null = null;
      let mode: string = '';
      let keyId: number | null = null;

      uiStore.subscribe((s) => {
        modal = s.modal;
        mode = s.keyModalMode;
        keyId = s.activeKeyId;
      })();

      expect(modal).toBe('key');
      expect(mode).toBe('edit');
      expect(keyId).toBe(123);
    });

    it('should open view modal', () => {
      uiStore.openModal('view', 456);

      let modal: string | null = null;
      let keyId: number | null = null;

      uiStore.subscribe((s) => {
        modal = s.modal;
        keyId = s.activeKeyId;
      })();

      expect(modal).toBe('view');
      expect(keyId).toBe(456);
    });

    it('should open delete modal', () => {
      uiStore.openModal('delete', 789);

      let modal: string | null = null;
      let keyId: number | null = null;

      uiStore.subscribe((s) => {
        modal = s.modal;
        keyId = s.activeKeyId;
      })();

      expect(modal).toBe('delete');
      expect(keyId).toBe(789);
    });

    it('should replace existing modal when opening another', () => {
      uiStore.openModal('view', 123);
      uiStore.openModal('delete', 456);

      let modal: string | null = null;
      let keyId: number | null = null;

      uiStore.subscribe((s) => {
        modal = s.modal;
        keyId = s.activeKeyId;
      })();

      expect(modal).toBe('delete');
      expect(keyId).toBe(456);
    });
  });

  describe('closeModal', () => {
    it('should close modal and reset state', () => {
      uiStore.openModal('view', 123);
      uiStore.closeModal();

      let modal: string | null = null;
      let keyId: number | null = null;

      uiStore.subscribe((s) => {
        modal = s.modal;
        keyId = s.activeKeyId;
      })();

      expect(modal).toBeNull();
      expect(keyId).toBeNull();
    });

    it('should handle closing when no modal is open', () => {
      expect(() => uiStore.closeModal()).not.toThrow();

      let modal: string | null = null;
      uiStore.subscribe((s) => (modal = s.modal))();

      expect(modal).toBeNull();
    });
  });

  describe('showToast', () => {
    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it('should show success toast', () => {
      uiStore.showToast('Key saved successfully', 'success');

      let toast: string | null = null;
      let type: string = '';

      uiStore.subscribe((s) => {
        toast = s.toast;
        type = s.toastType;
      })();

      expect(toast).toBe('Key saved successfully');
      expect(type).toBe('success');
    });

    it('should show error toast', () => {
      uiStore.showToast('Failed to save key', 'error');

      let toast: string | null = null;
      let type: string = '';

      uiStore.subscribe((s) => {
        toast = s.toast;
        type = s.toastType;
      })();

      expect(toast).toBe('Failed to save key');
      expect(type).toBe('error');
    });

    it('should default to info type', () => {
      uiStore.showToast('Information message');

      let type: string = '';
      uiStore.subscribe((s) => (type = s.toastType))();

      expect(type).toBe('info');
    });

    it('should auto-hide toast after 3 seconds', () => {
      uiStore.showToast('Temporary message');

      let toast: string | null = 'Temporary message';
      uiStore.subscribe((s) => (toast = s.toast))();

      expect(toast).toBe('Temporary message');

      // Advance time by 3 seconds
      vi.advanceTimersByTime(3000);

      // Wait for state update
      vi.runAllTimers();

      uiStore.subscribe((s) => (toast = s.toast))();
      expect(toast).toBeNull();
    });
  });

  describe('hideToast', () => {
    it('should hide toast immediately', () => {
      uiStore.showToast('Message');

      let toast: string | null = 'Message';
      uiStore.subscribe((s) => (toast = s.toast))();

      expect(toast).toBe('Message');

      uiStore.hideToast();

      uiStore.subscribe((s) => (toast = s.toast))();
      expect(toast).toBeNull();
    });
  });

  describe('setKeyModalMode', () => {
    it('should set key modal mode to add', () => {
      uiStore.setKeyModalMode('add');

      let mode: string = '';
      uiStore.subscribe((s) => (mode = s.keyModalMode))();

      expect(mode).toBe('add');
    });

    it('should set key modal mode to edit', () => {
      uiStore.setKeyModalMode('edit');

      let mode: string = '';
      uiStore.subscribe((s) => (mode = s.keyModalMode))();

      expect(mode).toBe('edit');
    });
  });

  describe('reset', () => {
    it('should reset all state to initial', () => {
      uiStore.openModal('view', 123);
      uiStore.showToast('Test message', 'error');
      uiStore.setKeyModalMode('edit');

      uiStore.reset();

      let modal: string | null = null;
      let keyId: number | null = null;
      let toast: string | null = null;
      let mode: string = '';

      uiStore.subscribe((s) => {
        modal = s.modal;
        keyId = s.activeKeyId;
        toast = s.toast;
        mode = s.keyModalMode;
      })();

      expect(modal).toBeNull();
      expect(keyId).toBeNull();
      expect(toast).toBeNull();
      expect(mode).toBe('add');
    });
  });

  describe('isModalOpen derived store', () => {
    it('should return false when no modal is open', () => {
      let isOpen = false;
      isModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(false);
    });

    it('should return true when key modal is open', () => {
      uiStore.openModal('key', null);

      let isOpen = false;
      isModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(true);
    });

    it('should return true when view modal is open', () => {
      uiStore.openModal('view', 123);

      let isOpen = false;
      isModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(true);
    });

    it('should return true when delete modal is open', () => {
      uiStore.openModal('delete', 123);

      let isOpen = false;
      isModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(true);
    });

    it('should return false after modal is closed', () => {
      uiStore.openModal('view', 123);
      uiStore.closeModal();

      let isOpen = false;
      isModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(false);
    });
  });

  describe('isKeyModalOpen derived store', () => {
    it('should return true when key modal is open', () => {
      uiStore.openModal('key', null);

      let isOpen = false;
      isKeyModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(true);
    });

    it('should return false when view modal is open', () => {
      uiStore.openModal('view', 123);

      let isOpen = false;
      isKeyModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(false);
    });
  });

  describe('isViewKeyModalOpen derived store', () => {
    it('should return true when view modal is open', () => {
      uiStore.openModal('view', 123);

      let isOpen = false;
      isViewKeyModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(true);
    });

    it('should return false when key modal is open', () => {
      uiStore.openModal('key', null);

      let isOpen = false;
      isViewKeyModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(false);
    });
  });

  describe('isDeleteModalOpen derived store', () => {
    it('should return true when delete modal is open', () => {
      uiStore.openModal('delete', 123);

      let isOpen = false;
      isDeleteModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(true);
    });

    it('should return false when view modal is open', () => {
      uiStore.openModal('view', 123);

      let isOpen = false;
      isDeleteModalOpen.subscribe((open) => (isOpen = open))();

      expect(isOpen).toBe(false);
    });
  });
});

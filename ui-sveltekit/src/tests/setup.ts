import { expect, vi, afterEach } from 'vitest';
import { cleanup } from '@testing-library/svelte';
import * as matchers from '@testing-library/jest-dom/matchers';

// Extend Vitest's expect with jest-dom matchers
expect.extend(matchers);

// Cleanup after each test
afterEach(() => {
  cleanup();
});

// Declare global type for __TAURI__
declare global {
  const __TAURI__: {
    tauri: {
      invoke: ReturnType<typeof vi.fn>;
    };
  };

  interface Window {
    __TAURI__?: {
      tauri: {
        invoke: ReturnType<typeof vi.fn>;
      };
    };
  }
}

// Mock Tauri API globally
(globalThis as any).__TAURI__ = {
  tauri: {
    invoke: vi.fn(),
  },
};

// Mock window.__TAURI__
Object.defineProperty(window, '__TAURI__', {
  value: {
    tauri: {
      invoke: vi.fn(),
    },
  },
  writable: true,
});

export {};

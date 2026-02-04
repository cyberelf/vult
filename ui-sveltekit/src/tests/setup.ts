import { expect, afterEach, vi } from 'vitest';
import { cleanup } from '@testing-library/svelte';
import * as matchers from '@testing-library/jest-dom/matchers';

// Extend Vitest's expect with jest-dom matchers
expect.extend(matchers);

// Cleanup after each test
afterEach(() => {
  cleanup();
});

// Mock Tauri API globally
global.__TAURI__ = {
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

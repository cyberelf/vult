/**
 * Theme store for managing light/dark mode
 */

import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'light' | 'dark';

// Get initial theme from localStorage or default to 'light'
function getInitialTheme(): Theme {
  if (!browser) return 'light';

  const stored = localStorage.getItem('vult-theme');
  if (stored === 'dark' || stored === 'light') {
    return stored;
  }

  // Default to light mode (don't check system preference)
  return 'light';
}

function applyTheme(theme: Theme) {
  if (browser) {
    document.documentElement.classList.toggle('dark', theme === 'dark');
  }
}

function createThemeStore() {
  const { subscribe, set, update } = writable<Theme>(getInitialTheme());

  return {
    subscribe,
    toggle: () => {
      update(currentTheme => {
        const newTheme = currentTheme === 'light' ? 'dark' : 'light';

        if (browser) {
          localStorage.setItem('vult-theme', newTheme);
          applyTheme(newTheme);
        }

        return newTheme;
      });
    },
    set: (theme: Theme) => {
      set(theme);

      if (browser) {
        localStorage.setItem('vult-theme', theme);
        applyTheme(theme);
      }
    },
    init: () => {
      if (browser) {
        const theme = getInitialTheme();
        set(theme);
        applyTheme(theme);
      }
    }
  };
}

export const themeStore = createThemeStore();

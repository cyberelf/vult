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
  
  // Check system preference as fallback
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  return prefersDark ? 'dark' : 'light';
}

function createThemeStore() {
  const { subscribe, set } = writable<Theme>(getInitialTheme());

  return {
    subscribe,
    toggle: () => {
      const newTheme = getInitialTheme() === 'light' ? 'dark' : 'light';
      set(newTheme);
      
      if (browser) {
        localStorage.setItem('vult-theme', newTheme);
        document.documentElement.setAttribute('data-theme', newTheme);
      }
    },
    set: (theme: Theme) => {
      set(theme);
      
      if (browser) {
        localStorage.setItem('vult-theme', theme);
        document.documentElement.setAttribute('data-theme', theme);
      }
    },
    init: () => {
      if (browser) {
        const theme = getInitialTheme();
        set(theme);
        document.documentElement.setAttribute('data-theme', theme);
      }
    }
  };
}

export const themeStore = createThemeStore();

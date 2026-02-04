# Design: SvelteKit + TypeScript Migration

## Status
**Status:** Draft | **Date:** 2025-02-03

## Overview

This document describes the technical design for migrating Vult's vanilla JavaScript frontend to SvelteKit + TypeScript with shadcn-svelte components. The migration preserves the existing Rust backend and Tauri commands while replacing the entire frontend implementation.

## Architecture

### Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Framework** | SvelteKit (Svelte 5) | Modern reactive framework with excellent TypeScript support |
| **Language** | TypeScript 5+ | Type safety for data models, API responses, and component props |
| **UI Library** | shadcn-svelte | Copy-paste component library with Tailwind CSS styling |
| **Styling** | Tailwind CSS | Utility-first CSS with dark mode support |
| **Desktop** | Tauri v2.1 | Cross-platform desktop framework (Rust backend) |
| **Build** | Vite | Fast build tool (included with SvelteKit) |
| **State** | Svelte Stores | Reactive state management for vault state, keys, UI |
| **Forms** | SvelteKit form actions + native validation | Server/client form handling |

### Project Structure

```
vult/
├── src/                          # Rust backend (UNCHANGED)
│   ├── main.rs                   # Tauri entry point
│   ├── auth.rs                   # Authentication
│   ├── commands.rs               # Tauri command handlers
│   ├── crypto.rs                 # Cryptography
│   ├── database.rs               # Database operations
│   └── clipboard.rs              # Clipboard management
│
├── ui-sveltekit/                 # NEW SvelteKit frontend
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/       # Svelte components
│   │   │   │   ├── vault/
│   │   │   │   │   ├── VaultScreen.svelte
│   │   │   │   │   ├── KeyTable.svelte
│   │   │   │   │   ├── KeyCard.svelte
│   │   │   │   │   └── SearchBar.svelte
│   │   │   │   ├── auth/
│   │   │   │   │   ├── SetupScreen.svelte
│   │   │   │   │   └── UnlockScreen.svelte
│   │   │   │   ├── modals/
│   │   │   │   │   ├── KeyModal.svelte
│   │   │   │   │   ├── ViewKeyModal.svelte
│   │   │   │   │   └── DeleteModal.svelte
│   │   │   │   └── ui/
│   │   │   │       └── shadcn/   # shadcn-svelte components
│   │   │   ├── stores/
│   │   │   │   ├── vault.ts      # Vault state (unlocked, keys)
│   │   │   │   ├── ui.ts         # UI state (modals, loading)
│   │   │   │   └── clipboard.ts  # Clipboard state
│   │   │   ├── types/
│   │   │   │   ├── api.ts        # Tauri command types
│   │   │   │   ├── models.ts     # Data models (ApiKey, VaultState)
│   │   │   │   └── index.ts      # Barrel exports
│   │   │   ├── services/
│   │   │   │   └── tauri.ts      # Tauri command wrappers
│   │   │   ├── utils/
│   │   │   │   └── escape.ts     # XSS prevention utilities
│   │   │   └── css/
│   │   │       └── app.css       # Tailwind imports + custom styles
│   │   ├── routes/
│   │   │   ├── +layout.svelte    # Root layout with providers
│   │   │   └── +page.svelte      # Main page (screen router)
│   │   └── app.html              # HTML template
│   ├── static/                   # Static assets
│   ├── tests/                    # Vitest tests
│   ├── components.json           # shadcn-svelte config
│   ├── tailwind.config.js        # Tailwind configuration
│   ├── svelte.config.js          # Svelte configuration
│   ├── vite.config.ts            # Vite configuration
│   └── tsconfig.json             # TypeScript configuration
│
├── ui/                           # OLD vanilla JS (TO BE DELETED)
│   ├── index.html
│   ├── app.js
│   └── styles.css
│
├── src-tauri/                    # Tauri configuration (UPDATED)
│   ├── tauri.conf.json           # Update build.frontendDist
│   └── capabilities/             # Tauri capabilities (UNCHANGED)
│
└── package.json                  # Root package.json
```

## Component Design

### Screen Routing

Instead of multiple HTML files, use a single-page application pattern with conditional rendering:

```typescript
// src/routes/+page.svelte
<script lang="ts">
  import { vaultState } from '$lib/stores/vault';
  import SetupScreen from '$lib/components/auth/SetupScreen.svelte';
  import UnlockScreen from '$lib/components/auth/UnlockScreen.svelte';
  import VaultScreen from '$lib/components/vault/VaultScreen.svelte';

  import { onMount } from 'svelte';

  onMount(async () => {
    await vaultState.initialize();
  });
</script>

{#if $vaultState.screen === 'setup'}
  <SetupScreen />
{:else if $vaultState.screen === 'unlock'}
  <UnlockScreen />
{:else if $vaultState.screen === 'vault'}
  <VaultScreen />
{/if}
```

### State Management

Use Svelte 5 stores with runes for reactive state:

```typescript
// src/lib/stores/vault.ts
import { writable, derived } from 'svelte/store';

export interface ApiKey {
  id: number;
  appName: string;
  keyName: string;
  apiUrl: string | null;
  description: string | null;
  keyValue: string; // encrypted
  createdAt: string;
  updatedAt: string;
}

interface VaultState {
  screen: 'setup' | 'unlock' | 'vault';
  isUnlocked: boolean;
  keys: ApiKey[];
  searchQuery: string;
  loading: boolean;
  error: string | null;
}

function createVaultStore() {
  const { subscribe, set, update } = writable<VaultState>({
    screen: 'setup',
    isUnlocked: false,
    keys: [],
    searchQuery: '',
    loading: false,
    error: null,
  });

  return {
    subscribe,
    initialize: async () => {
      const isInit = await invoke<boolean>('is_initialized');
      update(s => ({
        ...s,
        screen: isInit ? 'unlock' : 'setup',
      }));
    },
    unlock: async (pin: string) => {
      update(s => ({ ...s, loading: true, error: null }));
      try {
        await invoke('unlock_vault', { pin });
        const keys = await invoke<ApiKey[]>('list_api_keys');
        set({
          screen: 'vault',
          isUnlocked: true,
          keys,
          searchQuery: '',
          loading: false,
          error: null,
        });
      } catch (error) {
        update(s => ({
          ...s,
          loading: false,
          error: error as string,
        }));
      }
    },
    lock: async () => {
      await invoke('lock_vault');
      set({
        screen: 'unlock',
        isUnlocked: false,
        keys: [],
        searchQuery: '',
        loading: false,
        error: null,
      });
    },
    setSearchQuery: (query: string) => {
      update(s => ({ ...s, searchQuery: query }));
    },
  };
}

export const vaultState = createVaultStore();
export const filteredKeys = derived(
  vaultState,
  ($vault) => {
    if (!$vault.searchQuery) return $vault.keys;
    const query = $vault.searchQuery.toLowerCase();
    return $vault.keys.filter(key =>
      key.appName?.toLowerCase().includes(query) ||
      key.keyName.toLowerCase().includes(query) ||
      key.description?.toLowerCase().includes(query)
    );
  }
);
```

### Type-Safe Tauri Commands

Create TypeScript wrappers for all Tauri commands:

```typescript
// src/lib/types/api.ts
import type { InvokeArgs } from '@tauri-apps/api/tauri';

// Command types derived from Rust backend
export interface ApiKey {
  id: number;
  app_name: string;
  key_name: string;
  api_url: string | null;
  description: string | null;
  key_value: string; // encrypted
  created_at: string;
  updated_at: string;
}

export interface SessionState {
  is_unlocked: boolean;
  last_activity_secs: number;
}

export interface InitVaultArgs {
  pin: string;
}

export interface UnlockVaultArgs {
  pin: string;
}

export interface ChangePinArgs {
  oldPin: string;
  newPin: string;
}

export interface CreateApiKeyArgs {
  appName: string;
  keyName: string;
  apiUrl: string | null;
  description: string | null;
  keyValue: string;
}

export interface UpdateApiKeyArgs extends CreateApiKeyArgs {
  id: number;
}

// src/lib/services/tauri.ts
import { invoke } from '@tauri-apps/api/tauri';
import * as types from '$lib/types/api';

export async function initVault(args: types.InitVaultArgs): Promise<void> {
  return invoke('init_vault', args);
}

export async function unlockVault(args: types.UnlockVaultArgs): Promise<void> {
  return invoke('unlock_vault', args);
}

export async function lockVault(): Promise<void> {
  return invoke('lock_vault');
}

export async function listApiKeys(): Promise<types.ApiKey[]> {
  return invoke('list_api_keys');
}

export async function createApiKey(args: types.CreateApiKeyArgs): Promise<types.ApiKey> {
  return invoke('create_api_key', args);
}

export async function updateApiKey(args: types.UpdateApiKeyArgs): Promise<types.ApiKey> {
  return invoke('update_api_key', args);
}

export async function deleteApiKey(id: number): Promise<void> {
  return invoke('delete_api_key', { id });
}

export async function changePin(args: types.ChangePinArgs): Promise<void> {
  return invoke('change_pin', args);
}

export async function getSessionState(): Promise<types.SessionState> {
  return invoke('get_session_state');
}

export async function isInitialized(): Promise<boolean> {
  return invoke('is_initialized');
}

export async function copyToClipboard(text: string): Promise<void> {
  return invoke('copy_to_clipboard', { text });
}
```

## shadcn-svelte Integration

### Configuration

```json
// components.json
{
  "$schema": "https://shadcn-svelte.com/schema.json",
  "style": "new-york",
  "typescript": true,
  "tailwind": {
    "config": "tailwind.config.js",
    "css": "src/lib/css/app.css",
    "baseColor": "slate",
    "cssVariables": true
  },
  "aliases": {
    "components": "$lib/components/ui/shadcn",
    "utils": "$lib/utils"
  }
}
```

### Component Usage Example

```svelte
<!-- KeyModal.svelte -->
<script lang="ts">
  import * as Dialog from '$lib/components/ui/shadcn/dialog';
  import { Button } from '$lib/components/ui/shadcn/button';
  import { Input } from '$lib/components/ui/shadcn/input';
  import { Label } from '$lib/components/ui/shadcn/label';
  import { Textarea } from '$lib/components/ui/shadcn/textarea';

  export let open = false;
  export let onSave: (data: CreateKeyData) => Promise<void>;

  let formData = {
    appName: '',
    keyName: '',
    apiUrl: '',
    description: '',
    keyValue: '',
  };

  async function handleSubmit() {
    await onSave(formData);
    open = false;
    resetForm();
  }

  function resetForm() {
    formData = {
      appName: '',
      keyName: '',
      apiUrl: '',
      description: '',
      keyValue: '',
    };
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Add API Key</Dialog.Title>
      <Dialog.Description>
        Enter the details for your new API key.
      </Dialog.Description>
    </Dialog.Header>

    <form on:submit|preventDefault={handleSubmit}>
      <div class="space-y-4 py-4">
        <div class="space-y-2">
          <Label for="app-name">App Name</Label>
          <Input
            id="app-name"
            bind:value={formData.appName}
            placeholder="e.g., GitHub"
            required
          />
        </div>

        <div class="space-y-2">
          <Label for="key-name">Key Name</Label>
          <Input
            id="key-name"
            bind:value={formData.keyName}
            placeholder="e.g., Personal Access Token"
            required
          />
        </div>

        <div class="space-y-2">
          <Label for="key-value">API Key</Label>
          <Input
            id="key-value"
            type="password"
            bind:value={formData.keyValue}
            placeholder="ghp_xxxxxxxxxxxx"
            required
          />
        </div>

        <div class="space-y-2">
          <Label for="api-url">API URL (Optional)</Label>
          <Input
            id="api-url"
            type="url"
            bind:value={formData.apiUrl}
            placeholder="https://api.github.com"
          />
        </div>

        <div class="space-y-2">
          <Label for="description">Description (Optional)</Label>
          <Textarea
            id="description"
            bind:value={formData.description}
            placeholder="Used for GitHub API access"
            rows="3"
          />
        </div>
      </div>

      <Dialog.Footer>
        <Button type="button" variant="outline" on:click={() => open = false}>
          Cancel
        </Button>
        <Button type="submit">Save</Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
```

## Responsive Design Strategy

### Tailwind Breakpoints

```js
// tailwind.config.js
export default {
  theme: {
    screens: {
      'sm': '320px',
      'md': '768px',
      'lg': '1024px',
      'xl': '1280px',
      '2xl': '1536px',
    },
  },
};
```

### Container Widths

Use Tailwind's container queries and max-width utilities:

```svelte
<!-- VaultScreen.svelte -->
<div class="container max-w-[1200px] mx-auto px-4 md:px-6 lg:px-8">
  <!-- Content -->
</div>

<!-- Setup/Unlock screens -->
<div class="container max-w-[600px] mx-auto px-4">
  <!-- Form -->
</div>
```

### Table Transformation

```svelte
<!-- KeyTable.svelte -->
{#if keys.length === 0}
  <EmptyState />
{:else}
  <div class="rounded-md border">
    <table class="w-full hidden md:table">
      <!-- Desktop table view -->
      <thead>
        <tr class="border-b bg-muted/50">
          <th class="p-4 text-left">Key Name</th>
          <th class="p-4 text-left">App Name</th>
          <th class="p-4 text-left">API URL</th>
          <th class="p-4 text-left">Description</th>
          <th class="p-4 text-center">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each keys as key (key.id)}
          <tr class="border-b hover:bg-muted/50">
            <td class="p-4">{key.keyName}</td>
            <td class="p-4">{key.appName}</td>
            <td class="p-4">{key.apiUrl || '-'}</td>
            <td class="p-4">{key.description || '-'}</td>
            <td class="p-4 text-center">
              <KeyActions {key} />
            </td>
          </tr>
        {/each}
      </tbody>
    </table>

    <!-- Mobile card view -->
    <div class="md:hidden space-y-4 p-4">
      {#each keys as key (key.id)}
        <KeyCard {key} />
      {/each}
    </div>
  </div>
{/if}
```

## Migration Plan

### Phase 1: Project Setup (Foundation)

1. **Initialize SvelteKit project**
   ```bash
   npm create svelte@latest ui-sveltekit
   # Options: Skeleton, TypeScript, ESLint, Prettier, Playwright
   ```

2. **Install dependencies**
   ```bash
   cd ui-sveltekit
   npm install
   npm install -D tailwindcss postcss autoprefixer
   npx tailwindcss init -p
   ```

3. **Install shadcn-svelte**
   ```bash
   npx shadcn-svelte@latest init
   # Install required components: button, input, label, textarea, dialog, table
   ```

4. **Install Tauri API**
   ```bash
   npm install @tauri-apps/api
   ```

5. **Update Tauri configuration**
   - Update `src-tauri/tauri.conf.json`
   - Set `build.frontendDist` to `"../ui-sveltekit/.svelte-kit/output"`
   - Set `build.devUrl` to `"http://localhost:5173"`

### Phase 2: Type Definitions (Type Safety)

1. **Create API types** (`src/lib/types/api.ts`)
   - Mirror all Rust structs (ApiKey, SessionState, etc.)
   - Define command argument types
   - Define response types

2. **Create Tauri service** (`src/lib/services/tauri.ts`)
   - Type-safe wrappers for all commands
   - Error handling
   - JSDoc documentation

### Phase 3: Core Components (UI Foundation)

1. **Setup stores** (`src/lib/stores/`)
   - Vault state store
   - UI state store
   - Clipboard store

2. **Create layout** (`src/routes/+layout.svelte`)
   - Dark mode provider
   - Toast notifications
   - Global styles

3. **Create auth screens**
   - SetupScreen.svelte
   - UnlockScreen.svelte

4. **Create vault screens**
   - VaultScreen.svelte
   - KeyTable.svelte
   - KeyCard.svelte
   - SearchBar.svelte

### Phase 4: Modals and Interactions

1. **Create modal components**
   - KeyModal.svelte (add/edit)
   - ViewKeyModal.svelte
   - DeleteModal.svelte

2. **Implement CRUD operations**
   - Create key
   - Read keys (with search)
   - Update key (inline or modal)
   - Delete key

3. **Clipboard integration**
   - Copy to clipboard
   - Auto-clear after 45 seconds

### Phase 5: Testing and Polish

1. **Add Vitest tests**
   - Component tests
   - Store tests
   - Utility tests

2. **Accessibility audit**
   - Keyboard navigation
   - Screen reader testing
   - Focus management

3. **Responsive testing**
   - Test at 320px, 375px, 768px, 1024px, 1440px
   - Verify touch targets (44x44px minimum)
   - Test table/card transformation

### Phase 6: Integration and Build

1. **Update Tauri build**
   - Test dev mode: `cargo tauri dev`
   - Build for release: `cargo tauri build`

2. **Delete old UI**
   - Remove `ui/` directory
   - Update documentation

3. **Final testing**
   - End-to-end testing
   - Performance profiling
   - Security audit

## Technical Decisions

### Decision 1: Svelte 5 Runes vs Svelte 4

**Choice:** Svelte 5 with runes

**Rationale:**
- runes are the future of Svelte reactivity
- better TypeScript support
- more intuitive reactivity model
- SvelteKit is built with Svelte 5 support

### Decision 2: shadcn-svelte vs Skeleton UI

**Choice:** shadcn-svelte

**Rationale:**
- Copy-paste model = full control over components
- Better TypeScript support
- More active development
- Better dark mode implementation
- Aligns with web development trends

### Decision 3: Form Validation Strategy

**Choice:** Native HTML5 validation + SvelteKit form actions

**Rationale:**
- No additional dependencies needed
- Built-in browser accessibility
- Simple validation rules (minlength, required, pattern)
- shadcn-svelte components work with native validation

### Decision 4: State Management

**Choice:** Svelte stores (writable/derived)

**Rationale:**
- Built-in to Svelte
- Simple and sufficient for app size
- No need for external state management libraries
- Works well with TypeScript

### Decision 5: Routing Strategy

**Choice:** Single-page with conditional rendering (not file-based routing)

**Rationale:**
- App has simple screen flow (setup → unlock → vault)
- No URL-based navigation needed (desktop app)
- Simpler state management
- Better transition animations

### Decision 6: Styling Approach

**Choice:** Tailwind CSS + shadcn-svelte theming

**Rationale:**
- Tailwind is standard for modern web development
- shadcn-svelte provides consistent component styling
- CSS variables for easy theming (dark mode)
- Responsive utilities built-in
- No custom CSS needed for 95% of UI

## Risk Analysis

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **Tauri integration issues** | Medium | High | Test Tauri commands early; use type-safe wrappers |
| **Performance degradation** | Low | Medium | Profile bundle size; use Svelte 5 optimizations |
| **shadcn-svelte bugs** | Medium | Low | Can modify copied components; fallback to custom |
| **Type sync drift (Rust ↔ TS)** | Medium | High | Document types clearly; add tests; consider codegen |
| **Accessibility regressions** | Low | High | Accessibility audit; use shadcn components (a11y tested) |
| **Build configuration complexity** | Medium | Medium | Follow SvelteKit + Tauri official guide; test early |

## Performance Considerations

### Bundle Size

- **Estimated initial bundle:** ~150KB gzipped (SvelteKit runtime + shadcn components)
- **Tree-shaking:** Svelte's compiler removes unused code
- **Code splitting:** Lazy load modal components

### Runtime Performance

- **Reactivity:** Svelte 5 runes are compiled to optimized JavaScript
- **Virtual scrolling:** Consider for large key lists (100+ keys)
- **Debounced search:** 300ms delay on search input

## Security Considerations

### Type Safety Benefits

- **Compile-time checks:** Catch mismatched API types before runtime
- **Autocomplete:** IDE prevents passing wrong command arguments
- **Refactoring:** Safe renaming across frontend and command calls

### XSS Prevention

- **Auto-escaping:** Svelte escapes interpolated values by default
- **shadcn components:** Built-in XSS protection
- **Content Security Policy:** Keep existing CSP headers

### PIN Handling

- **No logging in type definitions:** Avoid logging PIN values in types
- **Sensitive data marking:** Mark encrypted fields with comments

## Testing Strategy

### Unit Tests (Vitest)

```typescript
// vault.test.ts
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { vaultState } from './vault';

describe('vaultState', () => {
  beforeEach(() => {
    vi.mock('$lib/services/tauri', () => ({
      isInitialized: vi.fn().mockResolvedValue(true),
      unlockVault: vi.fn().mockResolvedValue(undefined),
      listApiKeys: vi.fn().mockResolvedValue([]),
    }));
  });

  it('should initialize to unlock screen if vault exists', async () => {
    await vaultState.initialize();
    // Assert state
  });
});
```

### Component Tests

```svelte
<!-- SetupScreen.test.svelte -->
<script module>
  import { render, screen } from '@testing-library/svelte';
  import SetupScreen from './SetupScreen.svelte';
</script>

<script>
  render(SetupScreen);
</script>

<h1>Setup Vult</h1> <!-- Should be present -->
```

## Success Criteria

The migration is successful when:

1. ✅ All existing functionality works in the new SvelteKit UI
2. ✅ TypeScript compiles without `any` types
3. ✅ All Tauri commands have type-safe wrappers
4. ✅ Responsive design works at all breakpoints (320px - 4K)
5. ✅ Accessibility features maintained (keyboard nav, screen readers)
6. ✅ Build succeeds for development and production
7. ✅ Performance matches or exceeds vanilla JS implementation
8. ✅ Old `ui/` directory is removed
9. ✅ Documentation updated (README, CONTRIBUTING)
10. ✅ Tests pass (unit + component + e2e)

## Rollback Plan

If critical issues are found:

1. **Keep old `ui/` directory** until migration is verified
2. **Tauri config toggle:** Switch `build.frontendDist` between old and new
3. **Git branches:** Maintain branch for rollback
4. **Database compatibility:** No schema changes, so data is safe

## Open Questions

1. **Should we use SvelteKit's server-side features?**
   - **Answer:** No, desktop app doesn't need SSR
   - Use SPA mode (`export const ssr = false`)

2. **Should we generate TypeScript types from Rust?**
   - **Options:** `tauri-specta` vs manual types
   - **Recommendation:** Manual types (simpler for current scope)

3. **Should we use a form validation library?**
   - **Options:** `felte` vs native validation
   - **Recommendation:** Native validation (sufficient for current needs)

## References

- [SvelteKit + Tauri Guide](https://tauri.app/v2/guides/framework-guides/sveltekit/)
- [shadcn-svelte Docs](https://www.shadcn-svelte.com/)
- [Svelte 5 Runes Docs](https://svelte-5-preview.vercel.app/docs/runes)
- [Tailwind CSS Docs](https://tailwindcss.com/docs)
- [Tauri v2 API Docs](https://tauri.app/v2/api/js/)

# Proposal: SvelteKit + TypeScript Migration

## Why

The current vanilla JavaScript/CSS implementation lacks type safety and modern framework benefits. Migrating to SvelteKit with TypeScript will provide compile-time type checking, better IDE support, improved maintainability, and access to modern UI component libraries (shadcn-svelte) for a more polished user experience. This migration also positions the codebase for future enhancements and easier collaboration.

## What Changes

- **Frontend Framework**: Complete rewrite from vanilla JS to SvelteKit (Svelte 5)
- **Language**: Migrate from JavaScript to TypeScript for all frontend code
- **UI Components**: Integrate shadcn-svelte component library with Tailwind CSS
- **Build System**: Replace direct HTML serving with SvelteKit dev/build system
- **State Management**: Leverage Svelte stores instead of manual DOM manipulation
- **Styling**: Adopt Tailwind CSS via shadcn-svelte instead of custom CSS
- **Type Safety**: Define TypeScript interfaces for all data models and Tauri commands

## Capabilities

### New Capabilities

- **sveltekit-frontend**: SvelteKit application structure with routing, layouts, and server-side integration
- **typescript-types**: TypeScript type definitions for all data models, API keys, and Tauri command interfaces
- **svelte-stores**: Reactive state management using Svelte stores for vault state, keys, and UI state
- **component-library**: Integration of shadcn-svelte components (Button, Input, Table, Dialog, etc.)
- **tauri-integration**: SvelteKit-specific Tauri integration patterns with type-safe command invocation

### Modified Capabilities

- None - this is a complete frontend rewrite; backend (Rust) remains functionally unchanged

## Impact

### Affected Code

- **Complete frontend replacement**: [ui/](ui/) directory structure will be replaced with SvelteKit project
- **Deleted files**: [ui/index.html](ui/index.html), [ui/app.js](ui/app.js), [ui/styles.css](ui/styles.css), [ui/app.test.js](ui/app.test.js)
- **Backend preservation**: [src/](src/) Rust backend remains largely unchanged (Tauri commands compatibility maintained)
- **New dependencies**: `@sveltejs/kit`, `svelte`, `typescript`, `@tauri-apps/api`, `shadcn-svelte`, `tailwindcss`, etc.

### Dependencies

- **SvelteKit**: Latest version (Svelte 5 based)
- **TypeScript**: ^5.0 for type safety
- **shadcn-svelte**: UI component library
- **Tailwind CSS**: Utility-first CSS framework
- **Tauri**: Existing v2.1 integration maintained
- **Vitest**: Replace current test setup with SvelteKit-native testing

### Systems

- **Frontend Architecture**: Shifts from vanilla DOM manipulation to component-based reactive framework
- **Build Process**: SvelteKit dev server for development, optimized production builds
- **Testing**: Migrate from Vitest DOM testing to SvelteKit component testing
- **State Management**: Centralized reactive stores replacing scattered state variables
- **Type Safety**: Compile-time type checking replaces runtime type coercion issues

### Migration Strategy

1. **Phase 1 - Project Setup**: Initialize new SvelteKit project with TypeScript and Tauri
2. **Phase 2 - Component Library**: Install and configure shadcn-svelte with Tailwind CSS
3. **Phase 3 - Type Definitions**: Create TypeScript interfaces for all data models
4. **Phase 4 - Tauri Integration**: Set up type-safe Tauri command invocation
5. **Phase 5 - Screens Migration**: Rebuild Setup, Unlock, and Vault screens with components
6. **Phase 6 - State Management**: Implement Svelte stores for application state
7. **Phase 7 - Testing**: Set up component and integration tests
8. **Phase 8 - Styling**: Apply shadcn-svelte theming with custom dark mode colors

### User Experience

- **Preserved Features**: All existing functionality (PIN auth, API key CRUD, search, clipboard)
- **Enhanced UI**: Modern, polished components from shadcn-svelte
- **Better Performance**: Svelte's compiled components and reactive updates
- **Improved Developer Experience**: TypeScript autocomplete, type errors, and refactoring support

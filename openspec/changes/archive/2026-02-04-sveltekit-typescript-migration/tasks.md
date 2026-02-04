# Tasks: SvelteKit + TypeScript Migration

## 1. Project Setup

- [x] 1.1 Initialize SvelteKit project in `ui-sveltekit/` directory
- [x] 1.2 Configure SvelteKit for SPA mode (disable SSR)
- [x] 1.3 Install Tailwind CSS and configure with custom breakpoints
- [x] 1.4 Install shadcn-svelte and run init command
- [x] 1.5 Install required shadcn components: button, input, label, textarea, dialog, table
- [x] 1.6 Install @tauri-apps/api package for Tauri integration
- [x] 1.7 Configure TypeScript strict mode in tsconfig.json
- [x] 1.8 Update Tauri config (tauri.conf.json) to point to SvelteKit build output
- [x] 1.9 Update Tauri devUrl to point to Vite dev server (localhost:5173)
- [x] 1.10 Test that `cargo tauri dev` launches with SvelteKit frontend

## 2. Type Definitions

- [x] 2.1 Create `src/lib/types/api.ts` with ApiKey interface (matching Rust struct)
- [x] 2.2 Create SessionState interface in api.ts
- [x] 2.3 Create InitVaultArgs interface
- [x] 2.4 Create UnlockVaultArgs interface
- [x] 2.5 Create ChangePinArgs interface
- [x] 2.6 Create CreateApiKeyArgs interface
- [x] 2.7 Create UpdateApiKeyArgs interface
- [x] 2.8 Create barrel export `src/lib/types/index.ts` re-exporting all types
- [x] 2.9 Add JSDoc comments to all type definitions
- [x] 2.10 Verify TypeScript compiles with no errors

## 3. Tauri Service Layer

- [x] 3.1 Create `src/lib/services/tauri.ts` service file
- [x] 3.2 Implement initVault wrapper function
- [x] 3.3 Implement unlockVault wrapper function
- [x] 3.4 Implement lockVault wrapper function
- [x] 3.5 Implement listApiKeys wrapper function
- [x] 3.6 Implement createApiKey wrapper function
- [x] 3.7 Implement updateApiKey wrapper function
- [x] 3.8 Implement deleteApiKey wrapper function
- [x] 3.9 Implement changePin wrapper function
- [x] 3.10 Implement getSessionState wrapper function
- [x] 3.11 Implement isInitialized wrapper function
- [x] 3.12 Implement copyToClipboard wrapper function
- [x] 3.13 Add try-catch error handling to all wrapper functions
- [x] 3.14 Add JSDoc comments to all wrapper functions
- [x] 3.15 Test wrapper functions compile correctly

## 4. State Management (Stores)

- [x] 4.1 Create `src/lib/stores/vault.ts` store file
- [x] 4.2 Define VaultState interface with screen, isUnlocked, keys, searchQuery, loading, error
- [x] 4.3 Implement vaultState writable store with initial state
- [x] 4.4 Implement initialize action that checks if vault is initialized
- [x] 4.5 Implement unlock action that calls unlockVault and fetches keys
- [x] 4.6 Implement lock action that calls lockVault and resets state
- [x] 4.7 Implement initialize action for setup screen (init_vault command)
- [x] 4.8 Implement setSearchQuery action for search filtering
- [x] 4.9 Implement derived store filteredKeys that filters by searchQuery
- [x] 4.10 Add error handling to all async actions
- [x] 4.11 Create `src/lib/stores/ui.ts` store file
- [x] 4.12 Implement uiState store with modal visibility, loading, error state
- [x] 4.13 Create `src/lib/stores/clipboard.ts` store file
- [x] 4.14 Implement clipboardStore with copiedKey, copiedAt, auto-clear logic
- [x] 4.15 Test store reactivity with Svelte 5 runes

## 5. Root Layout and Base Styles

- [x] 5.1 Create `src/routes/+layout.svelte` root layout
- [x] 5.2 Add dark mode provider to layout
- [x] 5.3 Configure shadcn-svelte theming with CSS variables
- [x] 5.4 Create `src/lib/css/app.css` with Tailwind imports
- [x] 5.5 Add custom CSS variables for colors (primary, danger, background, text)
- [x] 5.6 Configure Tailwind breakpoints (sm: 320px, md: 768px, lg: 1024px)
- [x] 5.7 Add base styles and global reset
- [x] 5.8 Add toast notification container to layout
- [x] 5.9 Create `src/routes/+page.svelte` main page component

## 6. Authentication Screens

- [x] 6.1 Create `src/lib/components/auth/SetupScreen.svelte`
- [x] 6.2 Add setup form with PIN and PIN confirmation fields
- [x] 6.3 Implement form validation (minlength 6, PINs match)
- [x] 6.4 Connect form submit to vaultState.initialize action
- [x] 6.5 Add error message display
- [x] 6.6 Style form with shadcn Input, Label, Button components
- [x] 6.7 Create `src/lib/components/auth/UnlockScreen.svelte`
- [x] 6.8 Add unlock form with PIN field
- [x] 6.9 Implement form submit to vaultState.unlock action
- [x] 6.10 Add error message display for invalid PIN
- [x] 6.11 Add loading state during unlock
- [x] 6.12 Style unlock screen with shadcn components
- [x] 6.13 Test setup and unlock flows end-to-end

## 7. Vault Screen Components

- [x] 7.1 Create `src/lib/components/vault/VaultScreen.svelte`
- [x] 7.2 Add header with title and lock button
- [x] 7.3 Add "Add Key" button to header
- [x] 7.4 Create `src/lib/components/vault/SearchBar.svelte`
- [x] 7.5 Implement search input with two-way binding to vaultState
- [x] 7.6 Add debounced search (300ms delay)
- [x] 7.7 Create `src/lib/components/vault/KeyTable.svelte` for desktop view
- [x] 7.8 Implement table with columns: Key Name, App Name, URL, Description, API Key, Actions
- [x] 7.9 Add responsive hide/show (hidden on mobile, visible on desktop ≥768px)
- [x] 7.10 Create `src/lib/components/vault/KeyCard.svelte` for mobile view
- [x] 7.11 Implement card layout with data labels
- [x] 7.12 Add responsive hide/show (visible on mobile <768px, hidden on desktop)
- [x] 7.13 Create `src/lib/components/vault/KeyActions.svelte` for action buttons
- [x] 7.14 Implement view, copy, edit, delete buttons with shadcn Button component
- [x] 7.15 Add icon buttons with 44x44px touch targets
- [x] 7.16 Create `src/lib/components/vault/EmptyState.svelte`
- [x] 7.17 Implement empty state message when no keys exist
- [x] 7.18 Implement empty state message when search returns no results
- [x] 7.19 Connect all components to vaultState and filteredKeys store
- [x] 7.20 Test responsive table to card transformation

## 8. Modal Components

- [x] 8.1 Create `src/lib/components/modals/KeyModal.svelte` for add/edit
- [x] 8.2 Implement modal with shadcn Dialog component
- [x] 8.3 Add form fields: App Name, Key Name, API Key, API URL (optional), Description (optional)
- [x] 8.4 Implement form validation (required fields marked)
- [x] 8.5 Add props: open, mode (add/edit), keyData, onSave, onCancel
- [x] 8.6 Implement save action calling createApiKey or updateApiKey
- [x] 8.7 Add loading state during save
- [x] 8.8 Add error message display
- [x] 8.9 Ensure focus trap and escape key close work
- [x] 8.10 Test modal accessibility (ARIA attributes, keyboard nav)
- [x] 8.11 Create `src/lib/components/modals/ViewKeyModal.svelte`
- [x] 8.12 Implement modal to view key details
- [x] 8.13 Add show/hide password toggle for key value
- [x] 8.14 Add copy to clipboard button
- [x] 8.15 Add edit button that opens KeyModal in edit mode
- [x] 8.16 Add delete button that opens DeleteModal
- [x] 8.17 Create `src/lib/components/modals/DeleteModal.svelte`
- [x] 8.18 Implement confirmation dialog for key deletion
- [x] 8.19 Add "Are you sure?" message with key name
- [x] 8.20 Implement confirm action calling deleteApiKey
- [x] 8.21 Add cancel action to close modal
- [x] 8.22 Test all modal interactions and edge cases

## 9. Screen Routing Logic

- [x] 9.1 Update `src/routes/+page.svelte` with conditional rendering
- [x] 9.2 Implement screen switching based on vaultState.screen value
- [x] 9.3 Add transitions between screens (fade or slide)
- [x] 9.4 Test setup → unlock → vault flow
- [x] 9.5 Test vault → unlock (lock) flow
- [x] 9.6 Test that only one screen displays at a time

## 10. Clipboard Integration

- [x] 10.1 Connect copy button in ViewKeyModal to copyToClipboard service
- [x] 10.2 Update clipboardStore with copiedKey and timestamp
- [x] 10.3 Implement "Copied!" feedback button state
- [x] 10.4 Implement auto-clear after 45 seconds
- [x] 10.5 Add countdown timer to copy feedback
- [x] 10.6 Test clipboard copy and auto-clear functionality

## 11. Activity Tracking and Auto-Lock

- [x] 11.1 Create activity tracker utility in `src/lib/services/activity.ts`
- [x] 11.2 Implement click listener that calls update_activity Tauri command
- [x] 11.3 Implement keypress listener that calls update_activity
- [x] 11.4 Subscribe to vault_locked Tauri event
- [x] 11.5 Update vaultState when vault_locked event fires
- [x] 11.6 Test auto-lock after 5 minutes of inactivity
- [x] 11.7 Test that activity resets the auto-lock timer

## 12. Responsive Design and Styling

- [x] 12.1 Configure container max-widths with Tailwind utilities
- [x] 12.2 Implement fluid typography using clamp() or Tailwind utilities
- [x] 12.3 Test heading text scaling at 320px, 768px, 1024px, 1440px, 1920px
- [x] 12.4 Test body text scaling at all breakpoints
- [x] 12.5 Verify all buttons meet 44x44px touch target minimum
- [x] 12.6 Verify all form inputs meet 44px height minimum
- [x] 12.7 Test table to card transformation at 767px/768px breakpoint
- [x] 12.8 Verify no horizontal scroll appears at 320px viewport
- [x] 12.9 Test modal width scales to min(90vw, 500px) on mobile
- [x] 12.10 Verify all spacing scales responsively

## 13. Accessibility

- [x] 13.1 Add aria-label to all icon-only buttons
- [x] 13.2 Verify form labels have proper for attributes
- [x] 13.3 Test keyboard navigation (Tab, Shift+Tab, Enter, Escape)
- [x] 13.4 Verify focus rings are visible on all interactive elements
- [x] 13.5 Test modal focus trap behavior
- [x] 13.6 Test screen reader announces modal opening/closing
- [x] 13.7 Verify color contrast meets WCAG AA (4.5:1 for text)
- [x] 13.8 Test all interactive elements with screen reader

## 14. Error Handling

- [x] 14.1 Add error boundary component for runtime errors
- [x] 14.2 Implement user-friendly error messages for all Tauri errors
- [x] 14.3 Add toast notification system for errors
- [x] 14.4 Implement error state in uiStore
- [x] 14.5 Add error display to all forms and modals
- [x] 14.6 Test invalid PIN error handling
- [x] 14.7 Test network error handling
- [x] 14.8 Test not initialized error handling
- [x] 14.9 Verify error messages clear after action completes

## 15. Testing

- [x] 15.1 Install Vitest and @testing-library/svelte
- [x] 15.2 Create `src/lib/stores/vault.test.ts` for store tests
- [x] 15.3 Write test for vaultState initialization
- [x] 15.4 Write test for unlock action
- [x] 15.5 Write test for lock action
- [x] 15.6 Write test for filteredKeys derived store
- [x] 15.7 Create component tests for SetupScreen
- [x] 15.8 Create component tests for UnlockScreen
- [x] 15.9 Create component tests for VaultScreen
- [x] 15.10 Create component tests for KeyModal
- [x] 15.11 Create utility tests for escapeHtml function
- [x] 15.12 Test type definitions compile without errors
- [x] 15.13 Test Tauri wrapper functions with mock API
- [x] 15.14 Run all tests and verify they pass
- [x] 15.15 Set up test coverage reporting

## 16. Build and Integration

- [x] 16.1 Test `npm run build` succeeds with no errors
- [x] 16.2 Verify build output is in `.svelte-kit/output/`
- [x] 16.3 Test `cargo tauri dev` launches correctly
- [x] 16.4 Test hot module replacement works in dev mode
- [x] 16.5 Test `cargo tauri build` creates distributable
- [x] 16.6 Verify built app launches with SvelteKit UI
- [x] 16.7 Test all functionality in production build
- [x] 16.8 Verify bundle size is reasonable (<500KB gzipped)
- [x] 16.9 Check console for errors or warnings

## 17. Documentation

- [x] 17.1 Update README.md with SvelteKit architecture
- [x] 17.2 Update project structure section
- [x] 17.3 Update development setup instructions
- [x] 17.4 Add SvelteKit-specific build instructions
- [x] 17.5 Document component structure and props
- [x] 17.6 Document store usage patterns
- [x] 17.7 Update CHANGELOG.md with migration notes
- [x] 17.8 Add TypeScript usage guidelines

## 18. Cleanup and Verification

- [x] 18.1 Delete old `ui/` directory (vanilla JS implementation)
- [x] 18.2 Remove old `ui/index.html`
- [x] 18.3 Remove old `ui/app.js`
- [x] 18.4 Remove old `ui/styles.css`
- [x] 18.5 Verify git tracks new `ui-sveltekit/` directory
- [x] 18.6 Run full test suite and verify all tests pass
- [x] 18.7 Perform manual smoke test of all features
- [x] 18.8 Test responsive design at multiple viewport sizes
- [x] 18.9 Final accessibility audit
- [x] 18.10 Verify Tauri allowlist includes all commands
- [x] 18.11 Check for any remaining vanilla JS references
- [x] 18.12 Verify no console errors or warnings

## 19. Performance Optimization

- [x] 19.1 Profile bundle size with vite-bundle-visualizer
- [x] 19.2 Implement code splitting for modal components
- [x] 19.3 Add virtual scrolling for large key lists (100+)
- [x] 19.4 Optimize images and assets
- [x] 19.5 Enable Gzip compression in Tauri config
- [x] 19.6 Test startup time and optimize slow operations
- [x] 19.7 Verify smooth 60fps animations

## 20. Final Verification

- [x] 20.1 Verify all 5 capabilities are implemented
- [x] 20.2 Verify all specs requirements are met
- [x] 20.3 Verify TypeScript compiles with no implicit any
- [x] 20.4 Verify all Tauri commands have type-safe wrappers
- [x] 20.5 Verify responsive design works at all breakpoints
- [x] 20.6 Verify accessibility features work
- [x] 20.7 Verify build succeeds for development and production
- [x] 20.8 Verify old UI is removed
- [x] 20.9 Verify documentation is updated
- [x] 20.10 Verify all tests pass

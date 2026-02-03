# Tasks: SvelteKit + TypeScript Migration

## 1. Project Setup

- [ ] 1.1 Initialize SvelteKit project in `ui-sveltekit/` directory
- [ ] 1.2 Configure SvelteKit for SPA mode (disable SSR)
- [ ] 1.3 Install Tailwind CSS and configure with custom breakpoints
- [ ] 1.4 Install shadcn-svelte and run init command
- [ ] 1.5 Install required shadcn components: button, input, label, textarea, dialog, table
- [ ] 1.6 Install @tauri-apps/api package for Tauri integration
- [ ] 1.7 Configure TypeScript strict mode in tsconfig.json
- [ ] 1.8 Update Tauri config (tauri.conf.json) to point to SvelteKit build output
- [ ] 1.9 Update Tauri devUrl to point to Vite dev server (localhost:5173)
- [ ] 1.10 Test that `cargo tauri dev` launches with SvelteKit frontend

## 2. Type Definitions

- [ ] 2.1 Create `src/lib/types/api.ts` with ApiKey interface (matching Rust struct)
- [ ] 2.2 Create SessionState interface in api.ts
- [ ] 2.3 Create InitVaultArgs interface
- [ ] 2.4 Create UnlockVaultArgs interface
- [ ] 2.5 Create ChangePinArgs interface
- [ ] 2.6 Create CreateApiKeyArgs interface
- [ ] 2.7 Create UpdateApiKeyArgs interface
- [ ] 2.8 Create barrel export `src/lib/types/index.ts` re-exporting all types
- [ ] 2.9 Add JSDoc comments to all type definitions
- [ ] 2.10 Verify TypeScript compiles with no errors

## 3. Tauri Service Layer

- [ ] 3.1 Create `src/lib/services/tauri.ts` service file
- [ ] 3.2 Implement initVault wrapper function
- [ ] 3.3 Implement unlockVault wrapper function
- [ ] 3.4 Implement lockVault wrapper function
- [ ] 3.5 Implement listApiKeys wrapper function
- [ ] 3.6 Implement createApiKey wrapper function
- [ ] 3.7 Implement updateApiKey wrapper function
- [ ] 3.8 Implement deleteApiKey wrapper function
- [ ] 3.9 Implement changePin wrapper function
- [ ] 3.10 Implement getSessionState wrapper function
- [ ] 3.11 Implement isInitialized wrapper function
- [ ] 3.12 Implement copyToClipboard wrapper function
- [ ] 3.13 Add try-catch error handling to all wrapper functions
- [ ] 3.14 Add JSDoc comments to all wrapper functions
- [ ] 3.15 Test wrapper functions compile correctly

## 4. State Management (Stores)

- [ ] 4.1 Create `src/lib/stores/vault.ts` store file
- [ ] 4.2 Define VaultState interface with screen, isUnlocked, keys, searchQuery, loading, error
- [ ] 4.3 Implement vaultState writable store with initial state
- [ ] 4.4 Implement initialize action that checks if vault is initialized
- [ ] 4.5 Implement unlock action that calls unlockVault and fetches keys
- [ ] 4.6 Implement lock action that calls lockVault and resets state
- [ ] 4.7 Implement initialize action for setup screen (init_vault command)
- [ ] 4.8 Implement setSearchQuery action for search filtering
- [ ] 4.9 Implement derived store filteredKeys that filters by searchQuery
- [ ] 4.10 Add error handling to all async actions
- [ ] 4.11 Create `src/lib/stores/ui.ts` store file
- [ ] 4.12 Implement uiState store with modal visibility, loading, error state
- [ ] 4.13 Create `src/lib/stores/clipboard.ts` store file
- [ ] 4.14 Implement clipboardStore with copiedKey, copiedAt, auto-clear logic
- [ ] 4.15 Test store reactivity with Svelte 5 runes

## 5. Root Layout and Base Styles

- [ ] 5.1 Create `src/routes/+layout.svelte` root layout
- [ ] 5.2 Add dark mode provider to layout
- [ ] 5.3 Configure shadcn-svelte theming with CSS variables
- [ ] 5.4 Create `src/lib/css/app.css` with Tailwind imports
- [ ] 5.5 Add custom CSS variables for colors (primary, danger, background, text)
- [ ] 5.6 Configure Tailwind breakpoints (sm: 320px, md: 768px, lg: 1024px)
- [ ] 5.7 Add base styles and global reset
- [ ] 5.8 Add toast notification container to layout
- [ ] 5.9 Create `src/routes/+page.svelte` main page component

## 6. Authentication Screens

- [ ] 6.1 Create `src/lib/components/auth/SetupScreen.svelte`
- [ ] 6.2 Add setup form with PIN and PIN confirmation fields
- [ ] 6.3 Implement form validation (minlength 6, PINs match)
- [ ] 6.4 Connect form submit to vaultState.initialize action
- [ ] 6.5 Add error message display
- [ ] 6.6 Style form with shadcn Input, Label, Button components
- [ ] 6.7 Create `src/lib/components/auth/UnlockScreen.svelte`
- [ ] 6.8 Add unlock form with PIN field
- [ ] 6.9 Implement form submit to vaultState.unlock action
- [ ] 6.10 Add error message display for invalid PIN
- [ ] 6.11 Add loading state during unlock
- [ ] 6.12 Style unlock screen with shadcn components
- [ ] 6.13 Test setup and unlock flows end-to-end

## 7. Vault Screen Components

- [ ] 7.1 Create `src/lib/components/vault/VaultScreen.svelte`
- [ ] 7.2 Add header with title and lock button
- [ ] 7.3 Add "Add Key" button to header
- [ ] 7.4 Create `src/lib/components/vault/SearchBar.svelte`
- [ ] 7.5 Implement search input with two-way binding to vaultState
- [ ] 7.6 Add debounced search (300ms delay)
- [ ] 7.7 Create `src/lib/components/vault/KeyTable.svelte` for desktop view
- [ ] 7.8 Implement table with columns: Key Name, App Name, URL, Description, API Key, Actions
- [ ] 7.9 Add responsive hide/show (hidden on mobile, visible on desktop ≥768px)
- [ ] 7.10 Create `src/lib/components/vault/KeyCard.svelte` for mobile view
- [ ] 7.11 Implement card layout with data labels
- [ ] 7.12 Add responsive hide/show (visible on mobile <768px, hidden on desktop)
- [ ] 7.13 Create `src/lib/components/vault/KeyActions.svelte` for action buttons
- [ ] 7.14 Implement view, copy, edit, delete buttons with shadcn Button component
- [ ] 7.15 Add icon buttons with 44x44px touch targets
- [ ] 7.16 Create `src/lib/components/vault/EmptyState.svelte`
- [ ] 7.17 Implement empty state message when no keys exist
- [ ] 7.18 Implement empty state message when search returns no results
- [ ] 7.19 Connect all components to vaultState and filteredKeys store
- [ ] 7.20 Test responsive table to card transformation

## 8. Modal Components

- [ ] 8.1 Create `src/lib/components/modals/KeyModal.svelte` for add/edit
- [ ] 8.2 Implement modal with shadcn Dialog component
- [ ] 8.3 Add form fields: App Name, Key Name, API Key, API URL (optional), Description (optional)
- [ ] 8.4 Implement form validation (required fields marked)
- [ ] 8.5 Add props: open, mode (add/edit), keyData, onSave, onCancel
- [ ] 8.6 Implement save action calling createApiKey or updateApiKey
- [ ] 8.7 Add loading state during save
- [ ] 8.8 Add error message display
- [ ] 8.9 Ensure focus trap and escape key close work
- [ ] 8.10 Test modal accessibility (ARIA attributes, keyboard nav)
- [ ] 8.11 Create `src/lib/components/modals/ViewKeyModal.svelte`
- [ ] 8.12 Implement modal to view key details
- [ ] 8.13 Add show/hide password toggle for key value
- [ ] 8.14 Add copy to clipboard button
- [ ] 8.15 Add edit button that opens KeyModal in edit mode
- [ ] 8.16 Add delete button that opens DeleteModal
- [ ] 8.17 Create `src/lib/components/modals/DeleteModal.svelte`
- [ ] 8.18 Implement confirmation dialog for key deletion
- [ ] 8.19 Add "Are you sure?" message with key name
- [ ] 8.20 Implement confirm action calling deleteApiKey
- [ ] 8.21 Add cancel action to close modal
- [ ] 8.22 Test all modal interactions and edge cases

## 9. Screen Routing Logic

- [ ] 9.1 Update `src/routes/+page.svelte` with conditional rendering
- [ ] 9.2 Implement screen switching based on vaultState.screen value
- [ ] 9.3 Add transitions between screens (fade or slide)
- [ ] 9.4 Test setup → unlock → vault flow
- [ ] 9.5 Test vault → unlock (lock) flow
- [ ] 9.6 Test that only one screen displays at a time

## 10. Clipboard Integration

- [ ] 10.1 Connect copy button in ViewKeyModal to copyToClipboard service
- [ ] 10.2 Update clipboardStore with copiedKey and timestamp
- [ ] 10.3 Implement "Copied!" feedback button state
- [ ] 10.4 Implement auto-clear after 45 seconds
- [ ] 10.5 Add countdown timer to copy feedback
- [ ] 10.6 Test clipboard copy and auto-clear functionality

## 11. Activity Tracking and Auto-Lock

- [ ] 11.1 Create activity tracker utility in `src/lib/services/activity.ts`
- [ ] 11.2 Implement click listener that calls update_activity Tauri command
- [ ] 11.3 Implement keypress listener that calls update_activity
- [ ] 11.4 Subscribe to vault_locked Tauri event
- [ ] 11.5 Update vaultState when vault_locked event fires
- [ ] 11.6 Test auto-lock after 5 minutes of inactivity
- [ ] 11.7 Test that activity resets the auto-lock timer

## 12. Responsive Design and Styling

- [ ] 12.1 Configure container max-widths with Tailwind utilities
- [ ] 12.2 Implement fluid typography using clamp() or Tailwind utilities
- [ ] 12.3 Test heading text scaling at 320px, 768px, 1024px, 1440px, 1920px
- [ ] 12.4 Test body text scaling at all breakpoints
- [ ] 12.5 Verify all buttons meet 44x44px touch target minimum
- [ ] 12.6 Verify all form inputs meet 44px height minimum
- [ ] 12.7 Test table to card transformation at 767px/768px breakpoint
- [ ] 12.8 Verify no horizontal scroll appears at 320px viewport
- [ ] 12.9 Test modal width scales to min(90vw, 500px) on mobile
- [ ] 12.10 Verify all spacing scales responsively

## 13. Accessibility

- [ ] 13.1 Add aria-label to all icon-only buttons
- [ ] 13.2 Verify form labels have proper for attributes
- [ ] 13.3 Test keyboard navigation (Tab, Shift+Tab, Enter, Escape)
- [ ] 13.4 Verify focus rings are visible on all interactive elements
- [ ] 13.5 Test modal focus trap behavior
- [ ] 13.6 Test screen reader announces modal opening/closing
- [ ] 13.7 Verify color contrast meets WCAG AA (4.5:1 for text)
- [ ] 13.8 Test all interactive elements with screen reader

## 14. Error Handling

- [ ] 14.1 Add error boundary component for runtime errors
- [ ] 14.2 Implement user-friendly error messages for all Tauri errors
- [ ] 14.3 Add toast notification system for errors
- [ ] 14.4 Implement error state in uiStore
- [ ] 14.5 Add error display to all forms and modals
- [ ] 14.6 Test invalid PIN error handling
- [ ] 14.7 Test network error handling
- [ ] 14.8 Test not initialized error handling
- [ ] 14.9 Verify error messages clear after action completes

## 15. Testing

- [ ] 15.1 Install Vitest and @testing-library/svelte
- [ ] 15.2 Create `src/lib/stores/vault.test.ts` for store tests
- [ ] 15.3 Write test for vaultState initialization
- [ ] 15.4 Write test for unlock action
- [ ] 15.5 Write test for lock action
- [ ] 15.6 Write test for filteredKeys derived store
- [ ] 15.7 Create component tests for SetupScreen
- [ ] 15.8 Create component tests for UnlockScreen
- [ ] 15.9 Create component tests for VaultScreen
- [ ] 15.10 Create component tests for KeyModal
- [ ] 15.11 Create utility tests for escapeHtml function
- [ ] 15.12 Test type definitions compile without errors
- [ ] 15.13 Test Tauri wrapper functions with mock API
- [ ] 15.14 Run all tests and verify they pass
- [ ] 15.15 Set up test coverage reporting

## 16. Build and Integration

- [ ] 16.1 Test `npm run build` succeeds with no errors
- [ ] 16.2 Verify build output is in `.svelte-kit/output/`
- [ ] 16.3 Test `cargo tauri dev` launches correctly
- [ ] 16.4 Test hot module replacement works in dev mode
- [ ] 16.5 Test `cargo tauri build` creates distributable
- [ ] 16.6 Verify built app launches with SvelteKit UI
- [ ] 16.7 Test all functionality in production build
- [ ] 16.8 Verify bundle size is reasonable (<500KB gzipped)
- [ ] 16.9 Check console for errors or warnings

## 17. Documentation

- [ ] 17.1 Update README.md with SvelteKit architecture
- [ ] 17.2 Update project structure section
- [ ] 17.3 Update development setup instructions
- [ ] 17.4 Add SvelteKit-specific build instructions
- [ ] 17.5 Document component structure and props
- [ ] 17.6 Document store usage patterns
- [ ] 17.7 Update CHANGELOG.md with migration notes
- [ ] 17.8 Add TypeScript usage guidelines

## 18. Cleanup and Verification

- [ ] 18.1 Delete old `ui/` directory (vanilla JS implementation)
- [ ] 18.2 Remove old `ui/index.html`
- [ ] 18.3 Remove old `ui/app.js`
- [ ] 18.4 Remove old `ui/styles.css`
- [ ] 18.5 Verify git tracks new `ui-sveltekit/` directory
- [ ] 18.6 Run full test suite and verify all tests pass
- [ ] 18.7 Perform manual smoke test of all features
- [ ] 18.8 Test responsive design at multiple viewport sizes
- [ ] 18.9 Final accessibility audit
- [ ] 18.10 Verify Tauri allowlist includes all commands
- [ ] 18.11 Check for any remaining vanilla JS references
- [ ] 18.12 Verify no console errors or warnings

## 19. Performance Optimization

- [ ] 19.1 Profile bundle size with vite-bundle-visualizer
- [ ] 19.2 Implement code splitting for modal components
- [ ] 19.3 Add virtual scrolling for large key lists (100+)
- [ ] 19.4 Optimize images and assets
- [ ] 19.5 Enable Gzip compression in Tauri config
- [ ] 19.6 Test startup time and optimize slow operations
- [ ] 19.7 Verify smooth 60fps animations

## 20. Final Verification

- [ ] 20.1 Verify all 5 capabilities are implemented
- [ ] 20.2 Verify all specs requirements are met
- [ ] 20.3 Verify TypeScript compiles with no implicit any
- [ ] 20.4 Verify all Tauri commands have type-safe wrappers
- [ ] 20.5 Verify responsive design works at all breakpoints
- [ ] 20.6 Verify accessibility features work
- [ ] 20.7 Verify build succeeds for development and production
- [ ] 20.8 Verify old UI is removed
- [ ] 20.9 Verify documentation is updated
- [ ] 20.10 Verify all tests pass

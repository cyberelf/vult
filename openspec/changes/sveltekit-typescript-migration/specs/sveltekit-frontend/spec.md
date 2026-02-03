# Spec: SvelteKit Frontend

Capability ID: `sveltekit-frontend`

## ADDED Requirements

### Requirement: Application uses SvelteKit framework
The system SHALL use SvelteKit (Svelte 5) as the frontend framework for all UI components, replacing the existing vanilla JavaScript implementation.

#### Scenario: Framework initialization
- **WHEN** the application starts
- **THEN** SvelteKit initializes with Svelte 5 runes
- **AND** the app renders as a single-page application
- **AND** no server-side rendering occurs (SPA mode)

### Requirement: Screen routing through conditional rendering
The system SHALL manage screen transitions (setup, unlock, vault) through conditional rendering based on vault state, not URL-based routing.

#### Scenario: Display setup screen on first launch
- **WHEN** user launches app for the first time
- **AND** vault is not initialized
- **THEN** system displays SetupScreen component
- **AND** unlock and vault screens are hidden

#### Scenario: Display unlock screen when vault exists
- **WHEN** user launches app
- **AND** vault is initialized but locked
- **THEN** system displays UnlockScreen component
- **AND** setup and vault screens are hidden

#### Scenario: Display vault screen after authentication
- **WHEN** user successfully unlocks vault with correct PIN
- **THEN** system transitions to VaultScreen component
- **AND** setup and unlock screens are hidden

### Requirement: Component file organization
The system SHALL organize Svelte components into logical directories under `src/lib/components/` with separate folders for auth, vault, modals, and shared UI components.

#### Scenario: Component directory structure exists
- **WHEN** frontend codebase is initialized
- **THEN** `src/lib/components/auth/` contains authentication screens
- **AND** `src/lib/components/vault/` contains vault UI components
- **AND** `src/lib/components/modals/` contains modal dialogs
- **AND** `src/lib/components/ui/shadcn/` contains shadcn-svelte components

### Requirement: Root layout configuration
The system SHALL provide a root layout (`src/routes/+layout.svelte`) that sets up global providers, theme context, and base styles.

#### Scenario: Layout applies dark theme
- **WHEN** application renders any screen
- **THEN** root layout applies dark theme CSS classes
- **AND** Tailwind CSS dark mode is enabled
- **AND** global styles are loaded

#### Scenario: Layout provides toast notifications
- **WHEN** any component triggers a toast notification
- **THEN** root layout renders toast container
- **AND** notifications display with proper z-index

### Requirement: Build configuration with Vite
The system SHALL use Vite as the build tool with proper configuration for SvelteKit, TypeScript, and Tailwind CSS.

#### Scenario: Development server starts correctly
- **WHEN** developer runs `npm run dev`
- **THEN** Vite dev server starts on localhost:5173
- **AND** hot module replacement works
- **AND** TypeScript type checking occurs

#### Scenario: Production build succeeds
- **WHEN** developer runs `npm run build`
- **THEN** SvelteKit builds static assets to `.svelte-kit/output`
- **AND** no TypeScript errors occur
- **AND** bundle is optimized for production

### Requirement: SPA mode with no SSR
The system SHALL disable server-side rendering and operate as a pure SPA since this is a desktop application with no URL-based navigation.

#### Scenario: SSR is disabled
- **WHEN** any page component renders
- **THEN** `export const ssr = false` is set
- **AND** no server-side rendering occurs
- **AND** all rendering happens client-side

### Requirement: Error handling with error boundaries
The system SHALL implement error boundaries to catch and display runtime errors gracefully.

#### Scenario: Runtime error displays gracefully
- **WHEN** a runtime error occurs in any component
- **THEN** error boundary catches the error
- **AND** user-friendly error message displays
- **AND** application does not crash

### Requirement: Responsive viewport meta tag
The system SHALL include proper viewport meta tag for responsive design across different window sizes.

#### Scenario: Viewport meta tag is present
- **WHEN** application HTML loads
- **THEN** `<meta name="viewport" content="width=device-width, initial-scale=1">` is present
- **AND** viewport scales correctly on different screen sizes

# Spec: Component Library

Capability ID: `component-library`

## ADDED Requirements

### Requirement: shadcn-svelte component integration
The system SHALL integrate shadcn-svelte component library using the copy-paste model, placing components in `src/lib/components/ui/shadcn/`.

#### Scenario: Required shadcn components are installed
- **WHEN** developer runs shadcn-svelte init
- **THEN** Button component is installed
- **AND** Input component is installed
- **AND** Label component is installed
- **AND** Textarea component is installed
- **AND** Dialog component is installed
- **AND** Table component is installed
- **AND** components are located in `src/lib/components/ui/shadcn/`

#### Scenario: Components can be imported and used
- **WHEN** Svelte file imports Button component
- **THEN** import path is `$lib/components/ui/shadcn/button`
- **AND** component renders correctly
- **AND** TypeScript types are available

### Requirement: Dark mode theming
The system SHALL configure shadcn-svelte components with dark mode as the default theme using CSS variables and Tailwind dark mode.

#### Scenario: All components use dark theme
- **WHEN** any shadcn component renders
- **THEN** dark mode CSS classes are applied
- **AND** colors use CSS variables defined in app.css
- **AND** components match dark theme color scheme

#### Scenario: Theme colors are consistent
- **WHEN** multiple shadcn components are displayed
- **THEN** all components use same color palette
- **AND** primary color is consistent (#2563eb)
- **AND** background color is consistent (#0f172a)
- **AND** text color is consistent (#f1f5f9)

### Requirement: Tailwind CSS configuration
The system SHALL configure Tailwind CSS with custom breakpoints, colors, and spacing that match the existing design system.

#### Scenario: Tailwind breakpoints are defined
- **WHEN** Tailwind config is loaded
- **THEN** sm breakpoint is 320px
- **AND** md breakpoint is 768px
- **AND** lg breakpoint is 1024px
- **AND** responsive utilities work correctly

#### Scenario: Custom colors are defined
- **WHEN** Tailwind config is loaded
- **THEN** primary color matches existing CSS variable
- **AND** danger color matches existing CSS variable
- **AND** background colors match existing theme
- **AND** text colors match existing theme

### Requirement: Responsive table to card transformation
The system SHALL implement a table component that displays as a table on desktop (≥768px) and transforms to stacked cards on mobile (<768px).

#### Scenario: Table displays on desktop
- **WHEN** viewport width is ≥768px
- **AND** keys array has data
- **THEN** table displays with headers (Key Name, App Name, URL, Description, Actions)
- **AND** rows display key data
- **AND** actions column contains action buttons

#### Scenario: Cards display on mobile
- **WHEN** viewport width is <768px
- **AND** keys array has data
- **THEN** table is hidden
- **AND** cards display in stacked layout
- **AND** each card shows one key's data
- **AND** card sections have labels (Key Name:, App Name:, etc.)

#### Scenario: Transformation is smooth
- **WHEN** viewport crosses 768px threshold
- **THEN** layout transitions between table and cards
- **AND** no horizontal scroll appears
- **AND** all content remains accessible

### Requirement: Modal components with accessibility
The system SHALL implement modal components (KeyModal, ViewKeyModal, DeleteModal) using shadcn-svelte Dialog with proper accessibility features.

#### Scenario: Modal opens with focus trap
- **WHEN** modal opens
- **THEN** focus is trapped within modal
- **AND** tab key cycles focus within modal
- **AND** focus moves to first interactive element

#### Scenario: Modal closes on escape key
- **WHEN** user presses Escape key
- **THEN** modal closes
- **AND** focus returns to trigger element

#### Scenario: Modal has ARIA attributes
- **WHEN** modal is open
- **THEN** role="dialog" is present
- **AND** aria-modal="true" is set
- **AND** aria-label describes modal purpose

### Requirement: Form validation with visual feedback
The system SHALL provide form validation with native HTML5 validation and visual error states using shadcn components.

#### Scenario: Required fields show validation
- **WHEN** user submits form without required fields
- **THEN** browser validation prevents submission
- **AND** required fields show error state
- **AND** error messages display below invalid fields

#### Scenario: PIN confirmation matches
- **WHEN** user enters PIN and confirmation
- **AND** PINs do not match
- **THEN** validation error displays "PINs do not match"
- **AND** form submission is prevented

#### Scenario: Minimum PIN length validation
- **WHEN** user enters PIN with <6 characters
- **THEN** validation error displays "PIN must be at least 6 characters"
- **AND** form submission is prevented

### Requirement: Touch targets meet minimum size
The system SHALL ensure all interactive elements meet WCAG 2.5.5 AAA minimum touch target size of 44x44px.

#### Scenario: Buttons are 44px minimum
- **WHEN** any button component renders
- **THEN** button has min-height of 44px
- **AND** button has min-width of 44px
- **AND** touch target is adequate for mobile

#### Scenario: Icon buttons are 44px minimum
- **WHEN** icon-only button renders (e.g., edit, delete)
- **THEN** button has padding to achieve 44x44px size
- **AND** icon is centered in touch target
- **AND** button is easily tappable

#### Scenario: Form inputs are 44px minimum
- **WHEN** text input or password input renders
- **THEN** input has min-height of 44px
- **AND** input is easily tappable on mobile

### Requirement: Fluid typography with clamp()
The system SHALL use Tailwind's fluid typography or custom clamp() utilities for smooth text scaling across viewport sizes.

#### Scenario: Heading text scales smoothly
- **WHEN** viewport width changes from 320px to 1920px
- **THEN** h1 text scales between 28px and 48px
- **AND** h2 text scales between 24px and 36px
- **AND** h3 text scales between 20px and 28px
- **AND** scaling is smooth with no discrete jumps

#### Scenario: Body text scales smoothly
- **WHEN** viewport width changes
- **THEN** body text scales between 16px and 18px
- **AND** small text scales between 14px and 16px
- **AND** text remains readable at all sizes

### Requirement: Responsive container widths
The system SHALL use responsive container widths with max-width constraints that adapt to viewport size.

#### Scenario: Vault container scales
- **WHEN** viewport width changes
- **THEN** vault container max-width scales between 400px and 1200px
- **AND** container uses 90vw on small screens
- **AND** container is centered with horizontal margin

#### Scenario: Auth container scales
- **WHEN** viewport width changes
- **THEN** setup/unlock container max-width scales between 320px and 600px
- **AND** container uses 80vw on small screens
- **AND** container is centered

### Requirement: Component prop interfaces
The system SHALL define TypeScript interfaces for all component props, ensuring type-safe component usage.

#### Scenario: Button props are typed
- **WHEN** Button component is used
- **THEN** props interface defines variant, size, disabled, onclick
- **AND** TypeScript validates prop values
- **AND** invalid variant causes compile error

#### Scenario: Modal props are typed
- **WHEN** KeyModal component is used
- **THEN** props interface defines open, onSave, onCancel, mode (add/edit)
- **AND** TypeScript validates callback signatures
- **AND** missing required prop causes compile error

### Requirement: Loading states and disabled interactions
The system SHALL provide loading states that disable user interactions during async operations.

#### Scenario: Button shows loading state
- **WHEN** form submission is in progress
- **THEN** submit button shows loading spinner
- **AND** button is disabled
- **AND** button cannot be clicked

#### Scenario: Form inputs disabled during save
- **WHEN** API key is being saved
- **THEN** all form inputs are disabled
- **AND** user cannot modify data
- **AND** focus is trapped until save completes

### Requirement: Empty state components
The system SHALL provide empty state components that display helpful messages when no data is available.

#### Scenario: Empty keys list shows message
- **WHEN** vault has no API keys
- **THEN** empty state component displays
- **AND** message says "No API keys yet. Add your first key to get started."
- **AND** "Add Key" button is prominent

#### Scenario: Search results empty shows message
- **WHEN** search returns no results
- **THEN** empty state component displays
- **AND** message says "No keys match your search."
- **AND** clear search option is available

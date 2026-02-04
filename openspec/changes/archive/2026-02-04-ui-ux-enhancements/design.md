## Context

Vult's current frontend is functional but feels like a web page wrapped in a window. The `KeyTable` component is rigid, hiding completely on small screens in favor of a separate mobile card view which is disjointed. The styling uses generic web patterns rather than desktop application conventions (e.g., density, typography, contrast). Editing requires a modal, which interrupts the workflow for minor changes like updating a description.

The application uses SvelteKit with Tailwind CSS. We need to leverage this stack to create a more responsive, native-feeling experience without introducing heavy UI component libraries that might bloat the secure, minimal nature of the app.

## Goals / Non-Goals

**Goals:**
- **Fluid Layout**: Application UI that gracefully adapts to any window size, from a small "widget" floating window to full screen.
- **Native Aesthetic**: Visual design that feels at home on Windows, macOS, and Linux (high density, system fonts, clear hierarchy).
- **Frictionless Editing**: Ability to edit key details (name, description, URL) directly in the list view without context switching.
- **Keyboard Navigation**: Full support for navigating and editing via keyboard.

**Non-Goals:**
- **Platform Specific mimicking**: We will not try to recreate exact OS controls (e.g., Fluent UI or Aqua) pixel-for-pixel, but rather aim for a high-quality neutral desktop theme.
- **Mobile Support**: While "responsive", the focus is on desktop window resizing, not touch targets for mobile devices.

## Decisions

### 1. CSS Grid & Container Queries for Layout
We will move away from simple media queries (`md:table`) and adopt a CSS Grid based shell layout combined with Container Queries for the data display.
- **Why?**: Media queries respond to the *viewport*, but in a desktop app, the available space for the content might be constrained by a sidebar or split pane. Container queries allow the `KeyTable` to switch visualization modes based on *its* own width, not the window width.
- **Alternatives**: pure Flexbox (harder to align 2D grids), JS-based resize observers (performance cost).

### 2. Semantic CSS Variable Theme System
We will refactor the Tailwind config to use semantic CSS variables (e.g., `--app-bg`, `--surface-elevated`, `--text-primary`) rather than direct utility colors.
- **Why?**: Enables easier theming and potential future support for "System" theme syncing (Light/Dark auto-detection) and high-contrast modes.
- **Structure**:
    ```css
    :root {
        --app-font-family: system-ui, -apple-system, sans-serif;
        --grid-spacing: 0.5rem;
    }
    ```

### 3. Component-Based Inline Editing State
Inline editing state will be creating a specialized `<EditableCell>` component.
- **Why?**: Keeps the complicated state of "view vs edit", input refs, and validation logic encapsulated. The main table just passes data and an `on:save` handler.
- **Experience**: Click-to-edit or Focus+Enter to edit. Escape to cancel, Enter/Blur to save.

### 4. Custom Titlebar Integration
We will refine the integration with Tauri's custom titlebar (if applicable) or window controls to ensure the header area feels integrated with the content.
- **Why?**: Standard web headers feel disconnected. A proper desktop app usually merges the title bar area with the toolbar.

## Risks / Trade-offs

- **Risk: Inline Edit Usability**
    - Users might accidentally edit data or not know how to save.
    - **Mitigation**: Visual cues (hover borders), explicit "save/cancel" icons on the active cell, and toast notifications on success.

- **Risk: Table Performance**
    - Inline edit components might add overhead if we render hundreds of keys.
    - **Mitigation**: Since this is a personal vault, key count is likely low (<100). If it grows, we can implement virtualization, but it's premature now.

- **Trade-off: Density vs Touch**n- We are prioritizing high information density (desktop mouse/keyboard) over touch-friendliness. This is an explicit choice for a developer tool.

## 1. Foundation & Theme

- [x] 1.1 Update `tailwind.config.js` to use CSS variable references for colors (semantic naming)
- [x] 1.2 Define CSS root variables in `ui-sveltekit/src/app.css` for background, surface, text, and borders
- [x] 1.3 Update global font stack to use system-native fonts (Segoe UI, San Francisco, etc.)
- [x] 1.4 Refactor main `app.html` / layout to use CSS Grid for the application shell

## 2. Layout & Responsiveness

- [x] 2.1 Enable `@tailwindcss/container-queries` plugin in Tailwind config
- [x] 2.2 Refactor `KeyTable.svelte` wrapper to be a query container
- [x] 2.3 Implement "Card View" styles using `@container` queries for narrow widths (<500px)
- [x] 2.4 Implement "Table View" styles using `@container` queries for wide widths (>600px)
- [x] 2.5 Verify and fix layout behavior at minimum width (350px)

## 3. Inline Editing Components

- [x] 3.1 Create `ui-sveltekit/src/lib/components/vault/EditableCell.svelte` component
- [x] 3.2 Implement view/edit mode toggling logic (click-to-edit, focus handling)
- [x] 3.3 Implement input handling: Enter to save, Escape to cancel, Blur behavior
- [x] 3.4 Add visual indicators for edit mode (borders, save/cancel icons)

## 4. Feature Integration

- [x] 4.1 Update `vault.ts` store (or creating a new slice) to handle checking/saving key updates
- [x] 4.2 Replace static text cells in `KeyTable.svelte` with `EditableCell` instances for Name, URL, and Description
- [x] 4.3 Integrate `EditableCell` into the "Card View" layout as well
- [x] 4.4 Remove old modal-based editing code/components if no longer needed
- [x] 4.5 Verify keyboard navigation (Tab through fields, Enter to edit) across the table

## 5. Polish & Fixes

- [x] 5.1 Relax layout density: increase table padding (p-4 to p-5 or p-6) and gap spacing
- [x] 5.2 Implement inline "View" mode toggle (expand row) instead of modal for viewing details
- [x] 5.3 Ensure `KeyActions` view button toggles the expanded row state
- [x] 5.4 Verify responsive behavior with new spacing adjustments

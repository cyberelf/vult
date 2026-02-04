## Why

The current UI has critical usability issues that undermine the desktop application experience: the layout doesn't adapt properly to window sizing, the visual design doesn't feel native to desktop platforms, and the modal-based workflow for editing API keys breaks user flow. These issues create friction and make the application feel like a ported web app rather than a native desktop tool.

## What Changes

- **Responsive Layout System**: Implement a flexible layout that properly fits and adapts to window size changes
- **Desktop-Native Theme**: Redesign the visual language to match native desktop application patterns (Windows/macOS/Linux)
- **Inline Table Editing**: Replace modal-based editing with inline editing directly in the API keys table for improved fluidity
- **Native Window Integration**: Better integration with Tauri's desktop window APIs for proper sizing and behavior

## Capabilities

### New Capabilities
- `responsive-layout`: Layout system that adapts to window dimensions and maintains usability at different sizes
- `desktop-theme`: Visual design system that matches native desktop platform conventions
- `inline-editing`: In-place editing capabilities for table cells, eliminating modal interruptions

### Modified Capabilities
- None (this is an enhancement initiative without breaking API changes)

## Impact

- **UI Components**: Major updates to SvelteKit components ([KeyTable.svelte](ui-sveltekit/src/lib/components/vault/KeyTable.svelte), [EmptyState.svelte](ui-sveltekit/src/lib/components/vault/EmptyState.svelte), [KeyActions.svelte](ui-sveltekit/src/lib/components/vault/KeyActions.svelte))
- **Styling System**: Refactor CSS/styling approach to support desktop-native theming
- **Tauri Integration**: Enhanced use of Tauri window APIs for proper desktop behavior
- **User Interaction**: Complete redesign of the edit/view workflows to be inline rather than modal-based
- **State Management**: Updates to [vault store](ui-sveltekit/src/lib/stores/vault.ts) to support inline editing state

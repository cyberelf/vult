# Add API Key Secret Controls

## Why

Users adding an API key sometimes need a secure random secret and need to verify the value they entered before saving it.

## What Changes

- Add a key icon inside the API Key input that generates a cryptographically secure URL-safe secret in add mode.
- Add an eye icon that toggles whether the API Key input is masked.
- Keep generated secrets in the existing in-memory form state only.

## Impact

- Affected code: `KeyModal.svelte`, frontend utilities and tests.
- Dependencies: browser/WebView Web Crypto API and the existing `lucide-svelte` package.
- Backend, database schema, and Tauri IPC are unchanged.

## ADDED Requirements

### Requirement: Semantic Theme Variables
The application styling SHALL be driven by a set of semantic CSS variables (e.g., `--bg-surface`, `--text-primary`) rather than hardcoded hex values or utility-classes-only approaches, enabling consistent theming.

#### Scenario: Theming application
- **WHEN** the value of `--bg-surface` is changed in the root definition
- **THEN** all surface backgrounds in the application update to reflect the new color
- **AND** no hardcoded generic background colors remain in standard components

### Requirement: Desktop Density
The UI density (padding, font sizes, margins) SHALL match standard desktop application patterns, utilizing tighter spacing than the previous web-like layout.

#### Scenario: Visual density
- **WHEN** displaying the key table
- **THEN** reasonable number of rows (e.g., 10+) are visible on a standard 1080p screen without scrolling
- **AND** interactive touch targets are sized for mouse cursors (approx 24-32px) rather than touch (44px+)

### Requirement: System Fonts
The application SHALL use the operating system's default sans-serif font stack (Segoe UI on Windows, San Francisco on macOS, etc.) to blend in with the native environment.

#### Scenario: Font rendering
- **WHEN** the application is loaded on Windows
- **THEN** the text acts and looks like "Segoe UI"
- **WHEN** the application is loaded on macOS
- **THEN** the text acts and looks like "San Francisco" / system-ui

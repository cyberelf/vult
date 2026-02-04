## ADDED Requirements

### Requirement: Adaptive Grid Layout
The application main view SHALL utilize a CSS Grid-based shell that dynamically adjusts spacing and distribution based on the window size.

#### Scenario: Window resize
- **WHEN** the window is resized from a small size (300px width) to a large size (1200px width)
- **THEN** the main content area expands to fill available space
- **AND** margins/padding increase proportionally to maintain visual balance
- **AND** no horizontal scrollbars appear for the main page structure

### Requirement: Container-Aware Key List
The "key list" component SHALL use CSS Container Queries to switch between "Card View" (stacked details) and "Table View" (columns) based on the specific width available to the component, independent of the viewport width.

#### Scenario: Narrow container
- **WHEN** the component container width is less than ~500px
- **THEN** each API key entry renders as a vertical card
- **AND** the label and value appear stacked

#### Scenario: Wide container
- **WHEN** the component container width is greater than ~600px
- **THEN** the API key entries render as a single row in a table-like layout
- **AND** columns align vertically across entries

### Requirement: Minimum Viable Width
The UI SHALL remain fully functional and readable at a minimum window width of 350px, ensuring the "widget mode" use case is supported.

#### Scenario: Minimum width view
- **WHEN** the window is resized to 350px width
- **THEN** all critical actions (View Key, Copy, Edit) remain accessible
- **AND** text does not overlap or truncate in a way that obscures critical data

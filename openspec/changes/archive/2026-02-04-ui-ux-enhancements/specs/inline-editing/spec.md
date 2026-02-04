## ADDED Requirements

### Requirement: Inline Editing
The "Key Table" SHALL support editing of `Key Name`, `Description`, and `API URL` fields directly within the table/list presentation, without opening a modal dialog.

#### Scenario: Activation of edit mode
- **WHEN** a user clicks on an editable field (or focuses and presses Enter)
- **THEN** the text display is replaced by an input field
- **AND** the input field is automatically focused
- **AND** the row enters an "editing" state showing Save/Cancel options

### Requirement: Edit State Preservation
The application SHALL track the edit state of each row independently, allowing multiple rows to be in "edit mode" simultaneously, or ensuring one cancels the other (single-row edit is preferred for simplicity).

#### Scenario: Committing changes
- **WHEN** a user presses "Enter" in an active input field (or clicks a Save icon)
- **THEN** the new value is validated
- **WHEN** valid
- **THEN** the value is saved to the store
- **AND** the field reverts to "view" mode with the updated value

### Requirement: Edit Cancellation
Users SHALL be able to discard pending changes easily.

#### Scenario: Cancelling edit
- **WHEN** a user presses "Escape" in an active input field
- **THEN** the field reverts to "view" mode
- **AND** the original value is restored
- **AND** no changes are persisted

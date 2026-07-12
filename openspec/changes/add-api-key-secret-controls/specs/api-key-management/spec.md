## ADDED Requirements

### Requirement: API Key Secret Input Controls
The add-key form SHALL help users securely generate and review API key values before saving.

#### Scenario: Generate a random secret
- **WHEN** the user activates the random-secret control in the add-key form
- **THEN** the API Key input SHALL be populated with a cryptographically secure, URL-safe secret
- **AND** the generated value SHALL remain in the form's in-memory state until submitted or discarded

#### Scenario: Review an API key value
- **WHEN** the user activates the review control beside the API Key input
- **THEN** the input SHALL reveal its current value
- **AND** activating the control again SHALL mask the value

#### Scenario: Accessible controls
- **WHEN** assistive technology identifies either secret input control
- **THEN** the control SHALL expose an action-specific accessible label

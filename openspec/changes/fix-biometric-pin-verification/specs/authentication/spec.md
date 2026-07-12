## MODIFIED Requirements

### Requirement: Biometric Credential Enrollment
Biometric credential enrollment SHALL validate a PIN through the same secure verification path used for normal vault unlock before storing it with the platform credential provider.

#### Scenario: Enroll with the correct PIN
- **WHEN** an unlocked user enables biometric storage with the vault's correct PIN
- **THEN** the PIN SHALL be verified using the current full-hash authentication path
- **AND** the credential SHALL be stored using the platform-native secure store

#### Scenario: Reject an incorrect PIN
- **WHEN** an unlocked user attempts biometric enrollment with an incorrect PIN
- **THEN** enrollment SHALL fail with an authentication error
- **AND** the incorrect PIN SHALL NOT be stored

#### Scenario: Legacy verification compatibility
- **WHEN** biometric enrollment encounters a supported legacy PIN verification format
- **THEN** the canonical authentication path SHALL verify and migrate it using the same behavior as normal unlock

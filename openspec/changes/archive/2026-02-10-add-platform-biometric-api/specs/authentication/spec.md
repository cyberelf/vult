## MODIFIED Requirements

### Requirement: Biometric Authentication
The system SHALL provide optional Windows Hello biometric authentication for unlocking an initialized vault on Windows 10 (1903+) and Windows 11, using Windows.Security.Credentials.UI APIs and never persisting biometric data.

#### Scenario: Windows Hello availability check
- **WHEN** the app requests biometric availability on Windows
- **THEN** the library SHALL query UserConsentVerifier.CheckAvailabilityAsync
- **AND** the response SHALL indicate Available, NotConfigured, DeviceNotPresent, or NotSupported
- **AND** non-Windows platforms SHALL return NotSupported

#### Scenario: Successful Windows Hello unlock
- **WHEN** a user initiates biometric unlock on a Windows Hello-enabled device
- **AND** Windows Hello verification succeeds
- **THEN** the vault SHALL unlock and start a session

#### Scenario: Fallback to PIN
- **WHEN** Windows Hello is unavailable or the biometric attempt fails
- **THEN** the system SHALL allow the user to unlock with PIN
- **AND** the biometric failure SHALL NOT prevent PIN authentication
- **AND** non-Windows platforms SHALL always use PIN authentication

#### Scenario: Opt-in control
- **WHEN** the user disables Windows Hello in settings
- **THEN** the system SHALL not attempt biometric unlock
- **AND** the system SHALL require PIN for unlock

#### Scenario: Error mapping
- **WHEN** a Windows Hello attempt fails, is cancelled, or times out
- **THEN** the library SHALL return AuthError::BiometricFailed
- **AND** the error SHALL map all non-Verified results to the same error
- **AND** the error SHALL NOT expose Windows API-specific details

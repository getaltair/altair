## ADDED Requirements

### Requirement: Stack-Based Navigation with Decompose

The system SHALL use Decompose for stack-based navigation across all platforms.

#### Scenario: Navigation stack maintains history

- **WHEN** the user navigates to a new destination
- **THEN** the previous destination is pushed onto the back stack
- **AND** the new destination is displayed

#### Scenario: Back navigation pops stack

- **WHEN** the user performs a back action (gesture, button, or ESC key)
- **THEN** the current destination is removed from the stack
- **AND** the previous destination is restored

### Requirement: RootComponent as Navigation Root

The system SHALL have a RootComponent that owns the navigation stack and serves as the entry point for all navigation.

#### Scenario: RootComponent initializes with Home

- **WHEN** the application starts
- **THEN** the RootComponent creates a childStack with Home as the initial configuration
- **AND** the Home screen is displayed

#### Scenario: RootComponent survives configuration changes

- **WHEN** the device configuration changes (rotation, dark mode)
- **THEN** the RootComponent and its navigation state are preserved
- **AND** the current screen state is maintained

### Requirement: Type-Safe Navigation Destinations

Navigation destinations SHALL be defined as a sealed class hierarchy for compile-time safety.

#### Scenario: Config sealed class defines destinations

- **WHEN** a new screen is added to the application
- **THEN** a new Config subclass is added to represent that destination
- **AND** the compiler ensures exhaustive handling of all destinations

#### Scenario: Destinations with parameters

- **WHEN** a destination requires parameters (e.g., item ID)
- **THEN** the Config data class includes those parameters as properties
- **AND** the parameters are preserved across configuration changes

### Requirement: Platform-Native Back Handling

The navigation system SHALL integrate with platform-native back gestures and controls.

#### Scenario: Android predictive back gesture

- **WHEN** the user performs a predictive back gesture on Android
- **THEN** the navigation system handles the gesture natively
- **AND** the previous destination is shown during the gesture preview

#### Scenario: iOS swipe-to-go-back

- **WHEN** the user swipes from the left edge on iOS
- **THEN** the navigation system pops the current destination
- **AND** the transition animates smoothly

#### Scenario: Desktop ESC key

- **WHEN** the user presses ESC on Desktop
- **THEN** the navigation system handles it as a back action
- **AND** the previous destination is restored (if available)

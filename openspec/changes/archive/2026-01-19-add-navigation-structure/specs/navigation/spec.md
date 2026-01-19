## ADDED Requirements

### Requirement: Main Navigation Destinations

The system SHALL provide navigation destinations for the five primary areas: Home, Guidance, Knowledge, Tracking, and Settings.

#### Scenario: Config sealed class includes all destinations

- **WHEN** a developer needs to navigate to a primary area
- **THEN** they can use the corresponding Config entry (Config.Home, Config.Guidance, Config.Knowledge, Config.Tracking, Config.Settings)
- **AND** the compiler enforces exhaustive handling of all destinations

#### Scenario: Navigate to Guidance

- **WHEN** the user selects the Guidance destination
- **THEN** the GuidanceScreen is displayed
- **AND** the Guidance navigation item is visually selected

#### Scenario: Navigate to Knowledge

- **WHEN** the user selects the Knowledge destination
- **THEN** the KnowledgeScreen is displayed
- **AND** the Knowledge navigation item is visually selected

#### Scenario: Navigate to Tracking

- **WHEN** the user selects the Tracking destination
- **THEN** the TrackingScreen is displayed
- **AND** the Tracking navigation item is visually selected

#### Scenario: Navigate to Settings

- **WHEN** the user selects the Settings destination
- **THEN** the SettingsScreen is displayed
- **AND** the Settings navigation item is visually selected

### Requirement: Platform-Adaptive Navigation Shell

The system SHALL display a platform-appropriate navigation UI that adapts based on screen size and platform conventions.

#### Scenario: Mobile bottom navigation bar

- **WHEN** the application runs on a mobile device or compact window (width < 600dp)
- **THEN** a bottom navigation bar is displayed with icons for all 5 destinations
- **AND** the bar remains visible while navigating between destinations
- **AND** the current destination's icon is highlighted

#### Scenario: Desktop side navigation rail

- **WHEN** the application runs on desktop or expanded window (width >= 600dp)
- **THEN** a vertical navigation rail is displayed on the left side
- **AND** the rail shows icons (and optionally labels) for all 5 destinations
- **AND** the Settings item appears at the bottom of the rail

#### Scenario: Window resize triggers layout change

- **WHEN** a desktop window is resized across the 600dp threshold
- **THEN** the navigation UI transitions between bottom bar and side rail
- **AND** the current destination is preserved

### Requirement: Navigation Shell Integration with Auth Flow

The navigation shell SHALL only be displayed when the user is authenticated.

#### Scenario: Authenticated user sees navigation shell

- **WHEN** a user is authenticated (AuthState.Authenticated)
- **THEN** the main navigation shell is displayed with Home as the initial destination
- **AND** the navigation bar/rail is visible

#### Scenario: Unauthenticated user sees auth flow

- **WHEN** a user is not authenticated (AuthState.Unauthenticated)
- **THEN** the Login screen is displayed without the navigation shell
- **AND** no bottom bar or side rail is visible

### Requirement: Home/Today Screen

The system SHALL provide a Home screen that serves as the landing page after authentication.

#### Scenario: Home displays welcome content

- **WHEN** the Home destination is selected
- **THEN** a screen is displayed with the title "Today" or "Home"
- **AND** placeholder content indicating where daily summary will appear

#### Scenario: Home is default after login

- **WHEN** a user successfully authenticates
- **THEN** they are navigated to the Home destination
- **AND** the Home icon is selected in the navigation

### Requirement: Guidance Module Placeholder Screen

The system SHALL provide a placeholder Guidance screen until the full module is implemented.

#### Scenario: Guidance placeholder displays module info

- **WHEN** the Guidance destination is selected
- **THEN** a screen is displayed with "Guidance" as the title
- **AND** an icon representing task/quest management
- **AND** text describing the module's purpose (quest-based task execution)
- **AND** an indicator that full functionality is coming

### Requirement: Knowledge Module Placeholder Screen

The system SHALL provide a placeholder Knowledge screen until the full module is implemented.

#### Scenario: Knowledge placeholder displays module info

- **WHEN** the Knowledge destination is selected
- **THEN** a screen is displayed with "Knowledge" as the title
- **AND** an icon representing notes/documents
- **AND** text describing the module's purpose (personal knowledge management)
- **AND** an indicator that full functionality is coming

### Requirement: Tracking Module Placeholder Screen

The system SHALL provide a placeholder Tracking screen until the full module is implemented.

#### Scenario: Tracking placeholder displays module info

- **WHEN** the Tracking destination is selected
- **THEN** a screen is displayed with "Tracking" as the title
- **AND** an icon representing inventory/items
- **AND** text describing the module's purpose (inventory and asset management)
- **AND** an indicator that full functionality is coming

### Requirement: Settings Placeholder Screen

The system SHALL provide a placeholder Settings screen until the full settings are implemented.

#### Scenario: Settings placeholder displays configuration options

- **WHEN** the Settings destination is selected
- **THEN** a screen is displayed with "Settings" as the title
- **AND** an icon representing settings/configuration
- **AND** text indicating where account, theme, and preferences will appear

### Requirement: Navigation Keyboard Accessibility

The navigation system SHALL support keyboard navigation on desktop platforms.

#### Scenario: Tab navigation through rail items

- **WHEN** a user presses Tab on desktop
- **THEN** focus moves between navigation rail items
- **AND** focused items display a visible focus indicator

#### Scenario: Enter/Space activates navigation item

- **WHEN** a navigation rail item is focused and user presses Enter or Space
- **THEN** that destination is selected
- **AND** the corresponding screen is displayed

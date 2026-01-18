# Design System Capability

The Altair Design System provides consistent, accessible UI components across all platforms using Compose Unstyled primitives with custom styling.

## ADDED Requirements

### Requirement: Design Token System

The system SHALL provide a centralized `AltairTheme` object containing design tokens for colors, typography, spacing, and border radii that can be accessed statically throughout the application.

#### Scenario: Access color tokens

- **WHEN** a component needs the primary accent color
- **THEN** it accesses `AltairTheme.Colors.accent` which returns `Color(0xFF6366F1)`

#### Scenario: Access typography tokens

- **WHEN** a component needs headline styling
- **THEN** it accesses `AltairTheme.Typography.headlineMedium` which returns a `TextStyle` with 20sp font size and medium weight

#### Scenario: Access spacing tokens

- **WHEN** a component needs standard padding
- **THEN** it accesses `AltairTheme.Spacing.md` which returns `16.dp`

### Requirement: Color Palette

The system SHALL provide a dark-first color palette with the following semantic categories: backgrounds (background, surface, surfaceElevated, surfaceHover), borders (border, borderFocused), text (textPrimary, textSecondary, textTertiary), accent colors (accent, accentHover), and status colors (success, warning, error).

#### Scenario: Dark background hierarchy

- **WHEN** rendering the application background
- **THEN** `AltairTheme.Colors.background` provides near-black (`0xFF0A0A0B`)
- **AND** `AltairTheme.Colors.surface` provides slightly lighter (`0xFF141415`)
- **AND** `AltairTheme.Colors.surfaceElevated` provides card/dialog background (`0xFF1C1C1E`)

#### Scenario: Text contrast hierarchy

- **WHEN** rendering text content
- **THEN** `AltairTheme.Colors.textPrimary` provides high-contrast white-ish (`0xFFEEEEEF`)
- **AND** `AltairTheme.Colors.textSecondary` provides muted gray (`0xFF8E8E93`)
- **AND** `AltairTheme.Colors.textTertiary` provides disabled/hint gray (`0xFF636366`)

### Requirement: Theme Provider

The system SHALL provide an `AltairThemeProvider` composable that establishes theme context for descendant components, enabling optional runtime theme customization via CompositionLocal.

#### Scenario: Wrap application with theme

- **WHEN** the application root composable is wrapped with `AltairThemeProvider`
- **THEN** all descendant components can access theme values
- **AND** CompositionLocal overrides are respected for dynamic theming

### Requirement: Focus Ring Accessibility

The system SHALL provide a `Modifier.focusRing()` extension that renders a visible focus indicator when a component receives keyboard focus, using `AltairTheme.Colors.borderFocused` as the ring color.

#### Scenario: Button receives keyboard focus

- **WHEN** a user tabs to an `AltairButton`
- **THEN** a visible focus ring appears around the button using the `borderFocused` color
- **AND** the ring disappears when focus moves away

#### Scenario: Focus ring on text field

- **WHEN** a user clicks or tabs into an `AltairTextField`
- **THEN** a visible focus ring appears around the input
- **AND** the ring remains visible while the field has focus

### Requirement: AltairButton Component

The system SHALL provide an `AltairButton` composable that wraps Compose Unstyled `Button`, supporting variants (Primary, Secondary, Ghost, Danger), enabled/disabled states, and keyboard accessibility.

#### Scenario: Render primary button

- **WHEN** an `AltairButton` with `variant = ButtonVariant.Primary` is rendered
- **THEN** it displays with accent background color (`AltairTheme.Colors.accent`)
- **AND** primary text color
- **AND** rounded corners using `AltairTheme.Radii.md`

#### Scenario: Render secondary button

- **WHEN** an `AltairButton` with `variant = ButtonVariant.Secondary` is rendered
- **THEN** it displays with surface background color
- **AND** a visible border using `AltairTheme.Colors.border`

#### Scenario: Render ghost button

- **WHEN** an `AltairButton` with `variant = ButtonVariant.Ghost` is rendered
- **THEN** it displays with transparent background
- **AND** shows background tint on hover

#### Scenario: Render danger button

- **WHEN** an `AltairButton` with `variant = ButtonVariant.Danger` is rendered
- **THEN** it displays with error color background (`AltairTheme.Colors.error`)

#### Scenario: Disabled button state

- **WHEN** an `AltairButton` has `enabled = false`
- **THEN** it displays with reduced opacity
- **AND** does not respond to click events

### Requirement: AltairTextField Component

The system SHALL provide an `AltairTextField` composable that wraps Compose Unstyled `TextField`, supporting labels, placeholder text, error states, and keyboard accessibility.

#### Scenario: Render text field with label

- **WHEN** an `AltairTextField` with `label = "Email"` is rendered
- **THEN** the label appears above the input
- **AND** the input has surface background with border

#### Scenario: Text field error state

- **WHEN** an `AltairTextField` has `isError = true` and `errorMessage = "Invalid email"`
- **THEN** the border changes to error color (`AltairTheme.Colors.error`)
- **AND** the error message appears below the input in error color

#### Scenario: Text field placeholder

- **WHEN** an `AltairTextField` with `placeholder = "Enter your email"` is empty
- **THEN** the placeholder text appears in tertiary text color
- **AND** disappears when the user types

### Requirement: AltairCard Component

The system SHALL provide an `AltairCard` composable that renders a surface container with elevated background, optional border, and rounded corners.

#### Scenario: Render card container

- **WHEN** an `AltairCard` is rendered with content
- **THEN** it displays with `surfaceElevated` background
- **AND** subtle border using `AltairTheme.Colors.border`
- **AND** rounded corners using `AltairTheme.Radii.lg`

#### Scenario: Card with custom padding

- **WHEN** an `AltairCard` is rendered with `contentPadding = AltairTheme.Spacing.lg`
- **THEN** the content has 24dp padding inside the card

### Requirement: AltairCheckbox Component

The system SHALL provide an `AltairCheckbox` composable that wraps Compose Unstyled `Checkbox`, supporting checked/unchecked states, labels, and keyboard accessibility.

#### Scenario: Render unchecked checkbox

- **WHEN** an `AltairCheckbox` with `checked = false` is rendered
- **THEN** it displays an empty checkbox with border
- **AND** clicking toggles it to checked state

#### Scenario: Render checked checkbox

- **WHEN** an `AltairCheckbox` with `checked = true` is rendered
- **THEN** it displays a filled checkbox with checkmark icon
- **AND** uses accent color for the fill

#### Scenario: Checkbox with label

- **WHEN** an `AltairCheckbox` with `label = "Remember me"` is rendered
- **THEN** the label appears next to the checkbox
- **AND** clicking the label toggles the checkbox

### Requirement: AltairDialog Component

The system SHALL provide an `AltairDialog` composable that wraps Compose Unstyled `Dialog`, supporting title, content, action buttons, and dismissal handling.

#### Scenario: Render confirmation dialog

- **WHEN** an `AltairDialog` with title, content, and confirm/cancel buttons is shown
- **THEN** it displays centered with elevated surface background
- **AND** a semi-transparent backdrop
- **AND** focus is trapped within the dialog

#### Scenario: Dialog dismissal

- **WHEN** a user clicks outside the dialog or presses Escape
- **THEN** the `onDismiss` callback is invoked
- **AND** the dialog closes

#### Scenario: Dialog action buttons

- **WHEN** an `AltairDialog` has `confirmText = "Delete"` and `variant = Danger`
- **THEN** the confirm button renders with danger styling
- **AND** the cancel button renders with ghost styling

### Requirement: AltairDropdownMenu Component

The system SHALL provide an `AltairDropdownMenu` composable that wraps Compose Unstyled `Dropdown`, supporting menu items, keyboard navigation, and dismissal.

#### Scenario: Open dropdown menu

- **WHEN** a user clicks a dropdown trigger
- **THEN** a menu appears below/beside the trigger
- **AND** the first item is focused for keyboard navigation

#### Scenario: Navigate dropdown with keyboard

- **WHEN** a dropdown is open and user presses arrow keys
- **THEN** focus moves between menu items
- **AND** pressing Enter selects the focused item

#### Scenario: Dismiss dropdown

- **WHEN** a user clicks outside the dropdown or presses Escape
- **THEN** the dropdown closes
- **AND** focus returns to the trigger element

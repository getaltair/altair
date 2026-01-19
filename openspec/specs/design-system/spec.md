# design-system Specification

## Purpose
TBD - created by archiving change add-design-system. Update Purpose after archive.
## Requirements
### Requirement: Design Token System

The system SHALL provide a centralized `AltairTheme` object containing design tokens for colors, typography, spacing, and border radii that can be accessed statically throughout the application.

#### Scenario: Access color tokens

- **WHEN** a component needs the primary accent color
- **THEN** it accesses `AltairTheme.Colors.accent` which returns `Color(0xFF5C6AC4)`

#### Scenario: Access typography tokens

- **WHEN** a component needs headline styling
- **THEN** it accesses `AltairTheme.Typography.headlineMedium` which returns a `TextStyle` with 20sp font size and semibold weight

#### Scenario: Access spacing tokens

- **WHEN** a component needs standard padding
- **THEN** it accesses `AltairTheme.Spacing.md` which returns `16.dp`

### Requirement: Color Palette

The system SHALL provide a dark-first color palette with the following semantic categories: backgrounds (background, backgroundElevated, backgroundSubtle, backgroundHover, backgroundPressed), borders (border, borderSubtle, borderFocused), text (textPrimary, textSecondary, textTertiary, textDisabled, textInverse), accent colors (accent, accentHover, accentPressed, accentSubtle), and status colors (success, warning, error with hover/pressed variants).

#### Scenario: Dark background hierarchy

- **WHEN** rendering the application background
- **THEN** `AltairTheme.Colors.background` provides near-black (`0xFF0D0D0D`)
- **AND** `AltairTheme.Colors.backgroundElevated` provides card/dialog background (`0xFF171717`)
- **AND** `AltairTheme.Colors.backgroundSubtle` provides slightly lighter surface (`0xFF1F1F1F`)

#### Scenario: Text contrast hierarchy

- **WHEN** rendering text content
- **THEN** `AltairTheme.Colors.textPrimary` provides high-contrast white-ish (`0xFFF5F5F5`)
- **AND** `AltairTheme.Colors.textSecondary` provides muted gray (`0xFFA3A3A3`)
- **AND** `AltairTheme.Colors.textTertiary` provides disabled/hint gray (`0xFF737373`)

### Requirement: Theme Provider

The system SHALL provide an `AltairThemeProvider` composable that establishes theme context for descendant components, enabling runtime theme customization via `LocalAltairColors` CompositionLocal.

#### Scenario: Wrap application with theme

- **WHEN** the application root composable is wrapped with `AltairThemeProvider`
- **THEN** all descendant components can access theme colors via `LocalAltairColors.current`
- **AND** CompositionLocal overrides are respected for dynamic theming (e.g., light mode)

### Requirement: Focus Ring Accessibility

The system SHALL provide a `Modifier.focusRing()` extension that renders a visible focus indicator when a component receives keyboard focus, using `AltairTheme.Colors.borderFocused` as the ring color.

#### Scenario: Button receives keyboard focus

- **WHEN** a user tabs to an `AltairButton`
- **THEN** a visible focus ring appears around the button using the `borderFocused` color
- **AND** the ring disappears when focus moves away

#### Scenario: Theme-aware focus ring

- **WHEN** a component uses `Modifier.themedFocusRing()`
- **THEN** it uses `LocalAltairColors.current.borderFocused` for the ring color
- **AND** responds to theme changes

### Requirement: AltairButton Component

The system SHALL provide an `AltairButton` composable that wraps Compose Unstyled `UnstyledButton`, supporting variants (Primary, Secondary, Ghost, Danger), enabled/disabled states, and keyboard accessibility.

#### Scenario: Render primary button

- **WHEN** an `AltairButton` with `variant = ButtonVariant.Primary` is rendered
- **THEN** it displays with accent background color from theme (`colors.accent`)
- **AND** primary text color
- **AND** rounded corners using `AltairTheme.Radii.md`

#### Scenario: Render secondary button

- **WHEN** an `AltairButton` with `variant = ButtonVariant.Secondary` is rendered
- **THEN** it displays with backgroundSubtle background color
- **AND** a visible border using theme's `border` color

#### Scenario: Render ghost button

- **WHEN** an `AltairButton` with `variant = ButtonVariant.Ghost` is rendered
- **THEN** it displays with transparent background
- **AND** shows background tint on hover

#### Scenario: Render danger button

- **WHEN** an `AltairButton` with `variant = ButtonVariant.Danger` is rendered
- **THEN** it displays with error color background from theme (`colors.error`)

#### Scenario: Disabled button state

- **WHEN** an `AltairButton` has `enabled = false`
- **THEN** it displays with backgroundSubtle background and textDisabled color
- **AND** does not respond to click events

### Requirement: AltairTextField Component

The system SHALL provide an `AltairTextField` composable that uses Compose Foundation `BasicTextField` with custom decoration, supporting labels, placeholder text, error states, helper text, and visual transformations (e.g., password masking).

#### Scenario: Render text field with label

- **WHEN** an `AltairTextField` with `label = "Email"` is rendered
- **THEN** the label appears above the input in theme's secondary text color
- **AND** the input has backgroundElevated background with border

#### Scenario: Text field error state

- **WHEN** an `AltairTextField` has `isError = true` and `errorMessage = "Invalid email"`
- **THEN** the border changes to error color from theme
- **AND** the error message appears below the input in error color

#### Scenario: Text field helper text

- **WHEN** an `AltairTextField` has `helperText = "Required"` and `isError = false`
- **THEN** the helper text appears below the input in tertiary text color
- **AND** helper text is hidden when error message is shown

#### Scenario: Text field placeholder

- **WHEN** an `AltairTextField` with `placeholder = "Enter your email"` is empty
- **THEN** the placeholder text appears in tertiary text color
- **AND** disappears when the user types

### Requirement: AltairCard Component

The system SHALL provide an `AltairCard` composable that renders a surface container with elevated background, optional border, configurable elevation shadow, and rounded corners.

#### Scenario: Render card container

- **WHEN** an `AltairCard` is rendered with content
- **THEN** it displays with `backgroundElevated` background from theme
- **AND** border using theme's `border` color
- **AND** rounded corners using `AltairTheme.Radii.lg`

#### Scenario: Card with elevation

- **WHEN** an `AltairCard` is rendered with `elevation = CardElevation.Medium`
- **THEN** the card displays with a 4dp shadow

#### Scenario: Card with custom padding

- **WHEN** an `AltairCard` is rendered with `contentPadding = AltairTheme.Spacing.lg`
- **THEN** the content has 24dp padding inside the card

### Requirement: AltairCheckbox Component

The system SHALL provide an `AltairCheckbox` composable that uses a custom Box-based implementation with clickable modifier, supporting checked/unchecked states, labels, and keyboard accessibility.

#### Scenario: Render unchecked checkbox

- **WHEN** an `AltairCheckbox` with `checked = false` is rendered
- **THEN** it displays a 20dp box with border and transparent background
- **AND** clicking toggles it to checked state

#### Scenario: Render checked checkbox

- **WHEN** an `AltairCheckbox` with `checked = true` is rendered
- **THEN** it displays with accent color background and checkmark
- **AND** no visible border when checked

#### Scenario: Checkbox with label

- **WHEN** an `AltairCheckboxRow` with `label = "Remember me"` is rendered
- **THEN** the label appears next to the checkbox
- **AND** clicking the label or checkbox toggles the state

### Requirement: AltairDialog Component

The system SHALL provide an `AltairDialog` composable that uses Compose UI `Dialog` window, supporting title, content, action buttons, and dismissal handling.

#### Scenario: Render confirmation dialog

- **WHEN** an `AltairConfirmDialog` with title, message, and confirm/cancel buttons is shown
- **THEN** it displays centered with elevated surface background
- **AND** 8dp shadow elevation

#### Scenario: Dialog dismissal

- **WHEN** a user clicks outside the dialog or presses back
- **THEN** the `onDismissRequest` callback is invoked
- **AND** the dialog closes

#### Scenario: Dialog action buttons

- **WHEN** an `AltairConfirmDialog` has `isDestructive = true`
- **THEN** the confirm button renders with Danger variant styling
- **AND** the cancel button renders with Ghost variant styling

### Requirement: AltairDropdownMenu Component

The system SHALL provide an `AltairDropdownMenu` composable that uses Compose UI `Popup` window, supporting menu items with text and dividers.

#### Scenario: Open dropdown menu

- **WHEN** a dropdown `expanded = true`
- **THEN** a menu appears as a popup at the specified offset
- **AND** displays with elevated background and border

#### Scenario: Dismiss dropdown

- **WHEN** a user clicks outside the dropdown
- **THEN** the `onDismissRequest` callback is invoked
- **AND** the dropdown closes

#### Scenario: Menu item interaction

- **WHEN** an `AltairDropdownMenuItem` is clicked
- **THEN** the `onClick` callback is invoked
- **AND** disabled items show textDisabled color and don't respond to clicks


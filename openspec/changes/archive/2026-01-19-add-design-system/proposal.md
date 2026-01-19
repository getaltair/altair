# Change: Add Altair Design System with Compose Unstyled

## Why

Altair requires a custom UI component library that works consistently across Android, iOS, and Desktop without looking platform-specific. Material Design's opinionated styling feels out of place on desktop/iOS and creates an "Android app ported to other platforms" experience. ADR-008 established Compose Unstyled as the headless component foundation with a custom Linear-inspired Altair theme to achieve a professional, productivity-focused aesthetic.

## What Changes

- Add Compose Unstyled dependency to version catalog and composeApp
- Create `AltairTheme` object with design tokens (colors, typography, spacing, radii)
- Create base UI components wrapping Compose Unstyled primitives:
  - `AltairButton` (Primary, Secondary, Ghost, Danger variants)
  - `AltairTextField` (standard input with label, error states)
  - `AltairCard` (surface container with elevation)
  - `AltairCheckbox` (for checkpoints, settings)
  - `AltairDialog` (confirmations, modals)
  - `AltairDropdownMenu` (context menus, selectors)
- Create `AltairThemeProvider` composable for theme injection
- Implement focus ring utility for keyboard accessibility
- Add dark theme support (dark-first design, light theme optional)

## Impact

- **Affected specs**: New `design-system` capability (no existing specs modified)
- **Affected code**:
  - `gradle/libs.versions.toml` - Add compose-unstyled dependency
  - `composeApp/build.gradle.kts` - Add dependency
  - `composeApp/src/commonMain/kotlin/.../ui/theme/` - New theme directory
  - `composeApp/src/commonMain/kotlin/.../ui/components/` - New components directory
- **Dependencies**: Compose Unstyled 1.50+ (composables.com)
- **Breaking changes**: None (additive only)

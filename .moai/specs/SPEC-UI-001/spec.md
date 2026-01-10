# SPEC-UI-001: Altair Design System Foundation

## Metadata

| Field            | Value                           |
| ---------------- | ------------------------------- |
| **SPEC ID**      | SPEC-UI-001                     |
| **Title**        | Altair Design System Foundation |
| **GitHub Issue** | #42                             |
| **Priority**     | High                            |
| **Status**       | Planned                         |
| **Created**      | 2026-01-09                      |
| **Labels**       | enhancement, frontend           |

## Traceability

- **Source**: [GitHub Issue #42](https://github.com/getaltair/altair/issues/42)
- **ADR Reference**: [ADR-008: Compose Unstyled with Custom Altair Theme](../../../docs/adr/008-compose-unstyled-altair-theme.md)
- **Implementation Plan**: Section 1.2 - Altair Design System Foundation

---

## Environment

### Platform Context

- **Framework**: Compose Multiplatform v1.9.3
- **Language**: Kotlin v2.3.0
- **Targets**: Desktop (JVM), Android, iOS
- **Current UI State**: Minimal Material 3 placeholder in App.kt

### Design Philosophy

- **Aesthetic**: Linear.app-inspired professional productivity design
- **Theme Priority**: Dark-first design with high contrast
- **Component Foundation**: Compose Unstyled headless primitives
- **Accessibility**: Built-in keyboard navigation, focus management, screen reader support

---

## Assumptions

| ID  | Assumption                                                               | Confidence | Risk if Wrong                                      |
| --- | ------------------------------------------------------------------------ | ---------- | -------------------------------------------------- |
| A1  | Compose Unstyled v1.49+ is compatible with Compose Multiplatform 1.9.3   | High       | Component library selection requires re-evaluation |
| A2  | Inter font family is available for all target platforms                  | Medium     | Fall back to system fonts or bundle font resources |
| A3  | CompositionLocal is the appropriate pattern for theme token distribution | High       | Theme architecture redesign needed                 |
| A4  | Design tokens from ADR-008 are finalized and approved                    | High       | Token values require stakeholder review            |

---

## Requirements

### R1: Design Token System (Ubiquitous)

The system shall provide design tokens accessible throughout the application for consistent styling.

#### R1.1: Color Tokens (Ubiquitous)

The system shall define a dark-first color palette with the following categories:

**Background Colors:**

- background: #0A0A0B (near-black base)
- surface: #141415 (slightly elevated)
- surfaceElevated: #1C1C1E (cards, dialogs)
- surfaceHover: #232326 (interactive hover states)

**Border Colors:**

- border: #2E2E32 (subtle structural borders)
- borderFocused: #6366F1 (focus ring indicator)

**Text Colors:**

- textPrimary: #EEEEEF (high contrast primary text)
- textSecondary: #8E8E93 (muted secondary text)
- textTertiary: #636366 (disabled/hint text)

**Accent Colors:**

- accent: #6366F1 (indigo primary action)
- accentHover: #818CF8 (accent interactive state)

**Status Colors:**

- success: #22C55E (positive states)
- warning: #F59E0B (caution states)
- error: #EF4444 (error states)

**Energy Level Colors (Guidance Module):**

- energy1: #22C55E (low effort)
- energy5: #EF4444 (high effort)

#### R1.2: Typography Tokens (Ubiquitous)

The system shall define typography using the Inter font family with the following scale:

- displayLarge: 32sp, SemiBold weight
- headlineMedium: 20sp, Medium weight
- bodyLarge: 16sp, Normal weight
- bodyMedium: 14sp, Normal weight
- labelSmall: 12sp, Medium weight

#### R1.3: Spacing Tokens (Ubiquitous)

The system shall define a spacing scale:

- xs: 4dp
- sm: 8dp
- md: 16dp
- lg: 24dp
- xl: 32dp

#### R1.4: Border Radius Tokens (Ubiquitous)

The system shall define border radius values:

- sm: 4dp
- md: 6dp
- lg: 8dp
- full: 9999dp (pill/circular)

---

### R2: Theme Provider (Event-Driven)

**WHEN** the application initializes **THEN** the AltairTheme composable shall provide design tokens via CompositionLocal to all descendant composables.

#### R2.1: CompositionLocal Structure

The theme shall expose the following local compositions:

- LocalAltairColors: Color token access
- LocalAltairTypography: Typography token access
- LocalAltairSpacing: Spacing token access
- LocalAltairShapes: Border radius token access

#### R2.2: Theme Access Pattern

**WHEN** a composable requires theme values **THEN** it shall access them via `AltairTheme.colors`, `AltairTheme.typography`, `AltairTheme.spacing`, or `AltairTheme.shapes` accessors.

---

### R3: Compose Unstyled Integration (State-Driven)

**IF** the design system is initialized **THEN** Compose Unstyled dependency shall be available for headless component primitives.

#### R3.1: Dependency Configuration

The system shall add Compose Unstyled v1.49+ to the commonMain source set dependencies.

#### R3.2: Component Foundation

Styled components shall wrap Compose Unstyled primitives to inherit:

- Accessibility semantics
- Keyboard navigation
- Focus management
- Platform-consistent behavior

---

### R4: Initial Styled Components (Event-Driven)

**WHEN** the design system is active **THEN** the following styled components shall be available:

#### R4.1: AltairButton

**WHEN** an AltairButton is rendered **THEN** it shall:

- Support three variants: Primary, Secondary, Ghost
- Apply appropriate background, border, and text colors per variant
- Display focus ring on keyboard focus
- Apply hover state styling on pointer hover
- Accept enabled/disabled state affecting opacity and interactivity

**Primary Variant:**

- Background: accent color
- Text: textPrimary
- Border: none

**Secondary Variant:**

- Background: surface color
- Text: textPrimary
- Border: 1dp border color

**Ghost Variant:**

- Background: transparent
- Text: textSecondary
- Border: none

#### R4.2: AltairTextField

**WHEN** an AltairTextField is rendered **THEN** it shall:

- Display with surface background color
- Apply border color, changing to borderFocused on focus
- Use textPrimary for input text
- Use textTertiary for placeholder text
- Apply md border radius
- Support single-line and multi-line modes

#### R4.3: AltairCard

**WHEN** an AltairCard is rendered **THEN** it shall:

- Display with surfaceElevated background
- Apply border color with md border radius
- Apply surfaceHover background on pointer hover
- Accept content as a composable slot

---

### R5: Negative Requirements (Unwanted)

#### R5.1: No Material Design Dependency

The design system shall NOT depend on Material Design components for styled components.

#### R5.2: No Platform-Specific Styling

The design system shall NOT apply platform-specific styling that differs between desktop, Android, and iOS.

#### R5.3: No Hardcoded Values

Styled components shall NOT use hardcoded color, spacing, or typography values; all values shall reference design tokens.

---

### R6: Preview and Verification (Optional)

**WHERE** development tooling supports it, the system shall provide:

#### R6.1: Component Preview Screen

A preview composable demonstrating all component variants in a single scrollable view.

#### R6.2: Token Gallery

A preview composable showing all design tokens (colors, typography, spacing) for visual verification.

---

## Specifications

### File Structure

```
composeApp/src/commonMain/kotlin/com/getaltair/altair/
├── ui/
│   ├── theme/
│   │   ├── AltairTheme.kt       # Theme provider and CompositionLocals
│   │   ├── Color.kt             # Color token definitions
│   │   ├── Typography.kt        # Typography token definitions
│   │   ├── Spacing.kt           # Spacing token definitions
│   │   └── Shape.kt             # Border radius definitions
│   ├── components/
│   │   ├── AltairButton.kt      # Button component
│   │   ├── AltairTextField.kt   # TextField component
│   │   └── AltairCard.kt        # Card component
│   └── preview/
│       └── ComponentPreview.kt  # Preview screen for verification
```

### Technical Constraints

| Constraint                       | Value                               |
| -------------------------------- | ----------------------------------- |
| Minimum Compose Unstyled Version | 1.49+                               |
| Font Loading                     | Platform resource system or bundled |
| Color Format                     | ARGB hexadecimal (0xFFRRGGBB)       |
| Density Independence             | All dimensions in dp/sp             |

### Integration Points

- **App.kt**: Replace MaterialTheme with AltairTheme wrapper
- **Future Components**: All UI components shall use AltairTheme tokens
- **Module Screens**: Guidance, Knowledge, Tracking modules consume theme

---

## Dependencies

### External Libraries

| Library          | Version | Purpose                       |
| ---------------- | ------- | ----------------------------- |
| Compose Unstyled | 1.49+   | Headless component primitives |
| Inter Font       | Latest  | Typography (if bundled)       |

### Internal Dependencies

| Module     | Dependency Type               |
| ---------- | ----------------------------- |
| composeApp | Direct - theme implementation |
| shared     | None                          |
| server     | None                          |

---

## Risks and Mitigations

| Risk                                     | Probability | Impact | Mitigation                                      |
| ---------------------------------------- | ----------- | ------ | ----------------------------------------------- |
| Compose Unstyled version incompatibility | Low         | High   | Test integration before implementation          |
| Inter font unavailable on platform       | Medium      | Low    | Implement font fallback chain                   |
| Theme performance overhead               | Low         | Medium | Use remember/derivedStateOf for computed values |
| Focus ring styling differences           | Medium      | Low    | Test across all three platforms                 |

---

## Related SPECs

- **Successor**: SPEC-UI-002 (Additional Components - when created)
- **Depends On**: None (foundation SPEC)
- **Related ADR**: ADR-008 (Compose Unstyled Altair Theme)

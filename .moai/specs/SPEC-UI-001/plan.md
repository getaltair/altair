# SPEC-UI-001: Implementation Plan

## Traceability

- **SPEC ID**: SPEC-UI-001
- **Title**: Altair Design System Foundation
- **GitHub Issue**: #42

---

## Implementation Strategy

### Approach

Implement the design system in a bottom-up approach:

1. Define immutable token objects (no dependencies)
2. Create CompositionLocal providers (depends on tokens)
3. Implement theme wrapper (depends on providers)
4. Build styled components (depends on theme)
5. Create preview screen (depends on all above)

### Technical Approach

- **Pattern**: Object declarations for tokens, data classes for token groups
- **Distribution**: CompositionLocal for tree-wide access
- **Styling**: Extension functions and modifiers wrapping Compose Unstyled primitives
- **Testing**: Visual verification via preview composables

---

## Milestones

### Milestone 1: Token Definitions (Primary Goal)

**Objective**: Establish all design token values as immutable Kotlin objects.

**Tasks**:

1.1. Create `ui/theme/Color.kt`

- Define AltairColors data class with all color properties
- Create darkColors() factory function with ADR-008 values
- Add Color extension functions if needed (e.g., withAlpha)

  1.2. Create `ui/theme/Typography.kt`

- Define AltairTypography data class with TextStyle properties
- Configure Inter font family loading
- Implement fallback font chain for platform compatibility

  1.3. Create `ui/theme/Spacing.kt`

- Define AltairSpacing data class with Dp properties
- Include all scale values (xs through xl)

  1.4. Create `ui/theme/Shape.kt`

- Define AltairShapes data class with RoundedCornerShape properties
- Include all radius values (sm, md, lg, full)

**Completion Criteria**:

- All token files compile without errors
- Token values match ADR-008 specifications exactly
- No hardcoded values outside token definitions

---

### Milestone 2: Theme Infrastructure (Primary Goal)

**Objective**: Implement CompositionLocal-based theme distribution.

**Tasks**:

2.1. Create `ui/theme/AltairTheme.kt`

- Define LocalAltairColors CompositionLocal
- Define LocalAltairTypography CompositionLocal
- Define LocalAltairSpacing CompositionLocal
- Define LocalAltairShapes CompositionLocal

  2.2. Implement AltairTheme composable

- Accept optional custom token overrides
- Provide all CompositionLocals via CompositionLocalProvider
- Wrap content slot

  2.3. Create AltairTheme companion accessors

- `AltairTheme.colors` - current LocalAltairColors value
- `AltairTheme.typography` - current LocalAltairTypography value
- `AltairTheme.spacing` - current LocalAltairSpacing value
- `AltairTheme.shapes` - current LocalAltairShapes value

**Completion Criteria**:

- AltairTheme composable wraps content successfully
- Theme accessors return correct token values within theme scope
- Default error when accessed outside theme scope

---

### Milestone 3: Compose Unstyled Integration (Primary Goal)

**Objective**: Add and configure Compose Unstyled dependency.

**Tasks**:

3.1. Update `gradle/libs.versions.toml`

- Add compose-unstyled version entry
- Add compose-unstyled library entries for required components

  3.2. Update `composeApp/build.gradle.kts`

- Add Compose Unstyled to commonMain dependencies
- Verify dependency resolution

  3.3. Verify integration

- Import Compose Unstyled primitives in test file
- Confirm API availability

**Completion Criteria**:

- Gradle sync succeeds
- Compose Unstyled imports resolve
- Build completes for all targets

---

### Milestone 4: Styled Components (Primary Goal)

**Objective**: Implement initial styled component set.

**Tasks**:

4.1. Create `ui/components/AltairButton.kt`

- Define ButtonVariant enum (Primary, Secondary, Ghost)
- Implement AltairButton composable wrapping Compose Unstyled Button
- Apply variant-specific styling using theme tokens
- Implement focus ring via focusable modifier
- Handle enabled/disabled states
- Support content slot with RowScope

  4.2. Create `ui/components/AltairTextField.kt`

- Implement AltairTextField wrapping Compose Unstyled TextField
- Apply theme colors for background, border, text
- Implement focus state border change
- Support value/onValueChange pattern
- Support placeholder text
- Support single-line and multi-line modes

  4.3. Create `ui/components/AltairCard.kt`

- Implement AltairCard as styled container
- Apply surfaceElevated background
- Apply border styling
- Implement hover state (desktop)
- Accept content slot

**Completion Criteria**:

- All three components compile
- Components render with correct styling
- Focus states work via keyboard navigation
- Hover states work on desktop

---

### Milestone 5: Preview and Verification (Secondary Goal)

**Objective**: Create verification tooling for visual review.

**Tasks**:

5.1. Create `ui/preview/ComponentPreview.kt`

- Implement scrollable preview showing all components
- Include all button variants with labels
- Include text field in various states
- Include card with sample content
- Apply AltairTheme wrapper

  5.2. Create token gallery section

- Display color swatches with labels
- Display typography samples
- Display spacing visualization
- Display shape samples

  5.3. Update App.kt integration

- Replace MaterialTheme with AltairTheme
- Render ComponentPreview for verification
- Document reversion path after verification

**Completion Criteria**:

- Preview screen renders on desktop
- All component variants visible
- All token values visually verifiable
- Screenshot captures possible for documentation

---

### Milestone 6: Platform Verification (Secondary Goal)

**Objective**: Verify consistent rendering across all target platforms.

**Tasks**:

6.1. Desktop verification

- Run via `./gradlew :composeApp:run`
- Verify all components render correctly
- Test keyboard navigation and focus
- Test hover states

  6.2. Android verification

- Run via `./gradlew :composeApp:installDebug`
- Verify all components render correctly
- Test touch interactions
- Verify font rendering

  6.3. iOS verification

- Build and run via Xcode
- Verify all components render correctly
- Test touch interactions
- Verify font rendering

**Completion Criteria**:

- Components render identically on all platforms
- No platform-specific visual differences
- Interactions work appropriately per platform

---

## Technical Approach Details

### Font Loading Strategy

```kotlin
// Typography.kt approach
val InterFontFamily = FontFamily(
    Font(Res.font.inter_regular, FontWeight.Normal),
    Font(Res.font.inter_medium, FontWeight.Medium),
    Font(Res.font.inter_semibold, FontWeight.SemiBold)
)

// Fallback if resources unavailable
val AltairFontFamily = try {
    InterFontFamily
} catch (e: Exception) {
    FontFamily.Default
}
```

### Theme Access Pattern

```kotlin
// AltairTheme.kt structure
object AltairTheme {
    val colors: AltairColors
        @Composable
        @ReadOnlyComposable
        get() = LocalAltairColors.current

    val typography: AltairTypography
        @Composable
        @ReadOnlyComposable
        get() = LocalAltairTypography.current

    // ... spacing, shapes
}

@Composable
fun AltairTheme(
    colors: AltairColors = darkColors(),
    typography: AltairTypography = AltairTypography(),
    spacing: AltairSpacing = AltairSpacing(),
    shapes: AltairShapes = AltairShapes(),
    content: @Composable () -> Unit
) {
    CompositionLocalProvider(
        LocalAltairColors provides colors,
        LocalAltairTypography provides typography,
        LocalAltairSpacing provides spacing,
        LocalAltairShapes provides shapes,
        content = content
    )
}
```

### Focus Ring Implementation

```kotlin
// Modifier extension for focus ring
fun Modifier.altairFocusRing(
    shape: Shape = RoundedCornerShape(AltairTheme.shapes.md)
): Modifier = composed {
    var isFocused by remember { mutableStateOf(false) }
    this
        .onFocusChanged { isFocused = it.isFocused }
        .then(
            if (isFocused) {
                Modifier.border(
                    width = 2.dp,
                    color = AltairTheme.colors.borderFocused,
                    shape = shape
                )
            } else {
                Modifier
            }
        )
}
```

---

## Architecture Design Direction

### Layer Separation

```
Presentation Layer (UI)
├── theme/          # Token definitions and theme provider
├── components/     # Styled reusable components
└── preview/        # Development verification tools

Domain Layer
└── (No design system dependencies)

Data Layer
└── (No design system dependencies)
```

### Extensibility Points

1. **Custom Colors**: Pass custom AltairColors to AltairTheme
2. **Light Theme**: Create lightColors() factory for future light mode
3. **Component Variants**: Add new variants via sealed class extension
4. **New Components**: Follow established pattern with Compose Unstyled base

---

## Risks and Response Plans

### Risk: Compose Unstyled API Breaking Changes

**Probability**: Low
**Response**: Pin to specific version, isolate Compose Unstyled usage to component implementations only

### Risk: Font Resource Loading Failures

**Probability**: Medium
**Response**: Implement robust fallback chain, log font loading issues, degrade gracefully to system fonts

### Risk: Theme Performance in Large Trees

**Probability**: Low
**Response**: Use `@Stable` annotations on token classes, avoid recomposition triggers in token access

### Risk: Platform-Specific Focus Behavior

**Probability**: Medium
**Response**: Test focus ring on all platforms early, implement platform-specific adjustments if needed

---

## Dependencies and Blockers

### Prerequisites

- Compose Multiplatform 1.9.3 project structure (complete)
- Gradle version catalog setup (complete)
- ADR-008 approved and finalized (complete)

### External Dependencies

| Dependency       | Status    | Notes                            |
| ---------------- | --------- | -------------------------------- |
| Compose Unstyled | Available | Verify version compatibility     |
| Inter Font       | Available | May need to bundle or use system |

### No Blockers Identified

This SPEC has no dependencies on other SPECs or incomplete infrastructure.

---

## Notes

- All component implementations should include KDoc documentation
- Preview composables should be annotated with `@Preview` for IDE tooling
- Token values must exactly match ADR-008 specifications
- Material3 import statements should be removed from files using AltairTheme

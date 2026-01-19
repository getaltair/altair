# Design: Altair Design System

## Context

Altair is a cross-platform Compose Multiplatform application targeting Android, iOS, and Desktop. The UI must feel native to none of these platforms individually, instead presenting a consistent, professional aesthetic inspired by Linear's productivity-focused design language.

**Constraints:**
- Must work identically across Android, iOS, and Desktop
- No Material Design dependency (opinionated and Android-centric)
- Accessibility requirements: keyboard navigation, focus management, screen reader support
- Dark-first design with ADHD-friendly visual clarity
- Performance: minimal runtime overhead for theming

**Stakeholders:** End users across all platforms, accessibility users

## Goals / Non-Goals

### Goals
- Establish reusable design tokens (colors, typography, spacing, radii)
- Create accessible base components with built-in keyboard/focus handling
- Achieve Linear-inspired dark aesthetic: high contrast, minimal chrome, professional
- Support theme customization for future light mode

### Non-Goals
- Platform-adaptive styling (iOS-specific, Android-specific)
- Animation library (use Compose built-ins)
- Complex component library beyond base components (data tables, etc.)
- Runtime theme switching in initial implementation

## Decisions

### Decision 1: Compose Unstyled as Foundation

**What:** Use Compose Unstyled headless components for behavior/accessibility, style with custom Altair theme.

**Why:**
- Provides keyboard navigation, focus management, ARIA semantics out of the box
- Zero styling opinions - complete visual control
- Actively maintained, production-stable (v1.49+)
- Single Kotlin files - can copy/modify if needed

**Alternatives considered:**
- **Material 3**: Rejected - too Android-centric, would need to override everything
- **Carbon Compose**: Rejected - v0.7 (unstable), missing Dialog/Modal, IBM aesthetic
- **Build from scratch**: Rejected - significant effort for accessibility basics

### Decision 2: Static Theme Object

**What:** Implement `AltairTheme` as a Kotlin object with nested objects for token categories.

**Why:**
- Simple, no runtime overhead for static colors
- Type-safe access: `AltairTheme.Colors.accent`
- Easy to understand and maintain
- Compose `CompositionLocal` used only for optional dynamic overrides

**Pattern:**
```kotlin
object AltairTheme {
    object Colors { ... }
    object Typography { ... }
    object Spacing { ... }
    object Radii { ... }
}
```

### Decision 3: Component Wrapping Pattern

**What:** Altair components wrap Compose Unstyled primitives (for buttons) or Compose Foundation primitives (for other components), applying theme colors via `LocalAltairColors`.

**Why:**
- Clear separation: Unstyled handles behavior, Altair handles appearance
- Theme-aware via CompositionLocal for light/dark mode support
- Easy to update styling without changing behavior logic

**Pattern:**
```kotlin
@Composable
fun AltairButton(
    onClick: () -> Unit,
    variant: ButtonVariant = ButtonVariant.Primary,
    modifier: Modifier = Modifier,
    content: @Composable RowScope.() -> Unit
) {
    val colors = LocalAltairColors.current

    UnstyledButton(  // From Compose Unstyled
        onClick = onClick,
        modifier = modifier,
        backgroundColor = variant.backgroundColor(colors),
        contentColor = variant.contentColor(colors),
        shape = RoundedCornerShape(AltairTheme.Radii.md),
        content = content,
    )
}
```

### Decision 4: Focus Ring Utility

**What:** Create `Modifier.focusRing()` extension for consistent focus indication.

**Why:**
- Accessibility requirement for keyboard users
- Consistent visual language across all interactive components
- Centralized focus styling updates

### Decision 5: Directory Structure

**What:**
```
composeApp/src/commonMain/kotlin/.../ui/
├── theme/
│   ├── AltairTheme.kt          # Design tokens
│   ├── AltairThemeProvider.kt  # CompositionLocal setup
│   └── FocusRing.kt            # Focus indicator utility
└── components/
    ├── AltairButton.kt
    ├── AltairTextField.kt
    ├── AltairCard.kt
    ├── AltairCheckbox.kt
    ├── AltairDialog.kt
    └── AltairDropdownMenu.kt
```

**Why:**
- Separates tokens from components
- Each component is self-contained
- Easy to add new components without touching others

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|------------|
| Compose Unstyled breaking changes | Medium | Pin version, copy components if abandoned |
| Design token inconsistency | Low | Code review, lint rules for direct color usage |
| Missing component coverage | Low | Add components incrementally as needed |
| Theme performance on low-end | Low | Static object avoids runtime allocation |

## Migration Plan

1. Add Compose Unstyled dependency (non-breaking)
2. Create theme directory and `AltairTheme` object
3. Create base components one by one
4. Migrate existing screens to use Altair components (can be incremental)
5. No database/API changes required

**Rollback:** Remove Compose Unstyled dependency and theme directory; no data migration needed.

## Open Questions

- [ ] Should energy level colors (1-5 scale) be part of core theme or Guidance module?
- [ ] Font choice: Inter vs system font stack for cross-platform consistency?
- [ ] Include light theme in initial implementation or defer?

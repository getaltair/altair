# ADR-008: Compose Unstyled with Custom Altair Theme

| Field        | Value           |
| ------------ | --------------- |
| **Status**   | Accepted        |
| **Date**     | 2026-01-09      |
| **Deciders** | Robert Hamilton |

## Context

Altair uses Compose Multiplatform for UI across desktop, Android, and iOS (ADR-001). We need a UI
component foundation that:

1. Works consistently across all platforms without looking "Android-native"
2. Supports a custom aesthetic inspired by Linear's professional, productivity-focused design
3. Avoids Material Design's opinionated styling that feels out of place on desktop/iOS
4. Provides accessible, keyboard-navigable components with proper focus handling
5. Allows full control over visual styling without fighting framework defaults

## Decision

Use **Compose Unstyled** (composables.com) as the headless component foundation, with a custom
**Altair design system** built on top, inspired by Linear's aesthetic.

**Compose Unstyled** provides:

- Unstyled/headless component primitives (behavior without styling)
- Accessibility semantics, keyboard navigation, focus management built-in
- No Material Design dependency
- Compose Multiplatform support (Android, iOS, Desktop, Web)

**Altair theme** provides:

- Custom design tokens (colors, typography, spacing, motion)
- Styled components wrapping Compose Unstyled primitives
- Linear-inspired aesthetic: dark-first, high-contrast, professional, functional

## Consequences

### Positive

- **Full aesthetic control**: Every visual detail is customizable; no Material baggage
- **Platform consistency**: Same look across desktop, Android, iOS—not "Android app on other
  platforms"
- **Linear-inspired**: Professional, productivity-focused aesthetic (dark mode, neutral palette,
  clean typography)
- **Stable foundation**: Compose Unstyled v1.49+ is production-stable
- **Accessibility included**: Focus rings, keyboard navigation, screen reader support handled by
  primitives
- **No lock-in**: Components are single Kotlin files; can copy and modify if needed

### Negative

- **Design effort required**: Must create entire visual system (colors, typography, component
  styles)
- **More upfront work**: 2-3 days to build basic component library vs. using pre-styled system
- **No community themes**: Unlike Material, no ecosystem of pre-built themes to start from
- **Documentation**: Must document our own design system for consistency

### Neutral

- Learning curve similar to any Compose component library
- Can evolve aesthetic over time without framework constraints

## Altair Design System

### Design Tokens

```kotlin
object AltairTheme {
    object Colors {
        // Backgrounds (dark-first, Linear-inspired)
        val background = Color(0xFF0A0A0B)           // Near-black
        val surface = Color(0xFF141415)              // Slightly lighter
        val surfaceElevated = Color(0xFF1C1C1E)      // Cards, dialogs
        val surfaceHover = Color(0xFF232326)         // Hover states

        // Borders
        val border = Color(0xFF2E2E32)               // Subtle borders
        val borderFocused = Color(0xFF6366F1)        // Focus rings

        // Text
        val textPrimary = Color(0xFFEEEEEF)          // High contrast
        val textSecondary = Color(0xFF8E8E93)        // Muted
        val textTertiary = Color(0xFF636366)         // Disabled/hints

        // Accent (indigo, can be customized)
        val accent = Color(0xFF6366F1)
        val accentHover = Color(0xFF818CF8)

        // Status
        val success = Color(0xFF22C55E)
        val warning = Color(0xFFF59E0B)
        val error = Color(0xFFEF4444)

        // Energy levels (Guidance module)
        val energy1 = Color(0xFF22C55E)              // Low effort
        val energy5 = Color(0xFFEF4444)              // High effort
    }

    object Typography {
        val fontFamily = FontFamily(/* Inter or similar */)

        // Type scale
        val displayLarge = TextStyle(fontSize = 32.sp, fontWeight = FontWeight.SemiBold)
        val headlineMedium = TextStyle(fontSize = 20.sp, fontWeight = FontWeight.Medium)
        val bodyLarge = TextStyle(fontSize = 16.sp, fontWeight = FontWeight.Normal)
        val bodyMedium = TextStyle(fontSize = 14.sp, fontWeight = FontWeight.Normal)
        val labelSmall = TextStyle(fontSize = 12.sp, fontWeight = FontWeight.Medium)
    }

    object Spacing {
        val xs = 4.dp
        val sm = 8.dp
        val md = 16.dp
        val lg = 24.dp
        val xl = 32.dp
    }

    object Radii {
        val sm = 4.dp
        val md = 6.dp
        val lg = 8.dp
        val full = 9999.dp
    }
}
```

### Component Examples

```kotlin
// Button built on Compose Unstyled
@Composable
fun AltairButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    variant: ButtonVariant = ButtonVariant.Primary,
    enabled: Boolean = true,
    content: @Composable RowScope.() -> Unit
) {
    Button(  // From Compose Unstyled
        onClick = onClick,
        enabled = enabled,
        modifier = modifier
            .background(
                color = when (variant) {
                    ButtonVariant.Primary -> AltairTheme.Colors.accent
                    ButtonVariant.Secondary -> AltairTheme.Colors.surface
                    ButtonVariant.Ghost -> Color.Transparent
                },
                shape = RoundedCornerShape(AltairTheme.Radii.md)
            )
            .border(
                width = 1.dp,
                color = when (variant) {
                    ButtonVariant.Secondary -> AltairTheme.Colors.border
                    else -> Color.Transparent
                },
                shape = RoundedCornerShape(AltairTheme.Radii.md)
            )
            .padding(horizontal = 12.dp, vertical = 8.dp)
            .focusRing(AltairTheme.Colors.borderFocused)
    ) {
        content()
    }
}
```

### Compose Unstyled Components Used

| Component     | Altair Usage                            |
| ------------- | --------------------------------------- |
| Button        | All buttons, icon buttons               |
| TextField     | Quest titles, note content, search      |
| Checkbox      | Checkpoint completion, settings toggles |
| Toggle Switch | Settings, feature toggles               |
| Dialog        | Confirmations, quest creation, settings |
| Modal         | Full-screen overlays, focus mode        |
| Bottom Sheet  | Mobile quick actions, filters           |
| Dropdown Menu | Context menus, Epic selection           |
| Tab Group     | Module navigation, view switching       |
| Slider        | Energy budget, numeric inputs           |
| Scroll Area   | Lists, note content                     |
| Tooltip       | Help text, shortcuts hints              |
| Progress      | Sync status, AI processing              |
| Radio Group   | Single-select options                   |

## Alternatives Considered

### Alternative 1: Carbon Compose

IBM's Carbon Design System for Compose Multiplatform.

**Pros:**

- Pre-styled professional aesthetic (dashboards, HMI)
- Theme system (White, Grey 10, Grey 90, Grey 100)
- Touchscreen adaptation mode

**Rejected because:**

- v0.7 (pre-1.0); single maintainer
- Missing critical components: Modal, Dialog, Data Table
- Locked into IBM Carbon aesthetic; deviation is fighting the library
- Carbon's IBM-corporate-neutral differs from Linear's dark, gradient-forward style

### Alternative 2: Material 3 (Default)

Google's Material Design for Compose.

**Rejected because:**

- Looks distinctly "Android" on desktop/iOS
- Material's opinionated styling (ripples, elevation, shapes) doesn't match Linear aesthetic
- Would need to override nearly everything, making Material a hindrance
- Large touch targets designed for mobile feel wrong on desktop

### Alternative 3: Compose Cupertino

Apple HIG design system for Compose Multiplatform.

**Rejected because:**

- iOS-specific aesthetic; looks wrong on Windows/Linux/Android
- Not the professional/productivity aesthetic we want
- Would alienate Windows/Linux users (primary platforms)

### Alternative 4: Build Everything from Scratch

Use only compose.foundation without any component library.

**Rejected because:**

- Must implement all accessibility, focus management, keyboard navigation
- Massive time investment for basic components
- Error-prone reimplementation of solved problems

## References

- [Compose Unstyled Documentation](https://composables.com/docs/compose-unstyled/)
- [Compose Unstyled GitHub](https://github.com/composablehorizons/compose-unstyled)
- [Linear Design Aesthetic](https://linear.app/now/how-we-redesigned-the-linear-ui)
- [ADR-001: Kotlin Multiplatform Architecture](./001-single-tauri-application.md)
- PRD Core, Section 6: ADHD-Optimized Design Principles

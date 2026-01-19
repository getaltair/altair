## Context

Altair is a life management ecosystem with three core modules (Guidance, Knowledge, Tracking) unified by system-level features. Users need to navigate between these modules efficiently. The application runs on Android, iOS, and Desktop, each with different navigation idioms:

- **Mobile**: Bottom navigation is standard (Material Design, Apple HIG)
- **Desktop**: Side navigation rail or sidebar is preferred for larger screens

The existing navigation uses Decompose with a stack-based approach. The auth flow (Login/Register) already works. This design extends the navigation to support the main application shell.

**Stakeholders:** End users on all platforms, developers implementing feature modules.

**Constraints:**
- Must use existing Altair Design System components
- Must work with Decompose stack navigation
- Must respect platform conventions (bottom nav on mobile, sidebar on desktop)

## Goals / Non-Goals

**Goals:**
- Establish consistent navigation shell across all platforms
- Create clear entry points for Guidance, Knowledge, Tracking, and Settings
- Provide placeholder screens that can be replaced with real implementations
- Support keyboard navigation on desktop

**Non-Goals:**
- Implementing actual module functionality (deferred to Phase 8)
- Deep linking or URL-based navigation (future enhancement)
- Animations between module transitions (simple fade is sufficient initially)

## Decisions

### Decision 1: Two-Level Navigation Architecture

**What:** Use a two-level navigation structure:
1. **Root level**: Auth flow (Login/Register) vs Main app shell
2. **Shell level**: Tab/rail navigation between modules (Home, Guidance, Knowledge, Tracking, Settings)

**Why:** This separates concerns cleanly — auth is a separate flow, and once authenticated, users navigate within the main shell. The shell owns the navigation bar/rail and child content area.

**Alternatives considered:**
- Single flat stack for everything — Rejected because it doesn't support tab-style navigation semantics
- Nested childStack per tab — Rejected as over-engineering for current needs; simple tab switching is sufficient

### Decision 2: Platform-Specific Navigation Components

**What:** Create two navigation components in `commonMain` with platform detection:
- `AltairBottomNavigationBar` — For mobile (Android/iOS)
- `AltairNavigationRail` — For desktop (JVM)

Use `expect/actual` for platform detection or check window size at runtime.

**Why:** Material Design and Apple HIG both recommend bottom navigation for mobile. Desktop users expect side navigation for productivity apps.

**Pattern:**
```kotlin
@Composable
fun NavigationShell(
    currentDestination: MainDestination,
    onDestinationSelected: (MainDestination) -> Unit,
    content: @Composable () -> Unit
) {
    val isCompactScreen = rememberIsCompactScreen() // Platform-specific

    if (isCompactScreen) {
        Column {
            Box(Modifier.weight(1f)) { content() }
            AltairBottomNavigationBar(currentDestination, onDestinationSelected)
        }
    } else {
        Row {
            AltairNavigationRail(currentDestination, onDestinationSelected)
            Box(Modifier.weight(1f)) { content() }
        }
    }
}
```

### Decision 3: MainComponent for Shell Navigation

**What:** Create a `MainComponent` that owns the shell navigation state (currently selected tab) and child components for each tab.

**Why:** Decompose's `childStack` is best for push/pop navigation (e.g., auth flow, drilling into details). For tab-style navigation where all tabs persist their state, a simple state holder with child components works better.

**Structure:**
```
RootComponent
├── Login flow (childStack)
│   ├── Config.Login → Child.Login
│   └── Config.Register → Child.Register
└── Config.Main → Child.Main(MainComponent)
    ├── HomeComponent (or HomeScreen directly)
    ├── GuidanceComponent (placeholder)
    ├── KnowledgeComponent (placeholder)
    ├── TrackingComponent (placeholder)
    └── SettingsComponent (placeholder)
```

### Decision 4: Simple Placeholder Screens

**What:** Each placeholder screen displays:
- Module icon and name
- Brief description of what the module will do
- "Coming soon" or similar indicator

**Why:** Placeholder screens allow navigation to work end-to-end while real implementations are built. They provide visual feedback that the navigation is functional.

### Decision 5: Settings Destination

**What:** Include Settings as a primary navigation destination (5th item).

**Why:**
- Users need quick access to account settings, theme, preferences
- Matches common patterns in productivity apps (Todoist, Notion, Obsidian)
- Avoids hamburger menu or hidden settings

**Navigation bar layout:**
| Position | Mobile (Bottom) | Desktop (Rail) |
|----------|-----------------|----------------|
| 1 | Home | Home |
| 2 | Guidance | Guidance |
| 3 | Knowledge | Knowledge |
| 4 | Tracking | Tracking |
| 5 | Settings | Settings (bottom of rail) |

## Risks / Trade-offs

### Risk: State Loss on Tab Switch

**Risk:** Users might lose unsaved work when switching tabs.

**Mitigation:**
- Placeholder screens have no state to lose
- Future implementations will use auto-save patterns
- Consider `SavedStateHandle` equivalent for complex forms

### Risk: Screen Size Detection Complexity

**Risk:** Detecting "mobile vs desktop" is imprecise with foldables, tablets, resizable windows.

**Mitigation:**
- Use width-based breakpoint (600dp threshold, following Material Design)
- Allow window resize to trigger layout change on desktop
- Test on various form factors

### Trade-off: Simplified Tab Navigation

**Trade-off:** Not using `childStack` per tab means each tab loses its internal navigation state when switching.

**Accepted because:**
- Placeholder screens have no internal navigation
- When real implementations come, we can upgrade specific tabs to use nested `childStack`
- Simplicity is preferred for initial implementation

## Migration Plan

No migration needed — this is additive functionality.

**Rollout:**
1. Add Config entries and RootComponent handling
2. Create NavigationShell component
3. Create placeholder screens
4. Update RootContent to render shell when authenticated
5. Manual testing on all platforms

**Rollback:** Revert commits if issues found; no data migration.

## Open Questions

None — design is straightforward for placeholder implementation.

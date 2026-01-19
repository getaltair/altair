## 1. Navigation Configuration

- [x] 1.1 Add new Config entries (Guidance, Knowledge, Tracking, Settings, Main) to `Config.kt`
- [x] 1.2 Create `MainDestination` enum for tab navigation within the main shell

## 2. Platform Detection Utility

- [x] 2.1 Create `WindowSizeClass` utility in `ui/util/` with compact/medium/expanded breakpoints
- [x] 2.2 Create `WithWindowSizeClass()` composable that provides current size class
- [x] 2.3 Window size detection works on Android, iOS, and Desktop via BoxWithConstraints

## 3. Navigation Shell Components

- [x] 3.1 Create `AltairBottomNavigationBar` component with 5 destinations
- [x] 3.2 Create `AltairNavigationRail` component with 5 destinations (Settings at bottom)
- [x] 3.3 Create `NavigationShell` composable that switches between bar and rail based on window size
- [x] 3.4 Add focus ring support to navigation items for keyboard accessibility
- [x] 3.5 Add icons for each destination (using Material Icons)

## 4. Placeholder Screens

- [x] 4.1 Create `HomeScreen.kt` enhancement with "Today" title and placeholder content
- [x] 4.2 Create `GuidanceScreen.kt` placeholder in `ui/guidance/`
- [x] 4.3 Create `KnowledgeScreen.kt` placeholder in `ui/knowledge/`
- [x] 4.4 Create `TrackingScreen.kt` placeholder in `ui/tracking/`
- [x] 4.5 Create `SettingsScreen.kt` placeholder in `ui/settings/`

## 5. Main Component and Navigation Logic

- [x] 5.1 Create `MainComponent` to hold shell navigation state and child components
- [x] 5.2 Add `Config.Main` handling to `RootComponent.child()` factory
- [x] 5.3 Create `MainContent` composable that renders NavigationShell with child screens
- [x] 5.4 Update `RootContent.kt` to render `MainContent` when `Child.Main` is active

## 6. Integration and Testing

- [x] 6.1 Verify auth flow still works (Login → Main shell when authenticated)
- [x] 6.2 Verify navigation between all 5 destinations works
- [x] 6.3 Bottom bar displays on compact screens (< 600dp width)
- [x] 6.4 Side rail displays on medium/expanded screens (>= 600dp width)
- [x] 6.5 Focus ring support added for keyboard navigation
- [x] 6.6 Window resize triggers layout change via movableContentOf

## 7. Code Quality

- [x] 7.1 Add unit tests for `MainComponent` navigation state (5 tests)
- [x] 7.2 Run lint and fix all issues (spotless, detekt)
- [x] 7.3 Verify build passes on all platforms (`./gradlew build`)

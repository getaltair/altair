# Change: Add Navigation Structure with Platform-Adaptive Shell

## Why

The application currently has basic stack navigation (Home, Login, Register) but lacks the main navigation destinations (Guidance, Knowledge, Tracking, Settings) and platform-appropriate navigation patterns. Phase 7.2 and 7.3 of the implementation plan require establishing the complete navigation shell with placeholder screens to enable feature development in subsequent phases.

## What Changes

- Add new `Config` destinations: Guidance, Knowledge, Tracking, Settings
- Create platform-adaptive navigation shell:
  - **Mobile (Android/iOS)**: Bottom navigation bar with 4 primary destinations
  - **Desktop**: Side navigation rail/sidebar with the same destinations
- Create placeholder screens for each module:
  - Home/Today view (enhanced from current)
  - Guidance (Quest list placeholder)
  - Knowledge (Note list placeholder)
  - Tracking (Item list placeholder)
  - Settings (configuration placeholder)
- Integrate navigation shell with existing auth flow (only shown when authenticated)

## Impact

- **Affected specs:**
  - `navigation` — MODIFIED to add platform-adaptive shell requirements
  - `navigation` — ADDED requirements for main destinations and placeholder screens

- **Affected code:**
  - `composeApp/src/commonMain/kotlin/.../navigation/Config.kt` — Add new Config entries
  - `composeApp/src/commonMain/kotlin/.../navigation/RootComponent.kt` — Add child creation for new destinations
  - `composeApp/src/commonMain/kotlin/.../navigation/RootContent.kt` — Render new screens with navigation shell
  - `composeApp/src/commonMain/kotlin/.../ui/components/` — New navigation components (BottomNavigationBar, SideNavRail)
  - `composeApp/src/commonMain/kotlin/.../ui/guidance/` — GuidanceScreen placeholder
  - `composeApp/src/commonMain/kotlin/.../ui/knowledge/` — KnowledgeScreen placeholder
  - `composeApp/src/commonMain/kotlin/.../ui/tracking/` — TrackingScreen placeholder
  - `composeApp/src/commonMain/kotlin/.../ui/settings/` — SettingsScreen placeholder

- **Dependencies:**
  - Requires Phase 7.1 (Altair Design System) — **Already completed**
  - Requires Decompose integration — **Already completed**

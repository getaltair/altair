# Tasks: Add Altair Design System

## 1. Dependencies

- [x] 1.1 Add Compose Unstyled version to `gradle/libs.versions.toml`
- [x] 1.2 Add Compose Unstyled library entries to version catalog
- [x] 1.3 Add Compose Unstyled dependency to `composeApp/build.gradle.kts` commonMain

## 2. Theme Foundation

- [x] 2.1 Create `composeApp/src/commonMain/kotlin/.../ui/theme/` directory
- [x] 2.2 Create `AltairTheme.kt` with color tokens (backgrounds, text, borders, accent, status)
- [x] 2.3 Add typography tokens to `AltairTheme` (display, headline, body, label scales)
- [x] 2.4 Add spacing tokens to `AltairTheme` (xs, sm, md, lg, xl)
- [x] 2.5 Add radii tokens to `AltairTheme` (sm, md, lg, full)
- [x] 2.6 Create `AltairThemeProvider.kt` with CompositionLocal setup
- [x] 2.7 Create `FocusRing.kt` with `Modifier.focusRing()` extension

## 3. Base Components

- [x] 3.1 Create `composeApp/src/commonMain/kotlin/.../ui/components/` directory
- [x] 3.2 Create `ButtonVariant.kt` enum (Primary, Secondary, Ghost, Danger)
- [x] 3.3 Create `AltairButton.kt` wrapping Compose Unstyled UnstyledButton
- [x] 3.4 Create `AltairTextField.kt` with label support
- [x] 3.5 Create `AltairCard.kt` surface container with border and optional elevation
- [x] 3.6 Create `AltairCheckbox.kt`
- [x] 3.7 Create `AltairDialog.kt`
- [x] 3.8 Create `AltairDropdownMenu.kt`

## 4. Integration

- [x] 4.1 Wrap `RootContent.kt` content with `AltairThemeProvider`
- [x] 4.2 Update `LoginScreen.kt` to use Altair components
- [x] 4.3 Update `RegisterScreen.kt` to use Altair components
- [x] 4.4 Update `HomeScreen.kt` to use Altair components
- [x] 4.5 Update `ErrorScreen.kt` to use Altair components

## 5. Testing & Documentation

- [x] 5.1 Create component preview composables for visual testing
- [x] 5.2 Verify components compile correctly on JVM target
- [x] 5.3 Verify components render correctly on Desktop (manual testing)
- [x] 5.4 Verify keyboard navigation works on all interactive components (manual testing)
- [x] 5.5 Update `docs/implementation-plan.md` to mark Phase 7.1 complete

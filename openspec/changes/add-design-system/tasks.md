# Tasks: Add Altair Design System

## 1. Dependencies

- [ ] 1.1 Add Compose Unstyled version to `gradle/libs.versions.toml`
- [ ] 1.2 Add Compose Unstyled library entries to version catalog
- [ ] 1.3 Add Compose Unstyled dependency to `composeApp/build.gradle.kts` commonMain

## 2. Theme Foundation

- [ ] 2.1 Create `composeApp/src/commonMain/kotlin/.../ui/theme/` directory
- [ ] 2.2 Create `AltairTheme.kt` with color tokens (backgrounds, text, borders, accent, status)
- [ ] 2.3 Add typography tokens to `AltairTheme` (display, headline, body, label scales)
- [ ] 2.4 Add spacing tokens to `AltairTheme` (xs, sm, md, lg, xl)
- [ ] 2.5 Add radii tokens to `AltairTheme` (sm, md, lg, full)
- [ ] 2.6 Create `AltairThemeProvider.kt` with CompositionLocal setup
- [ ] 2.7 Create `FocusRing.kt` with `Modifier.focusRing()` extension

## 3. Base Components

- [ ] 3.1 Create `composeApp/src/commonMain/kotlin/.../ui/components/` directory
- [ ] 3.2 Create `ButtonVariant.kt` enum (Primary, Secondary, Ghost, Danger)
- [ ] 3.3 Create `AltairButton.kt` wrapping Compose Unstyled Button
- [ ] 3.4 Create `AltairTextField.kt` wrapping Compose Unstyled TextField with label support
- [ ] 3.5 Create `AltairCard.kt` surface container with border and optional elevation
- [ ] 3.6 Create `AltairCheckbox.kt` wrapping Compose Unstyled Checkbox
- [ ] 3.7 Create `AltairDialog.kt` wrapping Compose Unstyled Dialog
- [ ] 3.8 Create `AltairDropdownMenu.kt` wrapping Compose Unstyled Dropdown

## 4. Integration

- [ ] 4.1 Wrap `App.kt` content with `AltairThemeProvider`
- [ ] 4.2 Update `LoginScreen.kt` to use Altair components
- [ ] 4.3 Update `RegisterScreen.kt` to use Altair components
- [ ] 4.4 Update `HomeScreen.kt` to use Altair components
- [ ] 4.5 Update `ErrorScreen.kt` to use Altair components

## 5. Testing & Documentation

- [ ] 5.1 Create component preview composables for visual testing
- [ ] 5.2 Verify components render correctly on Android emulator
- [ ] 5.3 Verify components render correctly on Desktop
- [ ] 5.4 Verify keyboard navigation works on all interactive components
- [ ] 5.5 Update `docs/implementation-plan.md` to mark Phase 7.1 complete

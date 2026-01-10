# SPEC-UI-001: Acceptance Criteria

## Traceability

- **SPEC ID**: SPEC-UI-001
- **Title**: Altair Design System Foundation
- **GitHub Issue**: #42

---

## Definition of Done

- [ ] All design token files created and compile without errors
- [ ] AltairTheme composable provides tokens via CompositionLocal
- [ ] Compose Unstyled dependency added and resolves correctly
- [ ] AltairButton renders all three variants with correct styling
- [ ] AltairTextField renders with proper focus state handling
- [ ] AltairCard renders with hover state (desktop)
- [ ] ComponentPreview screen displays all variants
- [ ] Components verified on desktop target
- [ ] Components verified on Android target
- [ ] Components verified on iOS target
- [ ] No Material Design imports in new files
- [ ] All token values match ADR-008 specification

---

## Test Scenarios

### TS1: Color Token Verification

#### TS1.1: Background Colors Available

**Given** the AltairTheme is applied
**When** a composable accesses AltairTheme.colors
**Then** background color value equals 0xFF0A0A0B
**And** surface color value equals 0xFF141415
**And** surfaceElevated color value equals 0xFF1C1C1E
**And** surfaceHover color value equals 0xFF232326

#### TS1.2: Text Colors Available

**Given** the AltairTheme is applied
**When** a composable accesses AltairTheme.colors
**Then** textPrimary color value equals 0xFFEEEEEF
**And** textSecondary color value equals 0xFF8E8E93
**And** textTertiary color value equals 0xFF636366

#### TS1.3: Accent and Status Colors Available

**Given** the AltairTheme is applied
**When** a composable accesses AltairTheme.colors
**Then** accent color value equals 0xFF6366F1
**And** success color value equals 0xFF22C55E
**And** warning color value equals 0xFFF59E0B
**And** error color value equals 0xFFEF4444

---

### TS2: Typography Token Verification

#### TS2.1: Typography Scale Available

**Given** the AltairTheme is applied
**When** a composable accesses AltairTheme.typography
**Then** displayLarge fontSize equals 32.sp
**And** displayLarge fontWeight equals FontWeight.SemiBold
**And** headlineMedium fontSize equals 20.sp
**And** bodyLarge fontSize equals 16.sp
**And** bodyMedium fontSize equals 14.sp
**And** labelSmall fontSize equals 12.sp

---

### TS3: Spacing Token Verification

#### TS3.1: Spacing Scale Available

**Given** the AltairTheme is applied
**When** a composable accesses AltairTheme.spacing
**Then** xs value equals 4.dp
**And** sm value equals 8.dp
**And** md value equals 16.dp
**And** lg value equals 24.dp
**And** xl value equals 32.dp

---

### TS4: Shape Token Verification

#### TS4.1: Border Radius Scale Available

**Given** the AltairTheme is applied
**When** a composable accesses AltairTheme.shapes
**Then** sm corner radius equals 4.dp
**And** md corner radius equals 6.dp
**And** lg corner radius equals 8.dp
**And** full corner radius equals 9999.dp

---

### TS5: AltairButton Component

#### TS5.1: Primary Button Rendering

**Given** the AltairTheme is applied
**When** an AltairButton with variant Primary is rendered
**Then** the button background color is accent (0xFF6366F1)
**And** the button text color is textPrimary
**And** the button has no visible border

#### TS5.2: Secondary Button Rendering

**Given** the AltairTheme is applied
**When** an AltairButton with variant Secondary is rendered
**Then** the button background color is surface (0xFF141415)
**And** the button has a 1dp border with border color
**And** the button text color is textPrimary

#### TS5.3: Ghost Button Rendering

**Given** the AltairTheme is applied
**When** an AltairButton with variant Ghost is rendered
**Then** the button background is transparent
**And** the button has no visible border
**And** the button text color is textSecondary

#### TS5.4: Button Focus State

**Given** an AltairButton is rendered
**When** the button receives keyboard focus
**Then** a focus ring with borderFocused color (0xFF6366F1) appears
**And** the focus ring has 2dp width

#### TS5.5: Button Disabled State

**Given** an AltairButton with enabled=false is rendered
**When** the button is displayed
**Then** the button appears with reduced opacity
**And** click events are not processed

#### TS5.6: Button Click Handler

**Given** an AltairButton with an onClick handler
**When** the user clicks the button
**Then** the onClick handler is invoked

---

### TS6: AltairTextField Component

#### TS6.1: TextField Default Rendering

**Given** the AltairTheme is applied
**When** an AltairTextField is rendered in unfocused state
**Then** the background color is surface
**And** the border color is border (0xFF2E2E32)
**And** the border radius is md (6.dp)

#### TS6.2: TextField Focus State

**Given** an AltairTextField is rendered
**When** the text field receives focus
**Then** the border color changes to borderFocused (0xFF6366F1)

#### TS6.3: TextField Placeholder

**Given** an AltairTextField with placeholder text
**When** the text field is empty and unfocused
**Then** the placeholder text is displayed
**And** the placeholder color is textTertiary

#### TS6.4: TextField Input

**Given** an AltairTextField with value and onValueChange
**When** the user types in the text field
**Then** onValueChange is called with the new value
**And** the input text color is textPrimary

---

### TS7: AltairCard Component

#### TS7.1: Card Default Rendering

**Given** the AltairTheme is applied
**When** an AltairCard is rendered
**Then** the background color is surfaceElevated (0xFF1C1C1E)
**And** the border color is border
**And** the border radius is md (6.dp)

#### TS7.2: Card Hover State (Desktop)

**Given** an AltairCard is rendered on desktop
**When** the pointer hovers over the card
**Then** the background color changes to surfaceHover (0xFF232326)

#### TS7.3: Card Content Slot

**Given** an AltairCard with content
**When** the card is rendered
**Then** the content is displayed within the card boundaries
**And** the content inherits the AltairTheme context

---

### TS8: Theme Provider

#### TS8.1: Theme Wrapping

**Given** a composable tree wrapped in AltairTheme
**When** any descendant composable accesses AltairTheme
**Then** the correct token values are returned

#### TS8.2: Theme Not Applied Error

**Given** a composable NOT wrapped in AltairTheme
**When** it attempts to access AltairTheme.colors
**Then** an appropriate error or default is provided

---

### TS9: Preview Screen

#### TS9.1: All Button Variants Displayed

**Given** the ComponentPreview composable
**When** it is rendered
**Then** Primary, Secondary, and Ghost button variants are visible
**And** each variant is labeled

#### TS9.2: TextField States Displayed

**Given** the ComponentPreview composable
**When** it is rendered
**Then** AltairTextField is visible
**And** placeholder text example is shown

#### TS9.3: Card Example Displayed

**Given** the ComponentPreview composable
**When** it is rendered
**Then** AltairCard with sample content is visible

---

### TS10: Cross-Platform Verification

#### TS10.1: Desktop Rendering

**Given** the application runs on desktop (JVM)
**When** the ComponentPreview is displayed
**Then** all components render with correct styling
**And** keyboard focus navigation works
**And** hover states activate on pointer hover

#### TS10.2: Android Rendering

**Given** the application runs on Android
**When** the ComponentPreview is displayed
**Then** all components render with correct styling
**And** touch interactions work correctly

#### TS10.3: iOS Rendering

**Given** the application runs on iOS
**When** the ComponentPreview is displayed
**Then** all components render with correct styling
**And** touch interactions work correctly

---

### TS11: Negative Tests

#### TS11.1: No Material Imports

**Given** the new design system files
**When** analyzing import statements
**Then** no androidx.compose.material3 imports exist

#### TS11.2: No Hardcoded Colors

**Given** the styled component implementations
**When** analyzing color usage
**Then** all colors reference AltairTheme.colors properties
**And** no raw Color(0xFF...) values exist in component code

#### TS11.3: No Hardcoded Dimensions

**Given** the styled component implementations
**When** analyzing dimension usage
**Then** spacing references AltairTheme.spacing properties
**And** shapes reference AltairTheme.shapes properties

---

## Quality Gate Criteria

### Code Quality

- [ ] All files follow Kotlin coding conventions
- [ ] KDoc present on public API surfaces
- [ ] No compiler warnings
- [ ] No deprecated API usage

### Visual Quality

- [ ] Colors match ADR-008 exactly (verified via color picker)
- [ ] Typography renders correctly on all platforms
- [ ] Spacing consistent across components
- [ ] Focus indicators clearly visible

### Accessibility

- [ ] Focus ring visible on keyboard navigation
- [ ] Sufficient color contrast (WCAG AA minimum)
- [ ] Interactive elements have appropriate touch targets (48dp minimum)

### Performance

- [ ] No unnecessary recompositions during static display
- [ ] Theme access does not cause recomposition cascade
- [ ] Preview renders without lag

---

## Verification Methods

### Manual Verification

1. **Visual Inspection**: Run ComponentPreview and compare against ADR-008 specifications
2. **Keyboard Navigation**: Tab through components, verify focus rings
3. **Color Verification**: Use color picker tool to verify exact hex values
4. **Cross-Platform**: Run on all three targets and compare screenshots

### Automated Verification (Future)

- Screenshot tests comparing against baseline images
- Unit tests for token value assertions
- Compose UI tests for interaction behavior

---

## Sign-Off Checklist

| Criterion                   | Verified | Notes |
| --------------------------- | -------- | ----- |
| Tokens match ADR-008        | [ ]      |       |
| AltairButton all variants   | [ ]      |       |
| AltairTextField focus state | [ ]      |       |
| AltairCard hover state      | [ ]      |       |
| Desktop rendering           | [ ]      |       |
| Android rendering           | [ ]      |       |
| iOS rendering               | [ ]      |       |
| No Material imports         | [ ]      |       |
| Preview screen complete     | [ ]      |       |

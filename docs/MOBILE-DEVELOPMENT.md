# Mobile Development Guide

> Quick reference for developing and testing Altair Guidance on mobile platforms (Android & iOS)

## Platform Status

### Android ✅

- **Status**: Enabled and configured
- **Bundle ID**: `com.getaltair.altair_guidance`
- **Min SDK**: Configured via Flutter
- **Target SDK**: Latest via Flutter

### iOS ✅

- **Status**: Enabled and configured
- **Bundle ID**: `com.getaltair.altairGuidance`
- **Deployment Target**: iOS 13.0
- **Display Name**: "Altair Guidance"

## Known Issues

### Android Build Issue (Arch Linux) - ✅ RESOLVED

**Problem (Historical):**

```
Error resolving plugin [id: 'dev.flutter.flutter-plugin-loader', version: '1.0.0']
> Configuring project with invalid directory
  Configuring project ':' without an existing directory is not allowed.
  The configured projectDirectory '/usr/lib/flutter/packages/flutter_tools/gradle'
  does not exist, can't be written to or is not a directory.
```

**Cause:**
The Arch Linux AUR Flutter package installs Flutter system-wide in `/usr/lib/flutter`, which is owned by root. Gradle's composite build feature (used in `android/settings.gradle.kts`) requires write access to the included build directory for caching and configuration.

**✅ Solution Implemented:**

Switched to **user-installed Flutter** in home directory:

```bash
# Clone Flutter to user directory
cd ~
git clone https://github.com/flutter/flutter.git -b stable flutter

# Add to PATH permanently
echo 'export PATH="$HOME/flutter/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
flutter --version
# Flutter 3.35.6 • channel stable

# Clean and rebuild
cd /path/to/altair_guidance
flutter clean
flutter build apk --debug

# ✓ Built build/app/outputs/flutter-apk/app-debug.apk (147MB)
```

**Result:** Android builds now work perfectly! No permission issues.

---

## Development Workflow

### Testing Mobile UI on Desktop

Since Android builds are currently blocked on Arch Linux, use desktop for mobile UI testing:

```bash
# Run on Linux desktop
flutter run -d linux

# The app should be responsive and work on smaller window sizes
# Resize window to mobile dimensions (e.g., 375x667 for iPhone)
```

### iOS Development (macOS Required)

```bash
# Run on iOS simulator (macOS only)
flutter run -d "iPhone 15"

# Build for iOS device
flutter build ios --release
```

### Android Development (When Build Issue Resolved)

```bash
# List available emulators
flutter emulators

# Launch emulator
flutter emulators --launch Medium_Phone_API_36.1

# Run on emulator
flutter run -d emulator-5554

# Build APK
flutter build apk --release
```

---

## Mobile-Specific Considerations

### 1. Screen Sizes

**Target Dimensions:**

- **Phone (Portrait)**: 375x667 (iPhone 8), 393x852 (Pixel 7)
- **Phone (Landscape)**: 667x375, 852x393
- **Tablet**: 768x1024 (iPad), 820x1180 (Pixel Tablet)

**Responsive Breakpoints:**

```dart
// In altair-ui package
const mobileBreakpoint = 600;  // Phone
const tabletBreakpoint = 900;  // Tablet
const desktopBreakpoint = 1200; // Desktop
```

### 2. Touch Interactions

**Minimum Touch Targets:**

- **Buttons**: 44x44 pt (iOS), 48x48 dp (Android)
- **Interactive elements**: Same as buttons
- **Spacing**: 8dp minimum between touch targets

**Gestures to Support:**

- Tap
- Long press
- Swipe (for navigation, dismissing items)
- Pull-to-refresh
- Pinch-to-zoom (for specific views like task details)

### 3. Performance

**Target Metrics:**

- **Frame rate**: 60 FPS minimum
- **App startup**: < 3 seconds cold start
- **Quick Capture**: < 1 second to open, < 3 seconds thought-to-save

**Optimization Techniques:**

- Lazy loading for long lists
- Image caching
- Minimize widget rebuilds
- Use `const` constructors where possible

### 4. Platform-Specific Features

**iOS:**

- Cupertino-style navigation when appropriate
- Haptic feedback for important actions
- Dynamic Type support
- Dark mode support

**Android:**

- Material Design 3 components
- Android-style back navigation
- App shortcuts
- Widget support (future consideration)

---

## UI Components Review

### Components to Test on Mobile

From `packages/altair-ui`:

1. **AltairButton**
   - ✅ Touch target size adequate?
   - ✅ Responsive to screen width?

2. **Quick Capture Widget**
   - ✅ Keyboard appearance on mobile?
   - ✅ Submit on mobile keyboard?

3. **Task Card**
   - ✅ Readable on small screens?
   - ✅ Touch targets for actions?

4. **Navigation**
   - ✅ Mobile nav patterns (bottom nav vs drawer)?
   - ✅ Gesture navigation support?

5. **Forms**
   - ✅ Mobile keyboard types?
   - ✅ Input validation visible?

---

## Testing Results (October 22, 2025)

### Android Testing on Emulator (API 36, 1080x2400)

**✅ Successful Tests:**

- App launches successfully without crashes
- AI Service initializes correctly (<http://localhost:8001/api>)
- Task system loads properly (0 tasks)
- Neo-brutalist UI renders correctly on mobile screen
- Orange accent theme visible and vibrant
- Floating Action Button (FAB) properly positioned
- Top navigation bar responsive with accessible icons
- Empty state displays with clear messaging

**📱 Mobile UI Optimizations Applied:**

- ✅ Quick Capture placeholder now platform-aware
  - Mobile (Android/iOS): "Quick capture..."
  - Desktop (Linux/macOS/Windows): "Quick capture (Ctrl/Cmd + K)..."
  - Fixed at `main.dart:532-534`

- ✅ Settings page with theme controls
  - Light/Dark/System theme options
  - Clean modal bottom sheet design
  - Moved from AppBar for cleaner mobile UI

- ✅ Platform-aware keyboard shortcuts
  - Desktop: Full keyboard shortcuts enabled
  - Mobile: Shortcuts disabled, tooltips hide keyboard hints
  - Conditional wrapping at `main.dart:673-739`

**🎯 Touch Interactions Implemented:**

- ✅ **Swipe-to-delete** - Swipe left on any task to delete
  - Red background with delete icon appears
  - Confirmation dialog prevents accidental deletions
  - Neo-brutalist styling consistent with app theme

- ✅ **Long-press menu** - Hold any task to show actions
  - Bottom sheet with Edit, Complete/Incomplete, Delete
  - Mobile-friendly modal interface
  - Quick access to common task actions

- ✅ **Pull-to-refresh** - Pull down list to reload tasks
  - Works on both full list and empty state
  - Material Design refresh indicator
  - Smooth animation and feedback

**⚙️ Platform Features Implemented:**

- ✅ **SafeArea Protection** - Content respects notches and system UI
  - Wraps main content in SafeArea widget
  - Prevents overlap with status bar, notch, home indicator
  - Automatic handling on all platforms

- ✅ **iOS Keyboard Dismissal** - Tap outside to dismiss keyboard
  - GestureDetector wrapping content
  - Unfocus on tap anywhere outside input
  - Native iOS behavior

- ✅ **Android Back Button** - Smart back navigation
  - PopScope handles back button presses
  - Dismisses keyboard if focused
  - Otherwise allows normal back navigation

- ✅ **Status Bar Styling** - Clean, modern appearance
  - Transparent status bar (edge-to-edge)
  - Dark icons for light mode (better contrast)
  - White navigation bar (Android)
  - SystemChrome configuration at app startup

**📊 Screen Resolution:**

- Physical size: 1080x2400 (typical modern Android phone)
- UI scales appropriately for mobile viewport

---

## Testing Checklist

### Phase 1.5 Step 2: Mobile Optimization

- [x] **UI Responsiveness**
  - [x] All screens adapt to mobile widths
  - [x] No horizontal scrolling required
  - [x] Text readable without zooming
  - [x] Touch targets meet minimum sizes

- [x] **Platform-Specific UI**
  - [x] Quick Capture placeholder mobile-appropriate
  - [x] FAB positioned correctly for mobile
  - [x] Navigation icons accessible on mobile

- [x] **Touch Interactions**
  - [x] All buttons tappable
  - [x] Swipe-to-delete for tasks (with confirmation dialog)
  - [x] Long-press context menu (Edit, Complete/Incomplete, Delete)
  - [x] Pull-to-refresh on task list (including empty state)

- [x] **Performance**
  - [x] Performance testing infrastructure setup
  - [x] Performance overlay available for debugging
  - [x] Code analyzed for anti-patterns (44+ const widgets)
  - [x] Comprehensive performance testing guide created
  - [ ] Baseline metrics measured on device
  - [ ] Performance targets validated

- [x] **Platform Features**
  - [x] iOS keyboard dismissal (tap outside to dismiss)
  - [x] Android back button handling (PopScope with keyboard unfocus)
  - [x] Status bar styling (transparent with dark icons)
  - [x] Safe area insets (SafeArea widget wrapping content)

- [ ] **Testing Coverage**
  - [x] Unit tests pass
  - [ ] Widget tests pass
  - [ ] Integration tests on mobile
  - [ ] Manual testing on real device

---

## Next Steps

1. **Resolve Android build issue** on Arch Linux
2. **Test on physical Android device** when builds work
3. **Test on iOS simulator** (requires macOS)
4. **Add mobile-specific integration tests**
5. **Create mobile CI/CD pipeline**
6. **Document platform-specific gotchas**

---

## Resources

- [Performance Testing Guide](../apps/altair_guidance/docs/PERFORMANCE-TESTING.md) - Comprehensive performance testing and optimization guide
- [Flutter Platform Integration](https://docs.flutter.dev/platform-integration)
- [Material Design for Mobile](https://m3.material.io/)
- [iOS Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/)
- [Android Design Guidelines](https://developer.android.com/design)

---

**Status**: 🚧 Phase 1.5 Step 1 Complete (iOS Setup), Step 2 In Progress (Mobile Optimization)
**Last Updated**: October 22, 2025

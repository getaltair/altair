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

## Testing Checklist

### Phase 1.5 Step 2: Mobile Optimization

- [ ] **UI Responsiveness**
  - [ ] All screens adapt to mobile widths
  - [ ] No horizontal scrolling required
  - [ ] Text readable without zooming
  - [ ] Touch targets meet minimum sizes

- [ ] **Touch Interactions**
  - [ ] All buttons tappable
  - [ ] Swipe gestures work where expected
  - [ ] Long press actions available
  - [ ] Pull-to-refresh implemented

- [ ] **Performance**
  - [ ] App starts in < 3 seconds
  - [ ] 60 FPS maintained during scrolling
  - [ ] Quick Capture < 1 second
  - [ ] No jank or stuttering

- [ ] **Platform Features**
  - [ ] iOS keyboard dismissal works
  - [ ] Android back button works
  - [ ] Status bar styling correct
  - [ ] Safe area insets respected

- [ ] **Testing Coverage**
  - [ ] Unit tests pass
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

- [Flutter Platform Integration](https://docs.flutter.dev/platform-integration)
- [Material Design for Mobile](https://m3.material.io/)
- [iOS Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/)
- [Android Design Guidelines](https://developer.android.com/design)

---

**Status**: 🚧 Phase 1.5 Step 1 Complete (iOS Setup), Step 2 In Progress (Mobile Optimization)
**Last Updated**: October 22, 2025

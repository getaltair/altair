# Device Testing Guide

> Comprehensive guide for testing Altair Guidance on physical Android and iOS devices

## Quick Reference

**Prerequisites:**

- Physical Android or iOS device
- USB cable
- Developer mode enabled on device
- For iOS: macOS with Xcode installed

**Testing Priorities:**

1. ✅ Core functionality (create/edit/delete tasks)
2. ✅ Touch interactions (swipe, long-press, pull-to-refresh)
3. ✅ Performance baselines (startup time, frame rate)
4. ✅ Platform-specific features (keyboard, back button)

---

## Android Device Testing

### Setup

#### 1. Enable Developer Options

On your Android device:

```
1. Go to Settings → About phone
2. Tap "Build number" 7 times
3. Developer options will be unlocked
4. Go to Settings → Developer options
5. Enable "USB debugging"
6. (Optional) Enable "Stay awake" for testing
```

#### 2. Connect Device

```bash
# Connect device via USB
# Accept USB debugging prompt on device

# Verify device is connected
flutter devices

# Expected output:
# Android SDK built for x86 64 • emulator-5554 • android-x64    • Android 12 (API 31) (emulator)
# Pixel 7 • 1A2B3C4D5E6F • android-arm64 • Android 14 (API 34)
```

#### 3. Install and Run

```bash
cd apps/altair_guidance

# Install debug build
flutter run -d <device-id>

# Or build and install manually
flutter build apk --debug
adb install build/app/outputs/flutter-apk/app-debug.apk
```

### Android Testing Checklist

#### Basic Functionality

- [ ] **App Launches**
  - App starts without crashes
  - Splash screen displays correctly
  - Initial load time < 3 seconds
  - No ANR (Application Not Responding) errors

- [ ] **Quick Capture**
  - FAB accessible and tappable
  - Keyboard appears on tap
  - Text input works
  - "Quick capture..." placeholder shows
  - Enter key creates task
  - Field clears after creation

- [ ] **Task List**
  - Tasks display correctly
  - Scrolling is smooth (60 FPS)
  - Empty state shows when no tasks
  - Pull-to-refresh works
  - Loading states display

#### Touch Interactions

- [ ] **Swipe to Delete**
  - Swipe left reveals delete action
  - Red background appears
  - Delete icon visible
  - Confirmation dialog appears
  - Task deletes on confirm
  - Swipe gesture is responsive

- [ ] **Long Press**
  - Long press shows context menu
  - Bottom sheet appears
  - Edit, Complete/Incomplete, Delete options visible
  - Actions work correctly
  - Sheet dismisses properly

- [ ] **Checkbox Toggle**
  - Checkbox taps register correctly
  - Status updates immediately
  - Visual feedback on change
  - No lag or delay

#### Platform Features

- [ ] **Navigation**
  - Back button dismisses keyboard
  - Back button exits app appropriately
  - Navigation drawer accessible
  - Settings accessible

- [ ] **Keyboard**
  - Keyboard appears on text input
  - Tap outside dismisses keyboard
  - Keyboard doesn't cover input fields
  - Auto-capitalization works

- [ ] **Status Bar**
  - Status bar styling correct
  - Edge-to-edge display
  - Safe area respected
  - Notch/camera cutout handled

#### Performance Testing

Run these tests and record results:

```bash
# Enable performance overlay
# In app: Triple-tap on title to show performance overlay

# Measure:
# - Startup time (cold start)
# - Frame rate during scrolling
# - Memory usage
# - Battery drain over 30 minutes
```

**Performance Targets:**

- **Cold start**: < 3 seconds
- **Frame rate**: 60 FPS (16.67ms per frame)
- **Memory**: < 150MB for basic usage
- **Battery**: < 5% drain per hour active use

**Record Baselines:**

| Metric | Target | Actual | Device |
|--------|--------|--------|--------|
| Cold start | < 3s | ___ | ___ |
| Hot reload | < 1s | ___ | ___ |
| Frame rate | 60 FPS | ___ | ___ |
| Memory (idle) | < 100MB | ___ | ___ |
| Memory (active) | < 150MB | ___ | ___ |

#### Screen Sizes

Test on multiple screen sizes if possible:

- [ ] **Small phone** (< 5.5" diagonal)
- [ ] **Medium phone** (5.5" - 6.5")
- [ ] **Large phone** (> 6.5")
- [ ] **Tablet** (7"+)

Record any UI issues with specific screen sizes.

---

## iOS Device Testing

### Setup

#### 1. Requirements

- macOS computer
- Xcode 15.0+ installed
- Apple Developer account (free tier works)
- iOS device with iOS 13.0+
- USB cable (Lightning or USB-C)

#### 2. Xcode Configuration

```bash
# Open Xcode and go to Settings → Accounts
# Add your Apple ID
# Select your team

# In project settings (apps/altair_guidance/ios/Runner.xcworkspace):
# 1. Select Runner target
# 2. Signing & Capabilities
# 3. Team: Select your team
# 4. Bundle Identifier: com.getaltair.altairGuidance
```

#### 3. Device Setup

On iOS device:

```
1. Connect via USB to Mac
2. Trust computer when prompted
3. Settings → General → VPN & Device Management
4. Trust your developer account
```

#### 4. Build and Run

```bash
cd apps/altair_guidance

# List available devices
flutter devices

# Run on connected iOS device
flutter run -d <device-id>

# Or build and install via Xcode:
# 1. Open ios/Runner.xcworkspace in Xcode
# 2. Select device from device dropdown
# 3. Press Run (Cmd+R)
```

### iOS Testing Checklist

#### Basic Functionality

- [ ] **App Launches**
  - App starts without crashes
  - Launch screen displays
  - Initial load < 3 seconds
  - No system alerts

- [ ] **Quick Capture**
  - FAB accessible
  - Keyboard appears
  - Return key creates task
  - Placeholder text correct
  - Haptic feedback on actions

- [ ] **Task List**
  - Tasks display correctly
  - Scrolling smooth
  - Rubber-band effect works
  - Pull-to-refresh works
  - Dynamic Type supported

#### Touch Interactions

- [ ] **Swipe Gestures**
  - Swipe to delete works
  - Gesture is iOS-like
  - Haptic feedback present
  - Animation smooth

- [ ] **Long Press**
  - Context menu appears
  - iOS-style sheet
  - Actions work
  - Haptic feedback

- [ ] **3D Touch/Haptic Touch** (if device supports)
  - Quick actions work
  - Preview works
  - Peek and pop functional

#### Platform Features

- [ ] **Navigation**
  - Swipe from edge goes back
  - Navigation feels iOS-native
  - Settings accessible
  - Modal dismissal works

- [ ] **Keyboard**
  - iOS keyboard appears
  - Tap outside dismisses
  - Keyboard toolbar present
  - Shortcuts work

- [ ] **Status Bar**
  - Status bar styling correct
  - Notch handled properly
  - Safe area respected
  - Dynamic Island compatible (iPhone 14 Pro+)

- [ ] **Dark Mode**
  - Switches with system
  - All screens support dark mode
  - Colors appropriate

#### Performance Testing

**Performance Targets:**

- **Cold start**: < 2.5 seconds
- **Frame rate**: 60 FPS (120 FPS on ProMotion displays)
- **Memory**: < 120MB for basic usage
- **Battery**: < 4% drain per hour

**Record Baselines:**

| Metric | Target | Actual | Device |
|--------|--------|--------|--------|
| Cold start | < 2.5s | ___ | ___ |
| Frame rate | 60/120 FPS | ___ | ___ |
| Memory (idle) | < 80MB | ___ | ___ |
| Memory (active) | < 120MB | ___ | ___ |

#### Device Models

Test on multiple models if possible:

- [ ] **iPhone SE** (small screen)
- [ ] **iPhone 13/14/15** (standard)
- [ ] **iPhone 13/14/15 Pro Max** (large, ProMotion)
- [ ] **iPad** (tablet size)

---

## Performance Measurement

### Using Flutter DevTools

1. **Start DevTools:**

   ```bash
   # Run app in profile mode
   flutter run --profile -d <device-id>

   # Open DevTools
   flutter pub global activate devtools
   flutter pub global run devtools
   ```

1. **Measure Metrics:**

   - **Timeline View**: Track frame rendering times
   - **Memory View**: Monitor memory usage and leaks
   - **Performance View**: CPU profiling
   - **Network View**: API call timing

1. **Record Issues:**

```
Frame Rate Issues:
- Location: [screen/action]
- FPS observed: [number]
- Expected: 60 FPS

Memory Issues:
- Location: [screen/action]
- Memory: [MB]
- Expected: < 150MB Android, < 120MB iOS
```

### Manual Performance Tests

#### Test 1: Cold Start

```bash
# Fully close app
# Clear from recent apps
# Launch app and time until usable

# Record time in seconds
```

#### Test 2: Scrolling Performance

```bash
# Create 50+ tasks
# Scroll rapidly up and down
# Note any frame drops
# Enable performance overlay to see FPS
```

#### Test 3: Quick Capture Speed

```bash
# From empty state
# Tap FAB
# Start timer
# Type text
# Press enter
# Stop timer when task appears

# Target: < 3 seconds total
```

#### Test 4: Memory Over Time

```bash
# Open DevTools memory view
# Use app normally for 10 minutes:
#   - Create 20 tasks
#   - Edit 10 tasks
#   - Delete 5 tasks
#   - Browse settings
#   - Use AI features (if enabled)
# Record peak memory usage
```

---

## Reporting Issues

### Issue Template

When you find a device-specific issue, report it with:

```markdown
**Device**: [Manufacturer Model, OS Version]
**Build**: [Debug/Release, Build number/commit SHA]
**Issue**: [Brief description]

**Steps to Reproduce**:
1. [Step 1]
2. [Step 2]
3. [Step 3]

**Expected**: [What should happen]
**Actual**: [What actually happens]

**Screenshots**: [If applicable]
**Logs**: [Attach relevant logs]

**Performance Impact**:
- Frame rate: [FPS]
- Memory: [MB]
- Crashes: [Yes/No]

**Workaround**: [If any]
```

### Getting Logs

**Android:**

```bash
# Real-time logs
adb logcat -s flutter

# Save logs to file
adb logcat -s flutter > android-logs.txt
```

**iOS:**

```bash
# Using Xcode Console (Window → Devices and Simulators → View Device Logs)
# Or using command line:
idevicesyslog > ios-logs.txt
```

---

## Automated Testing on Devices

### Using Firebase Test Lab (Android)

```bash
# Build test APK
flutter build apk

# Build instrumentation test APK
pushd android
./gradlew app:assembleAndroidTest
./gradlew app:assembleDebug -Ptarget=integration_test/app_test.dart
popd

# Submit to Firebase Test Lab
gcloud firebase test android run \
  --type instrumentation \
  --app build/app/outputs/apk/debug/app-debug.apk \
  --test build/app/outputs/apk/androidTest/debug/app-debug-androidTest.apk \
  --device model=Pixel7,version=33,locale=en,orientation=portrait
```

### Using BrowserStack/Sauce Labs

For cross-device testing at scale, consider cloud device farms:

- **BrowserStack**: App testing on 3000+ real devices
- **Sauce Labs**: Automated mobile testing
- **AWS Device Farm**: Test on AWS-managed devices

---

## Best Practices

### Before Testing

1. **Update Device OS**: Test on latest stable OS version
2. **Clear Storage**: Uninstall previous builds
3. **Enable Developer Tools**: Performance overlay, layout bounds
4. **Charge Device**: Keep above 50% battery

### During Testing

1. **Systematic Approach**: Follow checklist order
2. **Document Everything**: Screenshots, videos, notes
3. **Test Edge Cases**: Slow networks, low battery, interruptions
4. **Vary Conditions**: Portrait/landscape, light/dark mode

### After Testing

1. **File Issues**: Create GitHub issues for bugs found
2. **Update Baselines**: Record performance metrics
3. **Share Findings**: Post to team chat/Discord
4. **Iterate**: Retest after fixes

---

## Performance Baselines Registry

Document your device testing results here:

### Android Devices

| Device | OS | Build | Cold Start | FPS | Memory | Tester | Date |
|--------|----|----|-----------|-----|--------|--------|------|
| _Example: Pixel 7_ | _14_ | _debug_ | _2.3s_ | _60_ | _128MB_ | _@user_ | _2025-10-22_ |
| | | | | | | | |

### iOS Devices

| Device | OS | Build | Cold Start | FPS | Memory | Tester | Date |
|--------|----|----|-----------|-----|--------|--------|------|
| _Example: iPhone 15_ | _17.0_ | _debug_ | _2.1s_ | _60_ | _95MB_ | _@user_ | _2025-10-22_ |
| | | | | | | | |

---

## Resources

- [Flutter Performance Profiling](https://docs.flutter.dev/perf/ui-performance)
- [Android Debug Bridge (adb)](https://developer.android.com/tools/adb)
- [Xcode Instruments](https://developer.apple.com/xcode/features/)
- [Firebase Test Lab](https://firebase.google.com/docs/test-lab)
- [Performance Testing Guide](./apps/altair_guidance/docs/PERFORMANCE-TESTING.md)

---

**Last Updated**: October 22, 2025
**Maintained By**: Altair Development Team
**Questions?**: Open an issue or ask in Discord

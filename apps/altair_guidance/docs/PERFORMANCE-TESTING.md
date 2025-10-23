# Performance Testing Guide

> Comprehensive guide for measuring and optimizing Altair Guidance performance

## Performance Targets

### Mobile Performance Goals

| Metric | Target | Critical |
|--------|--------|----------|
| **Cold Start** | < 3 seconds | < 5 seconds |
| **Frame Rate** | 60 FPS | 30 FPS minimum |
| **Quick Capture Response** | < 1 second | < 2 seconds |
| **Task List Scroll** | 60 FPS | 45 FPS |
| **Memory Usage** | < 150 MB | < 250 MB |

## Testing Methods

### 1. Using Flutter Performance Overlay

**Enable in Debug Mode:**

```dart
// In main.dart MaterialApp:
showPerformanceOverlay: true,  // Toggle to enable
```

**What to Look For:**

- **Green bars** = Good (under 16ms per frame for 60 FPS)
- **Red/Yellow bars** = Frame drops, jank detected
- **GPU thread** (top) should stay under 16ms
- **UI thread** (bottom) should stay under 16ms

### 2. Using Flutter DevTools

**Launch DevTools:**

```bash
# Run app in profile mode
flutter run --profile

# Open DevTools (follow URL in terminal)
```

**Performance Tab Features:**

- Timeline view for frame analysis
- CPU profiler for hot spots
- Memory profiler for leaks
- Network profiler for API calls

### 3. Command Line Profiling

**Profile Mode Build:**

```bash
# Android
flutter build apk --profile
flutter install --profile

# iOS
flutter build ios --profile

# Linux (for testing)
flutter run -d linux --profile
```

### 4. Startup Time Measurement

**Manual Timing:**

1. Clear app from memory
2. Start timer
3. Launch app
4. Stop when first interactive frame appears
5. Record time

**Automated (Future):**

```dart
// Add to main():
void main() {
  final startTime = DateTime.now();

  runApp(const AltairGuidanceApp());

  WidgetsBinding.instance.addPostFrameCallback((_) {
    final loadTime = DateTime.now().difference(startTime);
    print('App startup time: ${loadTime.inMilliseconds}ms');
  });
}
```

## Performance Optimizations Applied

### Code-Level Optimizations

✅ **Const Constructors**

- 44+ const widgets in main.dart
- Reduces widget rebuild overhead
- Improves memory efficiency

✅ **BLoC Pattern**

- Efficient state management
- Prevents unnecessary rebuilds
- Clear separation of concerns

✅ **Platform-Specific Code**

- Keyboard shortcuts only on desktop
- Conditional widget trees for mobile/desktop
- Reduces runtime overhead on mobile

### Widget Optimizations

✅ **ReorderableListView**

- Efficient list rendering
- Minimal rebuilds on reorder
- Custom drag handles

✅ **RefreshIndicator**

- Native pull-to-refresh
- Async state updates
- Minimal performance impact

✅ **Dismissible**

- Hardware-accelerated swipe
- Efficient item removal
- Confirmation dialogs prevent accidents

## Common Performance Issues & Solutions

### Issue: Slow Scrolling

**Symptoms:**

- Frame drops during scroll
- Jank or stuttering
- Performance overlay shows red bars

**Solutions:**

1. Use `ListView.builder` instead of `ListView`
2. Add `const` to child widgets
3. Reduce widget tree complexity
4. Cache expensive computations

**Current Status:** ✅ Using `ReorderableListView.builder`

### Issue: Slow Startup

**Symptoms:**

- App takes > 3 seconds to launch
- Splash screen shows too long
- Blank screen before UI

**Solutions:**

1. Lazy-load dependencies
2. Defer non-critical initialization
3. Use `async` initialization
4. Minimize work in `main()`

**Current Status:** ✅ Minimal `main()`, async BLoC initialization

### Issue: Memory Leaks

**Symptoms:**

- Memory usage grows over time
- App slows down after use
- Crashes on low-memory devices

**Solutions:**

1. Dispose controllers/streams
2. Cancel subscriptions
3. Use `AutoDispose` for BLoCs
4. Profile with DevTools

**Current Status:** ✅ Proper disposal in StatefulWidgets

## Performance Testing Checklist

### Manual Testing

- [ ] **Cold Start Test**
  - Close app completely
  - Clear from recent apps
  - Launch and time to interactive
  - Target: < 3 seconds

- [ ] **Scroll Performance Test**
  - Create 50+ tasks
  - Scroll rapidly up and down
  - Check for jank or stuttering
  - Target: Smooth 60 FPS

- [ ] **Quick Capture Test**
  - Tap Quick Capture
  - Measure keyboard appearance time
  - Type and submit task
  - Target: < 1 second total

- [ ] **Navigation Test**
  - Navigate between pages
  - Check transition smoothness
  - Measure page load time
  - Target: Instant transitions

- [ ] **Gesture Response Test**
  - Swipe-to-delete tasks
  - Long-press for menu
  - Pull-to-refresh
  - Target: Immediate response

### Automated Testing

- [ ] **Integration Tests**
  - Measure frame rates during actions
  - Profile memory usage
  - Check for leaks

- [ ] **Widget Tests**
  - Verify efficient rebuilds
  - Test with large datasets
  - Measure render times

## Performance Monitoring in Production

### Metrics to Track

1. **Crash Rate** - Should be < 0.1%
2. **ANR Rate** (Android) - Should be < 0.1%
3. **Frame Rate** - Should average > 55 FPS
4. **Startup Time** - P50 < 2s, P95 < 4s
5. **Memory Usage** - Stable over time

### Tools for Production

- Firebase Performance Monitoring
- Sentry for error tracking
- Custom analytics events
- User feedback

## Next Steps

1. Run baseline performance tests on Android emulator
2. Run tests on iOS simulator (macOS required)
3. Test on physical devices (various models)
4. Profile with DevTools to find bottlenecks
5. Implement performance monitoring in CI/CD
6. Add performance regression tests

---

**Last Updated:** October 22, 2025
**Status:** 🚧 Performance testing infrastructure ready

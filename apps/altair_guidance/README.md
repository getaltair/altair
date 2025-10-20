# Altair Guidance

ADHD-friendly task and project management application.

## Overview

Altair Guidance is the first app in the Altair ecosystem, focusing on task management with features specifically designed for people with ADHD:

- Quick capture (< 3 seconds thought-to-save)
- Visual time tracking for time blindness
- AI-powered task breakdown
- Offline-first architecture
- Neo-brutalist UI for visual clarity

## Features

### Phase 1 (Current)

- ✅ Task management (create, read, update, delete)
- ✅ Quick capture widget (< 3 second task creation)
- ✅ Task list with status filtering
- ✅ Local SQLite storage
- ✅ Neo-brutalist UI theme
- ✅ BLoC state management
- ✅ Offline-first operation

### Planned Features

- AI-powered task breakdown
- Visual time tracking
- Project organization
- Tag-based filtering
- Multi-device sync (optional)

## Running the App

### Prerequisites

- Flutter 3.0+
- For Linux: GTK+ 3.0 development libraries
- For Android: Android SDK and emulator

### Development

```bash
# Navigate to app directory
cd apps/altair_guidance

# Get dependencies
flutter pub get

# Run on Linux
flutter run -d linux

# Run on Android
flutter run -d android
```

### Building

```bash
# Build for Linux
flutter build linux

# Build for Android
flutter build apk
```

## Architecture

### Dependencies

- `altair_core`: Business logic and data models
- `altair_auth`: Authentication (for future sync)
- `altair_ui`: Neo-brutalist UI components and theme
- `flutter_bloc`: State management
- `go_router`: Navigation
- `get_it`: Dependency injection

### Directory Structure

```
lib/
├── main.dart              # App entry point
├── bloc/                  # BLoC state management
│   └── task/             # Task BLoC (events, states, bloc)
└── test/                 # Widget and integration tests
    ├── bloc/             # BLoC tests
    └── widget_test.dart  # App-level widget tests
```

## Development Roadmap

### Month 1: Foundation

- [x] App skeleton with basic structure
- [x] Dependency injection setup (get_it)
- [x] Router configuration (go_router)
- [x] Task list UI

### Month 2: Core Features (Week 5-6 Complete)

- [x] Task CRUD operations
- [x] Quick capture UI
- [x] Task filtering by status
- [x] BLoC state management
- [ ] Project management
- [ ] Advanced search functionality

### Month 3: Advanced Features

- [ ] AI task breakdown
- [ ] Time tracking
- [ ] Settings and preferences
- [ ] Beta testing

## Testing

```bash
# Run unit tests
flutter test

# Run widget tests
flutter test test/widget_test.dart

# Run integration tests
flutter test integration_test/
```

## License

AGPL-3.0-or-later

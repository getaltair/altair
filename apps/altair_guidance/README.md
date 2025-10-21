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

### Completed Features

- ✅ Task management (create, read, update, delete)
- ✅ Quick capture widget (< 3 second task creation)
- ✅ Task list with status filtering
- ✅ Local SQLite storage
- ✅ Neo-brutalist UI theme
- ✅ BLoC state management
- ✅ Offline-first operation
- ✅ **AI-powered task breakdown** (Week 10)
  - Break down complex tasks into manageable subtasks
  - AI-generated time estimates per subtask
  - One-click creation of subtasks from AI suggestions
  - Support for parent-child task hierarchy
- ✅ Tag-based filtering and search
- ✅ Project organization
- ✅ Focus mode and keyboard shortcuts
- ✅ Drag & drop support

### Planned Features

- Visual time tracking
- Task prioritization AI assistance
- Time estimation AI assistance
- Contextual suggestions for task execution
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
- `http`: HTTP client for AI service communication
- `logger`: Logging and debugging

### AI Service Configuration

The app integrates with a FastAPI backend for AI-powered features. Configure via environment variables:

```bash
# Development (default)
AI_SERVICE_URL=http://localhost:8001/api

# Production (requires HTTPS and API key)
AI_SERVICE_URL=https://your-ai-service.com/api
AI_SERVICE_API_KEY=your-secret-key
```

**Security Notes:**

- HTTPS is enforced in release/production builds
- API key authentication using Bearer tokens
- Input validation on all AI requests
- Operation-specific timeouts prevent hanging

### Directory Structure

```
lib/
├── main.dart                    # App entry point
├── bloc/                        # BLoC state management
│   ├── task/                   # Task BLoC (events, states, bloc)
│   ├── ai/                     # AI BLoC (events, states, bloc)
│   ├── project/                # Project BLoC
│   └── tag/                    # Tag BLoC
├── services/                    # Service layer
│   └── ai/                     # AI service integration
│       ├── ai_config.dart      # Environment-based configuration
│       ├── ai_service.dart     # HTTP client for AI backend
│       └── models.dart         # Request/response models
├── features/                    # Feature-specific UI
│   └── ai/                     # AI features
│       └── task_breakdown_dialog.dart  # Task breakdown UI
├── pages/                       # App pages
│   ├── home_page.dart          # Main task list
│   ├── task_edit_page.dart     # Task editing
│   └── ...
└── test/                       # Tests
    ├── bloc/                   # BLoC tests
    │   ├── task/              # Task BLoC tests
    │   └── ai/                # AI BLoC tests
    ├── services/              # Service tests
    │   └── ai/               # AI service tests (92 tests: 18 config + 39 models + 21 service + 14 BLoC)
    └── widget_test.dart       # App-level widget tests
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

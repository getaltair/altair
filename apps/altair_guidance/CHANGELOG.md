# Changelog

All notable changes to Altair Guidance will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### AI Service Integration (Week 10)

- **AI Configuration System**: Environment-based configuration with development/production factories
  - Bearer token authentication support
  - HTTPS enforcement in production/release mode
  - Operation-specific configurable timeouts (60s breakdown, 45s prioritization, 30s estimation/suggestions, 5s health checks)
  - Base URL validation with proper error messages

- **AI Service HTTP Client**: Comprehensive AI service integration with error handling
  - Task breakdown into subtasks with time estimates
  - Task prioritization suggestions
  - Time estimation for tasks based on skill level
  - Contextual suggestions for task execution
  - Health check endpoint monitoring
  - Specific exception types for timeout and network errors
  - User-friendly error messages

- **AI BLoC State Management**: Complete state management for AI operations
  - Separate operation types (breakdown, prioritization, estimation, suggestions)
  - Loading states with operation context
  - Success and failure states with detailed error information
  - Safe service disposal with error handling

- **Task Breakdown Dialog**: Interactive UI for AI-powered task breakdown
  - Real-time loading states
  - Display of subtasks with order numbers, titles, descriptions, and time estimates
  - AI reasoning display
  - Retry functionality on errors
  - **Create Subtasks** implementation to convert AI suggestions into actual tasks
  - Success feedback via snackbar notifications
  - Support for parent task linking (subtask hierarchy)

- **Input Validation**: Comprehensive validation for all AI request models
  - SkillLevel enum with type-safe conversion
  - Field length limits (500 chars for titles, 5000 for descriptions, 2000 for context)
  - Range validation (1-20 subtasks, valid suggestion types)
  - Required field validation with clear error messages
  - Empty and whitespace string detection

- **Test Coverage**: 92 passing unit tests (100% pass rate)
  - AIConfig tests (18): Factory methods, URL validation, authentication, timeouts, SSL enforcement
  - Request model tests (39): Input validation, JSON serialization, edge cases, boundary values
  - AIService integration tests (21): HTTP client mocking, error handling, timeouts, network errors, authentication
  - AIBloc unit tests (14): State transitions, event handling, error propagation, service disposal

- **Security Enhancements**:
  - API key authentication with Bearer tokens
  - HTTPS enforcement in release/production mode
  - Input sanitization and validation
  - URL format validation

- **Error Handling Improvements**:
  - Timeout exception handling with user-friendly messages
  - Network error detection (SocketException, ClientException)
  - Operation-specific error context
  - Graceful degradation on service failures

#### Standalone Installers (Week 12)

- **Linux AppImage**: Portable single-file installer for all distributions
  - Automatic desktop integration support
  - No external dependencies required
  - Works on Ubuntu, Fedora, Arch, and all major distros
  - Build script with automatic icon generation
  - appimagetool integration for packaging

- **Linux Snap**: Alternative distribution format
  - Snapcraft configuration with strict confinement
  - Automatic dependency management
  - Integration with GNOME extensions
  - Support for desktop, Wayland, and X11

- **Linux AUR (Arch User Repository)**: Native Arch Linux packaging
  - PKGBUILD for building from source
  - Proper dependency management via pacman
  - Desktop integration with .desktop file
  - Installation via AUR helpers (yay, paru)
  - Wrapper script for proper execution
  - License installation to /usr/share/licenses

- **macOS DMG**: Professional drag-and-drop installer
  - Familiar macOS installation experience
  - Applications folder symlink included
  - DMG customization with create-dmg
  - Proper app bundle structure
  - Code signing support (optional)

- **Windows Installer**: Inno Setup-based installer
  - Professional installation wizard
  - Desktop shortcut option
  - Automatic uninstaller registration
  - Start menu integration
  - Portable extraction mode support

- **Cross-Platform Build Script**: Unified build system
  - Automatic platform detection
  - Version management via environment variables
  - Color-coded terminal output
  - Platform-specific validation
  - Comprehensive help documentation

- **CI/CD Integration**: Automated release workflow
  - GitHub Actions release workflow
  - Multi-platform concurrent builds (Linux, macOS, Windows)
  - Automatic artifact upload
  - Draft release creation with download links
  - Manual workflow dispatch support
  - Version tag-based automatic releases

- **Documentation**: Complete installer guide
  - Installation instructions for all platforms
  - Build-from-source instructions
  - Troubleshooting guides
  - CI/CD integration documentation
  - Advanced topics (code signing, portable modes, distribution channels)
  - Release checklist

#### Previous Features (Week 1-9)

- Neo-brutalist UI design system
- Task management (CRUD operations)
- Quick capture functionality
- Offline-first architecture with SQLite
- BLoC state management
- Tag-based filtering and search
- Project organization
- Focus mode and keyboard shortcuts
- Drag & drop support
- Backend AI service integration (FastAPI with OpenAI, Anthropic, Ollama)

### Changed

- Task breakdown dialog now creates actual tasks instead of just displaying suggestions
- AI service configuration moved from hardcoded values to environment-based system
- Error messages improved with specific timeout and network error types

### Fixed

- Critical: Added environment-based configuration for AI service URL
- Critical: Implemented authentication for AI service requests
- High Priority: Added input validation for all request models
- High Priority: Improved error handling with specific exception types
- Completed TODO for "Create Subtasks" functionality in task breakdown dialog

### Testing

- 57 comprehensive unit tests for AI configuration and request models
- All validation logic covered with edge cases
- Authentication and security configuration tested
- Error handling scenarios verified

## [0.1.0] - Initial Release

### Added

- Core task management functionality
- SQLite local storage
- Neo-brutalist UI theme
- BLoC state management
- Flutter application structure

---

*Note: This changelog will be updated as features are completed and released.*

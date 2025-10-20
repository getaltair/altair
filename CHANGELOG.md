# Changelog

All notable changes to the Altair project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Infrastructure (Month 1 Week 1-2)

- Complete monorepo structure with `apps/`, `packages/`, `services/`, and `infrastructure/` directories
- Pre-commit hooks with Ruff, mypy, markdownlint, and gitleaks
- GitHub Actions CI/CD pipeline with:
  - Python linting and type checking
  - Flutter linting and analysis
  - Automated testing with coverage reporting
  - Linux and Android builds
- Project documentation (README.md, CONTRIBUTING.md, CHANGELOG.md)
- AGPL-3.0 license

#### Backend Services

- FastAPI auth-service with:
  - JWT token generation and verification
  - Bcrypt password hashing
  - Placeholder endpoints for login, register, and refresh
  - Health check endpoint
  - Environment-based configuration with Pydantic
  - Type-safe with mypy strict mode

#### Flutter Packages

- **altair-core** package:
  - SQLite schema with migrations support
  - Task, Project, and Tag models with JSON serialization
  - TaskRepository with full CRUD operations
  - Database helper with desktop platform support
  - Date utilities for formatting

- **altair-auth** package:
  - JWT authentication with BLoC state management
  - Secure token storage using flutter_secure_storage
  - HTTP interceptor for automatic token injection
  - Login, register, logout, and refresh flows
  - User model and authentication states

- **altair-ui** package:
  - Neo-brutalist design system:
    - Light and dark themes
    - JetBrains Mono typography
    - High-contrast colors with bright accents
    - Thick borders (2px-6px)
    - No shadows or gradients
  - Design tokens (colors, spacing, typography, borders)
  - Core widgets (AltairButton, AltairCard, AltairTextField)
  - ADHD-friendly visual design principles

#### Applications

- **Altair Guidance** app skeleton:
  - Cross-platform support (Linux, Android, Windows, macOS)
  - Integration with altair-ui theme
  - Welcome screen with neo-brutalist components
  - Dependencies: flutter_bloc, go_router, get_it
  - Ready for Month 2 feature implementation

#### Core Task Management (Month 2)

- **Quick Capture Widget** (AltairQuickCapture):
  - Sub-3-second task creation optimized for ADHD users
  - Auto-focus on mount for immediate input
  - Submit on Enter key for keyboard-first workflow
  - Visual feedback with checkmark animation
  - Clear on submit with automatic refocus
  - Customizable hint text and accent colors
  - Empty and whitespace-only input validation
  - Comprehensive widget tests covering all functionality

- **Task BLoC Integration**:
  - TaskBloc with complete CRUD operations
  - Quick capture event handling with TaskQuickCaptureRequested
  - Task states: Initial, Loading, Loaded, Captured, Failure
  - Integration with TaskRepository for persistence
  - Search and filter functionality by status
  - Comprehensive BLoC tests with mocktail

- **Task List UI**:
  - Real-time task display with status-based accent colors
  - Checkbox for quick task completion
  - Delete functionality with confirmation
  - Empty state guidance for new users
  - Theme-aware colors for consistent UI
  - Success feedback with SnackBar notifications

- **Task Editing & Project Management** (Week 7):
  - Detailed task editing screen with full CRUD operations
  - Project model with status tracking (active, onHold, completed, cancelled)
  - ProjectRepository with CRUD operations, search, and task counting
  - Project BLoC for state management with comprehensive event handling
  - Projects page with list view and creation/editing capabilities
  - Task-to-project assignment with foreign key constraints
  - Project filtering by status
  - Integration of tasks and projects in the UI

- **Tags, Filtering & Search** (Week 7):
  - Tag model with color customization and usage tracking
  - TagRepository with full CRUD operations:
    - Create, read, update, and delete tags
    - Search tags by name
    - Find most-used tags with configurable limit
    - Usage count management (increment/decrement)
    - Batch operations for finding multiple tags by IDs
  - Tag BLoC for state management:
    - Complete event handling for all tag operations
    - Name conflict validation
    - Error handling with detailed logging
  - Tag UI components following neo-brutalist design:
    - AltairTagChip: Reusable tag chip with optional delete and selection
    - AltairTagSelector: Multi-select tag selector with search and inline creation
  - Tag filtering integration:
    - Enhanced TaskRepository with in-memory tag filtering
    - Enhanced ProjectRepository with in-memory tag filtering
    - TaskBloc tag filtering events and states
    - ProjectBloc tag filtering events and states
  - Comprehensive testing:
    - 31 TagRepository tests covering CRUD, search, and edge cases
    - 20 TagBloc tests covering all events and error scenarios

- **UX Polish** (Week 8):
  - **Keyboard Shortcuts**:
    - Comprehensive keyboard shortcuts system for power users
    - Shortcuts for all common actions (Ctrl/Cmd + N for new task, Ctrl/Cmd + K for quick capture, etc.)
    - Help dialog with categorized shortcuts (Shift + ?)
    - Support for both Control (Windows/Linux) and Command (macOS) modifiers
    - External focus node support in AltairQuickCapture widget
    - Action-based architecture using Flutter's Shortcuts and Actions widgets
  - **Focus Mode**:
    - ADHD-friendly focus mode to minimize distractions
    - Toggle with Ctrl/Cmd + D or app bar button
    - Hides drawer, filters, and floating action button when enabled
    - Visual indicator (yellow icon) when active
    - FocusModeCubit for state management
    - Keyboard shortcut integration
  - **Drag & Drop Reordering**:
    - Intuitive drag & drop task reordering using ReorderableListView
    - TaskReorderRequested event for state management
    - Smooth reordering UX with proper index handling
    - Maintains task order in UI state (persistence pending)
    - Unique ValueKey for each task item

#### Testing Infrastructure

- Comprehensive testing framework for all packages:
  - **altair-ui**: Widget tests for Button, Card, TextField, and Quick Capture components
  - **altair-core**:
    - Unit tests for Task model with serialization and equality tests
    - Unit tests for Project model with all properties and JSON serialization
    - Integration tests for TaskRepository (30+ tests covering CRUD, search, subtasks, time tracking)
    - Integration tests for ProjectRepository (comprehensive CRUD, search, edge cases, status transitions)
    - Integration tests for TagRepository (31 tests covering CRUD, search, usage tracking, edge cases)
  - **altair-auth**: BLoC tests for authentication flows with mockito and bloc_test
  - **altair_guidance**:
    - App-level tests for routing, theming, UI, and TaskBloc with mocktail
    - BLoC tests for ProjectBloc (15 tests covering all events and error handling)
    - BLoC tests for TagBloc (20 tests covering all tag operations and error scenarios)
- Testing documentation (docs/TESTING.md) with:
  - Complete testing guide for all test types
  - Best practices and examples
  - Coverage reporting instructions
  - CI/CD integration guidelines
- Helper scripts:
  - `scripts/test-all.sh` - Run all tests across the monorepo
  - `scripts/coverage.sh` - Generate coverage reports for all packages
- Mock generation setup with build_runner for auth package
- Total test suite: 150+ comprehensive tests across all packages

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.1] - 2025-10-17

### Added

- Project initialization
- Development environment setup with mise

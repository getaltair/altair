# Changelog

All notable changes to the Altair project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Database Service Infrastructure (Week 11)

- **altair-db-service** package:
  - Shared local SurrealDB instance for all Altair applications
  - Cross-app data integration with single database process
  - Platform-specific service installers (Linux systemd, macOS launchd, Windows Service)
  - Service lifecycle management (install, start, stop, uninstall)
  - Connection manager singleton for shared database access
  - Auto-start configuration for seamless user experience
  - Health check endpoints for service monitoring
  - Platform-agnostic installer abstraction
  - Secure credential management with system keychain integration
  - Comprehensive documentation with usage examples

- **Database Connection Management**:
  - AltairConnectionManager singleton for unified database access
  - Automatic schema initialization on first connection
  - Cross-app linking system for resource relationships
  - Full-text search across all resource types (tasks, notes, items)
  - Link creation and management between resources
  - Query utilities for common operations

- **SurrealDB Integration**:
  - TaskRepositorySurrealDB implementation in altair-core
  - Complete CRUD operations for tasks using SurrealDB
  - Subtask querying and parent-child relationships
  - Tag-based filtering with ALLINSIDE operator
  - Search functionality with full-text indexing
  - Time tracking support (estimated and actual minutes)
  - Metadata field for extensibility

- **Platform Support**:
  - Linux: systemd user service with auto-start
  - macOS: launchd service configuration
  - Windows: Windows Service installation
  - Android: Background service fallback
  - Platform-specific data directory handling

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

#### AI Integration (Month 3 Week 9)

- **AI Service (FastAPI)**:
  - Complete AI service backend for task assistance features
  - Multi-provider architecture supporting OpenAI, Anthropic, and Ollama
  - Provider factory pattern for easy switching between AI backends
  - Comprehensive error handling with retry logic using tenacity
  - Environment-based configuration with Pydantic settings
  - CORS middleware for cross-origin requests
  - Health check endpoint for service monitoring

- **AI Models and API**:
  - Request models:
    - TaskBreakdownRequest: Break tasks into subtasks (max 20)
    - TaskPrioritizationRequest: Prioritize multiple tasks
    - TimeEstimateRequest: Estimate task duration with skill level
    - ContextSuggestionRequest: Get contextual help and resources
  - Response models:
    - TaskBreakdownResponse: Subtask suggestions with time estimates
    - TaskPrioritizationResponse: Priority levels (critical/high/medium/low) with reasoning
    - TimeEstimateResponse: Optimistic/realistic/pessimistic estimates with confidence scores
    - ContextSuggestionResponse: Resources, tips, blockers, and tools

- **OpenAI Integration**:
  - GPT-4 Turbo support for high-quality AI responses
  - Task breakdown with ADHD-friendly subtask generation
  - Smart prioritization based on urgency and impact scores
  - Time estimation with three-point estimates
  - Contextual suggestions for resources and potential blockers
  - Automatic retry logic with exponential backoff
  - JSON response parsing with markdown cleanup

- **Anthropic Integration**:
  - Claude 3.5 Sonnet support for advanced reasoning
  - Same feature set as OpenAI (breakdown, prioritization, estimation, suggestions)
  - Async API calls with proper error handling
  - Structured JSON outputs for consistent responses

- **Ollama Integration**:
  - Local AI support for privacy-conscious users
  - Llama 3 model support (configurable)
  - HTTP API integration with local Ollama server
  - No API keys required - runs entirely offline
  - Same capabilities as cloud providers

- **AI Service Testing**:
  - 21 comprehensive tests covering all functionality
  - Unit tests for data models with validation
  - Factory pattern tests for provider selection
  - API endpoint tests with mocked AI responses
  - Health check tests
  - 100% test pass rate
  - pytest with async support and mocking

- **Documentation**:
  - Complete README with setup instructions
  - API endpoint documentation
  - Environment variable configuration guide
  - Example .env.example file
  - Provider-specific configuration details

#### AI Settings & Configuration (Week 10)

- **AI Settings UI**:
  - Settings page with AI provider configuration
  - Support for OpenAI, Anthropic, and Ollama providers
  - Provider selection with descriptions and visual feedback
  - Per-provider configuration sections:
    - OpenAI: API key and model selection (GPT-4 Turbo, GPT-4, GPT-3.5 Turbo)
    - Anthropic: API key and model selection (Claude 3.5 Sonnet, Claude 3 Opus, Claude 3 Sonnet)
    - Ollama: Server URL and model selection (Llama 3, Mistral, Code Llama)
  - Real-time validation with visual error indicators
  - Enable/disable toggle for AI features

- **AI Settings Model**:
  - AISettings model with provider-specific configurations
  - AIProvider enum with display names and default URLs
  - Validation logic for required API keys
  - JSON serialization (excluding sensitive API keys)
  - Computed properties for current provider state

- **Secure Storage**:
  - AISettingsRepository for settings persistence
  - API keys stored in flutter_secure_storage (platform keychain)
  - Non-sensitive settings in SharedPreferences
  - Separate storage keys per provider
  - Clear and load operations with error handling

- **Settings State Management**:
  - SettingsBloc for managing application settings
  - Settings events: Load, Update, Toggle, Save, Clear
  - Settings states: Initial, Loading, Loaded, Saving, Saved, Failure
  - Auto-save after configuration changes
  - Cached settings for performance

- **AI Provider Integration**:
  - AIConfig.fromSettings() implementation for all providers
  - Backend service integration for OpenAI and Anthropic
  - Direct Ollama server support or backend proxy
  - Configurable base URLs for custom deployments
  - API key injection via Authorization headers

- **Comprehensive Testing**:
  - 424 AISettings model tests (serialization, validation, computed properties)
  - 483 SettingsBloc tests (all events and state transitions)
  - 327 AISettingsRepository tests (persistence, secure storage, edge cases)
  - 623 SettingsPage UI tests (provider selection, configuration, validation)
  - All tests passing with full coverage

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
- Testing documentation (docs/TESTING-STRATEGY.md) with:
  - Comprehensive testing strategy for all test types
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

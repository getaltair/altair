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

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.0.1] - 2025-10-17

### Added

- Project initialization
- Development environment setup with mise

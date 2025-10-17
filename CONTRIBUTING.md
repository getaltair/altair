# Contributing to Altair

Thank you for your interest in contributing to Altair! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment for all contributors
- Remember that this project is built for the ADHD community

## Development Setup

### Prerequisites

- Flutter 3.0+ (for mobile/desktop development)
- Python 3.12+ (for backend services)
- uv (Python package manager)
- pnpm (if using Node.js tooling)
- pre-commit (for git hooks)
- mise (optional, for version management)

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/getaltair/altair.git
cd altair

# Install pre-commit hooks
pre-commit install

# Setup Flutter packages
cd packages/altair-core
flutter pub get

cd ../altair-ui
flutter pub get

# Setup backend services
cd ../../services/auth-service
uv sync
```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

Branch naming conventions:

- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions or updates
- `chore/` - Maintenance tasks

### 2. Make Your Changes

- Write clear, concise code
- Follow the coding standards (see below)
- Add tests for new functionality
- Update documentation as needed

### 3. Commit Your Changes

We use **Conventional Commits** for all commit messages:

```bash
# Format
<type>(<scope>): <subject>

# Examples
feat(tasks): add quick capture widget
fix(database): resolve migration error
docs(readme): update installation instructions
refactor(auth): simplify token validation
test(repositories): add task repository tests
chore(deps): update flutter dependencies
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

### 4. Run Linters and Tests

```bash
# Run all pre-commit hooks
pre-commit run --all-files

# Run Flutter tests
cd packages/altair-core
flutter test

# Run Python tests
cd ../../services/auth-service
uv run pytest
```

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub.

## Coding Standards

### Dart/Flutter

- Follow the [official Dart style guide](https://dart.dev/guides/language/effective-dart/style)
- Use `flutter format` for consistent formatting
- Run `flutter analyze` to catch potential issues
- Document public APIs with doc comments (`///`)
- Use meaningful variable and function names
- Keep functions small and focused

### Python

- Follow [PEP 8](https://pep8.org/)
- Use Ruff for linting and formatting
- Use type hints for all function signatures
- Write docstrings for all public functions/classes
- Keep functions under 50 lines when possible
- Use meaningful variable names

### General

- Write self-documenting code
- Add comments for complex logic
- Keep files under 500 lines
- One class/component per file
- Use consistent naming conventions

## Testing Guidelines

### Unit Tests

- Test individual functions and classes
- Mock external dependencies
- Aim for 70%+ code coverage
- Test edge cases and error conditions

### Widget Tests

- Test UI components in isolation
- Verify widget behavior
- Test user interactions
- Check widget state management

### Integration Tests

- Test complete features end-to-end
- Verify data flow between components
- Test real-world scenarios

## Documentation

### Code Documentation

- Document all public APIs
- Include usage examples
- Explain complex algorithms
- Update docs when code changes

### README Files

- Each package should have a README
- Include installation instructions
- Provide usage examples
- List dependencies

### Architecture Docs

- Keep architecture docs up to date
- Document major design decisions
- Explain trade-offs

## Pull Request Process

### Before Submitting

- [ ] Code follows project standards
- [ ] All tests pass
- [ ] Pre-commit hooks pass
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Commits follow conventional commits format

### PR Description

Include:

- What changes were made
- Why the changes were necessary
- How to test the changes
- Screenshots (for UI changes)
- Related issues

### Review Process

1. Automated checks run (CI/CD)
2. Code review by maintainers
3. Address feedback
4. Approval and merge

## CHANGELOG Updates

Update `CHANGELOG.md` for all user-facing changes:

```markdown
## [Unreleased]

### Added
- New feature description

### Changed
- Changed behavior description

### Fixed
- Bug fix description
```

## Issue Guidelines

### Reporting Bugs

Include:

- Clear description of the bug
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Flutter version, etc.)
- Screenshots or logs (if applicable)

### Feature Requests

Include:

- Clear description of the feature
- Use case / motivation
- Proposed implementation (optional)
- Alternative approaches (optional)

## Community

- 💬 Discord: [Join our community](https://discord.gg/altair)
- 🐛 Issues: [GitHub Issues](https://github.com/getaltair/altair/issues)
- 📧 Email: <dev@getaltair.com>

## License

By contributing to Altair, you agree that your contributions will be licensed under the AGPL-3.0-or-later license.

## Questions?

Don't hesitate to ask questions! You can:

- Open a discussion on GitHub
- Ask in Discord
- Email the maintainers

Thank you for contributing to Altair! 🚀

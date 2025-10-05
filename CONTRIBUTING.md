# Contributing to Altair

**Thank you for your interest in contributing to Altair!**

We're building an ADHD-friendly project management platform by and for the neurodivergent community. Whether you have ADHD or simply care about inclusive tools, your contributions are welcome.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Style Guidelines](#style-guidelines)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

---

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming, inclusive, and harassment-free experience for everyone, regardless of:

- Neurodiversity status
- Gender identity and expression
- Sexual orientation
- Disability
- Physical appearance
- Age
- Race
- Ethnicity
- Religion
- Technology choices

### Our Standards

**Positive behaviors:**

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what's best for the community
- Showing empathy towards community members
- Understanding that ADHD brains work differently—be patient

**Unacceptable behaviors:**

- Harassment, trolling, or insulting/derogatory comments
- Personal or political attacks
- Publishing others' private information
- Ableist language or assumptions
- Any conduct that could reasonably be considered inappropriate

### Enforcement

Violations of the Code of Conduct can be reported to <hello@getaltair.app>. All complaints will be reviewed and investigated promptly and fairly.

---

## How Can I Contribute?

### 🐛 Reporting Bugs

**Before submitting a bug report:**

1. Check the [existing issues](https://github.com/getaltair/altair/issues) to avoid duplicates
2. Try to reproduce the issue in the latest version
3. Gather relevant information (browser, OS, steps to reproduce)

**When submitting a bug report, include:**

- Clear, descriptive title
- Detailed description of the issue
- Steps to reproduce
- Expected vs. actual behavior
- Screenshots (if applicable)
- Environment details (OS, browser, version)
- Relevant logs or error messages

**Template:**

```markdown
### Description
[Clear description of the bug]

### Steps to Reproduce
1. Go to '...'
2. Click on '...'
3. See error

### Expected Behavior
[What should happen]

### Actual Behavior
[What actually happens]

### Environment
- OS: [e.g., macOS 13.0]
- Browser: [e.g., Chrome 120]
- Altair Version: [e.g., 0.1.0]

### Additional Context
[Screenshots, logs, etc.]
```

### 💡 Suggesting Features

We love feature ideas, especially those informed by ADHD experiences!

**Before suggesting a feature:**

1. Check [existing feature requests](https://github.com/getaltair/altair/labels/enhancement)
2. Consider whether it aligns with Altair's [core philosophy](README.md#philosophy)
3. Think about how it specifically helps ADHD users

**When suggesting a feature, include:**

- Clear, descriptive title
- Problem it solves (especially ADHD-related challenges)
- Proposed solution
- Alternative solutions considered
- How it fits into [roadmap](ROADMAP.md)

**Template:**

```markdown
### Problem Statement
[What challenge does this address? How does it affect ADHD users?]

### Proposed Solution
[Your idea for solving the problem]

### Alternatives Considered
[Other approaches you've thought about]

### Additional Context
[Screenshots, mockups, references]
```

### 📖 Improving Documentation

Documentation improvements are incredibly valuable:

- Fix typos or unclear explanations
- Add examples or tutorials
- Improve setup instructions
- Translate documentation
- Add ADHD-friendly tips or warnings

### 💻 Contributing Code

See [Development Setup](#development-setup) and [Development Workflow](#development-workflow) below.

### 🧪 Testing

- Test new features and report bugs
- Write automated tests
- Improve test coverage
- Test accessibility features

### 🎨 Design Contributions

- UI/UX improvements
- Accessibility enhancements
- Icon or illustration contributions
- Color scheme suggestions (ADHD-friendly)

---

## Getting Started

### Prerequisites

**Required:**

- Git
- Docker & Docker Compose (for easiest setup)
- Text editor or IDE (VS Code recommended)

**Alternative (without Docker):**

- Python 3.11+
- Node.js 18+ (for Flutter web)
- Flutter SDK 3.16+
- PostgreSQL 15+

### Fork the Repository

1. Fork the [Altair repository](https://github.com/getaltair/altair)
2. Clone your fork:

   ```bash
   git clone https://github.com/YOUR_USERNAME/altair.git
   cd altair
   ```

3. Add upstream remote:

   ```bash
   git remote add upstream https://github.com/getaltair/altair.git
   ```

---

## Development Setup

### Option 1: Docker (Recommended)

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/altair.git
cd altair

# Copy environment variables
cp .env.example .env

# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Backend will be at: http://localhost:8000
# Frontend will be at: http://localhost:3000
# API docs at: http://localhost:8000/docs
```

### Option 2: Manual Setup

**Backend (FastAPI):**

```bash
cd backend

# Create virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Set up database
# (Ensure PostgreSQL is running)
cp .env.example .env
# Edit .env with your database credentials

# Run migrations
alembic upgrade head

# Start development server
uvicorn app.main:app --reload --port 8000
```

**Frontend (Flutter):**

```bash
cd frontend

# Get dependencies
flutter pub get

# Run web app
flutter run -d chrome

# Or build for web
flutter build web
```

### Database Setup

```bash
# Start PostgreSQL (if not using Docker)
# macOS with Homebrew:
brew services start postgresql@15

# Create database
createdb altair_dev

# Run migrations
cd backend
alembic upgrade head
```

---

## Development Workflow

### 1. Pick an Issue

- Check [good first issue](https://github.com/getaltair/altair/labels/good%20first%20issue) label
- Comment on the issue to claim it
- Ask questions if anything is unclear

### 2. Create a Branch

```bash
# Update your fork
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 3. Make Changes

- Write clean, readable code
- Follow [style guidelines](#style-guidelines)
- Add tests for new functionality
- Update documentation as needed
- Keep commits focused and atomic

### 4. Test Your Changes

```bash
# Backend tests
cd backend
pytest

# Frontend tests
cd frontend
flutter test

# Run linters
cd backend
ruff check .
black --check .

cd frontend
flutter analyze
```

### 5. Commit Your Changes

See [Commit Guidelines](#commit-guidelines) below.

### 6. Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create PR via GitHub UI
```

---

## Style Guidelines

### Python (Backend)

We follow PEP 8 with some modifications:

- **Line length:** 88 characters (Black default)
- **Formatter:** Black
- **Linter:** Ruff
- **Type hints:** Required for all functions
- **Docstrings:** Required for public functions

**Example:**

```python
from typing import Optional
from pydantic import BaseModel


class Task(BaseModel):
    """Represents a task in the system.
    
    Attributes:
        id: Unique identifier
        title: Task title
        description: Detailed description
    """
    
    id: str
    title: str
    description: Optional[str] = None
    
    def mark_complete(self) -> None:
        """Mark this task as complete."""
        self.status = "completed"
        self.completed_at = datetime.now()
```

**Run formatters:**

```bash
# Format code
black .

# Check with linter
ruff check .
```

### Dart (Frontend)

We follow [Effective Dart](https://dart.dev/guides/language/effective-dart):

- **Formatter:** `dart format`
- **Analyzer:** `flutter analyze`
- **Naming:** camelCase for variables, PascalCase for classes
- **Comments:** Document public APIs

**Example:**

```dart
/// Represents a task in the application.
class Task {
  /// Creates a new task.
  Task({
    required this.id,
    required this.title,
    this.description,
  });

  /// Unique identifier for this task.
  final String id;
  
  /// The task's title.
  final String title;
  
  /// Optional detailed description.
  final String? description;
  
  /// Marks this task as complete.
  void markComplete() {
    status = TaskStatus.completed;
    completedAt = DateTime.now();
  }
}
```

**Run formatters:**

```bash
# Format code
dart format .

# Analyze
flutter analyze
```

### General Guidelines

- **Comments:** Explain *why*, not *what*
- **Variable names:** Descriptive and clear
- **Functions:** Do one thing well
- **File length:** Keep files under 500 lines when possible
- **ADHD-friendly:** Clear structure, good spacing, descriptive names

---

## Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat:** New feature
- **fix:** Bug fix
- **docs:** Documentation changes
- **style:** Code style changes (formatting, no logic change)
- **refactor:** Code refactoring
- **test:** Adding or updating tests
- **chore:** Maintenance tasks (dependencies, build, etc.)
- **perf:** Performance improvements

### Examples

```bash
# Feature
feat(tasks): add quick capture input

# Bug fix
fix(auth): resolve token refresh issue

# Documentation
docs(readme): update installation instructions

# Multiple paragraphs
feat(tasks): implement AI task breakdown

Integrates with local LLM to decompose complex tasks
into smaller, manageable subtasks. Uses Ollama for
self-hosted inference.

Closes #42
```

### Best Practices

- Use imperative mood ("add" not "added")
- Keep subject line under 72 characters
- Capitalize first letter of subject
- No period at end of subject
- Reference issues in footer

---

## Pull Request Process

### Before Submitting

- [ ] Code follows style guidelines
- [ ] All tests pass
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] Commits follow commit guidelines
- [ ] Branch is up to date with main

### PR Template

```markdown
## Description
[Clear description of what this PR does]

## Related Issues
Closes #[issue number]

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Refactoring
- [ ] Performance improvement

## Testing
[How you tested these changes]

## Screenshots (if applicable)
[Add screenshots]

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

### Review Process

1. **Automated Checks:** CI must pass
2. **Code Review:** At least one maintainer approval
3. **Testing:** Reviewer verifies functionality
4. **Feedback:** Address comments and suggestions
5. **Merge:** Squash and merge (usually)

### After Merge

- Delete your feature branch
- Update your fork's main branch
- Close related issues (if not auto-closed)

---

## Community

### Where to Ask Questions

- **GitHub Discussions:** General questions, ideas
- **GitHub Issues:** Bug reports, feature requests
- **Discord:** Real-time chat (coming soon)

### Getting Help

Don't hesitate to ask for help! We're all learning, and questions are always welcome:

- Comment on the issue you're working on
- Tag maintainers in discussions
- Join community calls (schedule TBD)

### ADHD-Friendly Contribution Tips

We understand contributing to open source can be overwhelming, especially with ADHD. Here are some tips:

**For getting started:**

- Pick small, well-defined issues first
- "Good first issue" label is your friend
- Ask for clarification—no question is too small
- Take breaks! Work in focused bursts

**For staying on track:**

- Set timer for focused work sessions
- Use the project's own task management (once available!)
- Don't try to fix everything at once
- Celebrate small wins

**If you get stuck:**

- Ask for help early
- It's okay to hand off an issue
- No judgment for incomplete work
- Your attempt helps others learn

### Recognition

We value all contributions! Contributors will be:

- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Celebrated in community channels
- Given credit in relevant documentation

---

## Development Resources

### Helpful Documentation

- [FastAPI Documentation](https://fastapi.tiangolo.com/)
- [Flutter Documentation](https://docs.flutter.dev/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [Riverpod Documentation](https://riverpod.dev/)

### Project-Specific Docs

- [Architecture Overview](ARCHITECTURE.md)
- [Feature Details](FEATURES.md)
- [Project Roadmap](ROADMAP.md)

### Learning Resources

- [ADHD-Friendly Design Patterns](FEATURES.md#design-principles)
- [API Documentation](http://localhost:8000/docs) (when running locally)

---

## Questions?

If you have any questions about contributing, please:

- Check [GitHub Discussions](https://github.com/getaltair/altair/discussions)
- Open an issue with the "question" label
- Email: <hello@getaltair.app>

**Thank you for contributing to Altair!** Your work helps make project management accessible to the ADHD community. 💙

---

**Last Updated:** October 2025

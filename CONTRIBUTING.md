# Contributing to Altair

**Welcome!** We're excited you want to help build ADHD-friendly project management tools.

📊 **Visual References:**
- [Contribution Workflow](diagrams/06-deployment-operations.md#contribution-workflow)
- [Development Environment Setup](diagrams/06-deployment-operations.md#development-environment)
- [Component Architecture](diagrams/05-component-architecture.md)
- [Testing Strategy](diagrams/06-deployment-operations.md#testing-strategy)

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [How Can I Contribute?](#how-can-i-contribute)
3. [Development Setup](#development-setup)
4. [Development Workflow](#development-workflow)
5. [Style Guidelines](#style-guidelines)
6. [Commit Messages](#commit-messages)
7. [Pull Request Process](#pull-request-process)
8. [CI/CD Pipeline](#cicd-pipeline)
9. [ADHD-Friendly Contributing Tips](#adhd-friendly-contributing-tips)

---

## Code of Conduct

This project follows our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

**TL;DR:**
- Be kind and respectful
- Use neurodiversity-affirming language
- Assume good intent
- Help create a supportive environment

---

## How Can I Contribute?

There are many ways to help, regardless of your technical skills!

### 🐛 Report Bugs

Found something broken? Help us fix it!

**Before submitting:**
1. Check [existing issues](https://github.com/getaltair/altair/issues)
2. Make sure you're on the latest version
3. Try to reproduce the bug

**When submitting:**
- Use the bug report template
- Describe what you expected vs what happened
- Include steps to reproduce
- Add screenshots if applicable
- Mention your OS/browser/app version

**[View Bug Report Flow](diagrams/06-deployment-operations.md#bug-reporting-process)**

### 💡 Suggest Features

Have an idea? We'd love to hear it!

**Before suggesting:**
1. Check [existing feature requests](https://github.com/getaltair/altair/discussions)
2. Consider if it's ADHD-specific or general
3. Think about implementation complexity

**When suggesting:**
- Explain the ADHD pain point it solves
- Describe your proposed solution
- Consider alternative approaches
- Add mockups if you can!

**Prioritization Criteria:**
- ADHD impact (high priority)
- Dogfooding value (can we use it now?)
- Effort vs value
- Fits project philosophy

**[View Feature Priority Matrix](diagrams/04-roadmap-planning.md#feature-priority-matrix)**

### 📝 Improve Documentation

Documentation is crucial for ADHD users!

**Ways to help:**
- Fix typos and grammar
- Clarify confusing sections
- Add examples and screenshots
- Improve ADHD-friendliness (shorter sentences, clearer structure)
- Translate to other languages (future)

**Documentation Guidelines:**
- Short sentences (15-20 words max)
- Active voice
- Bullet points over paragraphs
- Examples for complex concepts
- Visual diagrams where helpful

**[View Documentation Structure](diagrams/README.md)**

### 💻 Write Code

Ready to code? Awesome!

**Good First Issues:**
Look for issues labeled [`good first issue`](https://github.com/getaltair/altair/labels/good%20first%20issue)

**Areas needing help:**
- Backend (Python/FastAPI)
- Frontend (Flutter/Dart)
- UI/UX improvements
- Testing
- Performance optimization

**[View Component Architecture](diagrams/05-component-architecture.md)**

### 🎨 Design & UX

Design skills? Perfect!

**We need help with:**
- UI mockups and prototypes
- Icon design
- ADHD-friendly UX patterns
- Accessibility improvements
- Marketing materials

**Design Principles:**
- Minimal cognitive load
- Instant feedback
- Forgiving interface
- Multiple pathways to goals
- Sensory considerations

---

## Development Setup

### Prerequisites

- **Python:** 3.11 or higher
- **Flutter:** 3.16 or higher
- **PostgreSQL:** 15 or higher
- **Git:** Latest version
- **Docker:** (Optional but recommended)

**[View Detailed Setup Diagram](diagrams/06-deployment-operations.md#development-environment)**

### Quick Start (10 minutes)

**1. Clone the repository:**
```bash
git clone https://github.com/getaltair/altair.git
cd altair
```

**2. Backend setup:**
```bash
cd backend

# Create virtual environment
python -m venv venv

# Activate it
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt
pip install -r requirements-dev.txt  # Testing tools

# Set up database
createdb altair  # Or use Docker Compose

# Run migrations
alembic upgrade head

# Create .env file
cp .env.example .env
# Edit .env with your settings

# Start development server
python main.py
```

Backend runs at: http://localhost:8000  
API docs at: http://localhost:8000/docs

**3. Frontend setup:**
```bash
cd ../frontend

# Install dependencies
flutter pub get

# Run on web
flutter run -d chrome

# Or run on mobile emulator
flutter run  # Choose your device
```

**[View Complete Setup Guide](QUICK_START.md)**

### Docker Setup (Alternative)

**Easier for some, but slower iteration:**

```bash
# Start all services
docker-compose up -d

# Backend at: http://localhost:8000
# Frontend: Build manually (see frontend/README.md)

# View logs
docker-compose logs -f backend

# Stop all services
docker-compose down
```

**[View Docker Architecture](diagrams/06-deployment-operations.md#docker-deployment)**

---

## Development Workflow

### 1. Create a Branch

**[View Git Workflow Diagram](diagrams/06-deployment-operations.md#git-workflow)**

```bash
# Update main
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/quick-capture-hotkey

# Or for bug fix
git checkout -b fix/sync-conflict-resolution
```

**Branch naming:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation
- `refactor/` - Code refactoring
- `test/` - Adding tests
- `chore/` - Maintenance tasks

### 2. Make Changes

**ADHD-Friendly Tips:**
- 🎯 One feature per branch
- ⏱️ Commit frequently (don't lose work!)
- 🧪 Test as you go
- 📝 Update docs alongside code

**Testing:**
```bash
# Backend tests
cd backend
pytest

# Frontend tests
cd frontend
flutter test

# Run specific test
pytest tests/test_tasks.py::test_quick_capture
```

**[View Testing Strategy](diagrams/06-deployment-operations.md#testing-strategy)**

### 3. Commit Your Changes

```bash
# Stage changes
git add .

# Commit with good message (see below)
git commit -m "feat: add keyboard shortcut for quick capture"

# Push to your fork
git push origin feature/quick-capture-hotkey
```

**[See Commit Message Guidelines](#commit-messages)**

### 4. Create Pull Request

1. Go to GitHub
2. Click "New Pull Request"
3. Fill out PR template
4. Request review
5. Respond to feedback
6. Celebrate when merged! 🎉

**[View PR Process Diagram](diagrams/06-deployment-operations.md#pull-request-process)**

---

## Style Guidelines

### Python (Backend)

**We use:**
- **Formatter:** Black (line length: 88)
- **Linter:** Ruff
- **Type Hints:** Required for all functions
- **Docstrings:** Google style

**Example:**
```python
from typing import List, Optional
from datetime import datetime

async def get_user_tasks(
    user_id: str,
    project_id: Optional[str] = None,
    status: Optional[str] = None
) -> List[Task]:
    """
    Retrieve tasks for a specific user.
    
    Args:
        user_id: UUID of the user
        project_id: Optional project filter
        status: Optional status filter (todo/in_progress/done)
    
    Returns:
        List of Task objects
        
    Raises:
        UserNotFound: If user_id doesn't exist
    """
    # Implementation here
    pass
```

**Run formatters:**
```bash
# Format code
black .

# Check linting
ruff check .

# Type checking
mypy .
```

### Dart (Frontend)

**We use:**
- **Formatter:** `dart format`
- **Linter:** `flutter analyze`
- **Style:** Effective Dart

**Example:**
```dart
/// Fetches tasks for the current user.
///
/// Returns a [List] of [Task] objects, filtered by [projectId]
/// if provided. Throws [NetworkException] if the request fails.
Future<List<Task>> getUserTasks({
  String? projectId,
  TaskStatus? status,
}) async {
  final queryParams = <String, String>{};
  
  if (projectId != null) {
    queryParams['project_id'] = projectId;
  }
  
  if (status != null) {
    queryParams['status'] = status.value;
  }
  
  final response = await _dio.get(
    '/api/v1/tasks',
    queryParameters: queryParams,
  );
  
  return (response.data as List)
      .map((json) => Task.fromJson(json))
      .toList();
}
```

**Run formatters:**
```bash
# Format code
dart format .

# Analyze
flutter analyze

# Fix auto-fixable issues
dart fix --apply
```

### General Guidelines

**Code Quality:**
- ✅ Self-documenting code (clear variable names)
- ✅ Single responsibility principle
- ✅ DRY (Don't Repeat Yourself)
- ✅ KISS (Keep It Simple, Stupid)
- ✅ Comments explain *why*, not *what*

**ADHD-Friendly Code:**
- ✅ Small functions (<50 lines)
- ✅ Clear naming (no abbreviations)
- ✅ Obvious control flow
- ✅ Fail fast with clear errors
- ✅ Helpful error messages

---

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `style:` Formatting (no code change)
- `refactor:` Code restructuring
- `test:` Adding tests
- `chore:` Maintenance tasks
- `perf:` Performance improvement

### Examples

**Good:**
```
feat(tasks): add keyboard shortcut for quick capture

Implements Ctrl+N/Cmd+N to open quick capture dialog from anywhere
in the app. Includes tests and documentation.

Closes #42
```

**Good:**
```
fix(sync): resolve conflict resolution bug

Fixed issue where concurrent edits to the same task would cause
data loss. Now properly implements last-write-wins strategy.

Fixes #123
```

**Good:**
```
docs(readme): update setup instructions for M1 Macs

Added specific steps for installing PostgreSQL on Apple Silicon.
Clarified Python version requirements.
```

**Bad:**
```
fixed stuff
```

**Bad:**
```
WIP
```

**Bad:**
```
Updated files
```

---

## Pull Request Process

**[View Detailed PR Flow](diagrams/06-deployment-operations.md#pull-request-process)**

### Before Submitting

- [ ] Code follows style guidelines
- [ ] Tests pass locally
- [ ] Documentation updated (if needed)
- [ ] Self-review completed
- [ ] No unintended changes
- [ ] Commit messages are clear

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## ADHD Impact
How does this help ADHD users specifically?

## Testing
How has this been tested?

## Screenshots
If applicable

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-reviewed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] No new warnings
```

### Review Process

1. **Automated Checks**
   - Linting
   - Tests
   - Build verification

2. **Code Review**
   - At least 1 maintainer approval required
   - Reviewers provide constructive feedback
   - You respond to comments and make changes

3. **Merge**
   - Maintainer merges when approved
   - Branch automatically deleted
   - Changes deployed to development

**Timeline:**
- Initial review: 2-3 days
- Follow-up reviews: 1-2 days
- Emergency fixes: Same day

---

## CI/CD Pipeline

Our automated testing and quality pipeline ensures code quality and security for every contribution.

### Automated Checks

When you push code or open a PR, the following checks run automatically:

#### 1. **CI Workflow** (2-3 minutes)

**Linting & Formatting:**
- Ruff linter checks code quality
- Black formatter checks code style
- Ensures consistent Python code standards

**Testing Matrix:**
- Tests run on Python 3.11, 3.12, and 3.13
- All tests must pass on all versions
- Coverage reports generated automatically

**Code Coverage:**
- Minimum 70% coverage required
- Coverage reports uploaded to Codecov
- Coverage trends tracked over time
- PR comments show coverage changes

**How to run locally:**
```bash
cd backend

# Lint and format
uv run ruff check .
uv run black --check .

# Run tests with coverage
uv run pytest --cov=altair --cov-report=term

# Generate HTML coverage report
uv run pytest --cov=altair --cov-report=html
# Open htmlcov/index.html in browser
```

#### 2. **Security Scan** (3-5 minutes)

**Snyk Vulnerability Scan:**
- Scans Python dependencies for known vulnerabilities
- Fails on high-severity issues
- Runs on every push and weekly schedule

**CodeQL Analysis:**
- GitHub's semantic code analysis
- Finds security vulnerabilities in code
- Checks for common security anti-patterns

**Safety Dependency Check:**
- Additional Python dependency scanning
- Cross-checks against vulnerability databases

**How to prepare:**
- Keep dependencies up to date
- Review Dependabot PRs promptly
- Don't ignore security warnings

#### 3. **Dependency Review** (PRs only)

**On every pull request:**
- Reviews new dependencies being added
- Checks for known vulnerabilities
- Warns about license conflicts
- Fails on moderate+ severity issues

**How to avoid issues:**
- Add only necessary dependencies
- Use well-maintained packages
- Check package reputation first

### CI/CD Badges

Check our [README](README.md) for real-time status:

- [![CI](https://github.com/getaltair/altair/actions/workflows/ci.yml/badge.svg)](https://github.com/getaltair/altair/actions/workflows/ci.yml) - Build and test status
- [![Security](https://github.com/getaltair/altair/actions/workflows/security.yml/badge.svg)](https://github.com/getaltair/altair/actions/workflows/security.yml) - Security scan status
- [![codecov](https://codecov.io/gh/getaltair/altair/branch/main/graph/badge.svg)](https://codecov.io/gh/getaltair/altair) - Code coverage percentage

### Fixing CI Failures

**If tests fail:**
```bash
# Pull latest changes
git pull origin main

# Run tests locally
cd backend
uv run pytest -v

# Fix the failing tests
# Commit and push
```

**If linting fails:**
```bash
# Auto-fix most issues
cd backend
uv run ruff check --fix .
uv run black .

# Commit fixes
git add .
git commit -m "fix: resolve linting issues"
git push
```

**If coverage is too low:**
```bash
# Find uncovered code
uv run pytest --cov=altair --cov-report=term-missing

# Add tests for missing coverage
# Commit and push
```

**If security scan fails:**
- Review the vulnerability report in PR comments
- Update affected dependencies if possible
- If can't fix immediately, discuss with maintainers
- Security issues block merging

### Dependabot

**Automated dependency updates:**
- Runs weekly on Mondays at 9 AM EST
- Creates PRs for outdated packages
- Groups minor/patch updates together
- Auto-merges passing dev dependency updates

**How to handle Dependabot PRs:**
1. Review the changelog for breaking changes
2. Check if CI passes
3. Approve if everything looks good
4. Maintainers will merge

### Secret Management

**Required secrets (maintainers only):**
- `CODECOV_TOKEN` - Code coverage reporting
- `SNYK_TOKEN` - Security vulnerability scanning

**Never commit:**
- API keys
- Passwords
- Database credentials
- `.env` files

### Performance Optimizations

Our CI is optimized for speed:
- ✅ Caches Python dependencies (UV cache)
- ✅ Caches pip packages
- ✅ Runs jobs in parallel
- ✅ Skips CI for docs-only changes
- ✅ Cancels outdated runs automatically

**Average CI time:**
- Linting: 30 seconds
- Testing: 1-2 minutes per Python version
- Security scan: 3-5 minutes
- **Total: ~3 minutes** for standard PRs

### Troubleshooting

**"My PR shows failing checks but they passed locally"**
- Different Python version? Check matrix (3.11, 3.12, 3.13)
- Cached dependencies? Push updated `uv.lock`
- Platform differences? CI runs on Ubuntu

**"CI is taking too long"**
- Check Actions tab for bottlenecks
- Security scans run weekly (slower on Mondays)
- Large PRs take longer (more files to process)

**"Security scan found a vulnerability I can't fix"**
- Comment on the PR with context
- Tag maintainers for assistance
- We can add exceptions if justified
- Security team will help prioritize

### Best Practices

**Before opening a PR:**
- ✅ Run tests locally (`pytest`)
- ✅ Check linting (`ruff` and `black`)
- ✅ Review coverage (`pytest --cov`)
- ✅ Update dependencies (`uv sync`)
- ✅ Read security warnings

**ADHD-friendly CI tips:**
- 🎯 Fix one failing check at a time
- ⏱️ CI runs are fast (~3 min), use them!
- 📝 Save CI error messages for later
- 🤝 Ask for help if stuck
- 💾 Push often (CI catches issues early)

---

## ADHD-Friendly Contributing Tips

### For Contributors with ADHD

**Working on a contribution:**

**🎯 Start Small**
- Don't try to fix everything at once
- Pick ONE issue
- Break it into tiny steps
- Celebrate each commit!

**⏱️ Time-box Your Work**
- Set a timer (25 minutes)
- Focus on one thing
- Take breaks between sessions
- Don't hyperfocus past meals!

**📝 Track Your Progress**
- Keep notes of what you're doing
- Write down thoughts immediately
- Use Altair to manage your contribution! (dogfooding)
- Document blockers

**🤝 Ask for Help**
- Stuck? Ask on Discord
- No question is too basic
- We're all learning
- Maintainers want to help!

**💾 Commit Often**
- Don't wait for "perfect"
- Commit small changes frequently
- Push regularly (don't lose work!)
- Amend commits if needed

**✅ Use Checklists**
```markdown
## My contribution checklist
- [x] Found issue to work on
- [x] Set up development environment
- [x] Created feature branch
- [ ] Written code
- [ ] Added tests
- [ ] Updated docs
- [ ] Created PR
```

### For Reviewers

**When reviewing ADHD contributors:**

**✅ DO:**
- Be encouraging and specific
- Celebrate effort and progress
- Break feedback into clear action items
- Respond promptly (reduce anxiety)
- Offer pair programming for complex issues

**❌ DON'T:**
- Use vague feedback ("needs work")
- Pile on lots of nitpicks at once
- Make assumptions about skill level
- Ghost contributors

**Example Feedback:**

**Good:**
```
Great work on the quick capture feature! A few suggestions:

1. Line 45: Move this validation to a helper function
   - Example: `validate_task_title(title)`
   - Makes it reusable and testable

2. Tests needed:
   - Test with empty title
   - Test with very long title
   - Test with special characters

3. Documentation:
   - Add docstring to main function
   - Update README with new shortcut

Let me know if you need help with any of these!
```

**Bad:**
```
Needs refactoring. Too much logic in the controller.
Also needs tests and docs. Fix the linting errors too.
```

---

## Community

### Where to Connect

- **Discord:** [discord.gg/altair](https://discord.gg/altair) - Daily chat
- **GitHub Discussions:** Feature ideas and questions
- **Twitter:** [@getaltair](https://twitter.com/getaltair) - Updates
- **Email:** hello@getaltair.app - Direct contact

### Development Meetings

**Weekly Sync** (Optional)
- **When:** Fridays, 3 PM UTC
- **Where:** Discord voice channel
- **What:** Progress updates, blockers, celebrations
- **Recording:** Yes (for async participation)

**Monthly Community Call**
- **When:** Last Friday of month, 3 PM UTC
- **Where:** Discord voice channel
- **What:** Roadmap review, demos, Q&A
- **Everyone welcome!**

---

## Recognition

We value all contributions! Contributors are recognized:

- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Invited to contributor Discord channel
- Profile badge (future)
- Our endless gratitude! 💙

**Significant contributors** may be invited to join the core team.

---

## Questions?

**Before you ask:**
1. Check [documentation](DOCUMENTATION_INDEX.md)
2. Search [existing issues](https://github.com/getaltair/altair/issues)
3. Look through [Discord history](https://discord.gg/altair)

**Then ask away!**
- **Quick questions:** Discord #development channel
- **Complex topics:** GitHub Discussions
- **Private matters:** Email dev@getaltair.app

---

## License

By contributing to Altair, you agree that your contributions will be licensed under the same **AGPL-3.0** license that covers the project.

This ensures Altair remains open source forever and all improvements benefit the entire community.

---

**Thank you for contributing to Altair!** 🎉

Every bug report, feature suggestion, doc improvement, and line of code helps make project management more accessible for ADHD brains everywhere.

**Where focus takes flight.** ✨

---

**Last Updated:** October 2025  
**Questions?** Ask on [Discord](https://discord.gg/altair) or [GitHub Discussions](https://github.com/getaltair/altair/discussions)

---

## Related Documentation

- [Code of Conduct](CODE_OF_CONDUCT.md) - Community guidelines
- [Architecture](ARCHITECTURE.md) - Technical details and diagrams
- [Features](FEATURES.md) - What we're building
- [Roadmap](ROADMAP.md) - When we're building it
- [Quick Start](QUICK_START.md) - Fast development setup

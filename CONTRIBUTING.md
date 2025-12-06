# Contributing to Altair

> Where focus takes flight ✈️

Thanks for considering contributing to Altair! This project exists to help ADHD
folks stay productive, and every contribution makes that mission stronger.

## Quick Start (2 minutes)

**Want to help right now?**

1. Pick an issue labeled `good first issue`
2. Comment "I'd like to work on this"
3. Fork the repo and create a branch
4. Make your changes and submit a PR

**Not sure where to start?** Read on!

---

## Ways to Contribute

### 🐛 Report Bugs

Found something broken? [Open a bug report](https://github.com/getaltair/altair/issues/new?template=bug_report.md)

**Good bug reports include:**

- Clear description of the problem
- Steps to reproduce
- What you expected vs what happened
- Screenshots (if relevant)

### ✨ Suggest Features

Have an idea? [Open a feature request](https://github.com/getaltair/altair/issues/new?template=feature_request.md)

**Great feature requests explain:**

- The problem you're trying to solve
- How it helps ADHD users specifically
- Examples from other tools (optional)

### 💻 Contribute Code

Ready to code? Here's how:

1. **Find an issue** - Look for `good first issue` or `help wanted` labels
2. **Claim it** - Comment on the issue so others know you're working on it
3. **Set up** - Follow [setup instructions](../README.md#development-setup)
4. **Create branch** - Name it like `fix/bug-name` or `feature/feature-name`
5. **Make changes** - Follow our [code standards](#code-standards)
6. **Test** - Make sure it works on your device
7. **Submit PR** - Use our [PR template](.github/pull_request_template.md)

### 📝 Improve Documentation

Documentation helps everyone! You can:

- Fix typos or unclear instructions
- Add examples or tutorials
- Translate to other languages
- Improve ADHD-friendliness

### 🎨 Design Contributions

Help make Altair more ADHD-friendly:

- UI/UX improvements
- Icons and graphics
- Accessibility enhancements
- Design system documentation

---

## Code Standards

### ADHD-Friendly Principles

**Every contribution should:**

- Reduce cognitive load
- Support focus and energy management
- Make information scannable
- Minimize context switching
- Be accessible to neurodivergent users

### Code Style

- **Comments:** Explain _why_, not _what_
- **Names:** Clear and descriptive (readability over brevity)

### UI/UX Guidelines

- **Calm Focus aesthetic** - See [Design System](/docs/design-system.md)
- **Desktop-first** - Mobile is full-featured, not companion
- **Keyboard shortcuts** - Efficient workflows for power users
- **Progressive disclosure** - Don't overwhelm with options
- **Clear visual hierarchy** - Scannable interfaces

### Commit Messages

```
type: Brief description (50 chars max)

Longer explanation if needed:
- Why this change was made
- What problem it solves
- Any breaking changes

Closes #123
```

**Types:** `fix`, `feat`, `docs`, `refactor`, `test`, `chore`

**Examples:**

- `fix: Prevent crash when deleting quest with tasks`
- `feat: Add voice memo quick capture`
- `docs: Update installation guide for Windows`

---

## Review Process

### What to Expect

1. **Initial response** - Within 2-3 days
2. **Review** - Maintainer provides feedback
3. **Iteration** - Make requested changes
4. **Merge** - When ready, we'll merge and celebrate! 🎉

### Review Criteria

- Follows code standards
- Includes tests (for new features)
- Documentation updated
- ADHD-friendly principles applied
- No breaking changes (unless discussed)

---

## Getting Help

### Stuck on Something?

- **Discord** - [Join our community](#) (coming soon)
- **GitHub Discussions** - Ask questions
- **Issue comments** - Tag maintainers with `@getaltair`

### Response Times

This is an open-source project maintained by volunteers. We aim to respond
within 2-3 days, but sometimes it takes longer. Please be patient!

---

## Community Guidelines

### Be Kind and Respectful

- Use inclusive language
- Respect different perspectives
- Assume good intentions
- Provide constructive feedback

### Be ADHD-Aware

Many contributors have ADHD. That means:

- We might need reminders
- We appreciate clear, scannable communication
- We may hyperfocus on some things and forget others
- We're doing our best!

### No Harassment

We don't tolerate:

- Discriminatory language or behavior
- Personal attacks
- Spam or self-promotion
- Sharing private information

**See issue?** Report to project maintainers at [robert@getaltair.app]

---

## Technical Details

### Project Structure

```
altair/
├── apps/
│   ├── guidance/      # Task management app
│   ├── knowledge/     # Notes app
│   └── tracking/      # Inventory app
├── backend/           # FastAPI services
├── database/          # SurrealDB schemas
└── shared/            # Common code
```

### Tech Stack

- **Frontend:** Flutter (desktop + mobile)
- **Backend:** FastAPI (Python)
- **Database:** SurrealDB (local-first)
- **AI:** Claude/OpenAI APIs + local embeddings

### Development Workflow

1. **Dogfooding** - We use Altair to build Altair
2. **Phased approach** - Core features first, polish later
3. **ADHD testing** - Does it actually help us focus?
4. **Open source** - MIT license, community-driven

---

## License

By contributing, you agree that your contributions will be licensed under the
[AGPL 3.0 License](LICENSE).

---

## Questions?

**Not sure about something?** That's okay! Ask in:

- GitHub Discussions
- Issue comments
- Discord (coming soon)

**Remember:** There are no stupid questions. We were all beginners once!

---

## Thank You

Every contribution, no matter how small, makes Altair better for ADHD users
everywhere. We appreciate you!

> 🎯 **Mission:** Build productivity tools that actually work for ADHD brains
> 🚀 **Tagline:** Where focus takes flight
> 🌐 **Website:** [getaltair.app](https://getaltair.app)

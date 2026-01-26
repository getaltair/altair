# Claude Code Automations

This document describes all Claude Code automations configured for the Altair project.

**Last Updated**: 2026-01-26

---

## 🔌 MCP Servers (Active)

### 1. Context7
**Status**: ✅ Active
**Purpose**: Up-to-date documentation for Kotlin Multiplatform, Arrow, Koin, Ktor, and other libraries
**Usage**: Automatically available via `mcp__context7__` tools

### 2. SurrealDB MCP
**Status**: ✅ Active
**Purpose**: Direct database operations for desktop/server SurrealDB instances
**Usage**: Execute SurrealQL queries, manage graph relations, test schemas

### 3. JetBrains IntelliJ IDEA MCP
**Status**: ✅ Recommended (configure separately)
**Purpose**: Semantic code analysis, run configurations, project-aware search
**Setup**: Configure in `.mcp.json` or Claude settings

---

## ⚡ Hooks (Active)

### PostToolUse Hooks

#### 1. Auto-Format Kotlin Files (Write)
**Trigger**: After `Write(*.kt)`
**Action**: Runs `./gradlew spotlessApply --daemon -q`
**Purpose**: Automatically format Kotlin files using ktlint and Compose rules
**Timeout**: 30 seconds

#### 2. Auto-Format Kotlin Files (Edit)
**Trigger**: After `Edit(*.kt)`
**Action**: Runs `./gradlew spotlessApply --daemon -q`
**Purpose**: Automatically format Kotlin files using ktlint and Compose rules
**Timeout**: 30 seconds

### PreToolUse Hooks

#### 3. TODO Warning on Push
**Trigger**: Before `Bash(git push*)`
**Action**: Checks for TODO/FIXME in staged files
**Purpose**: Warns about committing temporary markers

**Note**: Additional protection hooks for version catalog, keystores, and gradle.properties were planned but removed because Claude Code hooks currently only support `type: "command"`. Use manual review and permissions instead.

---

## 🎯 Skills (User-Invocable)

### 1. gen-kmp-test
**Location**: `.claude/skills/gen-kmp-test/SKILL.md`
**Invocation**: `/gen-kmp-test <source-file-path> [test-style]`
**Purpose**: Generate Kotest test specifications for Kotlin Multiplatform code

**Features**:
- Supports BehaviorSpec, DescribeSpec, FunSpec styles
- Handles expect/actual patterns
- Includes Arrow Either matchers
- Uses Turbine for Flow testing
- Generates Mokkery mocks
- Places tests in correct source sets

**Example**:
```bash
/gen-kmp-test shared/src/commonMain/kotlin/domain/auth/AuthManager.kt BehaviorSpec
```

### 2. new-migration
**Location**: `.claude/skills/new-migration/SKILL.md`
**Invocation**: `/new-migration <platform> <description>`
**Purpose**: Generate database migrations for SQLDelight or SurrealDB

**Platforms**:
- `mobile` - Creates SQLDelight migration (`.sqm` file)
- `desktop` - Creates SurrealDB migration (Kotlin class)

**Features**:
- Enforces user_id isolation pattern
- Includes proper indexes and constraints
- Generates rollback migrations
- Validates schema rules

**Example**:
```bash
/new-migration mobile add-quest-tags-table
/new-migration desktop create-knowledge-nodes-graph
```

### 3. kotest (Pre-existing)
**Location**: `.claude/skills/kotest/SKILL.md`
**Invocation**: `/kotest`
**Purpose**: Kotest testing framework patterns and examples

### 4. arrow-patterns (Pre-existing)
**Location**: `.claude/skills/arrow-patterns/SKILL.md`
**Invocation**: `/arrow-patterns`
**Purpose**: Arrow functional error handling patterns

### 5. OpenSpec Skills (Pre-existing)
**Location**: `.claude/skills/openspec-*/SKILL.md`
**Purpose**: OpenSpec workflow for major changes

- `/openspec-new-change` - Start new change
- `/openspec-explore` - Explore mode
- `/openspec-continue-change` - Continue working on change
- `/openspec-apply-change` - Implement tasks
- `/openspec-verify-change` - Verify implementation
- `/openspec-archive-change` - Archive completed change
- `/openspec-bulk-archive-change` - Archive multiple changes
- `/openspec-ff-change` - Fast-forward through artifacts
- `/openspec-sync-specs` - Sync delta specs to main
- `/openspec-onboard` - Onboarding walkthrough

---

## 🤖 Subagents

### 1. arrow-validator
**Location**: `.claude/agents/arrow-validator.md`
**Type**: Project-level subagent
**Purpose**: Validate Arrow Either error handling patterns

**Scope**:
- `shared/src/commonMain/kotlin/domain/`
- `shared/src/commonMain/kotlin/data/`
- `shared/src/*/kotlin/data/repository/`
- `shared/src/commonMain/kotlin/api/`
- `composeApp/src/*/kotlin/service/`

**Validation Checks**:
1. Repository methods return `Either<DomainError, T>`
2. No raw exceptions in domain/data layers
3. External library exceptions wrapped
4. Proper Either chaining (flatMap, map, bind)
5. UI layer uses fold or onLeft/onRight
6. API layer returns `Either<NetworkError, T>`

**Invocation**:
Claude automatically delegates when appropriate, or request explicitly:

```bash
Use arrow-validator to check the authentication repository

Have arrow-validator review this PR for proper error handling

Run arrow-validator on the API client code
```

**Output Format**:
- Structured report with violations and correct patterns
- File paths with line numbers
- Current code vs recommended fixes
- Reference to existing correct implementations
- Priority ranking (High/Medium/Low)

---

## 🔐 Permissions

### Allowed Bash Commands

The following Gradle commands are pre-approved:
- `./gradlew test*`
- `./gradlew build*`
- `./gradlew :*:test`
- `./gradlew spotlessApply*`
- `./gradlew detekt*`
- `./gradlew ktlint*`
- `./gradlew assemble*`
- `./gradlew clean*`

---

## 📋 CI/CD Integration

### GitHub Actions

**File**: `.github/workflows/claude.yml`

**Triggers**:
- Issue comments with `@claude`
- PR review comments with `@claude`
- PR reviews with `@claude`
- New issues with `@claude` in title or body

**Permissions**:
- Read repository contents
- Read pull requests
- Read issues
- Read CI results

---

## 🚀 Usage Patterns

### Daily Development

1. **Write Kotlin code** → Auto-formatted with spotlessApply
2. **Edit repository** → Auto-formatted with spotlessApply
3. **Need tests?** → `/gen-kmp-test <file-path>`
4. **Need migration?** → `/new-migration <platform> <description>`
5. **Review errors?** → Invoke `arrow-validator` subagent

### Pull Request Workflow

1. **Create PR** → Tag `@claude` for automated review
2. **PR feedback** → Reply with `@claude` to get help
3. **Architecture validation** → Request `arrow-validator` analysis
4. **CI failures** → Claude can read CI results automatically

### Protected Operations

**Active protections**:
- **Git push with TODOs** → Shows warning

**Manual review required** (cannot be enforced via hooks):
- **Version catalog edits** (`gradle/libs.versions.toml`) - affects all modules
- **Gradle properties edits** - affects build configuration
- **Keystore operations** - security risk

**Note**: Claude Code hooks currently only support `type: "command"`. Confirmation and block hooks are not available.

---

## 📖 Documentation Updates

Updated `CLAUDE.md` to include:
- Skills section with `/gen-kmp-test` and `/new-migration`
- Subagents section with `arrow-validator`

---

## 🔄 Future Enhancements

Consider adding:
- **Pre-commit hook integration** for arrow-validator
- **Automatic test running** when repository files change
- **Detekt integration** in PostToolUse hooks
- **Platform-specific code reviewer** subagent
- **Component scaffolder** skill for new UI components
- **ADR generator** skill for architectural decisions

---

## 📚 Related Documentation

- `.claude/settings.json` - Hook and permission configuration
- `.claude/skills/*/SKILL.md` - Individual skill definitions
- `.claude/agents/arrow-validator.md` - Subagent specification
- `CLAUDE.md` - Main project guidance for Claude Code
- `.github/workflows/claude.yml` - CI integration

---

## 🆘 Troubleshooting

### Hook Not Running
- Check `.claude/settings.json` syntax is valid JSON
- Verify matcher pattern matches the tool call
- Check timeout hasn't expired (increase if needed)

### Skill Not Found
- Verify SKILL.md file exists in `.claude/skills/<name>/`
- Check YAML frontmatter is properly formatted
- Restart Claude if skill was just added

### Permission Denied
- Check if command is in `permissions.allow` array
- Add glob pattern to allow similar commands
- Request user confirmation if command needs approval

---

**Need help?** Ask Claude Code or check `.claude/` directory for all automation files.

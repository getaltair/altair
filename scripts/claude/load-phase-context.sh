#!/bin/bash
# load-phase-context.sh
# SessionStart hook: Loads relevant documentation for current implementation phase
# Output goes to stdout and is captured by Claude Code as session context

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DOCS_DIR="$PROJECT_ROOT/docs"
PHASE_FILE="$PROJECT_ROOT/.claude/current-phase"

# Read current phase (default to 1, can override with CLAUDE_PHASE env var)
CURRENT_PHASE="${CLAUDE_PHASE:-$(cat "$PHASE_FILE" 2>/dev/null || echo "1")}"

# =============================================================================
# Phase-to-Documentation Mapping
# Based on Altair's 12-phase implementation plan
# =============================================================================

# Maps phases to relevant ADR numbers
get_phase_adrs() {
    case "$1" in
        1)  echo "001 008 009" ;;           # Core libs: KMP arch, Theme, Core stack
        2)  echo "010 011 012 013 014" ;;   # Domain: Inbox, Initiative, Multi-user, Routines, SourceDoc
        3)  echo "002 009" ;;               # Repositories: DB strategy, Arrow errors
        4)  echo "005" ;;                   # RPC: kotlinx-rpc
        5)  echo "002" ;;                   # Database: Hybrid strategy
        6)  echo "012" ;;                   # Auth: Multi-user isolation
        7)  echo "008" ;;                   # UI: Compose Unstyled + Altair theme
        8)  echo "010 011 013 015" ;;       # Features: Inbox, Initiative, Routines, Rich text
        9)  echo "002 005" ;;               # Sync: DB + RPC
        10) echo "006" ;;                   # AI: Server-centralized
        11) echo "007" ;;                   # Platform: Docker deployment
        12) echo "009" ;;                   # Testing: Mokkery stack
        *)  echo "001 009" ;;               # Default: architecture + core stack
    esac
}

# Maps phases to relevant PRD sections
get_phase_prds() {
    case "$1" in
        8)  echo "guidance knowledge tracking" ;;  # Feature phase needs all module PRDs
        *)  echo "core" ;;                         # Most phases just need core PRD
    esac
}

# Maps phases to architecture docs
get_phase_arch() {
    case "$1" in
        2|3|8) echo "domain-model" ;;
        5|6|9) echo "persistence" ;;
        *)     echo "system-architecture" ;;
    esac
}

# =============================================================================
# Helper Functions
# =============================================================================

separator() {
    echo ""
    echo "════════════════════════════════════════════════════════════════════════════"
    echo ""
}

section() {
    echo "## $1"
    echo ""
}

# Extract section from markdown file (from pattern to next ## heading)
extract_section() {
    local file="$1"
    local start_pattern="$2"
    
    [[ ! -f "$file" ]] && return
    
    awk -v start="$start_pattern" '
        $0 ~ start { found=1 }
        found && /^## / && NR>1 { if ($0 !~ start) exit }
        found { print }
    ' "$file" 2>/dev/null
}

# =============================================================================
# Main Context Output
# =============================================================================

echo "# Altair Development Context"
echo ""
echo "**Phase:** $CURRENT_PHASE | **Session:** $(date '+%Y-%m-%d %H:%M')"
separator

# -----------------------------------------------------------------------------
# Current Phase from Implementation Plan
# -----------------------------------------------------------------------------

section "Phase $CURRENT_PHASE Tasks"

if [[ -f "$DOCS_DIR/implementation-plan.md" ]]; then
    # Extract current phase section
    awk -v phase="$CURRENT_PHASE" '
        /^## Phase '"$CURRENT_PHASE"':/ { found=1; print; next }
        found && /^## Phase [0-9]+/ { exit }
        found && /^---$/ { exit }
        found { print }
    ' "$DOCS_DIR/implementation-plan.md" 2>/dev/null || echo "See docs/implementation-plan.md"
fi

separator

# -----------------------------------------------------------------------------
# Relevant ADRs (Decision sections only for brevity)
# -----------------------------------------------------------------------------

section "Architecture Decisions"

for adr_num in $(get_phase_adrs "$CURRENT_PHASE"); do
    adr_file=$(find "$DOCS_DIR/adr" -name "*$adr_num*.md" -type f 2>/dev/null | head -1)
    if [[ -n "$adr_file" && -f "$adr_file" ]]; then
        adr_name=$(basename "$adr_file" .md)
        echo "### $adr_name"
        echo ""
        # Extract just the Decision section
        extract_section "$adr_file" "^## Decision"
        echo ""
    fi
done

separator

# -----------------------------------------------------------------------------
# Key Architecture Docs
# -----------------------------------------------------------------------------

section "Architecture Reference"

for arch_doc in $(get_phase_arch "$CURRENT_PHASE"); do
    arch_file="$DOCS_DIR/architecture/$arch_doc.md"
    [[ ! -f "$arch_file" ]] && continue
    
    echo "### From $arch_doc.md"
    echo ""
    
    case "$arch_doc" in
        system-architecture)
            extract_section "$arch_file" "## Technology Stack"
            ;;
        domain-model)
            extract_section "$arch_file" "## Entity Map"
            ;;
        persistence)
            extract_section "$arch_file" "## Database Architecture"
            ;;
    esac
    echo ""
done

separator

# -----------------------------------------------------------------------------
# Quick Reference (Always included)
# -----------------------------------------------------------------------------

section "Quick Reference"

cat << 'QUICKREF'
**Project Structure:**
```
altair/
├── composeApp/     # Compose Multiplatform (Android, iOS, Desktop)
├── server/         # Ktor server with kotlinx-rpc
├── shared/         # KMP shared module (models, DTOs, interfaces)
├── gradle/         # libs.versions.toml
└── docs/           # PRDs, ADRs, architecture
```

**Core Stack (ADR-009):**
| Component  | Library                     |
|------------|-----------------------------|
| DI         | Koin 4.x                    |
| Navigation | Decompose 3.x               |
| Errors     | Arrow 2.x (Either, Optics)  |
| Testing    | Mokkery 3.x + Turbine       |

**Data Layer (ADR-002):**
| Platform | Database           |
|----------|--------------------|
| Desktop  | SurrealDB embedded |
| Mobile   | SQLite (SQLDelight)|
| Server   | SurrealDB          |

**Key Patterns:**
- `Either<Error, T>` for fallible operations
- All entities include `userId` for multi-user isolation
- ULID for entity IDs
- Soft delete via `deletedAt` field
- `syncVersion` field for sync tracking
QUICKREF

separator

# -----------------------------------------------------------------------------
# Phase-Specific Checklist
# -----------------------------------------------------------------------------

section "Phase $CURRENT_PHASE Checklist"

case "$CURRENT_PHASE" in
    1)
        cat << 'EOF'
**Phase 1: Core Libraries & Architecture**

- [ ] Add to libs.versions.toml:
  - koin = "4.0.0" (koin-core, koin-compose, koin-compose-viewmodel)
  - decompose = "3.0.0" (decompose, extensions-compose)
  - arrow = "2.0.0" (arrow-core, arrow-optics) + KSP plugin
  - mokkery = "3.0.0"
  - turbine = "1.2.0"

- [ ] Wire Koin:
  - Android: `Application.onCreate()` → `startKoin { modules(appModule) }`
  - Desktop: `main()` → `startKoin { modules(appModule) }`
  - iOS: Similar initialization

- [ ] Setup Decompose navigation:
  - Create `RootComponent(ComponentContext)` with `StackNavigation<Config>`
  - Sealed class `Config` with destinations
  - Wire into Compose via `Children(stack) { ... }`

- [ ] Add Arrow to shared module with KSP for @optics

Verify: `./gradlew build` compiles all targets
EOF
        ;;
    2)
        cat << 'EOF'
**Phase 2: Domain Models in Shared**

Location: `shared/src/commonMain/kotlin/com/getaltair/altair/domain/`

System Entities:
- [ ] User (id, username, role, status, storageUsed, storageQuota)
- [ ] Initiative (id, userId, name, parentId, ongoing, status, focused)
- [ ] InboxItem (id, userId, content, source, createdAt)
- [ ] Routine (id, userId, name, schedule, timeOfDay, energyCost, active)

Guidance Entities:
- [ ] Epic (id, userId, title, status, initiativeId)
- [ ] Quest (id, userId, title, energyCost, status, epicId, routineId)
- [ ] Checkpoint (id, questId, title, completed, order)

Knowledge Entities:
- [ ] Note (id, userId, title, content, folderId, initiativeId)
- [ ] NoteLink, Folder, Tag, Attachment
- [ ] SourceDocument, SourceAnnotation

Tracking Entities:
- [ ] Item (id, userId, name, quantity, locationId, containerId)
- [ ] Location, Container, ItemTemplate, CustomField

All entities need: @Serializable, ULID id, userId, timestamps, syncVersion
EOF
        ;;
    3)
        cat << 'EOF'
**Phase 3: Repository Interfaces & Error Types**

Errors (Arrow Either pattern):
```kotlin
sealed interface AltairError {
    sealed interface QuestError : AltairError {
        data object NotFound : QuestError
        data object WipLimitExceeded : QuestError
    }
    // Similar for Note, Item, etc.
}
```

Repository pattern:
```kotlin
interface QuestRepository {
    suspend fun findById(id: String): Either<QuestError, Quest>
    suspend fun findByUser(userId: String): Either<AltairError, List<Quest>>
    suspend fun create(quest: Quest): Either<AltairError, Quest>
    suspend fun update(quest: Quest): Either<QuestError, Quest>
    suspend fun delete(id: String): Either<QuestError, Unit>
}
```

DTOs for RPC: CreateQuestRequest, SyncRequest, SyncResponse, etc.
EOF
        ;;
    *)
        echo "See docs/implementation-plan.md for Phase $CURRENT_PHASE details"
        ;;
esac

separator

echo "**Change phase:** \`echo N > .claude/current-phase\` or \`export CLAUDE_PHASE=N\`"
echo ""
echo "Ready to work on Phase $CURRENT_PHASE."

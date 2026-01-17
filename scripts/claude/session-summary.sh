#!/bin/bash
# session-summary.sh
# Stop hook: Generates a summary of what was accomplished in the session
# Can be used to update implementation-plan.md or create session notes

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PHASE_FILE="$PROJECT_ROOT/.claude/current-phase"
SESSION_LOG="$PROJECT_ROOT/.claude/session-log.md"

CURRENT_PHASE="${CLAUDE_PHASE:-$(cat "$PHASE_FILE" 2>/dev/null || echo "1")}"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M')

# =============================================================================
# Generate Session Summary
# =============================================================================

# Create or append to session log
mkdir -p "$(dirname "$SESSION_LOG")"

cat >> "$SESSION_LOG" << EOF

---

## Session: $TIMESTAMP (Phase $CURRENT_PHASE)

EOF

# Check for uncommitted changes
if command -v git &>/dev/null && [[ -d "$PROJECT_ROOT/.git" ]]; then
    cd "$PROJECT_ROOT"
    
    # Count changes
    STAGED=$(git diff --cached --name-only 2>/dev/null | wc -l)
    MODIFIED=$(git diff --name-only 2>/dev/null | wc -l)
    UNTRACKED=$(git ls-files --others --exclude-standard 2>/dev/null | wc -l)
    
    if [[ $STAGED -gt 0 ]] || [[ $MODIFIED -gt 0 ]] || [[ $UNTRACKED -gt 0 ]]; then
        cat >> "$SESSION_LOG" << EOF
### Uncommitted Changes

- **Staged:** $STAGED files
- **Modified:** $MODIFIED files  
- **Untracked:** $UNTRACKED files

EOF
        
        # List recently modified files
        RECENT_FILES=$(git diff --name-only 2>/dev/null | head -10)
        if [[ -n "$RECENT_FILES" ]]; then
            echo "**Recently modified:**" >> "$SESSION_LOG"
            echo '```' >> "$SESSION_LOG"
            echo "$RECENT_FILES" >> "$SESSION_LOG"
            echo '```' >> "$SESSION_LOG"
            echo "" >> "$SESSION_LOG"
        fi
    else
        echo "No uncommitted changes." >> "$SESSION_LOG"
        echo "" >> "$SESSION_LOG"
    fi
fi

# Add reminder
cat >> "$SESSION_LOG" << EOF
### Next Steps

- [ ] Review and commit changes
- [ ] Update docs/implementation-plan.md progress markers
- [ ] Run \`./gradlew build\` to verify

EOF

# Output summary to stderr (visible to user)
echo "" >&2
echo "════════════════════════════════════════════════════════════════════" >&2
echo "Session ended: $TIMESTAMP" >&2
echo "Phase: $CURRENT_PHASE" >&2
echo "" >&2

if command -v git &>/dev/null && [[ -d "$PROJECT_ROOT/.git" ]]; then
    cd "$PROJECT_ROOT"
    TOTAL_CHANGES=$((STAGED + MODIFIED + UNTRACKED))
    if [[ $TOTAL_CHANGES -gt 0 ]]; then
        echo "📝 $TOTAL_CHANGES uncommitted file(s)" >&2
        echo "" >&2
        echo "Remember to:" >&2
        echo "  • git add -p  (review changes)" >&2
        echo "  • git commit -m \"...\"" >&2
        echo "  • Update implementation-plan.md" >&2
    else
        echo "✓ Working tree clean" >&2
    fi
fi

echo "" >&2
echo "Session log: .claude/session-log.md" >&2
echo "════════════════════════════════════════════════════════════════════" >&2

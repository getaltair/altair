#!/bin/bash
# format-kotlin.sh
# PostToolUse hook for Write|Edit - auto-formats Kotlin files
# Silently succeeds if no formatter available (non-blocking)

set -euo pipefail

TOOL_INPUT="${1:-}"

# Extract file path from JSON input
FILE_PATH=$(echo "$TOOL_INPUT" | grep -oP '"(?:file_)?path"\s*:\s*"\K[^"]+' 2>/dev/null || echo "")

[[ -z "$FILE_PATH" ]] && exit 0

# Only process Kotlin files
if [[ ! "$FILE_PATH" =~ \.(kt|kts)$ ]]; then
    exit 0
fi

# Skip build scripts and generated code
if [[ "$FILE_PATH" =~ (build\.gradle\.kts|settings\.gradle\.kts|buildSrc/) ]]; then
    exit 0
fi

if [[ "$FILE_PATH" =~ (generated/|/build/) ]]; then
    exit 0
fi

BASENAME=$(basename "$FILE_PATH")
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# =============================================================================
# Try formatters in order of preference
# =============================================================================

# 1. ktlint via Gradle plugin (preferred for project consistency)
if [[ -x "$PROJECT_ROOT/gradlew" ]]; then
    # Check if ktlint plugin is configured
    if grep -q "ktlint" "$PROJECT_ROOT/build.gradle.kts" 2>/dev/null || \
       grep -q "ktlint" "$PROJECT_ROOT/gradle/libs.versions.toml" 2>/dev/null; then
        # Run ktlint format on specific file
        "$PROJECT_ROOT/gradlew" ktlintFormat -Pkotlin.incremental=false --quiet 2>/dev/null && \
            echo "✓ Formatted with ktlint (Gradle): $BASENAME" >&2 && exit 0
    fi
fi

# 2. ktlint binary (global installation)
if command -v ktlint &>/dev/null; then
    ktlint --format "$FILE_PATH" 2>/dev/null && \
        echo "✓ Formatted with ktlint: $BASENAME" >&2 && exit 0
fi

# 3. ktfmt binary
if command -v ktfmt &>/dev/null; then
    ktfmt --kotlinlang-style "$FILE_PATH" 2>/dev/null && \
        echo "✓ Formatted with ktfmt: $BASENAME" >&2 && exit 0
fi

# 4. ktfmt via jar (if downloaded to ~/.local/lib/)
KTFMT_JAR="${HOME}/.local/lib/ktfmt.jar"
if [[ -f "$KTFMT_JAR" ]]; then
    java -jar "$KTFMT_JAR" --kotlinlang-style "$FILE_PATH" 2>/dev/null && \
        echo "✓ Formatted with ktfmt.jar: $BASENAME" >&2 && exit 0
fi

# 5. IntelliJ IDEA command-line formatter
if command -v idea &>/dev/null; then
    idea format "$FILE_PATH" 2>/dev/null && \
        echo "✓ Formatted with IntelliJ: $BASENAME" >&2 && exit 0
fi

# No formatter found - silently succeed (don't block workflow)
# Uncomment below to show a hint:
# echo "ℹ️  No Kotlin formatter found. Consider installing ktlint." >&2

exit 0

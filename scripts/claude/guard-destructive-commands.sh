#!/bin/bash
# guard-destructive-commands.sh
# PreToolUse hook for Bash - blocks or confirms destructive commands
# Returns JSON: {"decision": "allow|deny|ask", "message": "..."}

set -euo pipefail

TOOL_INPUT="${1:-}"

# Extract command from JSON input
COMMAND=$(echo "$TOOL_INPUT" | grep -oP '"command"\s*:\s*"\K[^"]+' 2>/dev/null || echo "$TOOL_INPUT")
COMMAND=$(echo "$COMMAND" | tr -s ' ')  # Normalize whitespace

[[ -z "$COMMAND" ]] && { echo '{"decision": "allow"}'; exit 0; }

# =============================================================================
# BLOCKED: Always deny these dangerous patterns
# =============================================================================

BLOCKED_PATTERNS=(
    "rm -rf /"
    "rm -rf ~"
    "rm -rf \$HOME"
    "rm -rf \${HOME}"
    "rm -rf /*"
    "> /dev/sd"
    "mkfs"
    "dd if="
    ":(){ :|:& };"
    "chmod -R 777 /"
    "chmod 777 /"
)

for pattern in "${BLOCKED_PATTERNS[@]}"; do
    if [[ "$COMMAND" == *"$pattern"* ]]; then
        cat << EOF
{"decision": "deny", "message": "🚫 BLOCKED: Dangerous command pattern detected.\\n\\nThis command could cause system damage and is not permitted."}
EOF
        exit 0
    fi
done

# =============================================================================
# ASK: Require confirmation for potentially destructive commands
# =============================================================================

# Gradle clean build (slow, full rebuild)
if [[ "$COMMAND" =~ gradlew.*clean.*build ]] || [[ "$COMMAND" =~ gradle.*clean.*build ]]; then
    cat << EOF
{"decision": "ask", "message": "🔄 Full clean build requested\\n\\nThis will:\\n• Delete all build caches\\n• Rebuild from scratch (2-10 minutes)\\n\\nConsider \`./gradlew build\` for incremental build.\\n\\nProceed with clean build?"}
EOF
    exit 0
fi

# Gradle clean alone
if [[ "$COMMAND" =~ gradlew[[:space:]]+clean($|[[:space:]]) ]] || \
   [[ "$COMMAND" =~ gradle[[:space:]]+clean($|[[:space:]]) ]]; then
    cat << EOF
{"decision": "ask", "message": "🧹 Gradle clean requested\\n\\nThis deletes all build outputs. Next build will be slower.\\n\\nProceed?"}
EOF
    exit 0
fi

# Gradle publish/deploy/release
if [[ "$COMMAND" =~ gradlew.*(publish|deploy|release|upload) ]]; then
    cat << EOF
{"decision": "ask", "message": "📤 Publishing command detected\\n\\nCommand: $COMMAND\\n\\nThis may publish artifacts externally. Confirm this is intentional."}
EOF
    exit 0
fi

# Git force push
if [[ "$COMMAND" =~ git.*push.*(-f|--force) ]]; then
    cat << EOF
{"decision": "ask", "message": "⚠️  Force push detected\\n\\nThis will overwrite remote history and may affect collaborators.\\n\\nAre you sure?"}
EOF
    exit 0
fi

# Git reset --hard
if [[ "$COMMAND" =~ git.*reset.*--hard ]]; then
    cat << EOF
{"decision": "ask", "message": "⚠️  Hard reset detected\\n\\nThis will permanently discard uncommitted changes.\\n\\nProceed?"}
EOF
    exit 0
fi

# Git clean -f
if [[ "$COMMAND" =~ git.*clean.*-[a-zA-Z]*f ]]; then
    cat << EOF
{"decision": "ask", "message": "⚠️  Git clean with force\\n\\nThis will delete untracked files permanently.\\n\\nProceed?"}
EOF
    exit 0
fi

# Recursive rm (except common safe targets)
if [[ "$COMMAND" =~ rm[[:space:]]+-[a-zA-Z]*r ]] || [[ "$COMMAND" =~ rm[[:space:]]+-[a-zA-Z]*f[a-zA-Z]*r ]]; then
    # Allow common build directory cleanups
    if [[ "$COMMAND" =~ rm.*(build/|/build|\.gradle/|node_modules/|\.idea/|target/|\.kotlin/) ]]; then
        echo '{"decision": "allow"}'
        exit 0
    fi
    
    cat << EOF
{"decision": "ask", "message": "🗑️  Recursive delete detected\\n\\nCommand: $COMMAND\\n\\nPlease confirm the deletion scope is correct."}
EOF
    exit 0
fi

# Docker cleanup commands
if [[ "$COMMAND" =~ docker.*(prune|system.*rm) ]] && [[ "$COMMAND" =~ (-f|--force|-a|--all) ]]; then
    cat << EOF
{"decision": "ask", "message": "🐳 Docker cleanup command\\n\\nThis may remove containers, images, or volumes.\\n\\nProceed?"}
EOF
    exit 0
fi

# Database drop commands
if [[ "$COMMAND" =~ (DROP|drop)[[:space:]]+(DATABASE|TABLE|SCHEMA) ]]; then
    cat << EOF
{"decision": "ask", "message": "💾 Database drop command detected\\n\\nThis will permanently delete data.\\n\\nAre you absolutely sure?"}
EOF
    exit 0
fi

# =============================================================================
# ALLOW: Everything else
# =============================================================================

echo '{"decision": "allow"}'

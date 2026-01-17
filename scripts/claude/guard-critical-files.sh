#!/bin/bash
# guard-critical-files.sh
# PreToolUse hook for Write|Edit - protects critical configuration files
# Returns JSON: {"decision": "allow|deny|ask", "message": "..."}

set -euo pipefail

TOOL_INPUT="${1:-}"

# Extract file path from JSON input
FILE_PATH=$(echo "$TOOL_INPUT" | grep -oP '"(?:file_)?path"\s*:\s*"\K[^"]+' 2>/dev/null || echo "")

[[ -z "$FILE_PATH" ]] && { echo '{"decision": "allow"}'; exit 0; }

BASENAME=$(basename "$FILE_PATH")

# =============================================================================
# BLOCKED: Never modify these
# =============================================================================

BLOCKED=(
    "gradlew"
    "gradlew.bat"
    "gradle-wrapper.jar"
)

for blocked in "${BLOCKED[@]}"; do
    if [[ "$BASENAME" == "$blocked" ]]; then
        cat << EOF
{"decision": "deny", "message": "🚫 Cannot modify $BASENAME - this is a generated wrapper file."}
EOF
        exit 0
    fi
done

# =============================================================================
# ASK: Require confirmation for sensitive files
# =============================================================================

# Build configuration
if [[ "$BASENAME" == "settings.gradle.kts" ]] || \
   [[ "$BASENAME" == "gradle.properties" ]] || \
   [[ "$BASENAME" == "gradle-wrapper.properties" ]] || \
   [[ "$BASENAME" == "local.properties" ]]; then
    cat << EOF
{"decision": "ask", "message": "⚠️  Modifying build configuration: $BASENAME\\n\\nThis affects project-wide build settings. Confirm change is intentional."}
EOF
    exit 0
fi

# Version catalog
if [[ "$BASENAME" == "libs.versions.toml" ]]; then
    cat << EOF
{"decision": "ask", "message": "📦 Modifying version catalog\\n\\nChanges affect all modules. Ensure version compatibility."}
EOF
    exit 0
fi

# Root build.gradle.kts (not module-level)
if [[ "$BASENAME" == "build.gradle.kts" ]] && [[ ! "$FILE_PATH" =~ (composeApp|server|shared|buildSrc)/ ]]; then
    cat << EOF
{"decision": "ask", "message": "🏗️  Modifying root build.gradle.kts\\n\\nThis affects entire project. Confirm change."}
EOF
    exit 0
fi

# Secrets and credentials
if [[ "$FILE_PATH" =~ \.(env|jks|keystore|pem|key)$ ]] || \
   [[ "$BASENAME" =~ (secret|credential|password|token|apikey) ]]; then
    cat << EOF
{"decision": "ask", "message": "🔐 Modifying sensitive file: $BASENAME\\n\\nEnsure no secrets are being committed."}
EOF
    exit 0
fi

# Existing ADRs (should be immutable, create new instead)
if [[ "$FILE_PATH" =~ docs/adr/[0-9]{3}.*\.md$ ]] && [[ -f "$FILE_PATH" ]]; then
    cat << EOF
{"decision": "ask", "message": "📋 Modifying existing ADR: $BASENAME\\n\\nADRs are typically immutable. Consider:\\n- Update only the Status field\\n- Create a superseding ADR instead\\n\\nProceed anyway?"}
EOF
    exit 0
fi

# Docker/deployment configs
if [[ "$BASENAME" =~ ^(Dockerfile|docker-compose).*$ ]]; then
    cat << EOF
{"decision": "ask", "message": "🐳 Modifying deployment config: $BASENAME\\n\\nThis affects production deployment. Confirm."}
EOF
    exit 0
fi

# =============================================================================
# ALLOW: Everything else
# =============================================================================

echo '{"decision": "allow"}'

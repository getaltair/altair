#!/usr/bin/env bash
# Post-edit linting hook — auto-format on file save.
# Only includes formatters for stacks detected during init.
FILEPATH=""
[ -z "$FILEPATH" ] && exit 0
EXTENSION="${FILEPATH##*.}"
case "$EXTENSION" in
    # === Svelte/TypeScript/JavaScript ===
    svelte|ts|tsx|js|jsx)
        command -v bun &>/dev/null && bun prettier --write "$FILEPATH" 2>/dev/null && bun eslint --fix "$FILEPATH" 2>/dev/null
        ;;
    # === Rust ===
    rs)
        command -v rustfmt &>/dev/null && rustfmt "$FILEPATH" 2>/dev/null
        ;;
    # === Kotlin ===
    kt|kts)
        command -v ktlint &>/dev/null && ktlint -F "$FILEPATH" 2>/dev/null
        ;;
esac
exit 0

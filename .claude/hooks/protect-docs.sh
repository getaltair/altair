#!/usr/bin/env bash
# docs/ holds this project's normative design documents — the authority the
# implementation is checked against (see CLAUDE.md). Nothing may change a file
# under docs/ without a human explicitly unlocking it first.
#
# Why deny rather than ask: a PreToolUse "ask" decision was measured to be
# discarded under both bypass and auto permission modes, and an autoMode
# soft_deny rule is cleared by apparent user intent. "deny" is the only
# decision observed to actually hold, so approval is expressed out-of-band,
# by creating the unlock marker below.
#
# To approve docs/ changes:   touch .claude/.docs-unlock
# The marker stays valid for UNLOCK_TTL seconds, then the guard re-arms.
# The marker is also the unlock for editing this script, so a human-approved
# window is enough to maintain the guard itself.
#
# Reads the PreToolUse payload on stdin; silence means "not my business".
set -uo pipefail

UNLOCK_TTL=1800  # 30 minutes

input=$(cat)
root="${CLAUDE_PROJECT_DIR:-$PWD}"
unlock="$root/.claude/.docs-unlock"
tool=$(jq -r '.tool_name // ""' <<<"$input")
cmd=""

# A marker older than the TTL is treated as absent, so a forgotten unlock does
# not leave docs/ open indefinitely.
unlocked() {
  [ -f "$unlock" ] || return 1
  local now mtime
  now=$(date +%s)
  mtime=$(stat -c %Y "$unlock" 2>/dev/null || echo 0)
  [ $((now - mtime)) -le "$UNLOCK_TTL" ]
}

deny() {
  jq -n --arg r "$1" '{hookSpecificOutput:{hookEventName:"PreToolUse",permissionDecision:"deny",permissionDecisionReason:$r}}'
  exit 0
}

guard() {
  unlocked && exit 0
  deny "$1 docs/ holds this project's design authority and is gated. A human must approve by running: touch .claude/.docs-unlock (valid ${UNLOCK_TTL}s), then retry."
}

# True when $cmd writes to something matching regex $1. The mutating token must
# reach the target without crossing a command separator, so reading one path
# while writing an unrelated one does not trip the guard:
#   cat docs/x.md && rm /tmp/y   -> not a docs/ write
#   ls .claude/.docs-unlock 2>/dev/null -> not a marker write
writes_to() {
  local t="$1"
  grep -qE "(>>?[[:space:]]*[^|&;]*${t})|(\b(rm|mv|cp|tee|truncate|install|patch|shred|chmod|chown|ln|dd|touch|python3?|node|ruby)\b[^|&;]*${t})|((sed|perl)[[:space:]]+[^|&;]*-i[^|&;]*${t})|(\bgit[[:space:]]+(checkout|restore|apply|rm|mv|clean|stash)\b[^|&;]*${t})" <<<"$cmd"
}

# The approval marker and this script are what make the gate mean anything. They
# are writable only inside a human-approved unlock window — otherwise a session
# could quietly unlock itself or edit the guard away.
self_defence() { # <path-or-command text>
  case "$1" in
    *.docs-unlock*)
      unlocked && return 0
      deny "Creating the docs/ approval marker is reserved for a human. Run it yourself: touch .claude/.docs-unlock" ;;
    *protect-docs.sh*)
      unlocked && return 0
      deny "This is the docs/ guard itself. Unlock first (touch .claude/.docs-unlock) or edit it yourself." ;;
  esac
}

case "$tool" in
  Edit|MultiEdit|Write|NotebookEdit)
    path=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // ""' <<<"$input")
    [ -n "$path" ] || exit 0
    self_defence "$path"
    case "$path" in
      /*) abs="$path" ;;
      *)  abs="$root/$path" ;;
    esac
    case "$abs" in
      "$root"/docs/*) guard "Refusing to change ${path##*/}." ;;
    esac
    ;;
  Bash)
    cmd=$(jq -r '.tool_input.command // ""' <<<"$input")

    writes_to '\.docs-unlock'   && self_defence ".docs-unlock"
    writes_to 'protect-docs\.sh' && self_defence "protect-docs.sh"

    # Writes naming docs/ directly. A leading slash is allowed (absolute paths);
    # a word character is not, so mydocs/ and subdocs/ are left alone.
    if grep -qE '(^|[^[:alnum:]_.-])docs/' <<<"$cmd" && writes_to '[^[:alnum:]_.-]?docs/'; then
      guard "Refusing a command that may modify docs/."
    fi

    # Same, for a working-directory change into docs/ followed by a write:
    #   cd docs && rm altair-vision.md
    if grep -qE '\bcd[[:space:]]+[^|&;]*docs(/|[[:space:]]|$)' <<<"$cmd" \
       && grep -qE '(>>?)|\b(rm|mv|cp|tee|truncate|install|patch|shred|dd|touch|python3?|node|ruby)\b|((sed|perl)[[:space:]]+[^|]*-i)' <<<"$cmd"; then
      guard "Refusing a command that changes directory into docs/ and then writes."
    fi
    ;;
esac
exit 0

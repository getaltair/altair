#!/usr/bin/env bash
# The repository's pre-commit hooks are the only thing between a local mistake
# and a red CI run, and they are trivially skipped: --no-verify, a -n on git
# commit, core.hooksPath pointed at nothing, SKIP= in front of prek. Every one
# is a single token, and reaching for one is always locally reasonable — the
# hook is slow, or it is complaining about something that "obviously" does not
# matter.
#
# It did matter. Merge commits made with --no-verify and core.hooksPath=/dev/null
# shipped a Cargo.lock describing neither branch's dependency set. Local
# `cargo test` passed, because cargo rewrites that file rather than complaining
# about it, and CI failed two pushes later under the name of an unrelated hook.
#
# So: an agent may not skip the hooks. A human may, out of band:
#
#     touch .claude/.hooks-unlock
#
# Valid for UNLOCK_TTL seconds, then the guard re-arms.
#
# Why deny rather than ask: the same reasoning protect-docs.sh records beside
# it — a PreToolUse "ask" is discarded under bypass and auto permission modes,
# and an autoMode soft_deny is cleared by apparent user intent. "deny" holds.
#
# Reads the PreToolUse payload on stdin; silence means "not my business".

set -uo pipefail

UNLOCK_TTL=1800  # 30 minutes

root="${CLAUDE_PROJECT_DIR:-$PWD}"
unlock="$root/.claude/.hooks-unlock"

deny() {
  jq -n --arg r "$1" '{hookSpecificOutput:{hookEventName:"PreToolUse",permissionDecision:"deny",permissionDecisionReason:$r}}'
  exit 0
}

# A guard that breaks must not thereby permit what it exists to refuse. An
# earlier revision of this script lost a variable during an edit and allowed
# every bypass while printing an error nobody was reading, so any unexpected
# failure below lands here and denies.
fail_closed() {
  deny "The hook-bypass guard failed while checking this command (line ${1:-?}), so it is refused rather than allowed. Fix .claude/hooks/no-hook-bypass.sh — a human can unlock with: touch .claude/.hooks-unlock"
}
trap 'fail_closed $LINENO' ERR

input=$(cat)
tool=$(jq -r '.tool_name // ""' <<<"$input")

unlocked() {
  [ -f "$unlock" ] || return 1
  local now mtime
  now=$(date +%s)
  mtime=$(stat -c %Y "$unlock" 2>/dev/null || echo 0)
  [ $((now - mtime)) -le "$UNLOCK_TTL" ]
}

guard() {
  unlocked && exit 0
  deny "$1

The pre-commit hooks are not optional for an agent. If a hook is failing, fix
what it reports; if the hook itself is wrong, change it in a commit of its own.
A human can open a ${UNLOCK_TTL}s window with: touch .claude/.hooks-unlock"
}

# A file tool cannot skip a git hook, so only one thing matters on that path:
# neither this script nor its marker may be changed without a human. The
# Bash-only revision of this guard could be edited away with Write, which made
# the rest of it decorative.
case "$tool" in
  Bash) ;;
  Edit|MultiEdit|Write|NotebookEdit)
    path=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // ""' <<<"$input")
    case "$path" in
      *no-hook-bypass.sh|*.hooks-unlock)
        unlocked && exit 0
        deny "This is the hook-bypass guard itself, and a file tool is no more allowed to change it than a shell is. A human must unlock first: touch .claude/.hooks-unlock" ;;
    esac
    exit 0 ;;
  *) exit 0 ;;
esac

cmd=$(jq -r '.tool_input.command // ""' <<<"$input")
[ -n "$cmd" ] || exit 0

# Heredoc bodies are prose. Commit messages and pull request bodies discuss
# these flags — this script's own commit message names the marker it protects —
# and a guard that fires on writing about it would be worse than no guard,
# because the way round it would become routine. Everything below matches the
# stripped text, self-defence included.
stripped=$(awk '
  BEGIN { skip = 0 }
  skip == 0 && match($0, /<<-?[[:space:]]*'"'"'?"?[A-Za-z_][A-Za-z0-9_]*'"'"'?"?/) {
    tag = substr($0, RSTART, RLENGTH)
    gsub(/^<<-?[[:space:]]*|['"'"'"]/, "", tag)
    skip = 1; print; next
  }
  skip == 1 { if ($0 ~ "^[[:space:]]*" tag "[[:space:]]*$") skip = 0; next }
  { print }
' <<<"$cmd") || fail_closed $LINENO

# True when the command writes to something matching regex $1. Mirrors
# protect-docs.sh: the mutating token must reach the target without crossing a
# command separator, and sed/perl count only with -i — so `sed -n 1,20p` on this
# script is a read, not an attempt to edit the guard away.
writes_to() {
  grep -qE "(>>?[[:space:]]*[^|&;]*$1)|(\b(rm|mv|cp|tee|truncate|install|patch|shred|chmod|chown|ln|dd|touch|python3?|node|ruby)\b[^|&;]*$1)|((sed|perl)[[:space:]]+[^|&;]*-i[^|&;]*$1)|(\bgit[[:space:]]+(checkout|restore|apply|rm|mv|clean|stash)\b[^|&;]*$1)" <<<"$stripped"
}

# The marker and this script are what make the gate mean anything, so they are
# writable only inside a window a human already opened.
if writes_to '\.hooks-unlock' && ! unlocked; then
  deny "Creating the hook-bypass marker is reserved for a human. Run it yourself: touch .claude/.hooks-unlock"
fi
if writes_to 'no-hook-bypass' && ! unlocked; then
  deny "This is the hook-bypass guard itself. A human must unlock before it can be changed."
fi

# 1. --no-verify, on git or anything else that honours it.
grep -qE '(^|[[:space:]])--no-verify([[:space:]]|$)' <<<"$stripped" &&
  guard "Refusing \`--no-verify\`: it skips the pre-commit hooks."

# 2. git commit -n. Only for commit — `git push -n` is --dry-run and `git log -n`
#    takes a count, and neither skips anything.
grep -qE '\bgit\b[^|&;]*\bcommit\b[^|&;]*(^|[[:space:]])-n([[:space:]]|$)' <<<"$stripped" &&
  guard "Refusing \`git commit -n\`: it is --no-verify spelled shorter."

# 3. core.hooksPath aimed anywhere, by -c, --config, git config, or the env.
grep -qE 'core\.hooksPath' <<<"$stripped" &&
  guard "Refusing to redirect core.hooksPath: it disables every repository hook at once."

# 4. The skip variables the hook runners honour.
grep -qE '(^|[[:space:]])(SKIP|PREK_SKIP|PRE_COMMIT_ALLOW_NO_CONFIG|HUSKY|HUSKY_SKIP_HOOKS)=' <<<"$stripped" &&
  guard "Refusing to set a hook-skipping environment variable."

# 5. Explicit --no-hooks, used by some porcelain.
grep -qE '(^|[[:space:]])--no-hooks([[:space:]]|$)' <<<"$stripped" &&
  guard "Refusing \`--no-hooks\`."

exit 0

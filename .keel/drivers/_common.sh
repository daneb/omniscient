#!/bin/sh
# Shared helpers for keel driver adapters.
#
# Four adapters were doing the same five things — read the task, build a prompt,
# check the tool exists, diff the tree, emit a result — with four chances to get
# the JSON escaping subtly different. This is that logic once.
#
# A driver sources this, then does the one thing that is actually specific to it:
# invoking its tool. If you find yourself adding tool-specific logic *here*, it
# belongs in the adapter.

# keel_emit <status> <detail> [files-json]
# Always prints exactly one keel.driverresult/1 object. Every exit path in an
# adapter must go through this: a driver that dies without emitting is
# indistinguishable from one that hung.
keel_emit() {
  _status="$1"
  _detail="$2"
  _files="${3:-[]}"
  printf '{"schema":"keel.driverresult/1","status":"%s","files_changed":%s,"detail":"%s"}\n' \
    "$_status" "$_files" \
    "$(printf '%s' "$_detail" | sed 's/\\/\\\\/g; s/"/\\"/g' | tr -d '\n' | cut -c1-500)"
}

# keel_require <binary> <install hint>
# Blocks — never fails — when a tool is absent. A tool you have not installed
# says nothing about whether the agent can do the work.
keel_require() {
  command -v "$1" >/dev/null 2>&1 && return 0
  keel_emit blocked "$1 is not on PATH${2:+ ($2)}"
  exit 0
}

# keel_prompt <task-json>
# The task's prompt plus the constraints keel is going to gate on anyway. Stated
# to the agent because a constraint discovered at G2 costs a whole run.
keel_prompt() {
  printf '%s' "$1" | python3 -c '
import json, sys
t = json.load(sys.stdin)
out = [t["prompt"], ""]
scope = ", ".join(t.get("scope", []))
if scope:
    out.append("Stay strictly inside this scope: " + scope)
if t.get("budget_lines"):
    out.append("Keep the whole change under %d lines." % t["budget_lines"])
out.append("Edit the working tree directly. Do not commit.")
print("\n".join(out))
'
}

# keel_field <task-json> <key>
#
# Note the explicit None check rather than `or ''`: a budget of 0 is falsy in
# Python, and coercing it to empty is how "no change expected" silently became
# "no budget stated".
keel_field() {
  printf '%s' "$1" | python3 -c "
import json, sys
try:
    v = json.load(sys.stdin).get('$2')
except Exception:
    v = None
print('' if v is None else v)
"
}

# keel_changed_files
# What git says changed, as a JSON array. keel verifies against its own diff
# regardless — this is the driver's claim, and a mismatch is itself informative.
keel_changed_files() {
  git status --porcelain 2>/dev/null \
    | awk '{ $1=""; sub(/^ /, ""); print }' \
    | python3 -c 'import json,sys; print(json.dumps([l.strip() for l in sys.stdin if l.strip()]))'
}

keel_dirty_count() {
  git status --porcelain 2>/dev/null | wc -l | tr -d ' '
}

# keel_finish <before-count> <tool-name> <task-json>
# The common ending: ok if anything changed, failed if the tool ran and changed
# nothing. "Ran and did nothing" is an agentic failure, not a blocked one — the
# tool worked, the task did not get done.
#
# Except when nothing was asked for. A task budgeting zero lines of diff is
# saying "no change expected" — that is what keel's conformance probe does — and
# reporting `failed` there would mean every conformant driver failed the probe
# designed to confirm it is conformant.
keel_finish() {
  _files=$(keel_changed_files)
  _after=$(keel_dirty_count)
  _budget=$(keel_field "${3:-}" budget_lines)
  if [ "$1" = "$_after" ] && [ "$_after" = "0" ]; then
    if [ "$_budget" = "0" ]; then
      keel_emit ok "$2 made no changes, and none were asked for" "$_files"
    else
      keel_emit failed "$2 ran but changed nothing" "$_files"
    fi
  else
    keel_emit ok "$2 completed" "$_files"
  fi
}

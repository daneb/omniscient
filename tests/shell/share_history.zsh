#!/usr/bin/env zsh
# AC-2 — with SHARE_HISTORY, record the command entered in THIS shell.
#
# SHARE_HISTORY makes zsh import other sessions' history before each prompt.
# A hook that asks history what just ran can therefore be handed a command
# typed in a different terminal — which is the reported symptom.
source "${${(%):-%x}:A:h}/_lib.zsh"
omni_setup
setopt SHARE_HISTORY

# Another terminal wrote to the shared history file a moment ago.
print -s "echo from-another-terminal"

# This terminal runs something else entirely.
omni_run "echo mine"
omni_await 1 || omni_fail "nothing was captured"

captured=$(omni_captured)
[[ $captured == "echo from-another-terminal" ]] && \
    omni_fail "recorded another terminal's command"
[[ $captured == "echo mine" ]] || \
    omni_fail "expected 'echo mine', got '$captured'"
omni_pass "SHARE_HISTORY does not misattribute a command"

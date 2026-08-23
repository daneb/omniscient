#!/usr/bin/env zsh
# AC-4 — with HIST_IGNORE_DUPS, running a command twice records it twice.
#
# History suppresses the duplicate; the two executions are still two events, and
# a usage tracker that undercounts repeats reports the wrong favourites.
source "${${(%):-%x}:A:h}/_lib.zsh"
omni_setup
setopt HIST_IGNORE_DUPS

omni_run "ls -la"
omni_run "ls -la"
omni_await 2 || omni_fail "expected 2 captures, got $(omni_count)"

[[ $(omni_count) -eq 2 ]] || omni_fail "expected 2 captures, got $(omni_count)"
omni_captured | grep -qx "ls -la" || omni_fail "the command text was not recorded"
omni_pass "a repeated command is recorded each time"

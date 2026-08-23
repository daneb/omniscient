#!/usr/bin/env zsh
# AC-3 — with HIST_IGNORE_SPACE, a space-prefixed command must not cause its
# predecessor to be recorded a second time.
#
# A space-prefixed command never enters history at all, so a hook reading
# history gets the PREVIOUS command: the real one is lost and the old one is
# double-counted.
source "${${(%):-%x}:A:h}/_lib.zsh"
omni_setup
setopt HIST_IGNORE_SPACE

print -s "git status"           # an earlier, real command
omni_run " echo hidden"         # leading space: never enters history
omni_await 1 || omni_fail "nothing was captured"

captured=$(omni_captured)
[[ $captured == "git status" ]] && \
    omni_fail "recorded the previous command instead of the one that ran"
[[ $captured == " echo hidden" ]] || \
    omni_fail "expected ' echo hidden', got '$captured'"
omni_pass "HIST_IGNORE_SPACE does not displace a command"

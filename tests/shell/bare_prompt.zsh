#!/usr/bin/env zsh
# AC-5 — a prompt with no preceding command records nothing.
#
# precmd fires on every prompt, including the first one of a session and the one
# after an empty line. Capturing there would invent commands nobody ran.
source "${${(%):-%x}:A:h}/_lib.zsh"
omni_setup

_omniscient_precmd          # a prompt with no preexec before it
sleep 0.1

[[ $(omni_count) -eq 0 ]] || omni_fail "a bare prompt recorded $(omni_count) command(s)"
omni_pass "a bare prompt records nothing"

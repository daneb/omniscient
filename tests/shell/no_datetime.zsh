#!/usr/bin/env zsh
# AC-3 — a shell without zsh/datetime still records commands.
#
# The hook now loads the module itself (AC-1), so "unavailable" can no longer
# mean "not yet loaded". It means the module is genuinely absent — a minimal
# zsh build, or a locked-down box. Simulated here by unloading it after the
# hook has been sourced, which is exactly the state the hook would find itself
# in: no $EPOCHREALTIME at capture time.
#
# The property under test is that capture is guarded on "a command ran", never
# on the timer. A hook that conditions on its clock records nothing at all,
# for every command, silently.
OMNI_BARE_SHELL=1
source "${${(%):-%x}:A:h}/_lib.zsh"
omni_setup

zmodload -u zsh/datetime 2>/dev/null
[[ -z ${EPOCHREALTIME:-} ]] \
    || omni_fail "harness error: EPOCHREALTIME survived unloading zsh/datetime"

omni_run "echo untimed"
omni_await 1 || omni_fail "no command was recorded without a clock"

[[ $(omni_captured) == "echo untimed" ]] \
    || omni_fail "expected 'echo untimed', got '$(omni_captured)'"
omni_pass "a shell with no clock still records commands"

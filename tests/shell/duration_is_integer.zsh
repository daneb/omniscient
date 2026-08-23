#!/usr/bin/env zsh
# AC-6 — --duration is always a non-negative integer.
#
# zsh arithmetic on floats yields a float; `capture --duration 12.7` cannot
# parse, so the capture is discarded. int() would fix it but needs a module the
# hook cannot assume — so this runs in a bare shell, where the criterion has to
# hold without mathfunc. With the module present the old hook already passed,
# which is exactly why testing only that case proved nothing.
OMNI_BARE_SHELL=1
source "${${(%):-%x}:A:h}/_lib.zsh"
omni_setup

# Record the whole argument list, not just the command, so --duration is visible.
cat > "$OMNI_TMP/bin/omniscient" <<STUB
#!/bin/sh
[ "\$1" = "capture" ] || exit 0
printf '%s\n' "\$*" >> "$OMNI_CAPTURED"
STUB
chmod +x "$OMNI_TMP/bin/omniscient"

omni_run "sleep 0.05"
omni_await 1 || omni_fail "nothing was captured"

line=$(omni_captured)
dur=${${line#*--duration }%% *}
[[ -n $dur ]] || omni_fail "no --duration in: $line"
[[ $dur == <-> ]] || omni_fail "duration '$dur' is not a non-negative integer"
omni_pass "duration is an integer ($dur ms)"

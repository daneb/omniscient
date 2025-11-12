# Multi-Shell Support Specification

**Document Version:** 1.1
**Created:** 2025-11-12
**Updated:** 2025-11-12
**Status:** Planning - Approved

## Overview

This document specifies the implementation plan for extending Omniscient's command history tracking beyond Zsh to support **Bash** and **Fish**.

**Scope Decision:** PowerShell support is deferred until native preexec hooks are available in PowerShell itself. We will focus on delivering excellent support for Unix shells (Zsh, Bash, Fish) rather than implementing workarounds for PowerShell.

## Current State: Zsh Implementation

### Architecture
Omniscient currently uses Zsh's native hook system:
- **preexec**: Captures start time before command execution
- **precmd**: Captures command, exit code, and calculates duration after execution
- **Background execution**: Uses `&!` (background and disown) to prevent shell blocking

### Key Features
```zsh
_omniscient_preexec() {
    export _OMNISCIENT_START=$EPOCHREALTIME
}

_omniscient_precmd() {
    local exit_code=$?
    local cmd=$(fc -ln -1 | sed 's/^[[:space:]]*//')
    local duration=$(( int((end - _OMNISCIENT_START) * 1000) ))
    omniscient capture --exit-code "$exit_code" --duration "$duration" "$cmd" &>/dev/null &!
}
```

### Strengths
- ✅ Native hook support (no external dependencies)
- ✅ High-precision timing (`$EPOCHREALTIME`)
- ✅ Clean command extraction (`fc -ln -1`)
- ✅ Silent background execution (`&!`)
- ✅ Zero shell impact

### Lessons Learned
1. Job control notifications required `&!` instead of just `&`
2. Integer conversion needed for duration (`int()` wrapper)
3. Output redirection required (`&>/dev/null`)

---

## Shell Support Roadmap

### Supported Shells
1. ✅ **Zsh** (v1.0) - Currently supported
2. 🚧 **Bash** (v1.1) - In planning - Largest Unix user base
3. 🚧 **Fish** (v1.2) - In planning - Modern shell with native event support

### Future Consideration
- **PowerShell** - Deferred until native hook support is available
  - Active feature requests: [#15271](https://github.com/PowerShell/PowerShell/issues/15271), [#14484](https://github.com/PowerShell/PowerShell/issues/14484)
  - Will revisit when PowerShell team adds official preexec/precmd hooks
  - Workarounds (PSReadLine hacks, Start-Job overhead) provide poor UX

---

## Bash Support

### Hook Mechanism

**Challenge:** Bash lacks native preexec/precmd hooks.

**Solution Options:**

#### Option 1: bash-preexec Library (Recommended)
Use the established [bash-preexec](https://github.com/rcaloras/bash-preexec) library:
- ✅ Mature, production-tested (used by iTerm2, Fig, Bashhub)
- ✅ Supports Bash 3.1+
- ✅ Provides zsh-style preexec/precmd functions
- ❌ External dependency (users must source it)

#### Option 2: Native PROMPT_COMMAND + DEBUG trap
Implement hooks directly using Bash primitives:
- ✅ No external dependencies
- ✅ Full control over implementation
- ❌ More complex to maintain
- ❌ Edge cases to handle (completion mode, subshells)

**Recommendation:** Start with Option 1 (bash-preexec) for v1.1, evaluate Option 2 for v2.0.

### Implementation Plan

#### Phase 1: Basic Support (v1.1)
```bash
# Installation requirement
# User must first install bash-preexec:
# curl -sSL https://github.com/rcaloras/bash-preexec/raw/master/bash-preexec.sh -o ~/.bash-preexec.sh

# ~/.bashrc additions
source ~/.bash-preexec.sh

_omniscient_preexec() {
    _OMNISCIENT_START=$(date +%s%N)
}

_omniscient_precmd() {
    local exit_code=$?
    local cmd=$(history 1 | sed 's/^[ ]*[0-9]*[ ]*//')

    if [[ -n "$_OMNISCIENT_START" ]]; then
        local end=$(date +%s%N)
        local duration=$(( (end - _OMNISCIENT_START) / 1000000 ))

        omniscient capture --exit-code "$exit_code" --duration "$duration" "$cmd" &>/dev/null &
        disown

        unset _OMNISCIENT_START
    fi
}

preexec_functions+=(_omniscient_preexec)
precmd_functions+=(_omniscient_precmd)
```

#### Phase 2: Native Implementation (v2.0)
```bash
# Standalone implementation without bash-preexec
# Using DEBUG trap and PROMPT_COMMAND

_omniscient_debug_trap() {
    if [[ "$BASH_COMMAND" != "$PROMPT_COMMAND" ]]; then
        _OMNISCIENT_CMD="$BASH_COMMAND"
        _OMNISCIENT_START=$(date +%s%N)
    fi
}

_omniscient_prompt_command() {
    local exit_code=$?

    if [[ -n "$_OMNISCIENT_START" ]]; then
        local end=$(date +%s%N)
        local duration=$(( (end - _OMNISCIENT_START) / 1000000 ))

        omniscient capture --exit-code "$exit_code" --duration "$duration" "$_OMNISCIENT_CMD" &>/dev/null &
        disown

        unset _OMNISCIENT_START
        unset _OMNISCIENT_CMD
    fi
}

trap '_omniscient_debug_trap' DEBUG
PROMPT_COMMAND="_omniscient_prompt_command${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
```

### Technical Details

**Timing:**
- Bash < 5.0: `date +%s%N` (nanosecond precision)
- Bash 5.0+: Can use `$EPOCHREALTIME` (microsecond precision)

**Command Extraction:**
- `history 1`: Gets last command
- `sed 's/^[ ]*[0-9]*[ ]*//'`: Strips history number

**Background Jobs:**
- Use `&` for background + explicit `disown`
- Bash doesn't have `&!` shorthand like Zsh

**Compatibility:**
- Test on Bash 3.1+ (most common versions)
- macOS ships with Bash 3.2 by default
- Linux typically has Bash 4.0+

---

## Fish Support

### Hook Mechanism

**Native Support:** Fish has built-in event handlers!

Fish provides:
- `fish_preexec`: Triggered before command execution
- `fish_postexec`: Triggered after command execution
- `$CMD_DURATION`: Built-in variable with command duration in milliseconds
- `$status`: Last exit code

### Implementation

```fish
# ~/.config/fish/config.fish

function _omniscient_preexec --on-event fish_preexec
    set -g _OMNISCIENT_CMD $argv[1]
end

function _omniscient_postexec --on-event fish_postexec
    set -l exit_code $status
    set -l duration $CMD_DURATION

    if test -n "$_OMNISCIENT_CMD"
        omniscient capture --exit-code $exit_code --duration $duration "$_OMNISCIENT_CMD" &>/dev/null &
        disown

        set -e _OMNISCIENT_CMD
    end
end
```

### Technical Details

**Strengths:**
- ✅ Native event system (no dependencies)
- ✅ Built-in `$CMD_DURATION` (accurate milliseconds)
- ✅ Clean event handler syntax
- ✅ Modern shell with good documentation

**Considerations:**
- Fish makes no guarantees on timing or that events fire for every command
- Need to test edge cases (multiline commands, job control)
- Background job syntax: `&` + `disown` (no `&!`)

**Compatibility:**
- Fish 3.0+ (stable event system)
- Fish 4.0+ (latest features)

---

## Implementation Phases

### Phase 1: v1.1 - Bash Support (6-8 weeks)

**Week 1-2: Core Implementation**
- [ ] Add `ShellType::Bash` to enum
- [ ] Implement `generate_bash()` method
- [ ] Add bash-preexec dependency documentation
- [ ] Create Bash-specific tests

**Week 3-4: Testing & Refinement**
- [ ] Test on Bash 3.2 (macOS default)
- [ ] Test on Bash 4.x, 5.x (Linux)
- [ ] Handle edge cases (multiline, pipes, redirects)
- [ ] Performance benchmarks

**Week 5-6: Documentation & Examples**
- [ ] Update README with Bash instructions
- [ ] Create `examples/bash_hook.sh`
- [ ] Installation script updates
- [ ] User migration guide (Zsh → Bash)

**Week 7-8: Shell Detection & Auto-setup**
- [ ] Implement shell auto-detection in `omniscient init`
- [ ] Generate appropriate hooks based on detected shell
- [ ] Add `--shell` flag for manual selection
- [ ] Update CLI help text

### Phase 2: v1.2 - Fish Support (4-6 weeks)

**Week 1-2: Core Implementation**
- [ ] Add `ShellType::Fish` to enum
- [ ] Implement `generate_fish()` method
- [ ] Leverage `$CMD_DURATION` for accuracy
- [ ] Create Fish-specific tests

**Week 3-4: Testing & Polish**
- [ ] Test on Fish 3.x, 4.x
- [ ] Edge case handling
- [ ] Performance validation
- [ ] Documentation and examples

**Week 5-6: Integration & Release**
- [ ] Update auto-detection logic
- [ ] Create `examples/fish_config.fish`
- [ ] Update README
- [ ] Migration guide
- [ ] Release v1.2

### Future: PowerShell Support (Deferred)

PowerShell support is deferred until native preexec/precmd hooks are added to PowerShell itself.

**Why Deferred:**
- No native hook mechanism (unlike Zsh, Bash with bash-preexec, or Fish)
- Workarounds have significant limitations and complexity
- PSReadLine integration is fragile and may interfere with user workflows
- Start-Job background execution has notable performance overhead

**Monitoring:**
- Track PowerShell issues [#15271](https://github.com/PowerShell/PowerShell/issues/15271) and [#14484](https://github.com/PowerShell/PowerShell/issues/14484)
- Will reconsider when PowerShell team adds official hook support
- Can evaluate community interest after Bash + Fish support is released

---

## Code Architecture

### Module Structure

```rust
// src/shell.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Zsh,
    Bash,
    Fish,
    // Future: PowerShell (when native hooks available)
}

impl ShellHook {
    fn generate(&self) -> String {
        match self.shell_type {
            ShellType::Zsh => self.generate_zsh(),
            ShellType::Bash => self.generate_bash(),
            ShellType::Fish => self.generate_fish(),
        }
    }

    fn generate_bash(&self) -> String {
        // Bash-specific implementation
    }

    fn generate_fish(&self) -> String {
        // Fish-specific implementation
    }

    fn installation_instructions(&self) -> String {
        // Shell-specific instructions
    }
}
```

### Shell Detection

```rust
// src/shell.rs

impl ShellHook {
    /// Auto-detect the current shell
    pub fn detect_shell() -> Result<ShellType> {
        // Check $SHELL environment variable
        if let Ok(shell) = std::env::var("SHELL") {
            if shell.contains("zsh") {
                return Ok(ShellType::Zsh);
            } else if shell.contains("bash") {
                return Ok(ShellType::Bash);
            } else if shell.contains("fish") {
                return Ok(ShellType::Fish);
            }
        }

        // Default to Zsh if detection fails
        Ok(ShellType::Zsh)
    }
}
```

### CLI Updates

```rust
// src/main.rs

#[derive(Subcommand)]
enum Commands {
    /// Initialize shell integration (generates hook code)
    Init {
        /// Specify shell type (auto-detected if not provided)
        #[arg(long)]
        shell: Option<String>,
    },
    // ... other commands
}

// In main()
Commands::Init { shell } => {
    let shell_type = if let Some(shell_name) = shell {
        match shell_name.as_str() {
            "zsh" => ShellType::Zsh,
            "bash" => ShellType::Bash,
            "fish" => ShellType::Fish,
            _ => return Err(Error::Config(format!("Unsupported shell: {}", shell_name))),
        }
    } else {
        ShellHook::detect_shell()?
    };

    let hook = ShellHook::new(shell_type);
    println!("{}", hook.generate());
    eprintln!("\n{}", hook.installation_instructions());
}
```

---

## Testing Strategy

### Unit Tests

Each shell implementation needs:
1. **Generation tests** - Verify hook code is generated correctly
2. **Component tests** - Check for required functions/variables
3. **Syntax tests** - Validate shell syntax (via shell -n)
4. **Edge case tests** - Multiline, special characters, etc.

```rust
#[test]
fn test_bash_hook_generation() {
    let hook = ShellHook::new(ShellType::Bash);
    let code = hook.generate();

    assert!(code.contains("_omniscient_preexec"));
    assert!(code.contains("_omniscient_precmd"));
    assert!(code.contains("preexec_functions"));
    assert!(code.contains("precmd_functions"));
}

#[test]
fn test_fish_hook_generation() {
    let hook = ShellHook::new(ShellType::Fish);
    let code = hook.generate();

    assert!(code.contains("fish_preexec"));
    assert!(code.contains("fish_postexec"));
    assert!(code.contains("CMD_DURATION"));
}
```

### Integration Tests

1. **Manual testing** in each shell environment
2. **Docker containers** with different shell versions
3. **GitHub Actions CI** for automated testing
4. **Real-world scenarios**: pipes, redirects, aliases, functions

### Compatibility Matrix

| Shell | Version | Platform | Status |
|-------|---------|----------|--------|
| Zsh | 5.8+ | macOS, Linux | ✅ Supported (v1.0) |
| Bash | 3.2 | macOS | 🚧 Planned (v1.1) |
| Bash | 4.x | Linux | 🚧 Planned (v1.1) |
| Bash | 5.x | Linux | 🚧 Planned (v1.1) |
| Fish | 3.x | macOS, Linux | 🚧 Planned (v1.2) |
| Fish | 4.x | macOS, Linux | 🚧 Planned (v1.2) |
| PowerShell | 5.1+ | Windows | ⏸️ Deferred (awaiting native hooks) |
| PowerShell | 7.x | All platforms | ⏸️ Deferred (awaiting native hooks) |

---

## Documentation Requirements

### User Documentation

1. **README updates**
   - Multi-shell support section
   - Shell-specific setup instructions
   - Compatibility matrix

2. **Installation guides** (per shell)
   - Prerequisites
   - Step-by-step setup
   - Troubleshooting
   - Uninstallation

3. **Examples directory**
   - `examples/zsh_hook.sh` (exists)
   - `examples/bash_hook.sh` (v1.1)
   - `examples/fish_config.fish` (v1.2)

### Developer Documentation

1. **Architecture documentation**
   - Shell abstraction design
   - Hook mechanism comparison
   - Implementation notes

2. **Testing guide**
   - How to test each shell
   - Docker test environments
   - CI/CD setup

3. **Contribution guide**
   - Adding new shell support
   - Shell-specific considerations
   - Code review checklist

---

## Risks & Mitigation

### Risk 1: bash-preexec Dependency (Bash)
**Impact:** Users must install external library
**Probability:** High
**Mitigation:**
- Document clearly in README with installation instructions
- Provide automated installation script
- Consider bundling bash-preexec (check license compatibility)
- Plan native implementation for v2.0 (lower priority)

### Risk 2: Fish Event Timing Guarantees
**Impact:** Fish docs say "no guarantees" on event timing
**Probability:** Low (well-tested in practice)
**Mitigation:**
- Thorough testing in real-world scenarios
- Document known limitations
- Monitor Fish issue tracker for event system changes
- Built-in $CMD_DURATION provides reliable timing

### Risk 3: Platform-Specific Issues
**Impact:** Behavior may vary across platforms (macOS vs Linux)
**Probability:** Medium
**Mitigation:**
- Extensive cross-platform testing (macOS, various Linux distros)
- Platform-specific documentation where needed
- CI testing on multiple platforms
- Community beta testing before release

### Risk 4: Shell Version Fragmentation
**Impact:** Old shell versions may not support features
**Probability:** Medium (especially Bash 3.2 on macOS)
**Mitigation:**
- Clear minimum version requirements
- Graceful error messages when unsupported
- Version detection in `omniscient init`
- Separate code paths for old vs new versions if needed

### Risk 5: User Migration Friction
**Impact:** Existing Zsh users may hesitate to try other shells
**Probability:** Low
**Mitigation:**
- Make migration seamless (export/import)
- Document side-by-side usage (multiple shells)
- Clear benefits in documentation
- Community examples and use cases

---

## Success Criteria

### v1.1 (Bash)
- [ ] Bash hook generation works correctly
- [ ] Commands captured accurately
- [ ] Duration accuracy within 10ms
- [ ] No shell performance impact (< 10ms overhead)
- [ ] Silent background execution (no job notifications)
- [ ] bash-preexec integration documented
- [ ] Documentation complete (README, examples, troubleshooting)
- [ ] Tested on Bash 3.2, 4.x, 5.x (macOS + Linux)
- [ ] CI/CD pipeline for Bash testing
- [ ] Release notes and migration guide

### v1.2 (Fish)
- [ ] Fish event handlers work reliably
- [ ] Leverages built-in $CMD_DURATION
- [ ] Clean integration with Fish config
- [ ] Silent background execution
- [ ] No performance impact
- [ ] Documentation complete (README, examples)
- [ ] Tested on Fish 3.x, 4.x (macOS + Linux)
- [ ] CI/CD pipeline for Fish testing
- [ ] Release notes and migration guide

---

## Alternative Approaches Considered

### 1. Shell-Agnostic Wrapper
Create a wrapper script that works across all shells.
- ❌ Rejected: Too invasive, affects all commands
- ❌ Performance overhead
- ❌ Breaks shell features (aliases, functions)

### 2. Kernel-Level Tracing (eBPF/dtrace)
Use system-level tracing instead of shell hooks.
- ❌ Rejected: Requires root/elevated privileges
- ❌ Platform-specific (Linux only for eBPF)
- ❌ Complex setup
- ✅ Future consideration for v3.0

### 3. Terminal Emulator Integration
Integrate with terminal emulators (iTerm2, Alacritty, etc.)
- ❌ Rejected: Requires terminal-specific implementations
- ❌ Doesn't work in SSH sessions
- ❌ Limited portability
- ✅ Possible complementary approach

---

## Open Questions

1. **Bash**: Should we bundle bash-preexec or require separate installation?
2. **PowerShell**: Is Start-Job performance acceptable, or should we use runspaces?
3. **Fish**: How reliable are the event handlers in edge cases?
4. **All**: Should we support older shell versions or set minimum requirements?
5. **Architecture**: Should shell-specific code be in separate files or keep in shell.rs?

---

## References

### Bash
- [bash-preexec](https://github.com/rcaloras/bash-preexec) - Preexec/precmd for Bash
- [Bash Manual - PROMPT_COMMAND](https://www.gnu.org/software/bash/manual/html_node/Bash-Variables.html)
- [Bash Manual - DEBUG trap](https://www.gnu.org/software/bash/manual/html_node/Bourne-Shell-Builtins.html#index-trap)

### Fish
- [Fish Documentation - Event Handlers](https://fishshell.com/docs/current/language.html#event-handlers)
- [Fish Documentation - fish_preexec](https://fishshell.com/docs/current/cmds/function.html#event-handlers)
- [Fish Documentation - $CMD_DURATION](https://fishshell.com/docs/current/language.html#envvar-CMD_DURATION)

### PowerShell
- [PowerShell Issue #15271 - Pre-exec hook](https://github.com/PowerShell/PowerShell/issues/15271)
- [PowerShell Issue #14484 - Pre-execution functionality](https://github.com/PowerShell/PowerShell/issues/14484)
- [PSReadLine Documentation](https://docs.microsoft.com/en-us/powershell/module/psreadline/)

### Cross-Shell Resources
- [Starship Prompt](https://github.com/starship/starship) - Multi-shell prompt (reference implementation)
- [Atuin](https://github.com/ellie/atuin) - Shell history sync (similar project)

---

## Appendix: Shell Comparison

| Feature | Zsh | Bash | Fish |
|---------|-----|------|------|
| **Native Hooks** | ✅ Yes | ❌ No (needs bash-preexec) | ✅ Yes |
| **Preexec** | ✅ preexec | ⚠️ Via library | ✅ fish_preexec |
| **Precmd** | ✅ precmd | ⚠️ Via library | ✅ fish_postexec |
| **High-Res Timing** | ✅ $EPOCHREALTIME | ⚠️ date +%s%N | ✅ $CMD_DURATION (built-in!) |
| **Exit Code** | ✅ $? | ✅ $? | ✅ $status |
| **Command Text** | ✅ fc -ln -1 | ✅ history 1 | ✅ Event arg |
| **Background Jobs** | ✅ &! (silent) | ⚠️ & + disown | ⚠️ & + disown |
| **Implementation Complexity** | ⭐ Simple | ⭐⭐ Medium | ⭐ Simple |
| **User Setup Complexity** | ⭐ Simple | ⭐⭐ Medium | ⭐ Simple |
| **Platform Support** | macOS, Linux | macOS, Linux | macOS, Linux |
| **External Dependencies** | None | bash-preexec | None |
| **Omniscient Support** | ✅ v1.0 | 🚧 v1.1 | 🚧 v1.2 |

### Key Insights

**Easiest to Implement:** Zsh (done) & Fish (native events)
**Most Users:** Bash (but requires bash-preexec library)
**Best Built-in Support:** Fish ($CMD_DURATION is perfect for our use case)

---

## Next Steps

### Immediate (Planning Complete)
1. ✅ Specification reviewed and approved
2. ✅ Scope refined: Bash + Fish only (PowerShell deferred)
3. Create GitHub issues for implementation phases
4. Update main [roadmap.md](roadmap.md) with timeline

### Phase 1: Bash Implementation (v1.1)
1. Create feature branch: `feature/bash-support`
2. Follow implementation checklist (6-8 weeks)
3. Beta testing with community
4. Release v1.1

### Phase 2: Fish Implementation (v1.2)
1. Create feature branch: `feature/fish-support`
2. Follow implementation checklist (4-6 weeks)
3. Beta testing with community
4. Release v1.2

### Future Monitoring
- Track PowerShell issues for native hook support
- Evaluate Windows user demand
- Reconsider PowerShell when native hooks arrive

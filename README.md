## Getting Started

most common mistake in the terminal? writing the word `clear`:

- lea
- lr
- clrea
- l
- and the list is looong

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Test locally

Phase 1 CLI: corrects mistyped `clear`, `ls`, `cd`. First time prompts "did you mean ...?", remembers your confirmation, auto-corrects next time. You always press Enter — nothing runs on its own.

> The CLI binary is named **`clreactl`** (not `clrea`) so the typo `clrea` stays free for the shell hook to catch.

### 1. Build

```sh
cd clrea
cargo build --release
```

Binary lands at `clrea/target/release/clreactl`.

### 2. Put `clreactl` on your PATH

Pick one:

```sh
# option A — symlink into ~/.local/bin (make sure it's on PATH)
mkdir -p ~/.local/bin
ln -sf "$PWD/target/release/clreactl" ~/.local/bin/clreactl

# option B — export the build dir for this shell only
export PATH="$PWD/target/release:$PATH"
```

If you previously symlinked an old `clrea` binary, remove it now:

```sh
rm -f ~/.local/bin/clrea
```

Verify:

```sh
which clreactl
clreactl --version
```

### 3. Sanity-check the binary (no shell hook yet)

```sh
clreactl suggest clrea     # prints: ask<TAB>clear   (exit 1)
clreactl suggest sl        # prints: ask<TAB>ls      (exit 1)
clreactl suggest hello     # prints nothing          (exit 2 = unknown)
```

### 4. Try the zsh hook in an isolated subshell first

This avoids touching your `.zshrc` until you're happy.

```sh
zsh -f                                # fresh zsh, no rc loaded
export PATH="$HOME/.local/bin:$PATH"  # so clreactl is found
eval "$(clreactl init zsh)"

clrea                           # → "did you mean 'clear'?" — press Enter
clrea                           # second time → auto-corrects to clear
sl                              # → "did you mean 'ls'?" — press Enter
cdd                             # → "did you mean 'cd'?" — press Enter

exit                            # leave the subshell
```

Inspect what got learned:

```sh
cat ~/.config/clrea/history.toml
```

To reject a suggestion: type any character (e.g. `n`) before pressing Enter. Nothing is learned, nothing runs.

#### If zsh asks "correct 'clera' to 'clrea'?" instead of clrea hooking in

That's zsh's built-in spell-correct (`setopt correct`) firing before `command_not_found_handler`. The shell hook disables it on load (`unsetopt correct correct_all`). If your prompt framework re-enables it, add this to your `.zshrc` _after_ the `eval` line:

```sh
unsetopt correct correct_all
```

### 5. Persist across all new shells

Once it behaves, append this to `~/.zshrc`:

```sh
eval "$(clreactl init zsh)"
```

Reload:

```sh
source ~/.zshrc
```

Bash users: same idea with `~/.bashrc` and `clreactl init bash`.

### 6. Reset learned history

```sh
rm ~/.config/clrea/history.toml
```

### 7. Add your own typo rules

Create `~/.config/clrea/rules.toml`:

```toml
[ls]
typos = ["lll", "lsa"]

[clear]
typos = ["clr"]
```

User rules merge with the built-in defaults.

### Safety

- Only `clear`, `ls`, `cd` can ever be auto-run — hard-coded whitelist.
- `clreactl` never executes anything by itself; the shell hook waits for your Enter.
- `clreactl learn <typo> <cmd>` rejects `ny `cmd` outside the whitelist.

after making changes:
Re-test steps:

```
cargo build --release
rm -f ~/.local/bin/clrea
ln -sf "$PWD/target/release/clreactl" ~/.local/bin/clreactl
  zsh -f
  export PATH="$HOME/.local/bin:$PATH"
  eval "$(clreactl init zsh)"
clrea # should prompt "did you mean 'clear'?"
clera # same
sl # → ls
```

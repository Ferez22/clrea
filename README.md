# clrea

A tiny shell helper that catches mistyped `clear`, `ls`, and `cd` in zsh/bash,
and asks you "did you mean ...?" — one keypress to confirm, any other key to
cancel. Nothing runs without your confirmation.

```
$ clrea
🔴 clrea: did you mean clear ?  [⏎ confirm · ⎋ cancel]
⚪ → clear
```

It learns from your typos over time (history is recorded), but it always asks
before doing anything. Only `clear`, `ls`, `cd` can ever be auto-run — that
list is hard-coded.

---

## Requirements

- Rust toolchain (1.85+ for Edition 2024) — install with:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- zsh or bash

---

## Build

```sh
cd clrea
cargo build --release
```

Binary: `clrea/target/release/clreactl`.

> The CLI binary is named **`clreactl`** (control), not `clrea`, so the typo
> `clrea` stays free for the shell hook to catch.

---

## Install

Put `clreactl` somewhere on your `PATH`:

```sh
# option A — symlink into ~/.local/bin (make sure it's on PATH)
mkdir -p ~/.local/bin
ln -sf "$PWD/target/release/clreactl" ~/.local/bin/clreactl

# option B — only for the current shell
export PATH="$PWD/target/release:$PATH"
```

Verify:

```sh
which clreactl
clreactl --version
```

Then load the shell hook. Pick zsh or bash:

```sh
# zsh — add to ~/.zshrc
eval "$(clreactl init zsh)"

# bash — add to ~/.bashrc
eval "$(clreactl init bash)"
```

Reload:

```sh
source ~/.zshrc   # or ~/.bashrc
```

---

## Usage

Just type. When you mistype:

```
$ clrea
🔴 clrea: did you mean clear ?  [⏎ confirm · ⎋ cancel]
```

- **Enter** → runs `clear`, records the mapping.
- **Any other key** (Esc, `n`, `q`, …) → cancels, nothing runs, nothing learned.

Built-in rules cover common typos: `clrea`, `claer`, `clera`, `cler`, `sl`,
`lss`, `dc`, `cdd`, etc. Anything within edit-distance 2 of `clear`/`ls`/`cd`
also matches.

### Add your own typos

Create `~/.config/clrea/rules.toml`:

```toml
[ls]
typos = ["lll", "lsa"]

[clear]
typos = ["clr"]
```

User rules merge with the built-in defaults.

### Inspect or reset learned history

```sh
cat   ~/.config/clrea/history.toml                               # Linux
cat   "$HOME/Library/Application Support/clrea/history.toml"     # macOS

rm    ~/.config/clrea/history.toml      # forget everything
```

---

## Test it locally before adding to your shell rc

Best to try in a clean subshell first.

### 1. Sanity-check the binary

```sh
clreactl suggest clrea     # → ask<TAB>clear   (exit 1)
clreactl suggest sl        # → ask<TAB>ls      (exit 1)
clreactl suggest hello     # → empty           (exit 2)
```

### 2. Try the hook in an isolated zsh

```sh
zsh -f                                # fresh zsh, no rc loaded
export PATH="$HOME/.local/bin:$PATH"
eval "$(clreactl init zsh)"

clrea          # → "did you mean 'clear'?" → press Enter
clera          # → same
sl             # → "did you mean 'ls'?"
cdd            # → "did you mean 'cd'?"
exit
```

If zsh shows `correct 'clera' to 'clrea'?` instead of clrea's prompt, your
prompt framework (Oh-My-Zsh, Powerlevel10k, …) re-enabled `setopt correct`.
The hook disables it on load, but if it comes back, add this **after** the
`eval` line in your `.zshrc`:

```sh
unsetopt correct correct_all
```

### 3. Persist

When happy, leave the `eval "$(clreactl init zsh)"` line in your `.zshrc`.

---

## Develop

### Run tests

```sh
cargo test
```

### Project layout

```
clrea/
├── Cargo.toml              # bin = "clreactl"
├── assets/rules.toml       # built-in typo rules
├── shell/
│   ├── clrea.zsh           # zsh hook
│   └── clrea.bash          # bash hook
└── src/
    ├── main.rs             # clap subcommands
    ├── suggest.rs          # dispatch logic
    ├── rules.rs            # rule loading + Levenshtein fallback
    └── history.rs          # learned-mappings storage
```

### Subcommands

```
clreactl suggest <typo>     # prints "ask\t<correct>" or nothing
clreactl learn <typo> <cmd> # records mapping; <cmd> must be in whitelist
clreactl init <shell>       # prints the shell hook (zsh|bash)
```

---

## Safety

- **Whitelist:** only `clear`, `ls`, `cd` can ever be executed by the hook.
  Hard-coded in `src/suggest.rs`.
- **Always asks:** the hook never executes anything until you press Enter.
- **`learn` is guarded:** `clreactl learn foo rm` is rejected — you cannot
  poison the history file with arbitrary commands.

---

## License

TBD.

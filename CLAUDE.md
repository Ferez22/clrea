# CLAUDE.md

Context for Claude Code working on this repo.

## What this project is

`clrea` is a small CLI + shell hook that catches mistyped `clear`, `ls`, `cd`
in zsh/bash and offers a one-keypress correction. Phase 1 scope only — no
auto-execution, no other commands.

Behaviour:
- User types a typo (e.g. `clrea`), hits Enter.
- Shell `command_not_found_handler` calls the binary `clreactl suggest <typo>`.
- If a match is found, the hook prints `🔴 clrea: did you mean clear ?`
  and reads a single key.
- Enter → run the corrected command, record the typo→cmd mapping in history.
- Any other key (incl. Esc) → cancel, nothing runs.
- Always prompts. No silent auto-correct (deliberate — user wanted explicit
  confirmation every time).

## Layout

```
clrea/                       # crate root
├── Cargo.toml               # bin = "clreactl" (NOT "clrea" — see gotcha)
├── assets/rules.toml        # built-in typo rules, baked in via include_str!
├── shell/
│   ├── clrea.zsh            # command_not_found_handler for zsh
│   └── clrea.bash           # command_not_found_handle for bash
└── src/
    ├── main.rs              # clap subcommands: suggest / learn / init
    ├── suggest.rs           # dispatch: history → rules → unknown
    ├── rules.rs             # default + user rules.toml, Levenshtein fallback
    └── history.rs           # ~/.config/clrea/history.toml, LRU 500, whitelist
README.md                    # user-facing docs
```

## Key conventions

- **Whitelist:** `suggest::WHITELIST = ["clear", "ls", "cd"]`. Only these can
  ever be auto-executed. `history::learn_into` rejects anything else. Keep
  this hard-coded — it's the safety boundary.
- **Exit codes** from `clreactl suggest <typo>`: `0` = (legacy "auto", not used
  now) / `1` = ask, `2` = unknown. Shell hook treats `1` as "show prompt".
- **Output format** of `suggest`: `<verb>\t<correct>` on stdout. Hook splits
  on tab.
- **Testability:** filesystem-touching code is split into pure helpers
  (`match_in`, `learn_into`, `load_from`, `save_to`, etc.) so tests don't need
  to mock dirs. Public `match_typo` / `lookup` / `learn` keep the original
  filesystem-backed API for `main.rs`.
- **Config dir:** `dirs::config_dir()/clrea/` — on macOS that's
  `~/Library/Application Support/clrea/`, on Linux `~/.config/clrea/`.
- **Rules merging:** built-in `assets/rules.toml` is `include_str!`'d at compile
  time. User file at `<config>/rules.toml` is merged on top (additive).

## Gotchas

- **Binary name is `clreactl`, not `clrea`.** If the binary were called `clrea`,
  typing `clrea` in the shell would just run the binary's help instead of
  triggering `command_not_found_handler`. Don't rename it back without
  reconciling that.
- **Zsh `setopt correct`** intercepts before `command_not_found_handler` and
  shows its own `correct 'clera' to ...?` prompt. The hook calls
  `unsetopt correct correct_all` on load. Some prompt frameworks (Oh-My-Zsh,
  Powerlevel10k) re-enable it — README documents the workaround.
- **`levenshtein("ls","cd") == 2`** so a naïve fallback would cross-match
  short whitelisted commands. `match_in` short-circuits with
  `WHITELIST.contains(&typo)` before the Levenshtein loop. Don't remove that
  guard.
- **`MAX_DISTANCE = 2`** in `rules.rs` is tuned for `clrea→clear` (distance 2).
  Tightening to 1 breaks that case.
- **Edition 2024** in `Cargo.toml`. Needs a recent enough Rust toolchain.

## Common tasks

- Run tests: `cargo test`
- Build release: `cargo build --release` → `target/release/clreactl`
- Manual smoke test: see "Testing locally" in `README.md`.
- Reset learned typos: `rm ~/.config/clrea/history.toml` (or
  `~/Library/Application Support/clrea/history.toml` on macOS).

## Out of scope (don't add without asking)

- Correcting commands beyond `clear`, `ls`, `cd`.
- Auto-executing without user keypress.
- Network calls, telemetry, update checks.
- Rewriting the user's shell line buffer (we only handle "command not
  found" — we never inject keystrokes).

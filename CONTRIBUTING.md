# Contributing to clrea

Thanks for your interest! clrea is small on purpose — read this before opening
a large PR.

## Project scope

Phase 1 corrects exactly three commands: `clear`, `ls`, `cd`. The whitelist is
hard-coded and is the safety boundary of the tool. PRs that broaden it will be
discussed before merging.

Out of scope (for now):
- Auto-executing without a user keypress.
- Rewriting the user's shell line buffer / injecting keystrokes.
- Network calls, telemetry, update checks.

## Getting started

```sh
git clone https://github.com/faresaouani/clrea
cd clrea/clrea
cargo build
cargo test
```

Everything lives under the inner `clrea/` crate dir.

## Local testing

See the "Test it locally" section of the [README](README.md). Best to test the
shell hook in a `zsh -f` subshell first.

## Before submitting a PR

```sh
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

CI runs the same on Ubuntu and macOS.

## Style

- Edition 2024. Keep dependencies minimal.
- Prefer pure functions for testable logic; isolate filesystem/IO at the edges
  (`load_from`/`save_to` style).
- Don't add comments that just describe what the code does. Comments should
  explain *why* — usually a non-obvious constraint or gotcha.
- New tests for new behavior. `cargo test` must stay green.

## Commit messages

Conventional-ish prefix is appreciated but not required:

```
feat: add fish shell hook
fix: handle empty typo gracefully
docs: clarify install steps
test: cover lru truncation
```

Keep the subject ≤ 72 chars. Body explains *why*, not *what*.

## Reporting bugs

Open an issue with:
- OS + shell + version (`zsh --version`, `bash --version`).
- `clreactl --version`.
- Minimal reproduction (the typo you typed and what happened).

## License

By contributing, you agree your contributions will be dual-licensed under
MIT OR Apache-2.0, matching the project license.

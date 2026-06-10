# Releasing clrea

Distribution is via **prebuilt binaries** attached to a GitHub Release. The
Homebrew formula downloads those binaries directly, so users never pull the
Rust/LLVM toolchain.

## 1. Tag the release

Bump `version` in `clrea/Cargo.toml` if needed, commit, then:

```sh
git tag v0.1.0
git push origin v0.1.0
```

Pushing a `v*` tag triggers `.github/workflows/release.yml`, which:

- builds `clreactl` for macOS arm64/x86_64 and Linux arm64/x86_64,
- packages each as `clreactl-<target>.tar.gz`,
- uploads each tarball plus a matching `.tar.gz.sha256` to the GitHub Release.

Wait for that workflow to finish (Actions tab).

## 2. Update the Homebrew tap

The tap lives in a separate repo: **`Ferez22/homebrew-clrea`**
(formula at `Formula/clrea.rb`).

For each new release, in `Formula/clrea.rb`:

- bump `version`,
- update the four `url` lines to the new tag,
- replace each `sha256` with the value from the matching `*.sha256` asset.

Grab all four checksums at once:

```sh
v=v0.1.0
for t in aarch64-apple-darwin x86_64-apple-darwin \
         aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu; do
  echo "$t:"
  curl -sL "https://github.com/Ferez22/clrea/releases/download/$v/clreactl-$t.tar.gz.sha256"
done
```

Commit and push the tap repo. Users update with `brew upgrade clrea`.

## First-time tap setup

If the tap repo does not exist yet:

```sh
# from the prepared tap directory (../homebrew-clrea relative to this repo)
cd ../homebrew-clrea
git init
git add .
git commit -m "clrea formula v0.1.0"
git branch -M main
# create the repo on GitHub named exactly: homebrew-clrea
git remote add origin https://github.com/Ferez22/homebrew-clrea.git
git push -u origin main
```

The repo **must** be named `homebrew-clrea` so that
`brew install Ferez22/clrea/clrea` resolves.

## Verify locally before publishing

```sh
brew install --build-from-source ./Formula/clrea.rb   # from the tap dir
brew test clrea
brew audit --strict --online clrea
```

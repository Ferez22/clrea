# Releasing clrea

Steps to cut a release and update the Homebrew tap.

## 1. Tag the release

Bump `version` in `clrea/Cargo.toml` if needed, commit, then:

```sh
git tag v0.1.0
git push origin v0.1.0
```

GitHub auto-generates a source tarball for any tag at:

```
https://github.com/Ferez22/clrea/archive/refs/tags/v0.1.0.tar.gz
```

(Optionally also create a GitHub Release from the tag for nicer notes.)

## 2. Compute the tarball SHA-256

```sh
curl -sL https://github.com/Ferez22/clrea/archive/refs/tags/v0.1.0.tar.gz | shasum -a 256
```

## 3. Update the Homebrew tap

The tap lives in a separate repo: **`Ferez22/homebrew-clrea`**
(formula at `Formula/clrea.rb`).

For a new release, update in `Formula/clrea.rb`:

- `url` — point at the new tag tarball
- `sha256` — the value from step 2

Commit and push the tap repo. Users get the update with `brew upgrade clrea`.

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

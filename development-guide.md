# Development Guide

This guide describes how to prepare and publish a new release of the `keepsorted` crate to GitHub and [crates.io](https://crates.io/crates/keepsorted).

## 1. Bump Version & Merge PR

1. Update `keepsorted` version in `Cargo.toml`, [#45](https://github.com/maksym-arutyunyan/keepsorted/pull/45/files)

## 2. Create GitHub Release

1. Identify the merge commit.
2. Go to [Releases](https://github.com/maksym-arutyunyan/keepsorted/releases) → **Draft a new release**.
3. Set:
   - Tag: `vX.X.X`
   - Target: the merge commit
   - Title: `vX.X.X`
   - Previous tag: last release
   - Notes: click **Generate release notes**, edit if needed
4. Click **Publish release**.

## 3. Publish to crates.io

1. Get a crates.io token at [Account Settings → API Tokens](https://crates.io/settings/tokens)
2. Authenticate:
   ```bash
   cargo login
   ```
2. Check out the release tag:
   ```bash
   git checkout vX.X.X
   ```
3. Publish the crate:
   ```bash
   cargo publish -p keepsorted
   ```
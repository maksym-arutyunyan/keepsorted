# Development Guide

## Release Preparation

There is one package published to crates.io: `keepsorted`.
Before publishing it, you should create a PR to bump the version of the package, and then cut a new release on GitHub after the PR with the new version is merged. Let's say it's version `vX.X.X`.

Here's an example PR bumping the versions: <TODO>.

## Steps to Cut a Release

1. Identify the commit for the release, e.g. <TODO>.
2. Draft a new pre-release:
    - Click on **Draft a new release** at the [releases page](https://github.com/dfinity/keepsorted/releases), and make sure the correct commit is selected.
    - Create a new tag named `vX.X.X`.
    - Set the title to `vX.X.X`.
    - Choose the previous tag as the last release.
    - Add release notes. GitHub can generate them by clicking **Generate release notes**, modify as needed.
3. Click **Publish release** when ready.

## Steps to Publish the Package to crates.io

1. Generate an API token to use with crates.io:  
   Log in to crates.io with your GitHub account, go to **Account Settings**, and generate a new token under **API Tokens**.
2. Run `cargo login` in the terminal and enter your API key when prompted.
3. Check out the repo at the tag created for the release, e.g. `git checkout vX.X.X`.
4. Publish the crate:
   - `cargo publish -p keepsorted`

# Publishing to crates.io

Choose one release path: `cargo-release`, manual Cargo publishing, or the manually dispatched GitHub workflow. Never use two paths to publish the same version.

## Prerequisites

The project uses the [MIT License](../LICENSE), declared as `license = "MIT"` in [Cargo.toml](../Cargo.toml). The package must retain both the root license and the [adapted theme's MIT notice](../src/theme/default/LICENSE-wgpu_pbr.txt).

Use Rust 1.92 or newer. Local work can use an existing newer toolchain; CI checks stable and Rust 1.92.0 on Windows. See [development](development.md) for the validation commands and platform-coverage limits.

Confirm the repository, version, license and intended release branch before proceeding. Normal package verification expects committed files. A dry run validates packaging and compilation, not registry authorization or final acceptance.

## Credentials

1. Sign in to [crates.io](https://crates.io/) and verify your account's email address.
2. Create a publishing token in [API Tokens](https://crates.io/settings/tokens). Limit it to `yummy_egui` and set an expiry. The account must have permission to publish this crate. Creating a new crate requires the corresponding token permission.
3. For local releases, run:

   ```sh
   cargo login --registry crates-io
   ```

   Enter the token at Cargo's prompt. Do not put it in a command, repository file or chat message.

Cargo normally stores the token in the user-level `$CARGO_HOME/credentials.toml`, defaulting to `~/.cargo/credentials.toml` (`%USERPROFILE%\.cargo\credentials.toml` on Windows). Treat it as a secret; never commit it. Repository-local credential files are ignored as a precaution, not as a recommended storage location.

For GitHub Actions, use the environment secret described below rather than committing or transferring a local credentials file. `GITHUB_TOKEN` is not a crates.io publishing token.

## Preflight

Complete the [development validation](development.md), including default/all-feature tests and docs, then inspect and verify the source package:

```sh
cargo package-list
cargo publish-check
cargo publish-check --all-features
```

`publish-check` is an alias in [.cargo/config.toml](https://github.com/gloridifice/yummy-egui/blob/master/.cargo/config.toml) for `cargo publish --dry-run --locked --registry crates-io`. It does not upload or replace unit tests. While preparing uncommitted edits, `cargo publish-check --allow-dirty` can check those edits; do not carry that flag into a real release.

The allowlist in `Cargo.toml` includes source, examples, translated READMEs, current top-level `doco` guides and specs, the lockfile and license notices. Check that embedded TOML files and both license notices are present. Credentials, `target/`, `.agents/`, `doco/tmp/` and change packages must stay out of the archive.

Every published version is immutable: it cannot be overwritten or reused. The hyphenated name `yummy-egui` and underscored name `yummy_egui` cannot be registered as separate crates. A successful dry run does not reserve a name or version.

## Recommended: cargo-release

Install `cargo-release` only if it is not already available:

```sh
cargo install cargo-release --locked
```

[release.toml](https://github.com/gloridifice/yummy-egui/blob/master/release.toml) permits releases from `master`, names tags `v{{version}}` and runs `cargo test --locked --all-features` as a pre-release hook. This test hook does not replace the full preflight. Run `cargo release config` to inspect the effective configuration.

After logging in, completing preflight and committing the workspace, preview a patch release:

```sh
cargo release patch
```

Review the proposed version, registry and push remote. To perform the release:

```sh
cargo release patch --execute
```

Without `--execute`, cargo-release only previews the release. With it, cargo-release increments the patch version, updates the manifest and lockfile, runs the hook, commits the version change, publishes to crates.io, creates the tag and pushes the branch/tag to `origin`. It prompts for confirmation; `--no-confirm` skips that prompt and should only be used for an explicitly authorized unattended release.

If the intended version was already set manually but has not been published, use `cargo release` to preview and `cargo release --execute` to publish that version. Do not add `patch` again: it would increment the version a second time. If the current version is already published, choose the appropriate version increment instead.

## Manual Cargo publishing

For a release without cargo-release:

1. Set an unused version in `Cargo.toml`.
2. Run `cargo check --all-targets --all-features` to refresh the package entry in `Cargo.lock`.
3. Run preflight and commit the prepared files, including the manifest and lockfile.
4. Upload:

   ```sh
   cargo publish --locked --registry crates-io
   ```

This command does not create a Git commit, tag or push. Keep those records in sync manually if using this path. Do not upload a version that cargo-release or the GitHub workflow has already published.

## GitHub Actions publishing

The [publishing workflow](https://github.com/gloridifice/yummy-egui/blob/master/.github/workflows/publish.yml) is triggered only through `workflow_dispatch`; ordinary pushes and pull requests do not upload a crate.

1. Push the workflow to the repository's default branch.
2. In **Settings → Environments**, create `crates-io`, restrict the permitted deployment branches and, where available, configure required reviewers.
3. Add an environment secret named `CARGO_REGISTRY_TOKEN` containing the crates.io publishing token.
4. Set an unused version in `Cargo.toml`, refresh `Cargo.lock`, commit and push the prepared release.
5. Open **Actions → Publish to crates.io → Run workflow**. Select the prepared branch or tag and enter the exact manifest version.

The workflow first runs reusable CI. The Ubuntu publishing job then checks that the requested version matches `Cargo.toml`, that a project license is declared and that the token is present. Only then does it run `cargo publish --locked --registry crates-io`. A failed CI job or failed check stops that path before upload.

This workflow does not increment versions or create release commits/tags. Its deployment branch policy is controlled by the GitHub environment; the `master` restriction in `release.toml` applies to cargo-release, not to this separate workflow.

## Partial failures and verification

If release automation fails, inspect the manifest, Git log, tags, crates.io version and remote refs before retrying. A release can fail after a version commit or after a successful upload.

Do not blindly rerun `cargo release patch --execute`: it could target the next patch rather than finish the intended release. If upload succeeded but push failed, repair the Git push instead of uploading again. If upload did not succeed, use cargo-release's step commands as appropriate for the observed state; do not delete published versions or rewrite shared tags to conceal a partial release.

Confirm that crates.io lists the intended version, that the release tag resolves to the intended commit, that the branch and tag reached `origin` when using cargo-release, and that `git status --short` is clean.

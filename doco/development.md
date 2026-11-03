# Development

Run commands from the repository root with Rust 1.92 or newer. Use an existing newer toolchain locally; there is no need to install the minimum version just to build the crate. CI checks that version separately.

## Build and validate

```sh
cargo fmt --all -- --check
cargo check --locked --all-targets --all-features
cargo test --locked
cargo test --locked --all-features
cargo test --locked --release --all-features
cargo doc --locked --no-deps --all-features
```

The two feature configurations matter: `dock` changes the public theme API and whether `egui_dock.toml` is read. Release-mode tests also exercise embedded themes against upstream serde types whose fields can differ between debug and release builds.

For documentation warnings to fail a local build, set `RUSTDOCFLAGS=-D warnings` using your shell's environment syntax before `cargo doc`. Generated API documentation is under `target/doc/yummy_egui/` and is not checked in.

## Native examples

```sh
cargo run --locked --example demo
cargo run --locked --example style_editor
```

The `demo` example has a fixed left sidebar with a vertical icon segmented selector: the grid icon opens the scrollable component gallery and the information icon opens the example information page. These SVGs are embedded from `examples/icons/`; the example installs `egui_extras` image loaders with the SVG feature enabled. The gallery state survives page switches. The `style_editor` example retains its separate preview and editor windows. Both examples construct native panels through `yummy_egui::panels` so their content margins match the theme's window margins (10 pt by default); the sidebar has no extra top spacer.

These examples use eframe's Glow backend and bundled default fonts. They need a graphical environment and the native dependencies required by eframe for the host platform. The launchers implement eframe 0.34's `App::ui` entry point and apply the theme once during setup.

Use the gallery to inspect narrow and wide forms, wrapped labels, slider editors, disabled controls and exclusive selection. The icon section shows default and overridden button sizes, native icon choices and connected icon segments; the editor's Icon buttons group changes shared button/icon sizing live. The style-editor example shows the gallery beside the live editor; check that edits affect the preview without replacing host fonts. These are manual visual checks, not assertions made by the headless tests.

## CI coverage

[CI](https://github.com/gloridifice/yummy-egui/blob/master/.github/workflows/ci.yml) runs on push, pull request and reusable-workflow calls. Its Windows matrix contains stable Rust and Rust 1.92.0. It checks formatting on stable, then checks all targets, tests default and all features, builds docs with warnings denied, and verifies packages through both dry-run publishing configurations.

CI does not currently run release-mode tests or launch graphical examples. The Ubuntu publish job is a release step, not a Linux GUI test suite. Do not interpret the workflow as full Linux, macOS or WebAssembly coverage.

## Package checks

```sh
cargo package-list
cargo publish-check
cargo publish-check --all-features
```

These aliases come from [.cargo/config.toml](https://github.com/gloridifice/yummy-egui/blob/master/.cargo/config.toml). `publish-check` runs `cargo publish --dry-run --locked --registry crates-io`, so it never uploads. Cargo normally requires the files being packaged to be committed; during development, append `--allow-dirty` to inspect pending changes. Do not use that flag for an actual release.

Check that the package contains source, embedded themes, examples, README translations, current `doco` guides/specs and license notices. It should not contain credentials, build output, agent tooling, temporary docs or tracked-change packages. Actual upload, versioning and recovery procedures are in [publishing](publishing.md).

## Maintaining current documentation

Use [architecture.md](architecture.md) for implemented boundaries and data flows, and [specs/](specs/) for public behavior and compatibility contracts. Update those documents when the implementation intentionally changes their facts. Investigate disagreements rather than changing a contract to conceal a bug.

Keep build and publishing instructions in their respective guides instead of duplicating them across every README. The root README and its translations remain short entry points. Doco documents use English. Routine documentation maintenance does not need a tracked change; Git records implementation history.

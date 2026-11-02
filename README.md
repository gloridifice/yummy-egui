<div align="center">

# yummy_egui

English | [简体中文](readme/zh-CN.md) | [日本語](readme/ja-JP.md) | [Français](readme/fr-FR.md)

</div>

Backend-independent widgets, parameter forms, selection lists, and TOML themes for egui 0.33.3.

The application owns its rendering backend, fonts, windows, selection state, and persistence. `eframe` is used only by the examples.

## Quick Start

Apply the theme once during application setup, then draw widgets in your existing UI:

```rust
use yummy_egui::{ParameterList, ParameterSlider, Theme};

fn setup(ctx: &egui::Context) {
    Theme::default().apply(ctx);
}

fn settings_ui(ui: &mut egui::Ui, gain: &mut f32) {
    ParameterList::new("settings").show(ui, |rows| {
        rows.row("gain", "Gain", "Output level", |ui| {
            ui.add(ParameterSlider::new(gain, 0.0..=1.0).max_decimals(2));
        });
    });
}
```

The host must supply fonts; `Theme::apply` does not install or replace them. Do not reapply the theme every frame if you want live style edits to remain visible.

To try the standalone examples, run these commands from the repository root:

```sh
cargo run --example demo
cargo run --example style_editor
```

## Features

- Parameter sliders with integer and floating-point values, logarithmic scales, steps, suffixes, and animated hover styling.
- Two-column parameter forms with wrapped labels, tooltips, grouped headings, and explicit row IDs.
- Scrollable selection lists with disabled rows and application-owned selection state.
- Panel buttons, action rows, exclusive choices, segmented controls, and H1–H3 headings.
- Embedded TOML themes, explicit external theme loading, and context-local styles.
- A live style editor for custom widgets and common egui settings; edits stay in memory and exclude dock styles.
- Optional `egui_dock` 0.18 integration through the `dock` feature.

## Themes

Start with the files in [`src/theme/default/`](src/theme/default/):

| File | Controls |
| --- | --- |
| `egui.toml` | Base egui style |
| `charming_egui.toml` | Custom widgets, lists, and headings |
| `egui_dock.toml` | Dock style, only with the `dock` feature |

Load an existing directory and explicitly apply the result:

```rust
fn load_theme(ctx: &egui::Context) -> Result<(), yummy_egui::ThemeError> {
    let theme = yummy_egui::Theme::load_dir("themes/my-theme")?;
    theme.apply(ctx);
    Ok(())
}
```

Missing files fall back to their embedded defaults. Each supplied file replaces a whole document, not individual fields; copy a bundled document before editing it. Unreadable or invalid files return an error. Loading does not watch files or write changes back. Use `Theme::from_sources(ThemeSources { .. })` to load TOML strings instead.

For dock styling, enable `features = ["dock"]` on the dependency and pass `theme.dock_style().clone()` to `DockArea::style(...)`. `Theme::apply` does not style dock areas automatically.

## Build from Source

With Rust 1.88+ installed, run from the repository root:

```sh
cargo build --release
cargo check --examples --all-features
cargo test --all-features
cargo doc --no-deps --all-features --open
```

The examples require a graphical environment and the native dependencies required by `eframe`'s Glow backend.

## License

Licensed under the [MIT License](LICENSE). The adapted egui theme retains its [MIT notice from wgpu_pbr](src/theme/default/LICENSE-wgpu_pbr.txt).

## Development

- [Publishing to crates.io (中文)](PUBLISHING.md)
- [Public API](src/lib.rs)
- [Component gallery](src/demo.rs) and [standalone example](examples/demo.rs)
- [Style editor example](examples/style_editor.rs)
- [Theme loading](src/theme/mod.rs) and [custom style definitions](src/theme/style.rs)

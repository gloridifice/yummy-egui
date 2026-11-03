# Current architecture

`yummy_egui` is a Rust library for egui 0.34, with reusable widgets, parameter forms, selection lists and TOML themes. It does not provide a renderer or application runtime.

## Runtime boundary

The host application owns the egui context, rendering backend, event loop, fonts, window lifecycle, parameter values, selection and persistence. The library draws into a host-provided `egui::Ui`; its demo and style editor also offer optional egui windows.

The crate uses Rust edition 2024 and requires Rust 1.92 or newer. [Cargo.toml](../Cargo.toml) declares egui 0.34.3, serde and toml as regular dependencies. The `dock` feature adds egui_dock 0.19.1. Default features are empty; eframe, egui's bundled fonts and egui_extras's SVG loader are development dependencies for examples and tests, not requirements imposed on the host.

## Module map

[The crate root](../src/lib.rs) exposes five public modules and re-exports the primary components.

| Module | Responsibility | Internal dependencies |
| --- | --- | --- |
| [`widgets`](../src/widgets/mod.rs) | Sliders, panel/icon buttons, action rows, text/icon exclusive selectors and headings | Reads context-local custom styles from `theme` |
| [`panels`](../src/panels.rs) | Native central/edge panel factories with window-matched content margins | Reads the current egui UI style |
| [`lists`](../src/lists/mod.rs) | Two-column parameter forms and scrollable selection rows | Uses `theme` for layout settings and widgets for group headings |
| [`theme`](../src/theme/mod.rs) | Embedded defaults, explicit TOML loading, validation, context installation and the live editor | Custom styles use `SliderAppearance`; the editor composes `widgets` and `lists` |
| [`demo`](../src/demo.rs) | Component gallery with caller-owned `DemoState` | Composes the public widgets and lists |
| [`examples`](../examples/) | Native eframe launchers for the gallery and editor | Uses the library as a host application would |

These are cooperating modules within one crate, not isolated architectural layers. In particular, theme data and widgets reference each other, while the editor depends on the components it edits. No runtime module depends on eframe.

## Theme and style flow

```text
Embedded TOML or explicit host-supplied files/strings
  -> Theme::from_sources / Theme::load_dir
  -> Deserialize + finite-number checks + custom-style validation
  -> Theme value (no context mutation)
  -> Theme::apply(host context)
       -> egui theme preference and global style on that context
       -> custom YummyStyle in that context's temporary data
  -> widgets and lists read current styles while drawing
```

The three documents are independent: `egui.toml`, `yummy_egui.toml` and, with `dock`, `egui_dock.toml`. Missing documents use embedded defaults; supplied documents replace whole documents. There is no file watcher or automatic write-back. [The theming contract](specs/theming.md) defines validation, errors and migration requirements.

Default themes are parsed lazily and cached, then cloned for callers. Custom component styles are shared through `Arc<YummyStyle>` in context-local egui data. Without an installed custom style, components use the embedded custom defaults. `Theme::apply` neither installs fonts nor applies a dock style; a host passes the dock style directly to its `DockArea`.

The live editor reads styles from the context, validates custom-style edits and installs accepted changes back into that context. It edits common egui settings and custom components, not dock styles or theme files. Reapplying a previously loaded `Theme` overwrites those live edits.

## Layout and input flow

`ParameterSlider` delegates value editing to egui's native slider and paints its skin over the track and handle, leaving the numeric editor visible. The display mapping does not replace egui's input handling.

`ParameterList` uses a shared two-column grid per group. Child UIs constrain column widths and clip oversized controls. Context-local layout data preserves measured cell heights across passes; changed heights request another egui pass for alignment. Selection rows use native selectable buttons with caller-supplied stable IDs inside a namespaced scroll area.

The `panels` factories return native egui builders with a frame whose inner margin comes from the current UI's `spacing.window_margin`. The bundled value is 10 pt, matching native windows and the dock theme's tab-body content margin. Directly constructed egui panels retain upstream defaults; the factories do not patch egui or synchronize independent dock documents.

Icon buttons use native empty button frames and input with separately centered glyph/image content, so their fixed square geometry is independent of font metrics, image dimensions and parent justification. Icon selectors share sizing with standalone icons; the connected variant shares track and edge treatment with the text segmented control. Host applications supply icon fonts and image loaders.

Native responses remain available for click handling, tooltips and context menus. Parameter values and selected items stay in host state. Layout caches, animations, focus and scroll state belong to egui, not to a library persistence layer. See [widgets and lists](specs/widgets-and-lists.md) for caller obligations and observable behavior.

## Development and distribution

Tests are colocated with the theme, panels, lists, slider, segmented control and icon widgets. They exercise theme loading and validation, layout and synthetic egui input without a native window. The two native examples provide manual visual checks.

CI runs on Windows with stable Rust and the declared minimum version; the manually dispatched publishing job runs on Ubuntu after CI passes. This is not a complete cross-platform validation matrix. The source package includes the current `doco` guides and specs, but not local temporary documentation or change packages.

- [Development and validation](development.md)
- [Publishing and release automation](publishing.md)

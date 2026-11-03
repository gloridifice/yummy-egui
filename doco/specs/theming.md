# Theming contract

This document describes the current loading and application behavior of `Theme`, `ThemeSources` and the custom component styles. The implementation is in [theme/mod.rs](../../src/theme/mod.rs) and [theme/style.rs](../../src/theme/style.rs).

## Documents and replacement

| Document | Deserialized value | Availability |
| --- | --- | --- |
| `egui.toml` | `egui::Style` | Always |
| `yummy_egui.toml` | `YummyStyle` | Always |
| `egui_dock.toml` | `egui_dock::Style` | Only with `dock` |

`Theme::default()` uses the [embedded documents](../../src/theme/default/). `Theme::from_sources` accepts independent optional TOML strings: each `None` selects the corresponding embedded document.

`Theme::load_dir` requires an existing directory. Within that directory, a missing document falls back to its embedded default. An unreadable or malformed document returns an error rather than silently selecting a default. With `dock` disabled, `egui_dock.toml` is not opened or parsed.

A supplied document replaces a whole document, with no field-by-field merge against the bundled theme. Defaults supplied by the target serde type still apply: for example, a minimal `egui.toml` can use egui's defaults, while the custom document requires its component sections. Omitted custom `headings` and `icon_button` sections are compatibility exceptions and use their bundled defaults. Omitted optional radii and selection-row spacing inherit the relevant egui style when drawn.

Theme loading does not watch files, create directories, save edits, mutate a context or install fonts. The host controls when to reload and whether to apply a successfully loaded value.

## Numeric and schema validation

All parsed documents reject floating-point values that are non-finite or cannot remain finite when converted to `f32`. Custom component structures reject unknown fields and additionally enforce:

| Constraint | Fields |
| --- | --- |
| Strictly positive, finite | Heading sizes; slider marker width, hover width and value-editor width; minimum action-button width; icon-button size and icon size |
| Nonnegative, finite | Heading margins; slider inset, outset, animation time and edge padding; parameter-list spacing; choices spacing; optional radii and selection-row spacing |
| Strictly between 0 and 1 | Parameter-list `name_ratio` |
| Hover width at least marker width | Slider `hover_width` relative to `width` |
| Fully opaque | Slider `background` alpha |
| Nonnegative | Panel-button margins |

The bundled `egui.toml` sets `spacing.window_margin` to 10 pt on all sides. Native `Window` frames and the `panels` factories use this value; it matches the bundled `egui_dock.toml`'s `tab.tab_body.inner_margin` of 10 pt. Direct native panels have separate upstream defaults. Dock styles remain independent: changing window/panel margins does not rewrite a loaded dock style, and external theme documents retain their supplied values. DockArea outer padding is a separate setting and is not part of this content-margin unification.

`icon_button.size` (default 28) and `icon_button.icon_size` (default 16) are shared by standalone icon buttons and both icon selectors. Widget `.size(...)` and `.icon_size(...)` overrides take precedence; rendering clamps icon size to button size. Existing external themes may omit the whole section, but a supplied section must contain both fields.

Distances are logical points. `slider.animation_ms` uses milliseconds; egui's own `animation_time` uses seconds. Colors use the serde representation of egui's color types, as shown in the bundled documents.

`YummyStyle::validate()` exposes the same custom-style checks for programmatically constructed styles. Callers supplying widget-specific overrides should validate those values before use; constructing an override does not automatically run the full theme loader.

Validation of egui and dock styles is not an exhaustive geometry validator. Beyond the finite-number check, their schema and defaults are governed by the corresponding upstream versions.

## Errors and application

- `ThemeError::Io` identifies the failing path and underlying I/O error, including a nonexistent or non-directory root.
- `ThemeError::Parse` identifies the document and TOML/schema error. Schema parsing uses the original text so diagnostics retain source positions.
- `ThemeError::Invalid` identifies the document and validation message.

A failed load returns no `Theme` and leaves an already applied context unchanged. A successful load also leaves the context unchanged until the host explicitly calls `Theme::apply`.

`Theme::apply` selects light/dark mode from `egui_style().visuals.dark_mode`, installs the egui global style and installs custom component styles on that context only. It does not replace the context's font definitions. Apply once during setup or after an intentional reload; applying every frame overwrites edits made by the live editor.

With `dock` enabled, `Theme::dock_style()` returns the loaded dock style. Pass its clone to `DockArea::style(...)`; `Theme::apply` does not style dock areas automatically.

## Editor and compatibility

`style_editor_window` edits custom components and common egui settings in memory. Invalid custom edits are reported instead of installed. Reset applies `Theme::default()`. It has no theme export, persistence or dock-style editor.

An external egui 0.33 document with a `[debug]` section needs these fields for the current egui 0.34 schema:

```toml
# Add inside the existing [debug] section:
warn_if_rect_changes_id = false
show_focused_widget = false
```

Serialized egui and dock styles are tied to their upstream schemas. Recheck external themes when changing those dependencies.

## Verification scenarios

[Theme tests](../../src/theme/tests.rs) cover these boundaries:

- An existing empty directory loads embedded defaults; supplying one document changes only that document.
- A custom document without headings or icon buttons keeps its supplied values and receives the corresponding bundled defaults.
- Invalid TOML, unsafe custom sizes and non-finite numbers return errors.
- Applying a loaded theme affects one context and preserves host font definitions.
- A malformed dock file is ignored without `dock` and rejected with `dock`.

Run both feature configurations as described in [development](../development.md).

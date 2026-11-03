# Widgets and lists contract

The host provides an `egui::Ui`, owns parameter and selection values, and handles responses. Components use context-local styles or embedded custom defaults; they do not create a backend or persist application data.

## Native panel factories

[`panels::central`, `left`, `right`, `top` and `bottom`](../../src/panels.rs) return native egui panel builders with `Frame::inner_margin` set to the current UI's `spacing.window_margin`. Apply `Theme` to use the bundled 10 pt on every side, matching native `Window` content and the bundled dock tab-body margin. Panel fills and all native builder options, IDs, resizing and responses remain available.

Construct the builder each frame to follow live or UI-local style edits. A later `.frame(...)` replaces the factory's frame, including its margins. Direct `egui::Panel` and `egui::CentralPanel` construction is unchanged and does not inherit `window_margin`; use these factories to avoid missing the override. Show edge panels before the central panel, as required by egui.

```rust
use yummy_egui::panels;

panels::left(ui, "navigation")
    .resizable(false)
    .exact_size(56.0)
    .show_inside(ui, |ui| { /* navigation */ });
panels::central(ui).show_inside(ui, |ui| { /* content */ });
```

The 10 pt applies to content insets, not DockArea outer padding, tab-bar margins, menu padding or widget padding. Extra `add_space` or nested frames add to these insets.

## Parameter slider

[`ParameterSlider`](../../src/widgets/slider.rs) accepts egui numeric types and a finite inclusive range. Logarithmic ranges must be nonnegative. Its builder forwards logarithmic mode, smallest-positive value, clamping, decimal precision, steps and suffixes to the native egui slider.

The default clamping policy is `SliderClamping::Always`. Use `.clamping(SliderClamping::Edits)` when an existing out-of-range value must remain unchanged merely by displaying it. Integer and logarithmic editing follow native egui behavior, including logarithmic ranges containing zero.

A `SliderAppearance` changes the skin, not numeric editing semantics. The skin covers the track and handle, not the numeric editor. Marker hover animation must not change the usable input range. A `.height(...)` override must be finite and positive; it sets a minimum interaction height, while text metrics and button padding can require more space. Skin-only changes must not alter numeric-editor height.

## Parameter forms

[`ParameterList`](../../src/lists/parameter.rs) lays out a shared two-column grid: labels on the left and caller-provided controls on the right. `ParameterRows::row` returns the control closure's result. Labels wrap within their column and expose descriptions as hover text; after layout settles, labels and controls share row centers.

The name/control width ratio comes from `ParameterListStyle::name_ratio`, after subtracting column spacing. Oversized controls must not push later rows or groups into inconsistent column positions. Child content is clipped to its column.

Callers must use distinct list IDs and stable, unique row IDs within each list. Display names do not need to be unique when explicit row IDs are supplied. The convenience API `parameter_table` / `parameter` uses names as row IDs, so those names must be unique within each group. `parameter` must run inside a parameter table or group; it requires the enclosing column layout.

`group` adds an H3 heading before a parameter table. Its group spacing is a minimum top gap, not an additional margin stacked on top of the heading's top margin.

## Selection rows

[`SelectionList`](../../src/lists/selection.rs) provides scrollable full-width rows, with optional framing, wrapping and a maximum scroll height. The host passes each row's selected state and decides what a click means; the list does not maintain an exclusive selected index.

Row IDs must be unique within a list and stable across reordering. Duplicate visible labels must not alias interaction state. The returned native `Response` supports context menus and tooltips.

Clicking an already selected enabled row still emits a click. Disabled rows remain visible but cannot activate. Opening a context menu must not activate a row as a primary click would.

## Exclusive selectors

[`choices`](../../src/widgets/choices.rs) draws separate equal-width buttons with theme spacing. Clicking a button assigns its choice to the caller's value.

[`segmented_control`](../../src/widgets/segmented.rs) draws a connected, equal-width row with rounded outer ends and truncated labels. It returns a combined response marked changed only when the selected value actually changes. Repeated clicks on the current selection do not report a value change. Disabled controls cannot change selection.

Choice values should be unique. An unmatched value leaves all segments unselected; an empty segment slice draws no buttons and does not change the value.

## Icon buttons and selectors

[`IconButton`, `IconChoices` and `IconSegmentedControl`](../../src/widgets/icon.rs) accept icon-font glyphs (`&str` or `egui::RichText`) and `egui::Image` values. Hosts install any required icon fonts and image loaders; the library does not bundle an icon set.

All three use `CharmingStyle::icon_button`: `size` is the square button/segment side length (28 pt by default), and `icon_size` is the glyph font size or image bounding-box side length (16 pt by default). Per-widget `.size(...)` and `.icon_size(...)` override the corresponding theme field and must be finite and positive. Icon size is clamped to the button size. Glyph font family and color are retained, but sizing uses `icon_size`, not a size embedded in `RichText`. Images retain egui's image options, including aspect-ratio handling. Contents are centered and clipped to the cell; font metrics, image dimensions, global button padding and justified parents do not enlarge the fixed frame.

- `IconButton` is a standalone native action button. `.enabled(false)` combines with parent disabling; `.label(...)` adds an accessible name and hover tooltip, not visible text. Repeated activation still returns a click.
- `IconChoices` is a compact row of native selectable buttons separated by `choices.spacing`.
- `IconSegmentedControl` is a compact, gapless row using the same background and rounded outer ends as the text segmented control. Unlike the text version, it does not fill the available width.

Both selectors default to horizontal. `.vertical()` stacks fixed square cells top to bottom; `IconChoices` still uses `choices.spacing` between cells, while `IconSegmentedControl` remains gapless and rounds only the top and bottom outer ends. The single icon button is direction-independent and can be placed in either parent layout.

Both selectors take an iterator of `(value, icon)` pairs with unique values and return a combined response. The caller owns the selected value. Only selecting a different value reports `changed()`; disabled controls cannot change selection. An unmatched value selects nothing, an empty iterator draws nothing, and one item remains a valid selector. Rows do not wrap or shrink their cells to fit narrow parents.

```rust
ui.add(IconButton::new("+").size(32.0).icon_size(20.0).label("Add"));
ui.add(IconChoices::new(&mut direction, [(0, "←"), (1, "→")]));
ui.add(IconSegmentedControl::new(&mut direction, [(0, "←"), (1, "→")]));
ui.add(IconChoices::new(&mut direction, [(0, "↑"), (1, "↓")]).vertical());
ui.add(IconSegmentedControl::new(&mut direction, [(0, "↑"), (1, "↓")]).vertical());
// Texture handles are usable without an image-loader dependency:
ui.add(IconButton::new(egui::Image::new(&texture)).label("Preview"));
```

## Panel actions and headings

[`PanelButton`](../../src/widgets/button.rs) fills the available width, wraps centered text and applies its own theme margins. Its enabled flag combines with the parent UI's enabled state. It retains native egui button interaction without changing unrelated buttons.

`action_row` accepts a nonempty fixed-size button array and returns the corresponding native responses. Buttons share a horizontal row when their available width, excluding margins, meets the configured minimum action width. Otherwise they stack vertically. The bundled minimum is 96 logical points.

[`h1`, `h2` and `h3`](../../src/widgets/heading.rs) use strong text color with custom sizes and vertical margins. H1/H2 add synthetic bold weight; H3 uses regular weight. They do not install font families. Their margins remain vertical even inside horizontal layouts, and their returned responses belong to the labels rather than the surrounding margins.

## Verification boundaries

The existing tests exercise input and layout without opening a native window:

- [Panel tests](../../src/panels/tests.rs): actual content insets for all five factories, both bundled defaults and live theme changes.
- [Slider tests](../../src/widgets/slider/tests.rs): native linear, logarithmic and integer edits; editor height; out-of-range display with edit-only clamping.
- [Parameter-form tests](../../src/lists/parameter/tests.rs): shared column ratios, oversized controls, distinct group IDs and row-center alignment after resize.
- [Selection tests](../../src/lists/selection/tests.rs): row identity through reordering, repeated activation, disabled rows and context menus.
- [Segmented-control tests](../../src/widgets/segmented/tests.rs): exclusive selection, repeated-click change reporting and disabled behavior.
- [Icon tests](../../src/widgets/icon/tests.rs): theme/override sizing in justified layouts, image geometry, action keyboard/pointer activation, disabled controls and both orientations' selector geometry and exclusive/empty/single-item behavior.

Panel buttons, choices and headings are also demonstrated by the [component gallery](../../src/demo.rs); they do not currently have separate focused test modules. See [development](../development.md) for automated and visual checks.

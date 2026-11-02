# Widgets and lists contract

The host provides an `egui::Ui`, owns parameter and selection values, and handles responses. Components use context-local styles or embedded custom defaults; they do not create a backend or persist application data.

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

## Panel actions and headings

[`PanelButton`](../../src/widgets/button.rs) fills the available width, wraps centered text and applies its own theme margins. Its enabled flag combines with the parent UI's enabled state. It retains native egui button interaction without changing unrelated buttons.

`action_row` accepts a nonempty fixed-size button array and returns the corresponding native responses. Buttons share a horizontal row when their available width, excluding margins, meets the configured minimum action width. Otherwise they stack vertically. The bundled minimum is 96 logical points.

[`h1`, `h2` and `h3`](../../src/widgets/heading.rs) use strong text color with custom sizes and vertical margins. H1/H2 add synthetic bold weight; H3 uses regular weight. They do not install font families. Their margins remain vertical even inside horizontal layouts, and their returned responses belong to the labels rather than the surrounding margins.

## Verification boundaries

The existing tests exercise input and layout without opening a native window:

- [Slider tests](../../src/widgets/slider/tests.rs): native linear, logarithmic and integer edits; editor height; out-of-range display with edit-only clamping.
- [Parameter-form tests](../../src/lists/parameter/tests.rs): shared column ratios, oversized controls, distinct group IDs and row-center alignment after resize.
- [Selection tests](../../src/lists/selection/tests.rs): row identity through reordering, repeated activation, disabled rows and context menus.
- [Segmented-control tests](../../src/widgets/segmented/tests.rs): exclusive selection, repeated-click change reporting and disabled behavior.

Panel buttons, choices and headings are also demonstrated by the [component gallery](../../src/demo.rs); they do not currently have separate focused test modules. See [development](../development.md) for automated and visual checks.

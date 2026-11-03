//! Backend-independent egui widgets and lists with embedded or external TOML themes.
//! Fonts, windows, selection state and persistence remain owned by the application.

pub mod demo;
pub mod lists;
pub mod panels;
pub mod theme;
pub mod widgets;

pub use demo::{DemoState, demo_ui, demo_window};
pub use lists::{
    ParameterList, ParameterRows, SelectionList, SelectionRows, group, parameter, parameter_table,
};
pub use theme::{CharmingStyle, Theme, ThemeError, ThemeSources, style_editor_window};
pub use widgets::{
    IconButton, IconChoices, IconSegmentedControl, PanelButton, ParameterSlider, SliderAppearance,
    action_row, choices, h1, h2, h3, segmented_control,
};

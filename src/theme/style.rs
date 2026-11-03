use crate::SliderAppearance;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};

/// Custom component styles deserialized from `yummy_egui.toml`.
///
/// Distances are logical points, not physical pixels.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct YummyStyle {
    /// Skin and dimensions for [`crate::ParameterSlider`] and connected segment tracks.
    pub slider: SliderAppearance,
    /// Margins and minimum horizontal row width for [`crate::PanelButton`] and [`crate::action_row`].
    pub panel_button: PanelButtonStyle,
    /// Shared sizing for [`crate::IconButton`], [`crate::IconChoices`], and [`crate::IconSegmentedControl`].
    #[serde(default)]
    pub icon_button: IconButtonStyle,
    /// Two-column layout settings for [`crate::ParameterList`], [`crate::parameter_table`], and [`crate::group`].
    pub parameter_list: ParameterListStyle,
    /// Framing, wrapping, and spacing for [`crate::SelectionList`].
    pub selection_list: SelectionListStyle,
    /// Horizontal button spacing for [`crate::choices`] and [`crate::IconChoices`].
    pub choices: ChoicesStyle,
    /// Typography sizes and vertical margins for [`crate::h1`], [`crate::h2`], and [`crate::h3`].
    ///
    /// Older external themes without `headings` keep the bundled heading defaults.
    #[serde(default)]
    pub headings: HeadingStyles,
}

/// Sizes and vertical margins for the three heading levels.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeadingStyles {
    /// Style for [`crate::h1`] (bold weight).
    pub h1: HeadingStyle,
    /// Style for [`crate::h2`] (bold weight).
    pub h2: HeadingStyle,
    /// Style for [`crate::h3`] and [`crate::group`] titles (regular weight).
    pub h3: HeadingStyle,
}

/// Font size and vertical margins for a single heading level, in logical points.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeadingStyle {
    /// Font size in logical points; must be finite and strictly positive.
    pub size: f32,
    /// Space added above the heading in logical points; must be finite and non-negative.
    pub margin_top: f32,
    /// Space added below the heading in logical points; must be finite and non-negative.
    pub margin_bottom: f32,
}

impl Default for HeadingStyles {
    fn default() -> Self {
        // Read only this section; do not recursively default a YummyStyle.
        #[derive(Deserialize)]
        struct Document {
            headings: HeadingStyles,
        }
        toml::from_str::<Document>(super::DEFAULT_YUMMY_EGUI)
            .expect("embedded headings must match their schema")
            .headings
    }
}

/// Outer margins and responsive stacking threshold for [`crate::PanelButton`] and [`crate::action_row`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PanelButtonStyle {
    /// Outer margin around each panel button; all sides must be non-negative.
    pub margin: egui::Margin,
    /// Minimum button width (excluding `margin`) before [`crate::action_row`] stacks vertically;
    /// must be finite and strictly positive.
    pub min_action_width: f32,
}

/// Fixed square button and glyph/image dimensions, in logical points.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IconButtonStyle {
    /// Square button or segment side length in logical points; must be finite and strictly positive.
    pub size: f32,
    /// Glyph font size or image bounding-box side length in logical points (clamped to `size`
    /// when drawn); must be finite and strictly positive.
    pub icon_size: f32,
}

impl Default for IconButtonStyle {
    fn default() -> Self {
        // Read only this section to support external themes predating icons.
        #[derive(Deserialize)]
        struct Document {
            icon_button: IconButtonStyle,
        }
        toml::from_str::<Document>(super::DEFAULT_YUMMY_EGUI)
            .expect("embedded icon button style must match its schema")
            .icon_button
    }
}

/// Two-column grid geometry and spacing for [`crate::ParameterList`] and [`crate::group`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterListStyle {
    /// Fraction of usable width allocated to the left name column; must be strictly between `0.0` and `1.0`.
    pub name_ratio: f32,
    /// Horizontal gap between the name and control columns in logical points; must be finite and non-negative.
    pub column_spacing: f32,
    /// Vertical gap between parameter rows in logical points; must be finite and non-negative.
    pub row_spacing: f32,
    /// Minimum top gap before a [`crate::group`] in logical points; the H3 top margin counts toward it.
    /// Must be finite and non-negative.
    pub group_spacing: f32,
    /// Whether alternating rows in the parameter grid draw striped backgrounds.
    pub striped: bool,
}

/// Visual and layout options for [`crate::SelectionList`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionListStyle {
    /// Whether to wrap the scroll area in an `egui::Frame::group`.
    pub frame: bool,
    /// Whether long row labels wrap (`true`) or truncate (`false`).
    pub wrap: bool,
    /// Optional vertical spacing between rows in logical points (`None` inherits egui's `item_spacing.y`);
    /// when `Some`, must be finite and non-negative.
    pub row_spacing: Option<f32>,
}

/// Spacing for [`crate::choices`] and [`crate::IconChoices`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChoicesStyle {
    /// Gap between adjacent choice buttons in logical points; must be finite and non-negative.
    pub spacing: f32,
}

fn defaults() -> &'static Arc<YummyStyle> {
    static STYLE: OnceLock<Arc<YummyStyle>> = OnceLock::new();
    STYLE.get_or_init(|| {
        let style: YummyStyle = toml::from_str(super::DEFAULT_YUMMY_EGUI)
            .expect("embedded yummy_egui theme must match its schema");
        style
            .validate()
            .expect("embedded component sizes must be valid");
        Arc::new(style)
    })
}

impl Default for YummyStyle {
    fn default() -> Self {
        defaults().as_ref().clone()
    }
}

impl YummyStyle {
    /// Read this Context's theme, or the embedded component defaults if unapplied.
    pub fn get(context: &egui::Context) -> Arc<Self> {
        context
            .data(|data| data.get_temp::<Arc<Self>>(egui::Id::new("yummy_egui style")))
            .unwrap_or_else(|| Arc::clone(defaults()))
    }

    pub(super) fn install(&self, context: &egui::Context) {
        context.data_mut(|data| {
            data.insert_temp(egui::Id::new("yummy_egui style"), Arc::new(self.clone()));
        });
    }

    /// Validate sizes before using a programmatically constructed style.
    pub fn validate(&self) -> Result<(), String> {
        for (name, style) in [
            ("h1", &self.headings.h1),
            ("h2", &self.headings.h2),
            ("h3", &self.headings.h3),
        ] {
            if !style.size.is_finite() || style.size <= 0.0 {
                return Err(format!("headings.{name}.size must be finite and positive"));
            }
            nonnegative(&format!("headings.{name}.margin_top"), style.margin_top)?;
            nonnegative(
                &format!("headings.{name}.margin_bottom"),
                style.margin_bottom,
            )?;
        }
        let slider = &self.slider;
        for (name, value) in [
            ("slider.inset", slider.inset),
            ("slider.outset", slider.outset),
            ("slider.animation_ms", slider.animation_ms),
            ("slider.edge_padding", slider.edge_padding),
            (
                "parameter_list.column_spacing",
                self.parameter_list.column_spacing,
            ),
            (
                "parameter_list.row_spacing",
                self.parameter_list.row_spacing,
            ),
            (
                "parameter_list.group_spacing",
                self.parameter_list.group_spacing,
            ),
            ("choices.spacing", self.choices.spacing),
        ] {
            nonnegative(name, value)?;
        }
        for (name, value) in [
            ("slider.background_radius", slider.background_radius),
            ("slider.marker_radius", slider.marker_radius),
            (
                "selection_list.row_spacing",
                self.selection_list.row_spacing,
            ),
        ] {
            if let Some(value) = value {
                nonnegative(name, value)?;
            }
        }
        for (name, value) in [
            ("slider.width", slider.width),
            ("slider.hover_width", slider.hover_width),
            ("slider.value_width", slider.value_width),
            ("icon_button.size", self.icon_button.size),
            ("icon_button.icon_size", self.icon_button.icon_size),
            (
                "panel_button.min_action_width",
                self.panel_button.min_action_width,
            ),
        ] {
            if !value.is_finite() || value <= 0.0 {
                return Err(format!("{name} must be finite and positive"));
            }
        }
        if slider.hover_width < slider.width {
            return Err("slider.hover_width must be at least slider.width".into());
        }
        // The skin overpaints the native slider; transparent paint would reveal it.
        if slider.background.a() != 255 {
            return Err("slider.background must be opaque".into());
        }
        let ratio = self.parameter_list.name_ratio;
        if !ratio.is_finite() || ratio <= 0.0 || ratio >= 1.0 {
            return Err("parameter_list.name_ratio must be strictly between 0 and 1".into());
        }
        let margin = self.panel_button.margin;
        if [margin.left, margin.right, margin.top, margin.bottom]
            .iter()
            .any(|v| *v < 0)
        {
            return Err("panel_button.margin must be non-negative".into());
        }
        Ok(())
    }
}

fn nonnegative(name: &str, value: f32) -> Result<(), String> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(format!("{name} must be finite and non-negative"))
    }
}

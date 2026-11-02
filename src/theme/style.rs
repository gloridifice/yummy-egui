use crate::SliderAppearance;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};

/// Custom component styles. Distances are logical points, not physical pixels.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharmingStyle {
    pub slider: SliderAppearance,
    pub panel_button: PanelButtonStyle,
    pub parameter_list: ParameterListStyle,
    pub selection_list: SelectionListStyle,
    pub choices: ChoicesStyle,
    /// Older external themes without headings keep the bundled heading defaults.
    #[serde(default)]
    pub headings: HeadingStyles,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeadingStyles {
    pub h1: HeadingStyle,
    pub h2: HeadingStyle,
    pub h3: HeadingStyle,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeadingStyle {
    pub size: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
}

impl Default for HeadingStyles {
    fn default() -> Self {
        // Read only this section; do not recursively default a CharmingStyle.
        #[derive(Deserialize)]
        struct Document {
            headings: HeadingStyles,
        }
        toml::from_str::<Document>(super::DEFAULT_CHARMING_EGUI)
            .expect("embedded headings must match their schema")
            .headings
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PanelButtonStyle {
    pub margin: egui::Margin,
    pub min_action_width: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterListStyle {
    pub name_ratio: f32,
    pub column_spacing: f32,
    pub row_spacing: f32,
    /// Minimum top gap before a group; the H3 top margin counts toward it.
    pub group_spacing: f32,
    pub striped: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionListStyle {
    pub frame: bool,
    pub wrap: bool,
    /// None inherits egui's vertical item spacing.
    pub row_spacing: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChoicesStyle {
    pub spacing: f32,
}

fn defaults() -> &'static Arc<CharmingStyle> {
    static STYLE: OnceLock<Arc<CharmingStyle>> = OnceLock::new();
    STYLE.get_or_init(|| {
        let style: CharmingStyle = toml::from_str(super::DEFAULT_CHARMING_EGUI)
            .expect("embedded yummy_egui theme must match its schema");
        style
            .validate()
            .expect("embedded component sizes must be valid");
        Arc::new(style)
    })
}

impl Default for CharmingStyle {
    fn default() -> Self {
        defaults().as_ref().clone()
    }
}

impl CharmingStyle {
    /// Read this Context's theme, or the embedded component defaults if unapplied.
    pub fn get(context: &egui::Context) -> Arc<Self> {
        context
            .data(|data| data.get_temp::<Arc<Self>>(egui::Id::new("charming_egui style")))
            .unwrap_or_else(|| Arc::clone(defaults()))
    }

    pub(super) fn install(&self, context: &egui::Context) {
        context.data_mut(|data| {
            data.insert_temp(egui::Id::new("charming_egui style"), Arc::new(self.clone()));
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

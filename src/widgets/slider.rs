//! Parameter-slider skin. egui still owns editing, clamping, stepping and input.
use egui::{Color32, Pos2, Rect, Response, Ui, Vec2, emath::Numeric};
use std::ops::RangeInclusive;

/// Slider skin; absent radii inherit the current egui style. Height is a separate
/// widget override, so changing colors never changes numeric editor sizing.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SliderAppearance {
    pub inset: f32,
    pub outset: f32,
    pub width: f32,
    pub hover_width: f32,
    pub value_width: f32,
    pub animation_ms: f32,
    pub background_radius: Option<f32>,
    pub marker_radius: Option<f32>,
    pub edge_padding: f32,
    pub background: Color32,
    pub marker: Color32,
    pub hover_marker: Color32,
}

impl Default for SliderAppearance {
    fn default() -> Self {
        crate::CharmingStyle::default().slider
    }
}

impl SliderAppearance {
    fn end_inset(&self, width: f32) -> f32 {
        // Padding is measured from the widest marker's outer edge, not its center.
        (self.hover_width * 0.5 + self.edge_padding).min((width * 0.5 - 0.5).max(0.0))
    }

    /// Paint a non-interactive swatch; fraction and hover blend are in 0..=1.
    /// The interactive widget uses this same painter.
    pub fn paint(&self, ui: &Ui, rect: Rect, fraction: f32, hover: f32) {
        let radius = f32::from(ui.visuals().widgets.inactive.corner_radius.nw);
        let height = egui::lerp(
            (rect.height() - 2.0 * self.inset).max(1.0)..=rect.height() + 2.0 * self.outset,
            hover,
        );
        let width = egui::lerp(self.width..=self.hover_width, hover).min(rect.width());
        let color = self.marker.lerp_to_gamma(self.hover_marker, hover);
        let inset = self.end_inset(rect.width());
        let center = Pos2::new(
            egui::lerp((rect.left() + inset)..=(rect.right() - inset), fraction),
            rect.center().y,
        );
        ui.painter().rect_filled(
            rect,
            self.background_radius.unwrap_or(radius),
            self.background,
        );
        ui.painter().rect_filled(
            Rect::from_center_size(center, Vec2::new(width, height)),
            self.marker_radius
                .unwrap_or(radius)
                .min(width.min(height) * 0.5),
            color,
        );
    }
}

/// Horizontal finite-range parameter slider; logarithmic ranges are non-negative.
/// All options below are forwarded to the native slider, including its DragValue.
#[must_use = "Add the slider to a Ui"]
pub struct ParameterSlider<'a, T: Numeric> {
    value: &'a mut T,
    range: RangeInclusive<T>,
    logarithmic: bool,
    smallest_positive: f64,
    clamping: egui::SliderClamping,
    max_decimals: Option<usize>,
    step: Option<f64>,
    suffix: String,
    appearance: Option<SliderAppearance>,
    height: Option<f32>,
}

impl<'a, T: Numeric> ParameterSlider<'a, T> {
    pub fn new(value: &'a mut T, range: RangeInclusive<T>) -> Self {
        Self {
            value,
            range,
            logarithmic: false,
            smallest_positive: if T::INTEGRAL { 1.0 } else { 1e-6 },
            clamping: egui::SliderClamping::Always,
            max_decimals: None,
            step: None,
            suffix: String::new(),
            appearance: None,
            height: None,
        }
    }
    pub fn logarithmic(mut self, enabled: bool) -> Self {
        self.logarithmic = enabled;
        self
    }
    pub fn smallest_positive(mut self, value: f64) -> Self {
        self.smallest_positive = value;
        self
    }
    pub fn clamping(mut self, value: egui::SliderClamping) -> Self {
        self.clamping = value;
        self
    }
    pub fn max_decimals(mut self, value: usize) -> Self {
        self.max_decimals = Some(value);
        self
    }
    pub fn step_by(mut self, value: f64) -> Self {
        self.step = Some(value);
        self
    }
    pub fn suffix(mut self, value: impl Into<String>) -> Self {
        self.suffix = value.into();
        self
    }
    pub fn appearance(mut self, value: &SliderAppearance) -> Self {
        self.appearance = Some(value.clone());
        self
    }
    /// Override the minimum interaction height in logical points, independently
    /// of the skin. Text and padding may still require a taller editor.
    pub fn height(mut self, height: f32) -> Self {
        assert!(height.is_finite() && height > 0.0);
        self.height = Some(height);
        self
    }
}

impl<T: Numeric> egui::Widget for ParameterSlider<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let appearance = self
            .appearance
            .unwrap_or_else(|| crate::CharmingStyle::get(ui.ctx()).slider.clone());
        let min = self.range.start().to_f64();
        let max = self.range.end().to_f64();
        debug_assert!(min.is_finite() && max.is_finite());
        debug_assert!(!self.logarithmic || min >= 0.0 && max >= 0.0);
        ui.scope(|ui| {
            // A color/skin override must not alter native editor dimensions.
            if let Some(height) = self.height {
                ui.spacing_mut().interact_size.y = height;
            }
            let height = ui
                .spacing()
                .interact_size
                .y
                .max(ui.text_style_height(&egui::TextStyle::Body));
            let gap = ui.spacing().item_spacing.x;
            let value_width = appearance.value_width.min(ui.available_width() * 0.4);
            let track_width = (ui.available_width() - value_width - gap).max(1.0);
            ui.spacing_mut().interact_size.x = value_width;
            ui.spacing_mut().slider_width = track_width;
            ui.spacing_mut().slider_rail_height = height;
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.add_space(appearance.outset + 1.0);
            // egui 0.34's rectangular handle has radius thickness / 2.5.
            // Use its footprint only for input: reserve a stable center range with
            // the full hover width + end padding, independent of hover animation.
            let aspect_ratio = appearance.end_inset(track_width) / (height / 2.5);
            let mut native = egui::Slider::new(&mut *self.value, self.range)
                .logarithmic(self.logarithmic)
                .smallest_positive(self.smallest_positive)
                .clamping(self.clamping)
                .suffix(self.suffix)
                .trailing_fill(false)
                .handle_shape(egui::style::HandleShape::Rect { aspect_ratio });
            if let Some(decimals) = self.max_decimals {
                native = native.max_decimals(decimals);
            }
            if let Some(step) = self.step {
                native = native.step_by(step);
            }
            // Keep the native handle inside the overpainted track even on hover.
            let widgets = &mut ui.visuals_mut().widgets;
            widgets.inactive.expansion = 0.0;
            widgets.hovered.expansion = 0.0;
            widgets.active.expansion = 0.0;
            let response = ui.add(native);
            let rect = Rect::from_center_size(
                Pos2::new(
                    response.rect.left() + track_width * 0.5,
                    response.rect.center().y,
                ),
                Vec2::new(track_width, response.rect.height()),
            );
            let active = ui.input(|input| {
                (response.hovered() && input.pointer.hover_pos().is_some_and(|p| rect.contains(p)))
                    || (response.dragged()
                        && input
                            .pointer
                            .press_origin()
                            .is_some_and(|p| rect.contains(p)))
            });
            let hover = ui.ctx().animate_bool_with_time(
                response.id.with("marker hover"),
                active,
                appearance.animation_ms / 1000.0,
            );
            let fraction = visual_fraction(
                self.value.to_f64(),
                min,
                max,
                self.logarithmic,
                self.smallest_positive,
            );
            // Opaque paint covers ONLY the native track/handle, not its numeric
            // editor. No shape inspection or reimplementation of native input.
            // Both fills intentionally have no hover, drag or focus outline.
            appearance.paint(ui, rect, fraction, hover);
            ui.add_space(appearance.outset + 1.0);
            response
        })
        .inner
    }
}

/// Display-only mapping; native egui performs the inverse mapping and edits.
/// Finite non-negative log ranges use egui's smallest-positive zero convention.
fn visual_fraction(value: f64, min: f64, max: f64, log: bool, smallest: f64) -> f32 {
    if min == max {
        return 0.5;
    }
    if min > max {
        return 1.0 - visual_fraction(value, max, min, log, smallest);
    }
    if value <= min {
        return 0.0;
    }
    if value >= max {
        return 1.0;
    }
    let fraction = if log {
        let low = if min > 0.0 {
            min.log10()
        } else if smallest < max {
            smallest.log10()
        } else {
            max.log10() - 10.0
        };
        (value.log10() - low) / (max.log10() - low)
    } else {
        (value - min) / (max - min)
    };
    if fraction.is_finite() {
        fraction.clamp(0.0, 1.0) as f32
    } else {
        0.5
    }
}

#[cfg(test)]
mod tests;

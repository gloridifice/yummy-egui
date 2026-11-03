//! Opt-in, full-width panel action buttons; native interaction and text metrics.

/// Theme padding and fixed vertical margins, local to panel action buttons.
struct PanelButtonSpacing {
    padding: egui::Margin,
    margin: egui::Margin,
}

impl PanelButtonSpacing {
    fn from_ui(ui: &egui::Ui) -> Self {
        let padding = ui.spacing().button_padding;
        Self {
            padding: egui::Margin::symmetric(padding.x as i8, padding.y as i8),
            margin: crate::YummyStyle::get(ui.ctx()).panel_button.margin,
        }
    }
}

/// A native egui button frame/interaction with centered, wrapped text and
/// theme padding and 4 pt top/bottom margins. Other buttons are untouched.
#[must_use = "Add the button to a Ui or an action_row"]
pub struct PanelButton {
    text: egui::WidgetText,
    enabled: bool,
}

impl PanelButton {
    /// Create a full-width panel button with the given label.
    pub fn new(text: impl Into<egui::WidgetText>) -> Self {
        Self {
            text: text.into(),
            enabled: true,
        }
    }

    /// Enable or disable the button (combined with the parent UI's enabled state).
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl egui::Widget for PanelButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let spacing = PanelButtonSpacing::from_ui(ui);
        let width = ui.available_width().max(0.0);
        // Keep insets inside even very narrow parent cells.
        let margin_left = f32::from(spacing.margin.left).min(width);
        let margin_right = f32::from(spacing.margin.right).min(width - margin_left);
        let button_width = width - margin_left - margin_right;
        let padding_left = f32::from(spacing.padding.left).min(button_width);
        let padding_right = f32::from(spacing.padding.right).min(button_width - padding_left);
        let mut job = std::sync::Arc::unwrap_or_clone(self.text.into_layout_job(
            ui.style(),
            egui::TextStyle::Button.into(),
            egui::Align::Center,
        ));
        job.wrap.max_width = button_width - padding_left - padding_right;
        job.halign = egui::Align::Center;
        job.justify = false;
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
        let height = (galley.size().y + spacing.padding.sum().y).max(ui.spacing().interact_size.y);
        let (_, outer) = ui.allocate_space(egui::vec2(width, height + spacing.margin.sum().y));
        let rect = egui::Rect::from_min_size(
            outer.min + egui::vec2(margin_left, f32::from(spacing.margin.top)),
            egui::vec2(button_width, height),
        );
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(egui::Layout::top_down_justified(egui::Align::Center)),
        );
        child.spacing_mut().button_padding = egui::Vec2::ZERO;
        child.add_enabled(self.enabled, |ui: &mut egui::Ui| {
            // egui owns hover/press/focus rendering and keyboard/pointer input.
            let response = ui.add(egui::Button::new(()).min_size(rect.size()));
            let text_rect = egui::Rect::from_min_max(
                rect.min + egui::vec2(padding_left, f32::from(spacing.padding.top)),
                rect.max - egui::vec2(padding_right, f32::from(spacing.padding.bottom)),
            );
            let pos = text_rect.center() - galley.rect.center().to_vec2();
            let color = ui.style().interact(&response).text_color();
            ui.painter()
                .with_clip_rect(ui.clip_rect().intersect(text_rect))
                .galley(pos, galley.clone(), color);
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    &galley.job.text,
                )
            });
            response
        })
    }
}

/// Equal-width actions with theme spacing. Below `min_action_width` (96 pt by
/// default) per button (excluding its margins), stack the group vertically.
/// Render once and return native responses.
///
/// # Panics
///
/// Panics if `N == 0`.
pub fn action_row<const N: usize>(
    ui: &mut egui::Ui,
    buttons: [PanelButton; N],
) -> [egui::Response; N] {
    assert!(N > 0, "action rows must contain at least one button");
    let gap = ui.spacing().item_spacing.x;
    let width = (ui.available_width() - gap * (N - 1) as f32) / N as f32;
    if width - PanelButtonSpacing::from_ui(ui).margin.sum().x
        < crate::YummyStyle::get(ui.ctx())
            .panel_button
            .min_action_width
    {
        buttons.map(|button| ui.add(button))
    } else {
        let mut buttons = buttons.into_iter();
        ui.columns_const(|columns: &mut [egui::Ui; N]| {
            std::array::from_fn(|i| columns[i].add(buttons.next().unwrap()))
        })
    }
}

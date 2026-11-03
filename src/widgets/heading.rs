//! RichText headings with vertical margins, including inside horizontal toolbars.
use crate::{YummyStyle, theme::HeadingStyle};
use egui::{Response, RichText, Ui};

/// High-contrast, bold 20 pt heading by default, with 12/8 pt top/bottom margins.
pub fn h1(ui: &mut Ui, text: impl Into<String>) -> Response {
    heading(
        ui,
        text.into(),
        &YummyStyle::get(ui.ctx()).headings.h1,
        true,
    )
}

/// High-contrast, bold 16 pt heading by default, with 10/6 pt top/bottom margins.
pub fn h2(ui: &mut Ui, text: impl Into<String>) -> Response {
    heading(
        ui,
        text.into(),
        &YummyStyle::get(ui.ctx()).headings.h2,
        true,
    )
}

/// High-contrast, regular 14 pt heading by default, with 8/4 pt top/bottom margins.
pub fn h3(ui: &mut Ui, text: impl Into<String>) -> Response {
    heading(
        ui,
        text.into(),
        &YummyStyle::get(ui.ctx()).headings.h3,
        false,
    )
}

fn heading(ui: &mut Ui, text: String, style: &HeadingStyle, bold: bool) -> Response {
    let color = ui.visuals().strong_text_color();
    let text = RichText::new(text).size(style.size).color(color);
    // A local vertical layout keeps margins vertical in horizontal toolbars and
    // Grid cells too. Return the label response: margins do not capture input.
    ui.scope_builder(
        egui::UiBuilder::new().layout(egui::Layout::top_down(egui::Align::Min)),
        |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.add_space(style.margin_top);
            let galley = egui::WidgetText::from(text).into_galley(
                ui,
                Some(egui::TextWrapMode::Wrap),
                ui.available_width(),
                egui::FontSelection::Default,
            );
            let response = ui.add(egui::Label::new(galley.clone()));
            if bold && ui.is_rect_visible(response.rect) {
                // egui's RichText::strong only changes color. A small offset
                // overprint gives H1/H2 actual synthetic weight without
                // requiring or replacing any of the host's font families.
                ui.painter()
                    .galley(response.rect.min + egui::vec2(0.35, 0.35), galley, color);
            }
            ui.add_space(style.margin_bottom);
            response
        },
    )
    .inner
}

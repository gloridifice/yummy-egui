use egui::{CornerRadius, Response, Stroke, Ui};

/// A connected, equal-width, single-row exclusive selector.
///
/// Only the two outer ends are rounded, with no outlines or separators.
/// Idle segments use the slider background; selection and hover use the theme.
/// Long labels
/// are truncated. Native buttons retain keyboard focus and disabled behavior.
/// Values should be unique; the caller owns selection (an unmatched value leaves
/// all segments unselected). An empty slice draws nothing. The returned response
/// is marked changed only when a different value is selected.
pub fn segmented_control<T: Copy + PartialEq>(
    ui: &mut Ui,
    value: &mut T,
    items: &[(T, &str)],
) -> Response {
    if items.is_empty() {
        return ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover());
    }
    let height =
        ui.spacing().interact_size.y.max(
            ui.text_style_height(&egui::TextStyle::Button) + 2.0 * ui.spacing().button_padding.y,
        );
    let width = ui.available_width().max(0.0);
    let (rect, mut response) =
        ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let radius = ui.visuals().widgets.inactive.corner_radius;
    let background = crate::CharmingStyle::get(ui.ctx()).slider.background;
    // Unselected native buttons skip painting their frame while idle, regardless
    // of the inactive fill. A shared track keeps native hover fills unchanged.
    ui.painter().rect_filled(rect, radius, background);
    let original = *value;
    ui.scope(|ui| {
        // Hover expansion and per-button outlines would break the joined edges.
        let widgets = &mut ui.visuals_mut().widgets;
        for visual in [
            &mut widgets.inactive,
            &mut widgets.hovered,
            &mut widgets.active,
            &mut widgets.open,
        ] {
            visual.expansion = 0.0;
            visual.bg_stroke = Stroke::NONE;
        }
        for (index, (choice, label)) in items.iter().enumerate() {
            let left = rect.left() + width * index as f32 / items.len() as f32;
            let right = rect.left() + width * (index + 1) as f32 / items.len() as f32;
            let segment = egui::Rect::from_min_max(
                egui::pos2(left, rect.top()),
                egui::pos2(right, rect.bottom()),
            );
            let corners = CornerRadius {
                nw: if index == 0 { radius.nw } else { 0 },
                sw: if index == 0 { radius.sw } else { 0 },
                ne: if index + 1 == items.len() {
                    radius.ne
                } else {
                    0
                },
                se: if index + 1 == items.len() {
                    radius.se
                } else {
                    0
                },
            };
            let inner = ui
                .scope(|ui| {
                    ui.set_clip_rect(ui.clip_rect().intersect(segment));
                    ui.put(
                        segment,
                        egui::Button::selectable(original == *choice, *label)
                            .corner_radius(corners)
                            .stroke(Stroke::NONE)
                            .frame(true)
                            .wrap_mode(egui::TextWrapMode::Truncate),
                    )
                })
                .inner;
            if inner.clicked() {
                *value = *choice;
            }
            response |= inner;
        }
    });
    if *value != original {
        response.mark_changed();
    }
    response
}

#[cfg(test)]
mod tests;

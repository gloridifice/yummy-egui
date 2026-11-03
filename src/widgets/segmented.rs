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
    let original = *value;
    let mut response = segmented_row(
        ui,
        egui::vec2(ui.available_width().max(0.0), height),
        items.len(),
        false,
        |ui, index, segment, corners| {
            let (choice, label) = items[index];
            let response = ui.put(
                segment,
                egui::Button::selectable(original == choice, label)
                    .corner_radius(corners)
                    .stroke(Stroke::NONE)
                    .frame(true)
                    .wrap_mode(egui::TextWrapMode::Truncate),
            );
            if response.clicked() {
                *value = choice;
            }
            response
        },
    );
    if *value != original {
        response.mark_changed();
    }
    response
}

/// Shared connected track and edge treatment for text and icon selectors.
pub(super) fn segmented_row(
    ui: &mut Ui,
    size: egui::Vec2,
    count: usize,
    vertical: bool,
    mut segment_ui: impl FnMut(&mut Ui, usize, egui::Rect, CornerRadius) -> Response,
) -> Response {
    let (id, outer) = ui.allocate_space(size);
    let rect = ui.layout().align_size_within_rect(size, outer);
    let mut response = ui.interact(rect, id, egui::Sense::hover());
    let radius = ui.visuals().widgets.inactive.corner_radius;
    let background = crate::YummyStyle::get(ui.ctx()).slider.background;
    // Unselected native buttons skip painting their frame while idle, regardless
    // of the inactive fill. A shared track keeps native hover fills unchanged.
    ui.painter().rect_filled(rect, radius, background);
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
        for index in 0..count {
            let axis = if vertical { 1 } else { 0 };
            let mut min = rect.min;
            let mut max = rect.max;
            min[axis] = rect.min[axis] + size[axis] * index as f32 / count as f32;
            max[axis] = rect.min[axis] + size[axis] * (index + 1) as f32 / count as f32;
            let segment = egui::Rect::from_min_max(min, max);
            let first = index == 0;
            let last = index + 1 == count;
            let round_sw = if vertical { last } else { first };
            let round_ne = if vertical { first } else { last };
            let corners = CornerRadius {
                nw: if first { radius.nw } else { 0 },
                sw: if round_sw { radius.sw } else { 0 },
                ne: if round_ne { radius.ne } else { 0 },
                se: if last { radius.se } else { 0 },
            };
            let inner = ui
                .scope(|ui| {
                    ui.set_clip_rect(ui.clip_rect().intersect(segment));
                    segment_ui(ui, index, segment, corners)
                })
                .inner;
            response |= inner;
        }
    });
    response
}

#[cfg(test)]
mod tests;

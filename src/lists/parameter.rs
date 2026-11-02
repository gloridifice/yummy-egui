//! Two-column parameter forms. Each group owns one Grid, not one Grid per row.
use crate::theme::{CharmingStyle, ParameterListStyle};
use egui::{Id, Ui};
use std::hash::Hash;

/// Bounded name/control columns, with wrapping names and vertically centered rows.
pub struct ParameterList {
    id: Id,
    style: Option<ParameterListStyle>,
}

impl ParameterList {
    pub fn new(id: impl Hash) -> Self {
        Self {
            id: Id::new(id),
            style: None,
        }
    }

    pub fn style(mut self, style: ParameterListStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn show<R>(self, ui: &mut Ui, rows: impl FnOnce(&mut ParameterRows<'_>) -> R) -> R {
        self.show_ui(ui, |ui| rows(&mut ParameterRows { ui }))
    }

    fn show_ui<R>(self, ui: &mut Ui, rows: impl FnOnce(&mut Ui) -> R) -> R {
        let style = self
            .style
            .unwrap_or_else(|| CharmingStyle::get(ui.ctx()).parameter_list.clone());
        let usable = (ui.available_width() - style.column_spacing).max(0.0);
        let widths = [usable * style.name_ratio, usable * (1.0 - style.name_ratio)];
        let key = widths_key();
        // A scoped stack supports the Ui-based convenience functions and nested
        // forms. State is Context-local and restored after each complete Grid.
        let previous = ui.data_mut(|data| {
            let old = data.get_temp::<[f32; 2]>(key);
            data.insert_temp(key, widths);
            old
        });
        let result = ui
            .push_id(self.id, |ui| {
                ui.spacing_mut().slider_width = (widths[1] - 55.0).max(0.0);
                egui::Grid::new((self.id, widths[0].to_bits()))
                    .num_columns(2)
                    .min_col_width(widths[0])
                    .spacing(egui::vec2(style.column_spacing, style.row_spacing))
                    .striped(style.striped)
                    .show(ui, rows)
                    .inner
            })
            .inner;
        ui.data_mut(|data| {
            if let Some(old) = previous {
                data.insert_temp(key, old);
            } else {
                data.remove::<[f32; 2]>(key);
            }
        });
        result
    }
}

/// Rows are only constructed inside a ParameterList. Row IDs are independent of
/// the displayed names, so duplicate labels need not share interaction state.
pub struct ParameterRows<'a> {
    ui: &'a mut Ui,
}

impl ParameterRows<'_> {
    pub fn row<R>(
        &mut self,
        id: impl Hash,
        name: &str,
        description: &str,
        control: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        row(self.ui, Id::new(id), name, description, control)
    }
}

fn widths_key() -> Id {
    Id::new("charming_egui parameter column widths")
}

/// Ui-based form convenience API. Use ParameterList::show for explicit row IDs.
pub fn parameter_table<R>(ui: &mut Ui, id: impl Hash, rows: impl FnOnce(&mut Ui) -> R) -> R {
    ParameterList::new(id).show_ui(ui, rows)
}

/// Append a row inside parameter_table/group. Names must be unique within a
/// group; use ParameterRows::row to supply a separate stable ID otherwise.
pub fn parameter<R>(
    ui: &mut Ui,
    name: &str,
    description: &str,
    control: impl FnOnce(&mut Ui) -> R,
) -> R {
    row(ui, Id::new(name), name, description, control)
}

/// H3 title followed by one shared parameter Grid.
pub fn group(ui: &mut Ui, title: &str, rows: impl FnOnce(&mut Ui)) {
    let style = CharmingStyle::get(ui.ctx());
    // Keep old themes' group_spacing as a minimum, not an extra H3 top margin.
    ui.add_space((style.parameter_list.group_spacing - style.headings.h3.margin_top).max(0.0));
    crate::h3(ui, title);
    parameter_table(ui, title, rows);
}

fn row<R>(
    ui: &mut Ui,
    id: Id,
    name: &str,
    description: &str,
    control: impl FnOnce(&mut Ui) -> R,
) -> R {
    let widths = ui
        .data(|data| data.get_temp::<[f32; 2]>(widths_key()))
        .expect("parameter rows must be inside a shared list");
    cell(ui, id.with("name"), widths[0], |ui| {
        ui.add(egui::Label::new(name).wrap())
            .on_hover_text(description);
    });
    let value = cell(ui, id.with("control"), widths[1], control);
    ui.end_row();
    value
}

/// Reserve exactly the column width even when the child asks for more.
fn cell<R>(ui: &mut Ui, id: Id, width: f32, contents: impl FnOnce(&mut Ui) -> R) -> R {
    let height_id = ui.make_persistent_id((id, "content height"));
    let previous_height = ui
        .data(|data| data.get_temp::<f32>(height_id))
        .unwrap_or(ui.spacing().interact_size.y);
    let mut origin = ui.available_rect_before_wrap().min;
    // Place contents at Grid's row center before painting AND registering hits.
    origin.y = (ui.next_widget_position().y - previous_height * 0.5).max(origin.y);
    let bounds = egui::Rect::from_min_size(origin, egui::vec2(width, 100000.0));
    let cell_id = ui.make_persistent_id(("parameter cell", id));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .id(cell_id)
            .max_rect(bounds)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    child.set_clip_rect(ui.clip_rect().intersect(bounds));
    child.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
    let value = contents(&mut child);
    let height = child.min_size().y;
    ui.data_mut(|data| data.insert_temp(height_id, height));
    if (height - previous_height).abs() > 0.01 {
        ui.ctx().request_discard("parameter cell height changed");
    }
    ui.allocate_space(egui::vec2(width, height));
    value
}

#[cfg(test)]
mod tests;

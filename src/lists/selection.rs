//! Scrollable full-width selectable rows. The application owns selection and
//! actions; even an already selected item returns its ordinary click response.
use crate::theme::{SelectionListStyle, YummyStyle};
use egui::{Id, Response, Ui};
use std::hash::Hash;

/// A vertically scrollable list of full-width selectable rows.
///
/// The host application owns selection state and decides how to handle clicks;
/// clicking an already selected enabled row still reports `clicked()`.
pub struct SelectionList {
    id: Id,
    max_height: f32,
    style: Option<SelectionListStyle>,
}

impl SelectionList {
    /// Create a selection list with a caller-supplied ID unique within the parent UI.
    pub fn new(id: impl Hash) -> Self {
        Self {
            id: Id::new(id),
            max_height: f32::INFINITY,
            style: None,
        }
    }

    /// Cap the vertical scroll area's height in logical points (`INFINITY` by default).
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Override the context's [`SelectionListStyle`] for this list only.
    pub fn style(mut self, style: SelectionListStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Render the scrollable list and return the closure's result with the outer response.
    pub fn show<R>(
        self,
        ui: &mut Ui,
        items: impl FnOnce(&mut SelectionRows<'_>) -> R,
    ) -> egui::InnerResponse<R> {
        let style = self
            .style
            .unwrap_or_else(|| YummyStyle::get(ui.ctx()).selection_list.clone());
        let frame = if style.frame {
            egui::Frame::group(ui.style())
        } else {
            egui::Frame::NONE
        };
        // Namespace both scroll state and item IDs. Labels and row order need not
        // be stable; caller-supplied IDs must be unique within this list.
        ui.push_id(self.id, |ui| {
            frame
                .show(ui, |ui| {
                    if let Some(spacing) = style.row_spacing {
                        ui.spacing_mut().item_spacing.y = spacing;
                    }
                    egui::ScrollArea::vertical()
                        .id_salt("items")
                        .max_height(self.max_height)
                        .show(ui, |ui| {
                            items(&mut SelectionRows {
                                ui,
                                wrap: style.wrap,
                            })
                        })
                        .inner
                })
                .inner
        })
    }
}

/// Row builder passed to [`SelectionList::show`].
///
/// Caller-supplied row IDs must be unique within the list and stable across
/// reordering; visible labels and row order need not be unique or stable.
pub struct SelectionRows<'a> {
    ui: &'a mut Ui,
    wrap: bool,
}

impl SelectionRows<'_> {
    /// Append an enabled selectable row and return its native [`Response`].
    pub fn item(
        &mut self,
        id: impl Hash,
        selected: bool,
        text: impl Into<egui::WidgetText>,
    ) -> Response {
        self.item_enabled(id, selected, true, text)
    }

    /// Append a row that may be disabled. A disabled row stays visible but cannot
    /// activate; context menus and tooltips can be attached to the returned [`Response`].
    pub fn item_enabled(
        &mut self,
        id: impl Hash,
        selected: bool,
        enabled: bool,
        text: impl Into<egui::WidgetText>,
    ) -> Response {
        let id = self.ui.make_persistent_id(("item", id));
        // push_id alone still salts native auto IDs with row order in egui 0.34.
        self.ui
            .scope_builder(egui::UiBuilder::new().id(id), |ui| {
                ui.add_enabled_ui(enabled, |ui| {
                    let wrap = if self.wrap {
                        egui::TextWrapMode::Wrap
                    } else {
                        egui::TextWrapMode::Truncate
                    };
                    let size = egui::vec2(ui.available_width(), ui.spacing().interact_size.y);
                    ui.add_sized(
                        size,
                        egui::Button::selectable(selected, text).wrap_mode(wrap),
                    )
                })
                .inner
            })
            .inner
    }
}

#[cfg(test)]
mod tests;

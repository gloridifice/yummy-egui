//! Native panel builders with the same content margins as themed egui windows.
//!
//! Each factory copies `ui.spacing().window_margin` into the panel frame when
//! called. Apply [`crate::Theme`] for the bundled 10 pt default, and construct
//! panels each frame to follow live theme edits. No global egui defaults are
//! patched: panels constructed directly through egui still use egui's defaults.
//!
//! The returned builders retain the native sizing, resizing and response APIs.
//! Calling `.frame(...)` afterwards intentionally replaces the themed frame.
//!
//! ```
//! # egui::__run_test_ui(|ui| {
//! yummy_egui::panels::left(ui, "navigation")
//!     .resizable(false)
//!     .exact_size(56.0)
//!     .show_inside(ui, |ui| { ui.label("Nav"); });
//! yummy_egui::panels::central(ui).show_inside(ui, |ui| {
//!     ui.label("Content");
//! });
//! # });
//! ```

/// A central panel using the current UI's window margin and panel fill.
pub fn central(ui: &egui::Ui) -> egui::CentralPanel {
    egui::CentralPanel::default()
        .frame(egui::Frame::central_panel(ui.style()).inner_margin(ui.spacing().window_margin))
}

/// A left panel using the current UI's window margin and panel fill.
pub fn left(ui: &egui::Ui, id: impl Into<egui::Id>) -> egui::Panel {
    themed(ui, egui::Panel::left(id))
}

/// A right panel using the current UI's window margin and panel fill.
pub fn right(ui: &egui::Ui, id: impl Into<egui::Id>) -> egui::Panel {
    themed(ui, egui::Panel::right(id))
}

/// A top panel using the current UI's window margin and panel fill.
pub fn top(ui: &egui::Ui, id: impl Into<egui::Id>) -> egui::Panel {
    themed(ui, egui::Panel::top(id))
}

/// A bottom panel using the current UI's window margin and panel fill.
pub fn bottom(ui: &egui::Ui, id: impl Into<egui::Id>) -> egui::Panel {
    themed(ui, egui::Panel::bottom(id))
}

fn themed(ui: &egui::Ui, panel: egui::Panel) -> egui::Panel {
    panel.frame(egui::Frame::side_top_panel(ui.style()).inner_margin(ui.spacing().window_margin))
}

#[cfg(test)]
mod tests;

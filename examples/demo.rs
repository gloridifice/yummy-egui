//! Run with `cargo run --example demo`.
use yummy_egui::{DemoState, IconSegmentedControl, Theme, demo_ui, panels};

struct DemoApp {
    state: DemoState,
    page: usize,
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        panels::left(ui, "navigation")
            .resizable(false)
            .exact_size(56.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    let icons = [
                        (
                            0,
                            egui::Image::new(egui::include_image!("icons/components.svg")),
                        ),
                        (1, egui::Image::new(egui::include_image!("icons/info.svg"))),
                    ];
                    ui.add(
                        IconSegmentedControl::new(&mut self.page, icons)
                            .vertical()
                            .size(36.0)
                            .icon_size(20.0),
                    )
                    .on_hover_text("Grid: Components\nInfo: About");
                });
            });
        panels::central(ui).show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.page {
                0 => demo_ui(ui, &mut self.state),
                _ => {
                    yummy_egui::h1(ui, "About this example");
                    ui.label("Reusable egui widgets with TOML themes.");
                    ui.label("Select the grid icon to explore the component gallery.");
                    ui.label("The left sidebar uses IconSegmentedControl::vertical() with embedded SVG icons.");
                    ui.label("Run the style_editor example to edit the theme live.");
                }
            });
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Charming egui demo",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 800.0]),
            ..Default::default()
        },
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Theme::default().apply(&cc.egui_ctx);
            Ok(Box::new(DemoApp {
                state: DemoState::default(),
                page: 0,
            }))
        }),
    )
}

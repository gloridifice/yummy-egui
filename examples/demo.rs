//! Run with `cargo run --example demo`.
use yummy_egui::{DemoState, Theme, demo_window};

struct DemoApp {
    state: DemoState,
    open: bool,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.checkbox(&mut self.open, "Show demo");
        });
        demo_window(ctx, &mut self.open, &mut self.state);
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Charming egui demo",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            Theme::default().apply(&cc.egui_ctx);
            Ok(Box::new(DemoApp {
                state: DemoState::default(),
                open: true,
            }))
        }),
    )
}

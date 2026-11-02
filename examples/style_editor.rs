//! Run with `cargo run --example style_editor`.
use yummy_egui::{DemoState, Theme, demo_window, style_editor_window};

struct StyleEditorApp {
    preview: DemoState,
    editor_open: bool,
    preview_open: bool,
}

impl eframe::App for StyleEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let [editor, preview] = yummy_egui::action_row(
                ui,
                [
                    yummy_egui::PanelButton::new("Show style editor"),
                    yummy_egui::PanelButton::new("Show preview"),
                ],
            );
            if editor.clicked() {
                self.editor_open = true;
            }
            if preview.clicked() {
                self.preview_open = true;
            }
        });
        style_editor_window(ctx, &mut self.editor_open);
        demo_window(ctx, &mut self.preview_open, &mut self.preview);
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Charming egui style editor",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 800.0]),
            ..Default::default()
        },
        Box::new(|cc| {
            Theme::default().apply(&cc.egui_ctx);
            Ok(Box::new(StyleEditorApp {
                preview: DemoState::default(),
                editor_open: true,
                preview_open: true,
            }))
        }),
    )
}

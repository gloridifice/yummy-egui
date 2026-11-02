use super::*;

#[test]
fn segments_select_exclusively_report_changes_and_respect_disabled_ui() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut value = 0;
    let mut frame = |events: Vec<egui::Event>, enabled: bool| {
        let mut result = (egui::Rect::NOTHING, false, value);
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(400.0, 200.0),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.add_enabled_ui(enabled, |ui| {
                        let response = segmented_control(
                            ui,
                            &mut value,
                            &[(0, "Same label"), (1, "Same label"), (2, "Last")],
                        );
                        result = (response.rect, response.changed(), value);
                    });
                });
            },
        );
        result
    };
    frame(vec![], true);
    let (rect, _, _) = frame(vec![], true);
    for (index, enabled, expected, changed) in [
        (1, true, 1, true),
        (1, true, 1, false),
        (2, false, 1, false),
        (2, true, 2, true),
        (0, true, 0, true),
    ] {
        // Let egui register the enabled state before dispatching the click.
        frame(vec![], enabled);
        let pos = egui::pos2(
            rect.left() + rect.width() * (index as f32 + 0.5) / 3.0,
            rect.center().y,
        );
        let mut result = (rect, false, 0);
        for pressed in [true, false] {
            result = frame(
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                enabled,
            );
        }
        assert_eq!(
            result.2, expected,
            "only the activated segment changes selection"
        );
        assert_eq!(
            result.1, changed,
            "changed reports value changes, not repeated clicks"
        );
    }
}

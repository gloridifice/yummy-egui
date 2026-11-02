use super::*;

fn frame<T: Numeric>(
    ctx: &egui::Context,
    value: &mut T,
    range: RangeInclusive<T>,
    log: bool,
    events: Vec<egui::Event>,
) -> egui::Rect {
    let mut bounds = Rect::NOTHING;
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(500.0, 200.0))),
            events,
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.set_width(400.0);
                bounds = ui
                    .add(
                        ParameterSlider::new(value, range.clone())
                            .logarithmic(log)
                            .clamping(egui::SliderClamping::Edits),
                    )
                    .rect;
            });
        },
    );
    bounds
}

fn press(position: Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(position),
        egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

// The skin must not change the native range semantics, especially log sliders
// containing zero. These are actual input events, not tests of a private formula.
#[test]
fn skinned_slider_keeps_native_linear_log_and_integer_edits() {
    for (range, log, expected) in [
        (-180.0_f64..=180.0, false, 0.0),
        (0.01..=100.0, true, 1.0),
        (0.0..=100.0, true, 0.01),
    ] {
        let ctx = egui::Context::default();
        let mut value = *range.start();
        let bounds = frame(&ctx, &mut value, range.clone(), log, vec![]);
        // The returned control includes its 64 pt editor and native 8 pt gap.
        let position = Pos2::new(
            bounds.left() + (bounds.width() - 64.0 - 8.0) * 0.5,
            bounds.center().y,
        );
        frame(&ctx, &mut value, range, log, press(position));
        assert!((value - expected).abs() < 1e-6, "{value} != {expected}");
    }
    let ctx = egui::Context::default();
    let mut value = 0_u32;
    let bounds = frame(&ctx, &mut value, 0..=100, false, vec![]);
    frame(
        &ctx,
        &mut value,
        0..=100,
        false,
        press(Pos2::new(
            bounds.left() + (bounds.width() - 64.0 - 8.0) * 0.5,
            bounds.center().y,
        )),
    );
    assert_eq!(value, 50);
}

#[test]
fn slider_editor_matches_standalone_editor_with_theme_height_and_padding() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    for (minimum_height, padding_y) in [(22.0, 1.0), (30.0, 1.0), (22.0, 6.0)] {
        let mut value = 1.0;
        let mut standalone = 1.0;
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.spacing_mut().interact_size.y = minimum_height;
                ui.spacing_mut().button_padding.y = padding_y;
                let expected = ui.add(egui::DragValue::new(&mut standalone)).rect.height();
                let actual = ui
                    .add(
                        ParameterSlider::new(&mut value, 0.0..=2.0)
                            .appearance(&SliderAppearance::default()),
                    )
                    .rect
                    .height();
                assert_eq!(
                    actual, expected,
                    "theme height {minimum_height}, padding {padding_y}"
                );
            });
        });
    }
}

#[test]
fn showing_out_of_slider_range_value_does_not_clamp_saved_parameter() {
    let ctx = egui::Context::default();
    let mut value = 80.0;
    frame(&ctx, &mut value, 0.0..=30.0, false, vec![]);
    frame(&ctx, &mut value, 0.0..=30.0, false, vec![]);
    assert_eq!(value, 80.0);
}

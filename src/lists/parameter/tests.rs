use super::*;
use crate::{ParameterSlider, Theme};

#[test]
fn parameter_groups_share_ratio_even_with_oversized_controls() {
    let ctx = egui::Context::default();
    for width in [240.0, 400.0] {
        for _ in 0..2 {
            let _ = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width + 16.0, 800.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.set_width(width);
                        let start = ui.cursor().left();
                        let expected = start + (width - 8.0) * 0.42 + 8.0;
                        let mut positions = Vec::new();
                        let mut ids = std::collections::HashSet::new();
                        for group in ["short group", "long group"] {
                            parameter_table(ui, group, |ui| {
                                for name in ["A", "A much longer name which wraps"] {
                                    parameter(ui, name, "Tooltip", |ui| {
                                        positions.push(ui.cursor().left());
                                        let response = ui.add_sized([500.0, 20.0], egui::Button::new(name));
                                        assert!(ids.insert(response.id), "same-name rows in different groups must not share interaction state");
                                    });
                                }
                            });
                        }
                        for x in positions {
                            assert!((x - expected).abs() < 1.0, "{x} != {expected}");
                        }
                    });
                },
            );
        }
    }
}

#[test]
fn parameter_names_and_controls_share_row_centers() {
    let ctx = egui::Context::default();
    Theme::default().apply(&ctx);
    for (width, height) in [(400.0, 22.0), (240.0, 22.0), (240.0, 30.0)] {
        let names = [
            "Rise threshold (m/s)",
            "Number",
            "A long parameter name that needs multiple lines (m/s)",
            "Enabled",
        ];
        let mut value = 0.25;
        let mut enabled = true;
        // Grid dimensions settle across passes, including after a panel resize.
        for frame in 0..3 {
            let mut controls = Vec::new();
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width + 16.0, 800.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    controls.clear();
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.set_width(width);
                        ui.spacing_mut().interact_size.y = height;
                        ParameterList::new("row alignment").show(ui, |list| {
                            for (index, name) in names.iter().enumerate() {
                                let response = list.row(index, name, "Tooltip", |ui| match index {
                                    1 => ui.add(egui::DragValue::new(&mut value)),
                                    3 => ui.checkbox(&mut enabled, ""),
                                    _ => ui.add(ParameterSlider::new(&mut value, 0.0..=1.0)),
                                });
                                controls.push(response.rect);
                            }
                        });
                    });
                },
            );
            if frame == 2 {
                for (name, control) in names.iter().zip(&controls) {
                    let text = output
                        .shapes
                        .iter()
                        .find_map(|shape| match &shape.shape {
                            egui::Shape::Text(text) if text.galley.job.text == *name => Some(text),
                            _ => None,
                        })
                        .expect("parameter name must be painted");
                    let center_y = text.pos.y + text.galley.size().y * 0.5;
                    assert!(
                        (center_y - control.center().y).abs() <= 1.0,
                        "{name}: text center {center_y}, control {control:?}, panel width {width}"
                    );
                }
            }
        }
    }
}

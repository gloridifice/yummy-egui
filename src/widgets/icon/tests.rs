use crate::{IconButton, IconChoices, IconSegmentedControl};
use egui::{Context, Event, Pos2, Rect, Response, Vec2};

fn frame(
    ctx: &Context,
    events: Vec<Event>,
    mut draw: impl FnMut(&mut egui::Ui),
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(400.0, 240.0))),
            events,
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show_inside(ui, |ui| draw(ui));
        },
    )
}

fn pointer(pos: Pos2, pressed: bool) -> Vec<Event> {
    vec![
        Event::PointerMoved(pos),
        Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

#[test]
fn theme_and_code_sizes_control_frames_and_icons_without_parent_stretching() {
    let ctx = Context::default();
    let mut style = crate::CharmingStyle::default();
    style.icon_button.size = 34.0;
    style.icon_button.icon_size = 18.0;
    let source = toml::to_string(&style).unwrap();
    crate::Theme::from_sources(crate::ThemeSources {
        charming_egui: Some(&source),
        ..Default::default()
    })
    .unwrap()
    .apply(&ctx);
    let texture = ctx.load_texture(
        "wide icon",
        egui::ColorImage::filled([80, 40], egui::Color32::WHITE),
        Default::default(),
    );
    let mut rects = Vec::new();
    let output = frame(&ctx, vec![], |ui| {
        // Global metrics and justified layouts must not enlarge fixed icon cells.
        ui.spacing_mut().button_padding = Vec2::splat(20.0);
        ui.spacing_mut().interact_size = Vec2::splat(80.0);
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
            rects.push(ui.add(IconButton::new(egui::Image::new(&texture))).rect);
            rects.push(
                ui.add(
                    IconButton::new(egui::Image::new(&texture))
                        .size(24.0)
                        .icon_size(12.0),
                )
                .rect,
            );
            rects.push(
                ui.add(
                    IconButton::new(egui::Image::new(&texture))
                        .size(10.0)
                        .icon_size(50.0),
                )
                .rect,
            );
            rects.push(
                ui.add(IconButton::new(egui::RichText::new("Long icon").size(100.0)).size(24.0))
                    .rect,
            );
            let mut value = 0;
            rects.push(
                ui.add(IconChoices::new(&mut value, [(0, "←"), (1, "→")]))
                    .rect,
            );
            rects.push(
                ui.add(IconSegmentedControl::new(&mut value, [(0, "←"), (1, "→")]).size(20.0))
                    .rect,
            );
        });
    });
    for (rect, expected) in rects.iter().zip([
        Vec2::splat(34.0),
        Vec2::splat(24.0),
        Vec2::splat(10.0),
        Vec2::splat(24.0),
        egui::vec2(68.0 + style.choices.spacing, 34.0),
        egui::vec2(40.0, 20.0),
    ]) {
        assert_eq!(rect.size(), expected);
    }
    // Inspect delivered image geometry, not the sizing helper: aspect ratio,
    // icon-size precedence and clipping limits must survive a real egui frame.
    let images: Vec<_> = output
        .shapes
        .iter()
        .filter_map(|shape| match &shape.shape {
            egui::Shape::Mesh(mesh) if mesh.texture_id == texture.id() => Some(mesh.calc_bounds()),
            egui::Shape::Rect(rect)
                if rect
                    .brush
                    .as_ref()
                    .is_some_and(|brush| brush.fill_texture_id == texture.id()) =>
            {
                Some(rect.rect)
            }
            _ => None,
        })
        .collect();
    assert_eq!(images.len(), 3);
    for ((image, button), expected) in images.iter().zip(&rects).zip([
        egui::vec2(18.0, 9.0),
        egui::vec2(12.0, 6.0),
        egui::vec2(10.0, 5.0),
    ]) {
        assert_eq!(image.size(), expected);
        assert!(
            (image.center() - button.center()).length() <= 0.75,
            "centered to pixel precision"
        );
    }
}

#[test]
fn icon_action_retains_repeated_pointer_keyboard_and_disabled_behavior() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    let mut response: Option<Response> = None;
    let mut draw = |events, parent_enabled, button_enabled, focus| {
        frame(&ctx, events, |ui| {
            ui.add_enabled_ui(parent_enabled, |ui| {
                let current = ui.add(IconButton::new("+").enabled(button_enabled).label("Add"));
                if focus {
                    current.request_focus();
                }
                response = Some(current);
            });
        });
        response.clone().unwrap()
    };
    draw(vec![], true, true, false);
    let pos = draw(vec![], true, true, false).rect.center();
    for (parent, button, expected) in [
        (true, true, true),
        (true, true, true),
        (false, true, false),
        (true, false, false),
    ] {
        draw(vec![], parent, button, false);
        draw(pointer(pos, true), parent, button, false);
        assert_eq!(
            draw(pointer(pos, false), parent, button, false).clicked(),
            expected
        );
    }
    draw(vec![], true, true, true);
    let pressed = draw(
        vec![Event::Key {
            key: egui::Key::Space,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        true,
        true,
        false,
    );
    assert!(
        pressed.clicked(),
        "native focused-button keyboard activation is preserved"
    );
}

#[test]
fn both_icon_selectors_handle_exclusive_changes_repeats_disabled_empty_and_single_items() {
    for (connected, vertical) in [(false, false), (true, false), (false, true), (true, true)] {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let mut value = 99; // An unmatched selection must stay unchanged until activation.
        let mut draw = |events, enabled, count| {
            let mut result = (Rect::NOTHING, false, value);
            frame(&ctx, events, |ui| {
                // Duplicate glyphs must not alias distinct options' interaction IDs.
                let items = (0..count).map(|choice| (choice, "●"));
                let response = if connected {
                    let widget = IconSegmentedControl::new(&mut value, items).size(30.0);
                    ui.add_enabled(enabled, if vertical { widget.vertical() } else { widget })
                } else {
                    let widget = IconChoices::new(&mut value, items).size(30.0);
                    ui.add_enabled(enabled, if vertical { widget.vertical() } else { widget })
                };
                result = (response.rect, response.changed(), value);
            });
            result
        };
        draw(vec![], true, 3);
        let (rect, changed, initial) = draw(vec![], true, 3);
        assert_eq!(initial, 99);
        assert!(!changed);
        let step = if connected {
            30.0
        } else {
            30.0 + crate::CharmingStyle::get(&ctx).choices.spacing
        };
        let expected_size = if vertical {
            egui::vec2(30.0, 2.0 * step + 30.0)
        } else {
            egui::vec2(2.0 * step + 30.0, 30.0)
        };
        assert_eq!(rect.size(), expected_size);
        for (index, enabled, expected, changed) in [
            (1, true, 1, true),
            (1, true, 1, false),
            (2, false, 1, false),
            (2, true, 2, true),
            (0, true, 0, true),
        ] {
            draw(vec![], enabled, 3);
            let offset = step * index as f32 + 15.0;
            let pos = rect.min
                + if vertical {
                    egui::vec2(15.0, offset)
                } else {
                    egui::vec2(offset, 15.0)
                };
            draw(pointer(pos, true), enabled, 3);
            let (_, actual_changed, actual) = draw(pointer(pos, false), enabled, 3);
            assert_eq!(
                (actual, actual_changed),
                (expected, changed),
                "connected={connected}, vertical={vertical}"
            );
        }
        let (empty, changed, actual) = draw(vec![], true, 0);
        assert_eq!((actual, changed), (0, false));
        assert_eq!(empty.size(), Vec2::ZERO);
        let (single, changed, actual) = draw(vec![], true, 1);
        assert_eq!((actual, changed), (0, false));
        assert_eq!(single.size(), Vec2::splat(30.0));
    }
}

#[test]
fn vertical_segments_round_only_the_outer_top_and_bottom_corners() {
    for (count, selected, expected) in [
        (
            3,
            0,
            egui::CornerRadius {
                nw: 4,
                ne: 4,
                sw: 0,
                se: 0,
            },
        ),
        (3, 1, egui::CornerRadius::ZERO),
        (
            3,
            2,
            egui::CornerRadius {
                nw: 0,
                ne: 0,
                sw: 4,
                se: 4,
            },
        ),
        (1, 0, egui::CornerRadius::same(4)),
    ] {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.global_style_mut(|style| {
            style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(4);
        });
        let fill = ctx.global_style().visuals.selection.bg_fill;
        let mut value = selected;
        let output = frame(&ctx, vec![], |ui| {
            ui.add(IconSegmentedControl::new(&mut value, (0..count).map(|i| (i, "●"))).vertical());
        });
        let corners: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Rect(rect) if rect.fill == fill => Some(rect.corner_radius),
                _ => None,
            })
            .collect();
        assert_eq!(corners, [expected], "count={count}, selected={selected}");
    }
}

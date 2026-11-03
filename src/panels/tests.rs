use super::*;

#[test]
fn all_panel_content_insets_match_the_theme_and_follow_live_changes() {
    for kind in 0..5 {
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        // First guard the bundled 10 pt contract, then ensure factories read the
        // current style rather than baking that default into each panel kind.
        for margin in [
            egui::Margin::same(10),
            egui::Margin {
                left: 11,
                right: 13,
                top: 15,
                bottom: 17,
            },
        ] {
            if margin != egui::Margin::same(10) {
                ctx.global_style_mut(|style| style.spacing.window_margin = margin);
            }
            for _ in 0..2 {
                let _ = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(400.0, 300.0),
                        )),
                        ..Default::default()
                    },
                    |ui| {
                        let response = if kind == 0 {
                            central(ui).show_inside(ui, |ui| ui.max_rect())
                        } else {
                            let panel = match kind {
                                1 => left(ui, "panel"),
                                2 => right(ui, "panel"),
                                3 => top(ui, "panel"),
                                _ => bottom(ui, "panel"),
                            };
                            panel
                                .resizable(false)
                                .exact_size(120.0)
                                .show_inside(ui, |ui| ui.max_rect())
                        };
                        let outer = response.response.rect;
                        let inner = response.inner;
                        assert_eq!(
                            inner.left() - outer.left(),
                            f32::from(margin.left),
                            "kind={kind}"
                        );
                        assert_eq!(
                            outer.right() - inner.right(),
                            f32::from(margin.right),
                            "kind={kind}"
                        );
                        assert_eq!(
                            inner.top() - outer.top(),
                            f32::from(margin.top),
                            "kind={kind}"
                        );
                        assert_eq!(
                            outer.bottom() - inner.bottom(),
                            f32::from(margin.bottom),
                            "kind={kind}"
                        );
                    },
                );
            }
        }
    }
}

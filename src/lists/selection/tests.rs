use super::*;

struct Harness {
    context: egui::Context,
    clicked: Vec<&'static str>,
    menu_open: bool,
}
impl Harness {
    fn new() -> Self {
        let context = egui::Context::default();
        crate::Theme::default().apply(&context);
        Self {
            context,
            clicked: Vec::new(),
            menu_open: false,
        }
    }
    fn frame(&mut self, order: [&'static str; 3], events: Vec<egui::Event>) -> Vec<Response> {
        let mut responses = Vec::new();
        self.menu_open = false;
        let _ = self.context.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(400.0, 600.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                responses.clear();
                egui::CentralPanel::default().show(ctx, |ui| {
                    SelectionList::new("items")
                        .max_height(200.0)
                        .show(ui, |rows| {
                            for id in order {
                                // Duplicate display names must not alias the interaction IDs.
                                let response =
                                    rows.item_enabled(id, id == "a", id != "disabled", "Item");
                                if response.clicked() {
                                    self.clicked.push(id);
                                }
                                response.context_menu(|ui| {
                                    self.menu_open = true;
                                    ui.label("Item actions");
                                });
                                responses.push(response);
                            }
                        });
                });
            },
        );
        responses
    }
    fn click(&mut self, position: egui::Pos2, button: egui::PointerButton) {
        for pressed in [true, false] {
            self.frame(
                ["a", "b", "disabled"],
                vec![
                    egui::Event::PointerMoved(position),
                    egui::Event::PointerButton {
                        pos: position,
                        button,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
        }
    }
}

#[test]
fn list_preserves_reactivation_disabled_rows_context_menus_and_item_identity() {
    let mut h = Harness::new();
    h.frame(["a", "b", "disabled"], vec![]);
    let rows = h.frame(["a", "b", "disabled"], vec![]);
    assert_ne!(rows[0].id, rows[1].id);
    let reordered = h.frame(["b", "a", "disabled"], vec![]);
    assert_eq!(rows[0].id, reordered[1].id);
    assert_eq!(rows[1].id, reordered[0].id);
    let rows = h.frame(["a", "b", "disabled"], vec![]);
    for _ in 0..2 {
        h.click(rows[0].rect.center(), egui::PointerButton::Primary);
    }
    assert_eq!(h.clicked, ["a", "a"], "selected rows still emit clicks");
    h.click(rows[2].rect.center(), egui::PointerButton::Primary);
    assert_eq!(h.clicked.len(), 2, "disabled rows must not activate");
    h.click(rows[1].rect.center(), egui::PointerButton::Secondary);
    h.frame(["a", "b", "disabled"], vec![]);
    assert!(h.menu_open, "native responses must retain context menus");
    assert_eq!(
        h.clicked.len(),
        2,
        "a context menu must not activate the item"
    );
}

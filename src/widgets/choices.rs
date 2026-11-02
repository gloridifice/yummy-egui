/// A bounded horizontal exclusive button row, not a row of independent toggles.
pub fn choices<T: Copy + PartialEq>(ui: &mut egui::Ui, value: &mut T, items: &[(T, &str)]) {
    let gap = crate::CharmingStyle::get(ui.ctx()).choices.spacing;
    let width = ((ui.available_width() - gap * items.len().saturating_sub(1) as f32)
        / items.len().max(1) as f32)
        .max(1.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        for (choice, label) in items {
            if ui
                .add_sized(
                    [width, ui.spacing().interact_size.y],
                    egui::Button::selectable(*value == *choice, *label).wrap(),
                )
                .clicked()
            {
                *value = *choice;
            }
        }
    });
}

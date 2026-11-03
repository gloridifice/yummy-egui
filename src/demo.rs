//! Interactive component gallery. The host owns state, fonts and theme setup.
use crate::{
    IconButton, IconChoices, IconSegmentedControl, PanelButton, ParameterList, ParameterSlider,
    SelectionList,
};

/// Persistent interaction state for the component gallery.
#[derive(Clone, Debug)]
pub struct DemoState {
    gain: f32,
    count: i32,
    frequency: f32,
    mode: usize,
    selected: usize,
    icon_mode: usize,
    clicks: usize,
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            gain: 0.5,
            count: 8,
            frequency: 440.0,
            mode: 0,
            selected: 0,
            icon_mode: 0,
            clicks: 0,
        }
    }
}

/// Show a closable gallery window. Apply a theme once in the host, not per frame.
pub fn demo_window(ctx: &egui::Context, open: &mut bool, state: &mut DemoState) {
    egui::Window::new("Yummy egui demo")
        .id(egui::Id::new("yummy_egui demo"))
        .open(open)
        .default_width(480.0)
        .default_pos(egui::pos2(20.0, 50.0))
        .vscroll(true)
        .show(ctx, |ui| demo_ui(ui, state));
}

/// Draw the gallery in a host-provided UI; use a distinct UI ID for each instance.
pub fn demo_ui(ui: &mut egui::Ui, state: &mut DemoState) {
    crate::h1(ui, "Yummy egui");
    ui.label("Interactive widgets using the current context theme.");
    crate::h2(ui, "Headings");
    crate::h3(ui, "Third-level heading");
    crate::h2(ui, "Parameter list");
    ParameterList::new("demo parameters").show(ui, |rows| {
        rows.row("gain", "Gain", "Floating-point slider", |ui| {
            ui.add(ParameterSlider::new(&mut state.gain, 0.0..=1.0).max_decimals(2));
        });
        rows.row("count", "Count", "Integer slider with steps", |ui| {
            ui.add(ParameterSlider::new(&mut state.count, 0..=32).step_by(1.0));
        });
        rows.row("frequency", "Frequency", "Logarithmic slider", |ui| {
            ui.add(
                ParameterSlider::new(&mut state.frequency, 20.0..=20000.0)
                    .logarithmic(true)
                    .suffix(" Hz")
                    .max_decimals(0),
            );
        });
    });
    crate::group(ui, "Grouped parameters", |ui| {
        crate::parameter(ui, "Mode", "Exclusive choices", |ui| {
            crate::choices(
                ui,
                &mut state.mode,
                &[(0, "Low"), (1, "Medium"), (2, "High")],
            );
        });
    });
    crate::group(ui, "Segmented control", |ui| {
        crate::parameter(ui, "Mode", "Connected exclusive choices", |ui| {
            crate::segmented_control(
                ui,
                &mut state.mode,
                &[(0, "Low"), (1, "Medium"), (2, "High")],
            );
        });
        crate::parameter(ui, "Disabled", "Disabled connected choices", |ui| {
            ui.add_enabled_ui(false, |ui| {
                crate::segmented_control(
                    ui,
                    &mut state.mode,
                    &[(0, "Low"), (1, "Medium"), (2, "High")],
                );
            });
        });
    });
    crate::parameter_table(ui, "demo table", |ui| {
        crate::parameter(ui, "Disabled", "Disabled slider appearance", |ui| {
            ui.add_enabled(false, ParameterSlider::new(&mut state.gain, 0.0..=1.0));
        });
    });
    crate::h2(ui, "Icon buttons");
    ui.horizontal(|ui| {
        if ui
            .add(IconButton::new("+").label("Count a click"))
            .clicked()
        {
            state.clicks += 1;
        }
        ui.add(
            IconButton::new("★")
                .size(40.0)
                .icon_size(24.0)
                .label("Size override"),
        );
        ui.add(IconButton::new("×").enabled(false).label("Disabled action"));
    });
    crate::group(ui, "Icon selectors", |ui| {
        crate::parameter(ui, "Native", "Separate native selectable icons", |ui| {
            ui.add(IconChoices::new(
                &mut state.icon_mode,
                [(0, "◀"), (1, "■"), (2, "▶")],
            ));
        });
        crate::parameter(ui, "Segmented", "Connected exclusive icons", |ui| {
            ui.add(IconSegmentedControl::new(
                &mut state.icon_mode,
                [(0, "◀"), (1, "■"), (2, "▶")],
            ));
        });
        crate::parameter(
            ui,
            "Vertical",
            "Top-to-bottom icon choices and segments",
            |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        IconChoices::new(&mut state.icon_mode, [(0, "◀"), (1, "■"), (2, "▶")])
                            .vertical(),
                    );
                    ui.add(
                        IconSegmentedControl::new(
                            &mut state.icon_mode,
                            [(0, "◀"), (1, "■"), (2, "▶")],
                        )
                        .vertical(),
                    );
                });
            },
        );
        crate::parameter(ui, "Disabled", "Disabled connected icons", |ui| {
            ui.add_enabled(
                false,
                IconSegmentedControl::new(&mut state.icon_mode, [(0, "◀"), (1, "■"), (2, "▶")]),
            );
        });
    });
    crate::h2(ui, "Selection list");
    SelectionList::new("demo selection")
        .max_height(140.0)
        .show(ui, |rows| {
            for (i, name) in [
                "First item",
                "Second item",
                "A longer item label that can wrap in a narrow window",
            ]
            .iter()
            .enumerate()
            {
                if rows.item(i, state.selected == i, *name).clicked() {
                    state.selected = i;
                }
            }
            rows.item_enabled("disabled", false, false, "Unavailable item");
        });
    ui.label(format!("Selected item: {}", state.selected + 1));
    crate::h2(ui, "Panel actions");
    if ui.add(PanelButton::new("Count a click")).clicked() {
        state.clicks += 1;
    }
    ui.label(format!("Clicks: {}", state.clicks));
    let [reset, _] = crate::action_row(
        ui,
        [
            PanelButton::new("Reset demo"),
            PanelButton::new("Disabled action").enabled(false),
        ],
    );
    if reset.clicked() {
        *state = DemoState::default();
    }
}

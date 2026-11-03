//! In-memory style editing built from yummy_egui components.
use super::{Theme, YummyStyle};
use crate::{PanelButton, ParameterSlider, group, parameter, segmented_control};
use egui::{Color32, Ui};

/// Edit the current context's styles using the library's own components.
/// Includes all custom style fields and common egui layout, font and visual
/// settings. Changes are live and are not persisted. Dock styles are not edited.
pub fn style_editor_window(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("Yummy egui style editor")
        .id(egui::Id::new("yummy_egui style editor"))
        .open(open)
        .default_width(480.0)
        .default_pos(egui::pos2(540.0, 50.0))
        .vscroll(true)
        .show(ctx, |ui| {
            crate::h1(ui, "Style editor");
            if ui.add(PanelButton::new("Reset to defaults")).clicked() {
                Theme::default().apply(ctx);
            }
            let page_id = ui.id().with("editor page");
            let mut page = ctx
                .data(|data| data.get_temp::<usize>(page_id))
                .unwrap_or(0);
            segmented_control(
                ui,
                &mut page,
                &[(0, "Components"), (1, "Layout"), (2, "Colors")],
            );
            ctx.data_mut(|data| data.insert_temp(page_id, page));
            let mut custom = YummyStyle::get(ctx).as_ref().clone();
            let before = custom.clone();
            let mut base = ctx.global_style().as_ref().clone();
            let base_before = base.clone();
            match page {
                0 => custom_editor(ui, &mut custom),
                1 => layout_editor(ui, &mut base),
                _ => visuals_editor(ui, &mut base),
            }
            if custom != before {
                match custom.validate() {
                    Ok(()) => custom.install(ctx),
                    Err(error) => {
                        crate::h3(ui, format!("Invalid style: {error}"));
                    }
                }
            }
            if base != base_before {
                ctx.set_global_style(base);
            }
        });
}

// These helpers are used only inside a shared parameter grid.
fn number<T: egui::emath::Numeric>(
    ui: &mut Ui,
    label: &str,
    value: &mut T,
    range: std::ops::RangeInclusive<T>,
) {
    parameter(ui, label, "", |ui| {
        ui.add(ParameterSlider::new(value, range));
    });
}

fn toggle(ui: &mut Ui, label: &str, value: &mut bool) {
    parameter(ui, label, "", |ui| {
        segmented_control(ui, value, &[(false, "Off"), (true, "On")])
    });
}

fn optional(ui: &mut Ui, label: &str, value: &mut Option<f32>) {
    let mut enabled = value.is_some();
    parameter(
        ui,
        label,
        "Inherit the base style, or use a custom value",
        |ui| {
            segmented_control(ui, &mut enabled, &[(false, "Inherit"), (true, "Override")]);
        },
    );
    if enabled {
        let value = value.get_or_insert(0.0);
        number(ui, &format!("{label} value"), value, 0.0..=100.0);
    } else {
        *value = None;
    }
}

fn color(ui: &mut Ui, title: &str, value: &mut Color32, opaque: bool) {
    group(ui, title, |ui| {
        parameter(ui, "Color", "Click to open the color picker", |ui| {
            if opaque {
                let mut rgb = [value.r(), value.g(), value.b()];
                if ui.color_edit_button_srgb(&mut rgb).changed() {
                    *value = Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
                }
            } else {
                ui.color_edit_button_srgba(value);
            }
        });
    });
}

fn custom_editor(ui: &mut Ui, style: &mut YummyStyle) {
    for (name, heading) in [
        ("H1", &mut style.headings.h1),
        ("H2", &mut style.headings.h2),
        ("H3", &mut style.headings.h3),
    ] {
        group(ui, name, |ui| {
            number(ui, "Font size", &mut heading.size, 1.0..=100.0);
            number(ui, "Top margin", &mut heading.margin_top, 0.0..=100.0);
            number(ui, "Bottom margin", &mut heading.margin_bottom, 0.0..=100.0);
        });
    }
    let s = &mut style.slider;
    group(ui, "Slider", |ui| {
        number(ui, "Inset", &mut s.inset, 0.0..=50.0);
        number(ui, "Outset", &mut s.outset, 0.0..=50.0);
        number(ui, "Marker width", &mut s.width, 0.1..=100.0);
        s.hover_width = s.hover_width.max(s.width);
        number(
            ui,
            "Hover marker width",
            &mut s.hover_width,
            s.width..=100.0,
        );
        number(ui, "Value editor width", &mut s.value_width, 1.0..=300.0);
        number(ui, "Animation (ms)", &mut s.animation_ms, 0.0..=2000.0);
        number(ui, "Edge padding", &mut s.edge_padding, 0.0..=100.0);
        optional(ui, "Background radius", &mut s.background_radius);
        optional(ui, "Marker radius", &mut s.marker_radius);
    });
    color(ui, "Slider background", &mut s.background, true);
    color(ui, "Slider marker", &mut s.marker, false);
    color(ui, "Slider hover marker", &mut s.hover_marker, false);
    group(ui, "Panel buttons", |ui| {
        let s = &mut style.panel_button;
        margin(ui, &mut s.margin);
        number(
            ui,
            "Minimum action width",
            &mut s.min_action_width,
            1.0..=500.0,
        );
    });
    group(ui, "Icon buttons", |ui| {
        number(ui, "Button size", &mut style.icon_button.size, 1.0..=100.0);
        number(
            ui,
            "Icon size",
            &mut style.icon_button.icon_size,
            1.0..=100.0,
        );
    });
    group(ui, "Parameter lists", |ui| {
        let s = &mut style.parameter_list;
        number(ui, "Name column ratio", &mut s.name_ratio, 0.01..=0.99);
        number(ui, "Column spacing", &mut s.column_spacing, 0.0..=100.0);
        number(ui, "Row spacing", &mut s.row_spacing, 0.0..=100.0);
        number(ui, "Group spacing", &mut s.group_spacing, 0.0..=100.0);
        toggle(ui, "Striped rows", &mut s.striped);
    });
    group(ui, "Selection lists", |ui| {
        toggle(ui, "Frame", &mut style.selection_list.frame);
        toggle(ui, "Wrap labels", &mut style.selection_list.wrap);
        optional(ui, "Row spacing", &mut style.selection_list.row_spacing);
    });
    group(ui, "Choices", |ui| {
        number(
            ui,
            "Button spacing",
            &mut style.choices.spacing,
            0.0..=100.0,
        )
    });
}

fn margin(ui: &mut Ui, margin: &mut egui::Margin) {
    for (name, value) in [
        ("Left", &mut margin.left),
        ("Right", &mut margin.right),
        ("Top", &mut margin.top),
        ("Bottom", &mut margin.bottom),
    ] {
        number(ui, name, value, 0..=127);
    }
}

fn layout_editor(ui: &mut Ui, style: &mut egui::Style) {
    group(ui, "Control dimensions", |ui| {
        let s = &mut style.spacing;
        number(ui, "Minimum width", &mut s.interact_size.x, 1.0..=300.0);
        number(ui, "Minimum height", &mut s.interact_size.y, 1.0..=100.0);
        number(
            ui,
            "Horizontal button padding",
            &mut s.button_padding.x,
            0.0..=50.0,
        );
        number(
            ui,
            "Vertical button padding",
            &mut s.button_padding.y,
            0.0..=50.0,
        );
        number(
            ui,
            "Horizontal item spacing",
            &mut s.item_spacing.x,
            0.0..=100.0,
        );
        number(
            ui,
            "Vertical item spacing",
            &mut s.item_spacing.y,
            0.0..=100.0,
        );
        number(ui, "Indent", &mut s.indent, 0.0..=100.0);
        number(ui, "Slider width", &mut s.slider_width, 1.0..=500.0);
        number(
            ui,
            "Slider rail height",
            &mut s.slider_rail_height,
            1.0..=100.0,
        );
        number(ui, "Combo width", &mut s.combo_width, 1.0..=500.0);
        number(ui, "Text edit width", &mut s.text_edit_width, 1.0..=500.0);
        number(
            ui,
            "Animation (seconds)",
            &mut style.animation_time,
            0.0..=2.0,
        );
    });
    group(ui, "Window / panel margins", |ui| {
        margin(ui, &mut style.spacing.window_margin)
    });
    group(ui, "Menu margins", |ui| {
        margin(ui, &mut style.spacing.menu_margin)
    });
    for (name, font) in &mut style.text_styles {
        group(ui, &format!("Font: {name:?}"), |ui| {
            number(ui, "Size", &mut font.size, 1.0..=100.0);
            parameter(
                ui,
                "Family",
                "Named host fonts are preserved until a built-in family is selected",
                |ui| {
                    let mut family = match font.family {
                        egui::FontFamily::Proportional => 0,
                        egui::FontFamily::Monospace => 1,
                        egui::FontFamily::Name(_) => 2,
                    };
                    let before = family;
                    segmented_control(ui, &mut family, &[(0, "Proportional"), (1, "Monospace")]);
                    if family != before {
                        font.family = if family == 0 {
                            egui::FontFamily::Proportional
                        } else {
                            egui::FontFamily::Monospace
                        };
                    }
                },
            );
        });
    }
}

fn visuals_editor(ui: &mut Ui, style: &mut egui::Style) {
    let v = &mut style.visuals;
    group(ui, "Visual options", |ui| {
        toggle(ui, "Dark mode hint", &mut v.dark_mode);
        toggle(ui, "Button frames", &mut v.button_frame);
        toggle(
            ui,
            "Collapsing header frames",
            &mut v.collapsing_header_frame,
        );
        toggle(ui, "Striped backgrounds", &mut v.striped);
    });
    color(ui, "Window fill", &mut v.window_fill, false);
    color(ui, "Panel fill", &mut v.panel_fill, false);
    color(ui, "Extreme background", &mut v.extreme_bg_color, false);
    color(ui, "Faint background", &mut v.faint_bg_color, false);
    color(ui, "Selection fill", &mut v.selection.bg_fill, false);
    color(ui, "Selection stroke", &mut v.selection.stroke.color, false);
    group(ui, "Selection outline", |ui| {
        number(ui, "Width", &mut v.selection.stroke.width, 0.0..=10.0)
    });
    for (name, widget) in [
        ("Noninteractive", &mut v.widgets.noninteractive),
        ("Inactive", &mut v.widgets.inactive),
        ("Hovered", &mut v.widgets.hovered),
        ("Active", &mut v.widgets.active),
        ("Open", &mut v.widgets.open),
    ] {
        ui.push_id(name, |ui| {
            crate::h2(ui, name);
            group(ui, "Geometry", |ui| {
                number(
                    ui,
                    "Background stroke width",
                    &mut widget.bg_stroke.width,
                    0.0..=10.0,
                );
                number(
                    ui,
                    "Foreground stroke width",
                    &mut widget.fg_stroke.width,
                    0.0..=10.0,
                );
                number(ui, "Expansion", &mut widget.expansion, 0.0..=20.0);
                number(ui, "Top left radius", &mut widget.corner_radius.nw, 0..=100);
                number(
                    ui,
                    "Top right radius",
                    &mut widget.corner_radius.ne,
                    0..=100,
                );
                number(
                    ui,
                    "Bottom left radius",
                    &mut widget.corner_radius.sw,
                    0..=100,
                );
                number(
                    ui,
                    "Bottom right radius",
                    &mut widget.corner_radius.se,
                    0..=100,
                );
            });
            color(ui, "Background fill", &mut widget.bg_fill, false);
            color(ui, "Weak background fill", &mut widget.weak_bg_fill, false);
            color(ui, "Background stroke", &mut widget.bg_stroke.color, false);
            color(ui, "Foreground stroke", &mut widget.fg_stroke.color, false);
        });
    }
}

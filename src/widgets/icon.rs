//! Fixed-size icon actions and exclusive selectors with native button input.
use egui::{AtomKind, Response, Ui, Vec2, Widget};

#[derive(Clone, Copy, Default)]
struct IconSizing {
    size: Option<f32>,
    icon_size: Option<f32>,
}

impl IconSizing {
    fn resolve(self, ui: &Ui) -> (f32, f32) {
        let style = crate::YummyStyle::get(ui.ctx());
        let size = self.size.unwrap_or(style.icon_button.size);
        let icon_size = self.icon_size.unwrap_or(style.icon_button.icon_size);
        (size, icon_size.min(size))
    }

    fn size(&mut self, size: f32) {
        assert!(
            size.is_finite() && size > 0.0,
            "icon button size must be finite and positive"
        );
        self.size = Some(size);
    }

    fn icon_size(&mut self, size: f32) {
        assert!(
            size.is_finite() && size > 0.0,
            "icon size must be finite and positive"
        );
        self.icon_size = Some(size);
    }
}

/// A square icon-only action button. Accepts a glyph, [`egui::RichText`] (for
/// host icon fonts), or [`egui::Image`]. It never stretches to the parent width.
/// Fonts and image loaders remain the host's responsibility.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// if ui.add(yummy_egui::IconButton::new("+").size(32.0).label("Add")).clicked() {
///     // Perform the action.
/// }
/// # });
/// ```
#[must_use = "Add the icon button to a Ui"]
pub struct IconButton<'a> {
    icon: AtomKind<'a>,
    sizing: IconSizing,
    enabled: bool,
    label: Option<String>,
}

impl<'a> IconButton<'a> {
    /// Create a fixed-size square icon button from a glyph, [`egui::RichText`], or [`egui::Image`].
    pub fn new(icon: impl Into<AtomKind<'a>>) -> Self {
        Self {
            icon: icon.into(),
            sizing: IconSizing::default(),
            enabled: true,
            label: None,
        }
    }

    /// Override the theme's square button side length in logical points.
    /// Panics unless finite and positive.
    pub fn size(mut self, size: f32) -> Self {
        self.sizing.size(size);
        self
    }

    /// Override the theme's glyph font size / image bounding-box side length.
    /// Clamped to the button size. Panics unless finite and positive.
    pub fn icon_size(mut self, size: f32) -> Self {
        self.sizing.icon_size(size);
        self
    }

    /// Enable or disable the button (combined with the parent UI's enabled state).
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set an accessible name and hover tooltip without adding visible text.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Widget for IconButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (size, icon_size) = self.sizing.resolve(ui);
        let response = allocate_fixed(ui, Vec2::splat(size));
        self.show_at(ui, response.rect, icon_size, None, None)
    }
}

/// Separate square icons using native egui selectable-button styling.
/// Values should be unique. Clicking the current value does not report a change.
/// An empty iterator draws nothing; an unmatched value selects nothing.
/// Defaults to horizontal; use `.vertical()` for a top-to-bottom column.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let mut direction = 0;
/// ui.add(yummy_egui::IconChoices::new(&mut direction, [(0, "←"), (1, "→")]).size(32.0));
/// # });
/// ```
#[must_use = "Add the icon choices to a Ui"]
pub struct IconChoices<'a, T> {
    selector: IconSelector<'a, T>,
}

impl<'a, T: Copy + PartialEq> IconChoices<'a, T> {
    /// Stack buttons top to bottom instead of the default left to right.
    pub fn vertical(mut self) -> Self {
        self.selector.vertical = true;
        self
    }

    /// Create an exclusive icon selector bound to `value` with `(choice, icon)` pairs.
    pub fn new<I: Into<AtomKind<'a>>>(
        value: &'a mut T,
        items: impl IntoIterator<Item = (T, I)>,
    ) -> Self {
        Self {
            selector: IconSelector::new(value, items),
        }
    }

    /// Override each square button's side length; must be finite and positive.
    pub fn size(mut self, size: f32) -> Self {
        self.selector.sizing.size(size);
        self
    }

    /// Override glyph / image size; must be finite and positive.
    pub fn icon_size(mut self, size: f32) -> Self {
        self.selector.sizing.icon_size(size);
        self
    }
}

impl<T: Copy + PartialEq> Widget for IconChoices<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.selector.show(ui, false)
    }
}

/// A compact, connected icon selector with the same track, hover and rounded
/// outer ends as [`super::segmented_control`]. Each segment is a fixed square;
/// the row does not fill the available width. Use `.vertical()` for a column.
/// Selection semantics match
/// [`IconChoices`], including empty, unmatched and disabled controls.
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// let mut direction = 0;
/// ui.add(yummy_egui::IconSegmentedControl::new(&mut direction, [(0, "←"), (1, "→")]));
/// # });
/// ```
#[must_use = "Add the icon segmented control to a Ui"]
pub struct IconSegmentedControl<'a, T> {
    selector: IconSelector<'a, T>,
}

impl<'a, T: Copy + PartialEq> IconSegmentedControl<'a, T> {
    /// Stack segments top to bottom, rounding only the top and bottom ends.
    pub fn vertical(mut self) -> Self {
        self.selector.vertical = true;
        self
    }

    /// Create a connected exclusive icon selector bound to `value` with `(choice, icon)` pairs.
    pub fn new<I: Into<AtomKind<'a>>>(
        value: &'a mut T,
        items: impl IntoIterator<Item = (T, I)>,
    ) -> Self {
        Self {
            selector: IconSelector::new(value, items),
        }
    }

    /// Override each square segment's side length; must be finite and positive.
    pub fn size(mut self, size: f32) -> Self {
        self.selector.sizing.size(size);
        self
    }

    /// Override glyph / image size; must be finite and positive.
    pub fn icon_size(mut self, size: f32) -> Self {
        self.selector.sizing.icon_size(size);
        self
    }
}

impl<T: Copy + PartialEq> Widget for IconSegmentedControl<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.selector.show(ui, true)
    }
}

struct IconSelector<'a, T> {
    value: &'a mut T,
    items: Vec<(T, AtomKind<'a>)>,
    sizing: IconSizing,
    vertical: bool,
}

impl<'a, T: Copy + PartialEq> IconSelector<'a, T> {
    fn new<I: Into<AtomKind<'a>>>(
        value: &'a mut T,
        items: impl IntoIterator<Item = (T, I)>,
    ) -> Self {
        Self {
            value,
            items: items
                .into_iter()
                .map(|(value, icon)| (value, icon.into()))
                .collect(),
            sizing: IconSizing::default(),
            vertical: false,
        }
    }

    fn show(self, ui: &mut Ui, connected: bool) -> Response {
        let count = self.items.len();
        if count == 0 {
            return allocate_fixed(ui, Vec2::ZERO);
        }
        let (size, icon_size) = self.sizing.resolve(ui);
        let along = |main, cross| {
            if self.vertical {
                egui::vec2(cross, main)
            } else {
                egui::vec2(main, cross)
            }
        };
        let original = *self.value;
        let mut items = self.items.into_iter();
        let mut draw = |ui: &mut Ui, rect, corners| {
            let (choice, icon) = items.next().unwrap();
            let response = IconButton::new(icon).show_at(
                ui,
                rect,
                icon_size,
                Some(original == choice),
                corners,
            );
            if response.clicked() {
                *self.value = choice;
            }
            response
        };
        let mut response = if connected {
            super::segmented::segmented_row(
                ui,
                along(size * count as f32, size),
                count,
                self.vertical,
                |ui, _, rect, corners| draw(ui, rect, Some(corners)),
            )
        } else {
            let gap = crate::YummyStyle::get(ui.ctx()).choices.spacing;
            let length = size * count as f32 + gap * (count - 1) as f32;
            let mut response = allocate_fixed(ui, along(length, size));
            let rect = response.rect;
            for index in 0..count {
                let button = egui::Rect::from_min_size(
                    rect.min + along(index as f32 * (size + gap), 0.0),
                    Vec2::splat(size),
                );
                response |= draw(ui, button, None);
            }
            response
        };
        if *self.value != original {
            response.mark_changed();
        }
        response
    }
}

fn allocate_fixed(ui: &mut Ui, size: Vec2) -> Response {
    // allocate_exact_size's Response still stretches in justified layouts.
    let (id, outer) = ui.allocate_space(size);
    let rect = ui.layout().align_size_within_rect(size, outer);
    ui.interact(rect, id, egui::Sense::hover())
}

// Keep native focus, keyboard, pointer and disabled behavior, but size the frame
// independently of glyph metrics, image dimensions and the parent's justification.
impl IconButton<'_> {
    fn show_at(
        self,
        ui: &mut Ui,
        rect: egui::Rect,
        icon_size: f32,
        selected: Option<bool>,
        corners: Option<egui::CornerRadius>,
    ) -> Response {
        let Self {
            icon,
            enabled,
            label,
            ..
        } = self;
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(egui::Layout::top_down_justified(egui::Align::Center)),
        );
        child.spacing_mut().button_padding = Vec2::ZERO;
        child.spacing_mut().interact_size = Vec2::ZERO;
        child.add_enabled(enabled, |ui: &mut Ui| {
            let mut button = match selected {
                Some(selected) => egui::Button::selectable(selected, ()),
                None => egui::Button::new(()),
            }
            .min_size(rect.size());
            if let Some(corners) = corners {
                button = button
                    .corner_radius(corners)
                    .stroke(egui::Stroke::NONE)
                    .frame(true);
            }
            let response = ui.add(button);
            ui.set_clip_rect(ui.clip_rect().intersect(rect));
            let color = ui
                .style()
                .button_style(response.widget_state(), selected.unwrap_or(false))
                .text_style
                .color;
            let default_label = match &icon {
                AtomKind::Text(text) => text.text().to_owned(),
                _ => String::new(),
            };
            let icon = match icon {
                AtomKind::Text(text) => {
                    let mut job = std::sync::Arc::unwrap_or_clone(text.into_layout_job(
                        ui.style(),
                        egui::TextStyle::Button.into(),
                        egui::Align::Center,
                    ));
                    for section in &mut job.sections {
                        section.format.font_id.size = icon_size;
                    }
                    job.wrap = Default::default();
                    AtomKind::Text(job.into())
                }
                AtomKind::Image(image) => {
                    AtomKind::Image(image.fit_to_exact_size(Vec2::splat(icon_size)))
                }
                other => other,
            };
            let sized = icon
                .into_sized(
                    ui,
                    egui::IntoSizedArgs {
                        available_size: Vec2::splat(icon_size),
                        wrap_mode: egui::TextWrapMode::Extend,
                        fallback_font: egui::TextStyle::Button.into(),
                    },
                )
                .sized;
            match sized {
                egui::SizedAtomKind::Text(galley) => {
                    let pos = rect.center() - galley.rect.center().to_vec2();
                    ui.painter().galley(pos, galley, color);
                }
                egui::SizedAtomKind::Image { image, size } => {
                    image.paint_at(ui, egui::Rect::from_center_size(rect.center(), size));
                }
                egui::SizedAtomKind::Empty { .. } => {}
            }
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    label.as_deref().unwrap_or(&default_label),
                )
            });
            if let Some(label) = label {
                response.on_hover_text(label)
            } else {
                response
            }
        })
    }
}

#[cfg(test)]
mod tests;

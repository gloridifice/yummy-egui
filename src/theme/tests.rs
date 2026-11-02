use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "charming-egui-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn write(&self, name: &str, text: &str) {
        fs::write(self.0.join(name), text).unwrap();
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn external_documents_replace_independently_and_missing_files_use_embedded_defaults() {
    let dir = Directory::new();
    let default = Theme::load_dir(&dir.0).unwrap();
    // Migration contract: these are not egui's own defaults.
    assert_eq!(default.egui_style().spacing.interact_size.y, 22.0);
    assert_eq!(default.charming_style().parameter_list.name_ratio, 0.42);
    let mut egui = default.egui_style().clone();
    egui.spacing.interact_size.y = 31.0;
    dir.write("egui.toml", &toml::to_string(&egui).unwrap());
    let loaded = Theme::load_dir(&dir.0).unwrap();
    assert_eq!(loaded.egui_style().spacing.interact_size.y, 31.0);
    assert_eq!(loaded.charming_style().slider.width, 2.0);

    // Pre-heading external themes retain their custom values and get only the
    // newly added heading section from the bundled defaults.
    let mut legacy: toml::Value = toml::from_str(DEFAULT_CHARMING_EGUI).unwrap();
    legacy.as_table_mut().unwrap().remove("headings");
    legacy["parameter_list"]["name_ratio"] = toml::Value::Float(0.35);
    dir.write("charming_egui.toml", &toml::to_string(&legacy).unwrap());
    let loaded = Theme::load_dir(&dir.0).unwrap();
    assert_eq!(loaded.charming_style().parameter_list.name_ratio, 0.35);
    assert_eq!(loaded.charming_style().headings.h3.size, 14.0);

    // A partial file replaces, rather than merging over, the bundled document.
    // egui accepts defaults; Dock requires a complete serialized Style.
    dir.write("egui.toml", "animation_time = 0.75");
    let loaded = Theme::load_dir(&dir.0).unwrap();
    assert_eq!(loaded.egui_style().animation_time, 0.75);
    assert_eq!(
        loaded.egui_style().spacing.interact_size,
        egui::Style::default().spacing.interact_size
    );
    assert!(matches!(
        Theme::load_dir(dir.0.join("missing")),
        Err(ThemeError::Io { .. })
    ));
}

#[test]
fn dock_file_is_loaded_only_when_the_feature_is_enabled() {
    let dir = Directory::new();
    dir.write("egui_dock.toml", "not valid [ TOML");
    #[cfg(not(feature = "dock"))]
    assert!(Theme::load_dir(&dir.0).is_ok());
    #[cfg(feature = "dock")]
    {
        assert!(matches!(
            Theme::load_dir(&dir.0),
            Err(ThemeError::Parse {
                file: "egui_dock.toml",
                ..
            })
        ));
        let mut dock = Theme::default().dock_style().clone();
        dock.separator.width = 7.0;
        dir.write("egui_dock.toml", &toml::to_string(&dock).unwrap());
        assert_eq!(
            Theme::load_dir(&dir.0)
                .unwrap()
                .dock_style()
                .separator
                .width,
            7.0
        );
    }
}

#[test]
fn applying_external_component_styles_is_context_local_and_preserves_host_fonts() {
    let dir = Directory::new();
    let mut style = CharmingStyle::default();
    style.parameter_list.name_ratio = 0.35;
    style.slider.marker = egui::Color32::RED;
    dir.write("charming_egui.toml", &toml::to_string(&style).unwrap());
    let theme = Theme::load_dir(&dir.0).unwrap();
    let context = egui::Context::default();
    let other = egui::Context::default();
    let family = egui::FontFamily::Name("host font family".into());
    let mut fonts = egui::FontDefinitions::default();
    fonts.families.insert(
        family.clone(),
        fonts.families[&egui::FontFamily::Proportional].clone(),
    );
    context.set_fonts(fonts);
    theme.apply(&context);
    assert_eq!(CharmingStyle::get(&context).parameter_list.name_ratio, 0.35);
    assert_eq!(
        CharmingStyle::get(&context).slider.marker,
        egui::Color32::RED
    );
    assert_eq!(CharmingStyle::get(&other).parameter_list.name_ratio, 0.42);
    let _ = context.run_ui(egui::RawInput::default(), |ui| {
        assert!(ui.fonts(|fonts| fonts.definitions().families.contains_key(&family)));
    });

    // A failed reload returns no Theme, so the current applied theme is unchanged.
    dir.write("egui.toml", "spacing = false");
    assert!(matches!(
        Theme::load_dir(&dir.0),
        Err(ThemeError::Parse {
            file: "egui.toml",
            ..
        })
    ));
    assert_eq!(
        CharmingStyle::get(&context).slider.marker,
        egui::Color32::RED
    );
}

#[test]
fn malformed_or_unsafe_component_sizes_are_reported_instead_of_silently_falling_back() {
    for (section, invalid) in [
        ("parameter_list", "name_ratio = 1.0"),
        ("parameter_list", "column_spacing = -1.0"),
        ("slider", "animation_ms = nan"),
        ("headings.h1", "size = 0.0"),
        ("headings.h1", "margin_top = -1.0"),
        ("headings.h3", "margin_bottom = inf"),
        ("slider", "width = 0.0"),
        ("slider", "hover_width = 1.0"),
        ("slider", "background = [21, 21, 21, 0]"),
    ] {
        let mut document: toml::Value = toml::from_str(DEFAULT_CHARMING_EGUI).unwrap();
        let mut table = &mut document;
        for key in section.split('.') {
            table = table.get_mut(key).unwrap();
        }
        let values: toml::Table = toml::from_str(invalid).unwrap();
        table.as_table_mut().unwrap().extend(values);
        let text = toml::to_string(&document).unwrap();
        assert!(
            matches!(
                Theme::from_sources(ThemeSources {
                    charming_egui: Some(&text),
                    ..Default::default()
                }),
                Err(ThemeError::Invalid {
                    file: "charming_egui.toml",
                    ..
                })
            ),
            "accepted {section}.{invalid}"
        );
    }
    assert!(
        Theme::from_sources(ThemeSources {
            charming_egui: Some("[slider]\nunknown = true"),
            ..Default::default()
        })
        .is_err()
    );
    assert!(
        Theme::from_sources(ThemeSources {
            egui: Some("animation_time = inf"),
            ..Default::default()
        })
        .is_err()
    );
}

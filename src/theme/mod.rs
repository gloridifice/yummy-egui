//! Three independent TOML documents. External files replace complete documents;
//! absent files use embedded defaults. Parsing never changes a Context or fonts.
mod editor;
mod style;
pub use editor::style_editor_window;
pub use style::{
    CharmingStyle, ChoicesStyle, HeadingStyle, HeadingStyles, IconButtonStyle, PanelButtonStyle,
    ParameterListStyle, SelectionListStyle,
};

use serde::de::DeserializeOwned;
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub const DEFAULT_EGUI: &str = include_str!("default/egui.toml");
pub const DEFAULT_CHARMING_EGUI: &str = include_str!("default/yummy_egui.toml");
#[cfg(feature = "dock")]
pub const DEFAULT_EGUI_DOCK: &str = include_str!("default/egui_dock.toml");

/// Each absent source falls back to the corresponding embedded document.
#[derive(Default)]
pub struct ThemeSources<'a> {
    pub egui: Option<&'a str>,
    pub charming_egui: Option<&'a str>,
    #[cfg(feature = "dock")]
    pub egui_dock: Option<&'a str>,
}

#[derive(Clone, Debug)]
pub struct Theme {
    egui: egui::Style,
    charming: CharmingStyle,
    #[cfg(feature = "dock")]
    dock: egui_dock::Style,
}

#[derive(Debug)]
pub enum ThemeError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        file: &'static str,
        source: toml::de::Error,
    },
    Invalid {
        file: &'static str,
        message: String,
    },
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Parse { file, source } => write!(f, "{file}: {source}"),
            Self::Invalid { file, message } => write!(f, "{file}: {message}"),
        }
    }
}
impl std::error::Error for ThemeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            Self::Invalid { .. } => None,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        static THEME: OnceLock<Theme> = OnceLock::new();
        THEME
            .get_or_init(|| {
                Self::from_sources(ThemeSources::default())
                    .expect("embedded themes must match their schemas")
            })
            .clone()
    }
}

impl Theme {
    pub fn from_sources(sources: ThemeSources<'_>) -> Result<Self, ThemeError> {
        let charming: CharmingStyle = parse(
            "yummy_egui.toml",
            sources.charming_egui.unwrap_or(DEFAULT_CHARMING_EGUI),
        )?;
        charming.validate().map_err(|message| ThemeError::Invalid {
            file: "yummy_egui.toml",
            message,
        })?;
        Ok(Self {
            egui: parse("egui.toml", sources.egui.unwrap_or(DEFAULT_EGUI))?,
            charming,
            #[cfg(feature = "dock")]
            dock: parse(
                "egui_dock.toml",
                sources.egui_dock.unwrap_or(DEFAULT_EGUI_DOCK),
            )?,
        })
    }

    /// Explicitly read an existing directory. Missing documents use defaults;
    /// unreadable/invalid documents return an error. No automatic watching or writes.
    /// With `dock` disabled, egui_dock.toml is not opened at all.
    pub fn load_dir(path: impl AsRef<Path>) -> Result<Self, ThemeError> {
        let path = path.as_ref();
        let metadata = fs::metadata(path).map_err(|source| ThemeError::Io {
            path: path.into(),
            source,
        })?;
        if !metadata.is_dir() {
            return Err(ThemeError::Io {
                path: path.into(),
                source: io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "theme path is not a directory",
                ),
            });
        }
        let egui = read_optional(&path.join("egui.toml"))?;
        let charming = read_optional(&path.join("yummy_egui.toml"))?;
        #[cfg(feature = "dock")]
        let dock = read_optional(&path.join("egui_dock.toml"))?;
        Self::from_sources(ThemeSources {
            egui: egui.as_deref(),
            charming_egui: charming.as_deref(),
            #[cfg(feature = "dock")]
            egui_dock: dock.as_deref(),
        })
    }

    /// Apply the theme to this Context only. Does not install or replace fonts.
    /// DockArea owns its style separately; pass `dock_style().clone()` to it.
    pub fn apply(&self, context: &egui::Context) {
        context.set_theme(if self.egui.visuals.dark_mode {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        });
        context.set_global_style(self.egui.clone());
        self.charming.install(context);
    }

    pub fn egui_style(&self) -> &egui::Style {
        &self.egui
    }
    pub fn charming_style(&self) -> &CharmingStyle {
        &self.charming
    }
    #[cfg(feature = "dock")]
    pub fn dock_style(&self) -> &egui_dock::Style {
        &self.dock
    }
}

fn read_optional(path: &Path) -> Result<Option<String>, ThemeError> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(ThemeError::Io {
            path: path.into(),
            source,
        }),
    }
}

fn parse<T: DeserializeOwned>(file: &'static str, source: &str) -> Result<T, ThemeError> {
    // First deserialize with the original text so schema errors retain line/column.
    let typed = toml::from_str(source).map_err(|source| ThemeError::Parse { file, source })?;
    let values: toml::Value =
        toml::from_str(source).map_err(|source| ThemeError::Parse { file, source })?;
    if !finite(&values) {
        return Err(ThemeError::Invalid {
            file,
            message: "numeric values must be finite".into(),
        });
    }
    Ok(typed)
}

fn finite(value: &toml::Value) -> bool {
    match value {
        toml::Value::Float(v) => v.is_finite() && (*v as f32).is_finite(),
        toml::Value::Array(values) => values.iter().all(finite),
        toml::Value::Table(values) => values.values().all(finite),
        _ => true,
    }
}

#[cfg(test)]
mod tests;

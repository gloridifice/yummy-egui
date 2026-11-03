//! Three independent TOML documents. External files replace complete documents;
//! absent files use embedded defaults. Parsing never changes a Context or fonts.
mod editor;
mod style;
pub use editor::style_editor_window;
pub use style::{
    ChoicesStyle, HeadingStyle, HeadingStyles, IconButtonStyle, PanelButtonStyle,
    ParameterListStyle, SelectionListStyle, YummyStyle,
};

use serde::de::DeserializeOwned;
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
    sync::OnceLock,
};

/// Embedded `egui.toml` document for [`egui::Style`].
pub const DEFAULT_EGUI: &str = include_str!("default/egui.toml");
/// Embedded `yummy_egui.toml` document for [`YummyStyle`].
pub const DEFAULT_YUMMY_EGUI: &str = include_str!("default/yummy_egui.toml");
/// Embedded `egui_dock.toml` document for [`egui_dock::Style`].
#[cfg(feature = "dock")]
pub const DEFAULT_EGUI_DOCK: &str = include_str!("default/egui_dock.toml");

/// Optional in-memory TOML documents for [`Theme::from_sources`].
///
/// Each absent (`None`) source falls back to the corresponding embedded document;
/// each supplied (`Some`) source replaces that entire document.
#[derive(Default)]
pub struct ThemeSources<'a> {
    /// Complete `egui.toml` text for [`egui::Style`], or `None` for [`DEFAULT_EGUI`].
    pub egui: Option<&'a str>,
    /// Complete `yummy_egui.toml` text for [`YummyStyle`], or `None` for [`DEFAULT_YUMMY_EGUI`].
    pub yummy_egui: Option<&'a str>,
    /// Complete `egui_dock.toml` text for [`egui_dock::Style`], or `None` for [`DEFAULT_EGUI_DOCK`].
    #[cfg(feature = "dock")]
    pub egui_dock: Option<&'a str>,
}

/// Parsed and validated theme containing [`egui::Style`], [`YummyStyle`], and
/// (with the `dock` feature) [`egui_dock::Style`].
///
/// Constructing or loading a `Theme` does not mutate any [`egui::Context`]; call
/// [`Theme::apply`] to install it into a specific context.
#[derive(Clone, Debug)]
pub struct Theme {
    egui: egui::Style,
    yummy: YummyStyle,
    #[cfg(feature = "dock")]
    dock: egui_dock::Style,
}

/// Error returned when reading, parsing, or validating a theme document fails.
#[derive(Debug)]
pub enum ThemeError {
    /// Failed to inspect the theme directory or read a theme file from disk.
    Io {
        /// Failing directory or file path.
        path: PathBuf,
        /// Underlying filesystem I/O error.
        source: io::Error,
    },
    /// A theme document is not valid TOML or does not match its serde schema.
    Parse {
        /// Logical document name (`"egui.toml"`, `"yummy_egui.toml"`, or `"egui_dock.toml"`).
        file: &'static str,
        /// Underlying TOML deserialization error with line/column diagnostics.
        source: toml::de::Error,
    },
    /// A parsed document contains non-finite numbers or violates [`YummyStyle::validate`].
    Invalid {
        /// Logical document name (`"egui.toml"`, `"yummy_egui.toml"`, or `"egui_dock.toml"`).
        file: &'static str,
        /// Human-readable validation failure reason.
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
    /// Parse and validate a theme from in-memory TOML strings.
    ///
    /// Any `None` field in [`ThemeSources`] uses the bundled default document.
    pub fn from_sources(sources: ThemeSources<'_>) -> Result<Self, ThemeError> {
        let yummy: YummyStyle = parse(
            "yummy_egui.toml",
            sources.yummy_egui.unwrap_or(DEFAULT_YUMMY_EGUI),
        )?;
        yummy.validate().map_err(|message| ThemeError::Invalid {
            file: "yummy_egui.toml",
            message,
        })?;
        Ok(Self {
            egui: parse("egui.toml", sources.egui.unwrap_or(DEFAULT_EGUI))?,
            yummy,
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
        let yummy = read_optional(&path.join("yummy_egui.toml"))?;
        #[cfg(feature = "dock")]
        let dock = read_optional(&path.join("egui_dock.toml"))?;
        Self::from_sources(ThemeSources {
            egui: egui.as_deref(),
            yummy_egui: yummy.as_deref(),
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
        self.yummy.install(context);
    }

    /// Borrow the parsed base [`egui::Style`].
    pub fn egui_style(&self) -> &egui::Style {
        &self.egui
    }

    /// Borrow the parsed custom [`YummyStyle`].
    pub fn yummy_style(&self) -> &YummyStyle {
        &self.yummy
    }

    /// Borrow the parsed [`egui_dock::Style`]. Pass a clone to `DockArea::style(...)`.
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

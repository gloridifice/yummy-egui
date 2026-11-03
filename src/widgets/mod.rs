//! Reusable widgets: parameter sliders, panel and icon buttons, exclusive
//! selectors, and H1–H3 headings.

mod button;
mod choices;
mod heading;
mod icon;
mod segmented;
mod slider;
pub use segmented::segmented_control;

pub use button::{PanelButton, action_row};
pub use choices::choices;
pub use heading::{h1, h2, h3};
pub use icon::{IconButton, IconChoices, IconSegmentedControl};
pub use slider::{ParameterSlider, SliderAppearance};

use serde::{Deserialize, Serialize};

use super::theme_z::ThemeZ;
use super::guitar_theme::GuitarTheme;
use super::shapes_theme::ShapesTheme;
use super::theme_colors::ThemeColors;
use super::theme_sizes::ThemeSizes;
use super::theme_texts::ThemeTexts;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct NotationTheme {
    pub _bypass_systems: bool,
    pub z: ThemeZ,
    pub sizes: ThemeSizes,
    pub colors: ThemeColors,
    pub texts: ThemeTexts,
    pub shapes: ShapesTheme,
    pub guitar: GuitarTheme,
}

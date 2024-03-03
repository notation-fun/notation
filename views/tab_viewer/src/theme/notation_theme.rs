use serde::{Deserialize, Serialize};
use edger_bevy::bevy_prelude::*;

use super::guitar_theme::GuitarTheme;
use super::shapes_theme::ShapesTheme;
use super::theme_colors::ThemeColors;
use super::theme_sizes::ThemeSizes;
use super::theme_texts::ThemeTexts;
use super::theme_z::ThemeZ;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default, Resource)]
pub struct NotationTheme {
    pub _bypass_systems: bool,
    pub z: ThemeZ,
    pub sizes: ThemeSizes,
    pub colors: ThemeColors,
    pub texts: ThemeTexts,
    pub shapes: ShapesTheme,
    pub guitar: GuitarTheme,
}

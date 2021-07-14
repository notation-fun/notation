use serde::{Deserialize, Serialize};

use super::core_theme::CoreTheme;
use super::fretted_theme::FrettedTheme;
use super::guitar_theme::GuitarTheme;
use super::syllable_theme::SyllableTheme;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct BevyTheme {
    pub core: CoreTheme,
    pub syllable: SyllableTheme,
    pub fretted: FrettedTheme,
    pub guitar: GuitarTheme,
}

impl Default for BevyTheme {
    fn default() -> Self {
        Self {
            core: CoreTheme::default(),
            syllable: SyllableTheme::default(),
            fretted: FrettedTheme::default(),
            guitar: GuitarTheme::default(),
        }
    }
}

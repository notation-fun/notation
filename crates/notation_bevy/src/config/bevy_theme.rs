use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::fretted_theme::FrettedTheme;
use super::guitar_theme::GuitarTheme;
use super::syllable_theme::SyllableTheme;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Inspectable)]
pub struct BevyTheme {
    pub background_color: Color,
    pub outline_color: Color,
    pub syllable: SyllableTheme,
    pub fretted: FrettedTheme,
    pub guitar: GuitarTheme,
}

impl Default for BevyTheme {
    fn default() -> Self {
        Self {
            background_color: Color::hex("FFF9F2").unwrap(),
            outline_color: Color::BLACK,
            syllable: SyllableTheme::default(),
            fretted: FrettedTheme::default(),
            guitar: GuitarTheme::default(),
        }
    }
}

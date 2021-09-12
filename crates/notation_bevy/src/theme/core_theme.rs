use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::prelude::ThemeColors;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct CoreTheme {
    pub background_color: Color,
    pub bar_separator_z: f32,
    pub beat_z: f32,
    pub bar_indicator_z: f32,
    pub pos_indicator_z: f32,
    pub mini_map_z: f32,
    pub mini_bar_z: f32,
}

impl Default for CoreTheme {
    fn default() -> Self {
        Self {
            background_color: ThemeColors::hex_linear("FFF9F2"),
            bar_separator_z: 2.0,
            beat_z: 0.0,
            bar_indicator_z: 19.0,
            pos_indicator_z: 20.0,
            mini_map_z: 22.0,
            mini_bar_z: 24.0,
        }
    }
}

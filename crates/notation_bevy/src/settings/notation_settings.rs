use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use super::layout_settings::LayoutSettings;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct NotationSettings {
    pub layout: LayoutSettings,
    pub play_speed: f32,
    pub always_show_fret: bool,
    pub melody_piano_mode: bool,
}

impl Default for NotationSettings {
    fn default() -> Self {
        Self {
            layout: LayoutSettings::default(),
            play_speed: 1.0,
            always_show_fret: false,
            melody_piano_mode: false,
        }
    }
}

use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use super::layout_settings::LayoutSettings;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct NotationSettings {
    pub layout: LayoutSettings,
    pub should_loop: bool,
    pub speed_factor: f32,
    pub always_show_fret: bool,
    pub melody_piano_mode: bool,
    pub mouse_dragged_panning: bool,
}

impl Default for NotationSettings {
    fn default() -> Self {
        Self {
            layout: LayoutSettings::default(),
            should_loop: false,
            speed_factor: 1.0,
            always_show_fret: false,
            melody_piano_mode: false,
            mouse_dragged_panning: false,
        }
    }
}

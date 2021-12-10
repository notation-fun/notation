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
    pub hide_bar_number: bool,
    pub always_show_fret: bool,
    pub melody_piano_mode: bool,
    pub allow_panning: bool,
    pub panning_line_size: f32,
    pub override_beat_size: Option<f32>,
    pub override_chord_size: Option<f32>,
}

impl Default for NotationSettings {
    fn default() -> Self {
        Self {
            layout: LayoutSettings::default(),
            should_loop: false,
            speed_factor: 1.0,
            hide_bar_number: false,
            always_show_fret: false,
            melody_piano_mode: false,
            allow_panning: true,
            panning_line_size: 32.0,
            override_beat_size: None,
            override_chord_size: None,
        }
    }
}

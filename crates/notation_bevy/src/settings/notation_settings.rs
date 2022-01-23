use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use super::layout_settings::LayoutSettings;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct NotationSettings {
    pub layout: LayoutSettings,
    pub add_ready_section: bool,
    pub should_loop: bool,
    pub speed_factor: f32,
    pub hide_bar_number: bool,
    pub hide_indicators: bool,
    pub show_guitar_syllable: bool,
    pub show_melody_pitch: bool,
    pub show_melody_syllable: bool,
    pub show_syllable_as_num: bool,
    pub always_show_fret: bool,
    pub melody_piano_mode: bool,
    pub allow_panning: bool,
    pub panning_line_size: f32,
    pub hide_guitar_view: bool,
    pub hide_chords_view: bool,
    pub hide_mini_map: bool,
    pub hide_shapes_lane: bool,
    pub hide_strings_lane: bool,
    pub hide_harmony_lane: bool,
    pub hide_lyrics_lane: bool,
    pub hide_melody_lane: bool,
    pub override_beat_size: Option<f32>,
    pub override_chord_size: Option<f32>,
    pub override_guitar_width: Option<f32>,
    pub override_guitar_y: Option<f32>,
}

impl Default for NotationSettings {
    fn default() -> Self {
        Self {
            layout: LayoutSettings::default(),
            add_ready_section: false,
            should_loop: false,
            speed_factor: 1.0,
            hide_bar_number: false,
            hide_indicators: false,
            show_guitar_syllable: true,
            show_melody_pitch: false,
            show_melody_syllable: true,
            show_syllable_as_num: true,
            always_show_fret: false,
            melody_piano_mode: false,
            allow_panning: true,
            panning_line_size: 32.0,
            hide_shapes_lane: false,
            hide_strings_lane: false,
            hide_harmony_lane: false,
            hide_lyrics_lane: false,
            hide_melody_lane: false,
            hide_guitar_view: false,
            hide_mini_map: false,
            hide_chords_view: false,
            override_beat_size: None,
            override_chord_size: None,
            override_guitar_width: None,
            override_guitar_y: None,
        }
    }
}

impl NotationSettings {
    pub fn show_melody_note(&self) -> bool {
        self.show_melody_pitch || self.show_melody_syllable
    }
    pub fn hide_all_lanes(&mut self) {
        self.hide_shapes_lane = true;
        self.hide_strings_lane = true;
        self.hide_harmony_lane = true;
        self.hide_lyrics_lane = true;
        self.hide_melody_lane = true;
    }
    pub fn hack_for_screenshot(&mut self) {
        self.add_ready_section = false;
        self.hide_indicators = true;
        self.hide_guitar_view = true;
        self.hide_chords_view = true;
        self.hide_mini_map = false;
        self.override_beat_size = Some(128.0);
        self.hide_all_lanes();
    }
}
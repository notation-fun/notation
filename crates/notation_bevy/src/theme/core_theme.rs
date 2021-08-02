use serde::{Deserialize, Serialize};

use bevy::prelude::*;

use notation_model::prelude::Signature;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct CoreTheme {
    pub background_color: Color,
    pub bar_separator_color: Color,
    pub bar_separator_z: f32,
    pub beat_color0: Option<Color>,
    pub beat_color1: Option<Color>,
    pub beat_color2: Option<Color>,
    pub beat_z: f32,
    pub pos_indicator_color: Color,
    pub pos_indicator_z: f32,
}

impl Default for CoreTheme {
    fn default() -> Self {
        Self {
            background_color: Color::hex("FFF9F2").unwrap(),
            bar_separator_color: Color::hex("D3B59C").unwrap(),
            bar_separator_z: 2.0,
            beat_color0: None,
            beat_color1: Some(Color::hex("00000010").unwrap()),
            beat_color2: None,
            beat_z: 0.0,
            pos_indicator_color: Color::hex("000000AA").unwrap(),
            pos_indicator_z: 20.0,
        }
    }
}

impl CoreTheme {
    pub fn get_beat_color(&self, signature: &Signature, beat: u8) -> Option<Color> {
        if beat == 0 {
            return self.beat_color0;
        }
        if signature.beats_per_bar % 4 == 0 {
            match beat % 4 {
                1 => self.beat_color1,
                2 => self.beat_color2,
                3 => self.beat_color1,
                _ => self.beat_color0,
            }
        } else if signature.beats_per_bar % 3 == 0 {
            match beat % 3 {
                1 => self.beat_color1,
                2 => self.beat_color2,
                _ => self.beat_color0,
            }
        } else {
            match beat % 2 {
                1 => self.beat_color1,
                _ => self.beat_color0,
            }
        }
    }
}

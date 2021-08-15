use notation_model::prelude::{Octave, PlayingState, Semitones, Syllable};
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct PlayingColors {
    pub idle: Color,
    pub current: Color,
    pub played: Color,
}
impl PlayingColors {
    pub fn new(idle: Color, current: Color, played: Color) -> Self {
        Self {
            idle,
            current,
            played,
        }
    }
    pub fn of_state(&self, state: &PlayingState) -> Color {
        match state {
            PlayingState::Idle => self.idle,
            PlayingState::Current => self.current,
            PlayingState::Played => self.played,
        }
    }
}

pub fn color_of_hex(hex: &str) -> Color {
    Color::hex(hex).unwrap()
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ThemeColors {
    pub syllables: [Color; 12],
    pub syllable_outline: PlayingColors,
    pub sections: [Color; 6],
    pub mini_map_back: Color,
    pub mini_bar_current_outline: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            syllable_outline: PlayingColors::new(
                color_of_hex("000000"),
                color_of_hex("CC00CC"),
                color_of_hex("FFF9F2"),
            ),
            syllables: [
                color_of_hex("E94F4F"), // Do
                color_of_hex("AAAAAA"), // Di, Ra
                color_of_hex("FFEB34"), // Re
                color_of_hex("AAAAAA"), // Ri, Me
                color_of_hex("59D7FF"), // Mi
                color_of_hex("C31F6E"), // Fa
                color_of_hex("AAAAAA"), // Fi, Se
                color_of_hex("FF8F28"), // So
                color_of_hex("AAAAAA"), // Si, Le
                color_of_hex("A3DC5B"), // La
                color_of_hex("AAAAAA"), // Li, Te
                color_of_hex("7C87E8"), // Ti
            ],
            sections: [
                color_of_hex("CC4444"),
                color_of_hex("44CC44"),
                color_of_hex("4444CC"),
                color_of_hex("CCCC44"),
                color_of_hex("44CCCC"),
                color_of_hex("CC44CC"),
            ],
            mini_map_back: color_of_hex("44444444"),
            mini_bar_current_outline: color_of_hex("CC00CC"),
        }
    }
}

impl ThemeColors {
    pub fn color_of_semitones(&self, v: Semitones) -> Color {
        let pos_val = if v.0 >= 0 { v.0 % 12 } else { v.0 % 12 + 12 };
        match pos_val {
            0 => self.syllables[0],
            1 => self.syllables[1],
            2 => self.syllables[2],
            3 => self.syllables[3],
            4 => self.syllables[4],
            5 => self.syllables[5],
            6 => self.syllables[6],
            7 => self.syllables[7],
            8 => self.syllables[8],
            9 => self.syllables[9],
            10 => self.syllables[10],
            11 => self.syllables[11],
            _ => self.syllables[11],
        }
    }

    pub fn color_of_syllable(&self, v: Syllable) -> Color {
        self.color_of_semitones(Semitones::from(v))
    }

    pub fn color_of_syllable_octave(&self, v: Syllable, _o: Octave) -> Color {
        self.color_of_semitones(Semitones::from(v))
    }
}

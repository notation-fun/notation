use notation_model::prelude::{Octave, Semitones, Syllable};
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct SyllableTheme {
    pub colors: [Color; 12],
}

impl Default for SyllableTheme {
    fn default() -> Self {
        Self {
            colors: [
                Color::hex("E94F4F").unwrap(), // Do
                Color::hex("AAAAAA").unwrap(), // Di, Ra
                Color::hex("FFEB34").unwrap(), // Re
                Color::hex("AAAAAA").unwrap(), // Ri, Me
                Color::hex("59D7FF").unwrap(), // Mi
                Color::hex("C31F6E").unwrap(), // Fa
                Color::hex("AAAAAA").unwrap(), // Fi, Se
                Color::hex("FF8F28").unwrap(), // So
                Color::hex("AAAAAA").unwrap(), // Si, Le
                Color::hex("A3DC5B").unwrap(), // La
                Color::hex("AAAAAA").unwrap(), // Li, Te
                Color::hex("7C87E8").unwrap(), // Ti
            ],
        }
    }
}

impl SyllableTheme {
    pub fn color_of_semitones(&self, v: Semitones) -> Color {
        let pos_val = if v.0 >= 0 { v.0 % 12 } else { v.0 % 12 + 12 };
        match pos_val {
            0 => self.colors[0],
            1 => self.colors[1],
            2 => self.colors[2],
            3 => self.colors[3],
            4 => self.colors[4],
            5 => self.colors[5],
            6 => self.colors[6],
            7 => self.colors[7],
            8 => self.colors[8],
            9 => self.colors[9],
            10 => self.colors[10],
            11 => self.colors[11],
            _ => self.colors[11],
        }
    }

    pub fn color_of_syllable(&self, v: Syllable) -> Color {
        self.color_of_semitones(Semitones::from(v))
    }

    pub fn color_of_syllable_octave(&self, v: Syllable, _o: Octave) -> Color {
        self.color_of_semitones(Semitones::from(v))
    }
}

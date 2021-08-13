use notation_model::prelude::{Octave, Semitones, Syllable};
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ThemeColors {
    pub syllables: [Color; 12],
    pub sections: [Color; 6],
    pub mini_map_back: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            syllables: [
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
            sections: [
                Color::hex("AA8888").unwrap(),
                Color::hex("88AA88").unwrap(),
                Color::hex("8888AA").unwrap(),
                Color::hex("AA88AA").unwrap(),
                Color::hex("88AAAA").unwrap(),
                Color::hex("AAAA88").unwrap(),
            ],
            mini_map_back: Color::hex("44444444").unwrap(),
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

use notation_core::prelude::{Semitones, Syllable, Octave, Solfege};
use serde::{Serialize, Deserialize};

use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SyllableTheme {
    pub colors: [Color; 12],
}

impl Default for SyllableTheme {
    fn default() -> Self {
        Self {
            colors: [
                Color::hex("E94F4F").unwrap(), // Do
                Color::hex("333333").unwrap(), // Di, Ra
                Color::hex("FFEB34").unwrap(), // Re
                Color::hex("333333").unwrap(), // Ri, Me
                Color::hex("59D7FF").unwrap(), // Mi
                Color::hex("C31F6E").unwrap(), // Fa
                Color::hex("333333").unwrap(), // Fi, Se
                Color::hex("FF8F28").unwrap(), // So
                Color::hex("333333").unwrap(), // Si, Le
                Color::hex("A3DC5B").unwrap(), // La
                Color::hex("333333").unwrap(), // Li, Te
                Color::hex("A3DC5B").unwrap(), // Ti
            ],
        }
    }
}

impl SyllableTheme {
    pub fn from_semitones(&self, v: Semitones) -> Color {
        let pos_val = if v.0 > 0 {v.0 % 12 } else {v.0 % 12 + 12};
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

    pub fn from_syllable(&self, v: Syllable) -> Color {
        self.from_semitones(Semitones::from(v))
    }

    pub fn from_syllable_octave(&self, v: Syllable, o: Octave) -> Color {
        self.from_semitones(Semitones::from(v) + Semitones::from(o))
    }

    pub fn from_solfege(&self, v: Solfege) -> Color {
        self.from_semitones(Semitones::from(v))
    }
}


#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Theme {
    pub background_color: Color,
    pub outline_color: Color,
    pub syllable: SyllableTheme,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background_color: Color::hex("D3B59C").unwrap(),
            outline_color: Color::BLACK,
            syllable: SyllableTheme::default(),
        }
    }
}
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::{Key, Note, Pitch, Semitones, Syllable, SyllableNote};

// https://hellomusictheory.com/learn/music-scales-beginners-guide/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Scale {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}
impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for Scale {
    fn default() -> Self {
        Self::Ionian
    }
}
impl Scale {
    #[allow(non_upper_case_globals)]
    pub const Major: Scale = Scale::Ionian;
    #[allow(non_upper_case_globals)]
    pub const Minor: Scale = Scale::Aeolian;

    pub fn to_ident(&self) -> String {
        format!("{}", self)
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "Major" => Self::Major,
            "Minor" => Self::Minor,
            "Ionian" => Self::Ionian,
            "Dorian" => Self::Dorian,
            "Phrygian" => Self::Phrygian,
            "Lydian" => Self::Lydian,
            "Mixolydian" => Self::Mixolydian,
            "Aeolian" => Self::Aeolian,
            "Locrian" => Self::Locrian,
            _ => Self::default(),
        }
    }
}

impl Scale {
    pub fn calc_do_semitones(&self, key: &Key) -> Semitones {
        let semitones = Semitones::from(*key).0
            + match self {
                Scale::Ionian => 0,
                Scale::Dorian => -2,
                Scale::Phrygian => -4,
                Scale::Lydian => -5,
                Scale::Mixolydian => 5,
                Scale::Aeolian => 3,
                Scale::Locrian => 1,
            };
        Semitones(semitones)
    }
    pub fn calc_syllable(&self, key: &Key, pitch: &Pitch) -> Syllable {
        (Semitones::from(*pitch) - self.calc_do_semitones(key)).into()
    }
    pub fn calc_pitch(&self, key: &Key, syllable: &Syllable) -> Pitch {
        (Semitones::from(*syllable) + self.calc_do_semitones(key)).into()
    }
    pub fn calc_syllable_note(&self, key: &Key, note: &Note) -> SyllableNote {
        (Semitones::from(*note) - self.calc_do_semitones(key)).into()
    }
    pub fn calc_note(&self, key: &Key, syllable_note: &SyllableNote) -> Note {
        (Semitones::from(*syllable_note) + self.calc_do_semitones(key)).into()
    }
}

use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::{Key, Note, Pitch, Semitones, Syllable, SyllableNote};

// https://hellomusictheory.com/learn/music-scales-beginners-guide/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Scale {
    Major,
    Minor,
    Dorian,
}
impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for Scale {
    fn default() -> Self {
        Self::Major
    }
}
impl Scale {
    pub fn to_ident(&self) -> String {
        format!("{}", self)
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "Major" => Self::Major,
            "Minor" => Self::Minor,
            "Dorian" => Self::Dorian,
            _ => Self::default(),
        }
    }
}

impl Scale {
    pub fn calc_do_semitones(&self, key: &Key) -> Semitones {
        let mut semitones = Semitones::from(*key).0
            + match self {
                Scale::Major => 0,
                Scale::Minor => 3,
                Scale::Dorian => -2,
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

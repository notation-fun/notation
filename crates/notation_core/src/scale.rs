use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::{Pitch, PitchName, Note, Semitones, Syllable, SyllableNote};

// https://hellomusictheory.com/learn/music-scales-beginners-guide/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Scale {
    Major,
    Minor,
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
            _ => Self::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Key {
    Natural(PitchName),
    Sharp(PitchName),
    Flat(PitchName),
}
impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Natural(p) => write!(f, "{}", p),
            Key::Sharp(p) => write!(f, "#{}", p),
            Key::Flat(p) => write!(f, "b{}", p),
        }
    }
}
impl Default for Key {
    fn default() -> Self {
        Self::C
    }
}

impl Key {
    pub const A: Self = Self::Natural(PitchName::A);
    pub const B: Self = Self::Natural(PitchName::B);
    pub const C: Self = Self::Natural(PitchName::C);
    pub const D: Self = Self::Natural(PitchName::D);
    pub const E: Self = Self::Natural(PitchName::E);
    pub const F: Self = Self::Natural(PitchName::F);
    pub const G: Self = Self::Natural(PitchName::G);

    pub const A_SHARP: Self = Self::Sharp(PitchName::A);
    pub const C_SHARP: Self = Self::Sharp(PitchName::C);
    pub const D_SHARP: Self = Self::Sharp(PitchName::D);
    pub const F_SHARP: Self = Self::Sharp(PitchName::F);
    pub const G_SHARP: Self = Self::Sharp(PitchName::G);

    pub const A_FLAT: Self = Self::Flat(PitchName::A);
    pub const B_FLAT: Self = Self::Flat(PitchName::B);
    pub const D_FLAT: Self = Self::Flat(PitchName::D);
    pub const E_FLAT: Self = Self::Flat(PitchName::E);
    pub const G_FLAT: Self = Self::Flat(PitchName::G);
}

impl Key {
    pub fn to_text(&self) -> String {
        format!("{}", self)
    }
    pub fn from_text(text: &str) -> Self {
        match text {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            "G" => Self::G,
            "#A" => Self::A_SHARP,
            "#C" => Self::C_SHARP,
            "#D" => Self::D_SHARP,
            "#F" => Self::F_SHARP,
            "#G" => Self::G_SHARP,
            "bA" => Self::A_FLAT,
            "bB" => Self::B_FLAT,
            "bD" => Self::D_FLAT,
            "bE" => Self::E_FLAT,
            "bG" => Self::G_FLAT,
            _ => Self::default(),
        }
    }
}

impl Key {
    pub fn to_ident(&self) -> String {
        match self {
            Key::Natural(p) => format!("{}", p),
            Key::Sharp(p) => format!("{}_SHARP", p),
            Key::Flat(p) => format!("{}_FLAT", p),
        }
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            "G" => Self::G,
            "A_SHARP" => Self::A_SHARP,
            "C_SHARP" => Self::C_SHARP,
            "D_SHARP" => Self::D_SHARP,
            "F_SHARP" => Self::F_SHARP,
            "G_SHARP" => Self::G_SHARP,
            "A_FLAT" => Self::A_FLAT,
            "B_FLAT" => Self::B_FLAT,
            "D_FLAT" => Self::D_FLAT,
            "E_FLAT" => Self::E_FLAT,
            "G_FLAT" => Self::G_FLAT,
            _ => Self::default(),
        }
    }
}

impl From<Key> for Semitones {
    fn from(v: Key) -> Self {
        match v {
            Key::Natural(x) => x.into(),
            Key::Sharp(x) => Semitones(1) + x.into(),
            Key::Flat(x) => Semitones(-1) + x.into(),
        }
    }
}

impl Scale {
    pub fn calc_do_semitones(&self, key: &Key) -> Semitones {
        let mut semitones = Semitones::from(*key).0
            + match self {
                Scale::Major => 0,
                Scale::Minor => 3,
            };
        if semitones < 0 {
            semitones += 12
        }
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

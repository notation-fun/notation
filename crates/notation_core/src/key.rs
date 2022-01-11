use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::{PitchName, Semitones, Syllable};

// https://hellomusictheory.com/learn/music-scales-beginners-guide/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Key {
    Natural(PitchName),
    Sharp(PitchName),
    Flat(PitchName),
}
impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_text())
    }
}
impl Default for Key {
    fn default() -> Self {
        Self::C
    }
}

impl Key {
    pub const C: Self = Self::Natural(PitchName::C);
    pub const D: Self = Self::Natural(PitchName::D);
    pub const E: Self = Self::Natural(PitchName::E);
    pub const F: Self = Self::Natural(PitchName::F);
    pub const G: Self = Self::Natural(PitchName::G);
    pub const A: Self = Self::Natural(PitchName::A);
    pub const B: Self = Self::Natural(PitchName::B);

    pub const C_SHARP: Self = Self::Sharp(PitchName::C);
    pub const D_SHARP: Self = Self::Sharp(PitchName::D);
    pub const E_SHARP: Self = Self::Sharp(PitchName::E);
    pub const F_SHARP: Self = Self::Sharp(PitchName::F);
    pub const G_SHARP: Self = Self::Sharp(PitchName::G);
    pub const A_SHARP: Self = Self::Sharp(PitchName::A);

    pub const D_FLAT: Self = Self::Flat(PitchName::D);
    pub const E_FLAT: Self = Self::Flat(PitchName::E);
    pub const G_FLAT: Self = Self::Flat(PitchName::G);
    pub const A_FLAT: Self = Self::Flat(PitchName::A);
    pub const B_FLAT: Self = Self::Flat(PitchName::B);

    pub const ALL: [Key ; 18] = [
        Key::C, Key::C_SHARP, Key::D_FLAT,
        Key::D, Key::D_SHARP, Key::E_FLAT,
        Key::E, Key::E_SHARP,
        Key::F, Key::F_SHARP, Key::G_FLAT,
        Key::G, Key::G_SHARP, Key::A_FLAT,
        Key::A, Key::A_SHARP, Key::B_FLAT,
        Key::B,
    ];
}

impl Key {
    pub fn to_text(&self) -> String {
        match self {
            Key::Natural(p) => format!("{}", p),
            Key::Sharp(p) => format!("#{}", p),
            Key::Flat(p) => format!("b{}", p),
        }
    }
    pub fn from_text(text: &str) -> Self {
        match text {
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            "G" => Self::G,
            "A" => Self::A,
            "B" => Self::B,
            "#C" => Self::C_SHARP,
            "#D" => Self::D_SHARP,
            "#E" => Self::E_SHARP,
            "#F" => Self::F_SHARP,
            "#G" => Self::G_SHARP,
            "#A" => Self::A_SHARP,
            "bD" => Self::D_FLAT,
            "bE" => Self::E_FLAT,
            "bG" => Self::G_FLAT,
            "bA" => Self::A_FLAT,
            "bB" => Self::B_FLAT,
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
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            "G" => Self::G,
            "A" => Self::A,
            "B" => Self::B,
            "C_SHARP" => Self::C_SHARP,
            "D_SHARP" => Self::D_SHARP,
            "E_SHARP" => Self::E_SHARP,
            "F_SHARP" => Self::F_SHARP,
            "G_SHARP" => Self::G_SHARP,
            "A_SHARP" => Self::A_SHARP,
            "D_FLAT" => Self::D_FLAT,
            "E_FLAT" => Self::E_FLAT,
            "G_FLAT" => Self::G_FLAT,
            "A_FLAT" => Self::A_FLAT,
            "B_FLAT" => Self::B_FLAT,
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

impl From<Syllable> for Key {
    fn from(v: Syllable) -> Self {
        match v {
            //Natural
            Syllable::Do => Self::C,
            Syllable::Re => Self::D,
            Syllable::Mi => Self::E,
            Syllable::Fa => Self::F,
            Syllable::So => Self::G,
            Syllable::La => Self::A,
            Syllable::Ti => Self::B,
            //Sharp
            Syllable::Di => Self::C_SHARP,
            Syllable::Ri => Self::D_SHARP,
            Syllable::Fi => Self::F_SHARP,
            Syllable::Si => Self::G_SHARP,
            Syllable::Li => Self::A_SHARP,
            //Flat
            Syllable::Ra => Self::D_FLAT,
            Syllable::Me => Self::E_FLAT,
            Syllable::Se => Self::G_FLAT,
            Syllable::Le => Self::A_FLAT,
            Syllable::Te => Self::B_FLAT,
        }
    }
}

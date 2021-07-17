use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::{Pitch, PitchName, Semitones, Syllable};

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
            Key::Sharp(p) => write!(f, "{}#", p),
            Key::Flat(p) => write!(f, "{}b", p),
        }
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
}

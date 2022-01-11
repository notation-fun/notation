use std::fmt::Display;
use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

use crate::prelude::Key;

// https://hellomusictheory.com/learn/
// http://openmusictheory.com/pitches.html
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum PitchName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Display for PitchName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum PitchSign {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Display for PitchSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            /* Unicode version, not very readable in console
            PitchSign::Natural => "",
            PitchSign::Sharp => "♯",
            PitchSign::Flat => "♭" ,
            PitchSign::DoubleSharp => "𝄪",
            PitchSign::DoubleFlat => "𝄫",
             */
            PitchSign::Natural => "",
            PitchSign::Sharp => "#",
            PitchSign::Flat => "b",
            PitchSign::DoubleSharp => "##",
            PitchSign::DoubleFlat => "bb",
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Pitch {
    pub name: PitchName,
    pub sign: PitchSign,
}

impl Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name, self.sign)
    }
}

impl Pitch {
    pub const C: Pitch = Self {
        name: PitchName::C,
        sign: PitchSign::Natural,
    };
    pub const D: Pitch = Self {
        name: PitchName::D,
        sign: PitchSign::Natural,
    };
    pub const E: Pitch = Self {
        name: PitchName::E,
        sign: PitchSign::Natural,
    };
    pub const F: Pitch = Self {
        name: PitchName::F,
        sign: PitchSign::Natural,
    };
    pub const G: Pitch = Self {
        name: PitchName::G,
        sign: PitchSign::Natural,
    };
    pub const A: Pitch = Self {
        name: PitchName::A,
        sign: PitchSign::Natural,
    };
    pub const B: Pitch = Self {
        name: PitchName::B,
        sign: PitchSign::Natural,
    };

    pub const C_SHARP: Pitch = Self {
        name: PitchName::C,
        sign: PitchSign::Sharp,
    };
    pub const D_SHARP: Pitch = Self {
        name: PitchName::D,
        sign: PitchSign::Sharp,
    };
    pub const E_SHARP: Pitch = Self {
        name: PitchName::E,
        sign: PitchSign::Sharp,
    };
    pub const F_SHARP: Pitch = Self {
        name: PitchName::F,
        sign: PitchSign::Sharp,
    };
    pub const G_SHARP: Pitch = Self {
        name: PitchName::G,
        sign: PitchSign::Sharp,
    };
    pub const A_SHARP: Pitch = Self {
        name: PitchName::A,
        sign: PitchSign::Sharp,
    };
    pub const B_SHARP: Pitch = Self {
        name: PitchName::B,
        sign: PitchSign::Sharp,
    };

    pub const C_FLAT: Pitch = Self {
        name: PitchName::C,
        sign: PitchSign::Flat,
    };
    pub const D_FLAT: Pitch = Self {
        name: PitchName::D,
        sign: PitchSign::Flat,
    };
    pub const E_FLAT: Pitch = Self {
        name: PitchName::E,
        sign: PitchSign::Flat,
    };
    pub const F_FLAT: Pitch = Self {
        name: PitchName::F,
        sign: PitchSign::Flat,
    };
    pub const G_FLAT: Pitch = Self {
        name: PitchName::G,
        sign: PitchSign::Flat,
    };
    pub const A_FLAT: Pitch = Self {
        name: PitchName::A,
        sign: PitchSign::Flat,
    };
    pub const B_FLAT: Pitch = Self {
        name: PitchName::B,
        sign: PitchSign::Flat,
    };

    pub const C_DOUBLE_SHARP: Pitch = Self {
        name: PitchName::C,
        sign: PitchSign::DoubleSharp,
    };
    pub const D_DOUBLE_SHARP: Pitch = Self {
        name: PitchName::D,
        sign: PitchSign::DoubleSharp,
    };
    pub const E_DOUBLE_SHARP: Pitch = Self {
        name: PitchName::E,
        sign: PitchSign::DoubleSharp,
    };
    pub const F_DOUBLE_SHARP: Pitch = Self {
        name: PitchName::F,
        sign: PitchSign::DoubleSharp,
    };
    pub const G_DOUBLE_SHARP: Pitch = Self {
        name: PitchName::G,
        sign: PitchSign::DoubleSharp,
    };
    pub const A_DOUBLE_SHARP: Pitch = Self {
        name: PitchName::A,
        sign: PitchSign::DoubleSharp,
    };
    pub const B_DOUBLE_SHARP: Pitch = Self {
        name: PitchName::B,
        sign: PitchSign::DoubleSharp,
    };

    pub const C_DOUBLE_FLAT: Pitch = Self {
        name: PitchName::C,
        sign: PitchSign::DoubleFlat,
    };
    pub const D_DOUBLE_FLAT: Pitch = Self {
        name: PitchName::D,
        sign: PitchSign::DoubleFlat,
    };
    pub const E_DOUBLE_FLAT: Pitch = Self {
        name: PitchName::E,
        sign: PitchSign::DoubleFlat,
    };
    pub const F_DOUBLE_FLAT: Pitch = Self {
        name: PitchName::F,
        sign: PitchSign::DoubleFlat,
    };
    pub const G_DOUBLE_FLAT: Pitch = Self {
        name: PitchName::G,
        sign: PitchSign::DoubleFlat,
    };
    pub const A_DOUBLE_FLAT: Pitch = Self {
        name: PitchName::A,
        sign: PitchSign::DoubleFlat,
    };
    pub const B_DOUBLE_FLAT: Pitch = Self {
        name: PitchName::B,
        sign: PitchSign::DoubleFlat,
    };
}

impl Pitch {
    pub fn new(name: PitchName, sign: PitchSign) -> Self {
        Self { name, sign }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Semitones(pub i8);

impl Add for Semitones {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Semitones(self.0 + rhs.0)
    }
}

impl Sub for Semitones {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Semitones(self.0 - rhs.0)
    }
}

impl From<i8> for Semitones {
    fn from(v: i8) -> Self {
        Self(v)
    }
}

impl From<PitchName> for Semitones {
    fn from(v: PitchName) -> Self {
        let v = match v {
            PitchName::C => 0,
            PitchName::D => 2,
            PitchName::E => 4,
            PitchName::F => 5,
            PitchName::G => 7,
            PitchName::A => 9,
            PitchName::B => 11,
        };
        Self::from(v)
    }
}

impl From<PitchSign> for Semitones {
    fn from(v: PitchSign) -> Self {
        let v = match v {
            PitchSign::Natural => 0,
            PitchSign::Sharp => 1,
            PitchSign::Flat => -1,
            PitchSign::DoubleSharp => 2,
            PitchSign::DoubleFlat => -2,
        };
        Self::from(v)
    }
}

impl From<Pitch> for Semitones {
    fn from(v: Pitch) -> Self {
        let name_val = Semitones::from(v.name).0;
        let sign_val = Semitones::from(v.sign).0;
        Self::from(name_val + sign_val)
    }
}

impl From<Semitones> for PitchSign {
    fn from(v: Semitones) -> Self {
        match v.0 {
            0 => Self::Natural,
            1 => Self::Sharp,
            -1 => Self::Flat,
            2 => Self::DoubleSharp,
            -2 => Self::DoubleFlat,
            _ => Self::Natural,
        }
    }
}

impl From<Semitones> for Pitch {
    fn from(v: Semitones) -> Self {
        let pos_val = if v.0 >= 0 { v.0 % 12 } else { v.0 % 12 + 12 };
        match pos_val {
            0 => Pitch::C,
            1 => Pitch::C_SHARP,
            2 => Pitch::D,
            3 => Pitch::D_SHARP,
            4 => Pitch::E,
            5 => Pitch::F,
            6 => Pitch::F_SHARP,
            7 => Pitch::G,
            8 => Pitch::G_SHARP,
            9 => Pitch::A,
            10 => Pitch::A_SHARP,
            11 => Pitch::B,
            _ => Pitch::C,
        }
    }
}

impl From<Key> for Pitch {
    fn from(v: Key) -> Self {
        match v {
            Key::Natural(name) => Self::new(name, PitchSign::Natural),
            Key::Sharp(name) => Self::new(name, PitchSign::Sharp),
            Key::Flat(name) => Self::new(name, PitchSign::Flat),
        }
    }
}

impl Pitch {
    pub fn to_text(&self) -> String {
        format!("{}", self)
    }
    pub fn from_text(text: &str) -> Self {
        match text {
            "C" => Pitch::C,
            "D" => Pitch::D,
            "E" => Pitch::E,
            "F" => Pitch::F,
            "G" => Pitch::G,
            "A" => Pitch::A,
            "B" => Pitch::B,
            "C#" => Pitch::C_SHARP,
            "D#" => Pitch::D_SHARP,
            "E#" => Pitch::E_SHARP,
            "F#" => Pitch::F_SHARP,
            "G#" => Pitch::G_SHARP,
            "A#" => Pitch::A_SHARP,
            "B#" => Pitch::B_SHARP,
            "Cb" => Pitch::C_FLAT,
            "Db" => Pitch::D_FLAT,
            "Eb" => Pitch::E_FLAT,
            "Fb" => Pitch::F_FLAT,
            "Gb" => Pitch::G_FLAT,
            "Ab" => Pitch::A_FLAT,
            "Bb" => Pitch::B_FLAT,
            "C##" => Pitch::C_DOUBLE_SHARP,
            "D##" => Pitch::D_DOUBLE_SHARP,
            "E##" => Pitch::E_DOUBLE_SHARP,
            "F##" => Pitch::F_DOUBLE_SHARP,
            "G##" => Pitch::G_DOUBLE_SHARP,
            "A##" => Pitch::A_DOUBLE_SHARP,
            "B##" => Pitch::B_DOUBLE_SHARP,
            "Cbb" => Pitch::C_DOUBLE_FLAT,
            "Dbb" => Pitch::D_DOUBLE_FLAT,
            "Ebb" => Pitch::E_DOUBLE_FLAT,
            "Fbb" => Pitch::F_DOUBLE_FLAT,
            "Gbb" => Pitch::G_DOUBLE_FLAT,
            "Abb" => Pitch::A_DOUBLE_FLAT,
            "Bbb" => Pitch::B_DOUBLE_FLAT,
            _ => Pitch::C,
        }
    }
}

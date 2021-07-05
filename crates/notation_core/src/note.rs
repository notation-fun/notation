use std::fmt::Display;
use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

use crate::prelude::Solfege;

// https://hellomusictheory.com/learn/
// http://openmusictheory.com/pitches.html
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
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

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum PitchSign {
    Natural,
    Sharp,
    Flat,
    DoubleSharp,
    DoubleFlat,
}

impl Display for PitchSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PitchSign::Natural => "",
            PitchSign::Sharp => "#",
            PitchSign::Flat => "b",
            PitchSign::DoubleSharp => "##",
            PitchSign::DoubleFlat => "bb",
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Pitch {
    pub name: PitchName,
    pub sign: PitchSign,
}

impl Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.sign, self.name)
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

    pub const C_DOUBA_FLAT_SHARP: Pitch = Self {
        name: PitchName::C,
        sign: PitchSign::DoubleSharp,
    };
    pub const D_DOUBA_FLAT_SHARP: Pitch = Self {
        name: PitchName::D,
        sign: PitchSign::DoubleSharp,
    };
    pub const E_DOUBA_FLAT_SHARP: Pitch = Self {
        name: PitchName::E,
        sign: PitchSign::DoubleSharp,
    };
    pub const F_DOUBA_FLAT_SHARP: Pitch = Self {
        name: PitchName::F,
        sign: PitchSign::DoubleSharp,
    };
    pub const G_DOUBA_FLAT_SHARP: Pitch = Self {
        name: PitchName::G,
        sign: PitchSign::DoubleSharp,
    };
    pub const A_DOUBA_FLAT_SHARP: Pitch = Self {
        name: PitchName::A,
        sign: PitchSign::DoubleSharp,
    };
    pub const B_DOUBA_FLAT_SHARP: Pitch = Self {
        name: PitchName::B,
        sign: PitchSign::DoubleSharp,
    };

    pub const C_DOUBA_FLAT_FLAT: Pitch = Self {
        name: PitchName::C,
        sign: PitchSign::DoubleFlat,
    };
    pub const D_DOUBA_FLAT_FLAT: Pitch = Self {
        name: PitchName::D,
        sign: PitchSign::DoubleFlat,
    };
    pub const E_DOUBA_FLAT_FLAT: Pitch = Self {
        name: PitchName::E,
        sign: PitchSign::DoubleFlat,
    };
    pub const F_DOUBA_FLAT_FLAT: Pitch = Self {
        name: PitchName::F,
        sign: PitchSign::DoubleFlat,
    };
    pub const G_DOUBA_FLAT_FLAT: Pitch = Self {
        name: PitchName::G,
        sign: PitchSign::DoubleFlat,
    };
    pub const A_DOUBA_FLAT_FLAT: Pitch = Self {
        name: PitchName::A,
        sign: PitchSign::DoubleFlat,
    };
    pub const B_DOUBA_FLAT_FLAT: Pitch = Self {
        name: PitchName::B,
        sign: PitchSign::DoubleFlat,
    };
}

// https://en.wikipedia.org/wiki/Scientific_pitch_notation
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Octave {
    N1,
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
}

impl Octave {
    pub const CENTER: Self = Self::P4;
}

impl Default for Octave {
    fn default() -> Self {
        Self::CENTER
    }
}

impl Display for Octave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Octave::N1 => "_",
            Octave::P0 => "0",
            Octave::P1 => "1",
            Octave::P2 => "2",
            Octave::P3 => "3",
            Octave::P4 => "4",
            Octave::P5 => "5",
            Octave::P6 => "6",
            Octave::P7 => "7",
            Octave::P8 => "8",
            Octave::P9 => "9",
            Octave::P10 => "10",
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub octave: Octave,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.pitch, self.octave)
    }
}

impl Note {
    pub fn new(pitch: Pitch, octave: Octave) -> Self {
        Self { pitch, octave }
    }
}

impl Note {
    pub const C_0: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P0,
    };
    pub const D_0: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P0,
    };
    pub const E_0: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P0,
    };
    pub const F_0: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P0,
    };
    pub const G_0: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P0,
    };
    pub const A_0: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P0,
    };
    pub const B_0: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P0,
    };

    pub const C_SHARP_0: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P0,
    };
    pub const D_SHARP_0: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P0,
    };
    pub const F_SHARP_0: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P0,
    };
    pub const G_SHARP_0: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P0,
    };
    pub const A_SHARP_0: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P0,
    };

    pub const D_FLAT_0: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P0,
    };
    pub const E_FLAT_0: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P0,
    };
    pub const G_FLAT_0: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P0,
    };
    pub const A_FLAT_0: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P0,
    };
    pub const B_FLAT_0: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P0,
    };

    pub const C_1: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P1,
    };
    pub const D_1: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P1,
    };
    pub const E_1: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P1,
    };
    pub const F_1: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P1,
    };
    pub const G_1: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P1,
    };
    pub const A_1: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P1,
    };
    pub const B_1: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P1,
    };

    pub const C_SHARP_1: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P1,
    };
    pub const D_SHARP_1: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P1,
    };
    pub const F_SHARP_1: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P1,
    };
    pub const G_SHARP_1: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P1,
    };
    pub const A_SHARP_1: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P1,
    };

    pub const D_FLAT_1: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P1,
    };
    pub const E_FLAT_1: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P1,
    };
    pub const G_FLAT_1: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P1,
    };
    pub const A_FLAT_1: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P1,
    };
    pub const B_FLAT_1: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P1,
    };

    pub const C_2: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P2,
    };
    pub const D_2: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P2,
    };
    pub const E_2: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P2,
    };
    pub const F_2: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P2,
    };
    pub const G_2: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P2,
    };
    pub const A_2: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P2,
    };
    pub const B_2: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P2,
    };

    pub const C_SHARP_2: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P2,
    };
    pub const D_SHARP_2: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P2,
    };
    pub const F_SHARP_2: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P2,
    };
    pub const G_SHARP_2: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P2,
    };
    pub const A_SHARP_2: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P2,
    };

    pub const D_FLAT_2: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P2,
    };
    pub const E_FLAT_2: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P2,
    };
    pub const G_FLAT_2: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P2,
    };
    pub const A_FLAT_2: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P2,
    };
    pub const B_FLAT_2: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P2,
    };

    pub const C_3: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P3,
    };
    pub const D_3: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P3,
    };
    pub const E_3: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P3,
    };
    pub const F_3: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P3,
    };
    pub const G_3: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P3,
    };
    pub const A_3: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P3,
    };
    pub const B_3: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P3,
    };

    pub const C_SHARP_3: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P3,
    };
    pub const D_SHARP_3: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P3,
    };
    pub const F_SHARP_3: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P3,
    };
    pub const G_SHARP_3: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P3,
    };
    pub const A_SHARP_3: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P3,
    };

    pub const D_FLAT_3: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P3,
    };
    pub const E_FLAT_3: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P3,
    };
    pub const G_FLAT_3: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P3,
    };
    pub const A_FLAT_3: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P3,
    };
    pub const B_FLAT_3: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P3,
    };

    pub const C_4: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P4,
    };
    pub const D_4: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P4,
    };
    pub const E_4: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P4,
    };
    pub const F_4: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P4,
    };
    pub const G_4: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P4,
    };
    pub const A_4: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P4,
    };
    pub const B_4: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P4,
    };

    pub const C_SHARP_4: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P4,
    };
    pub const D_SHARP_4: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P4,
    };
    pub const F_SHARP_4: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P4,
    };
    pub const G_SHARP_4: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P4,
    };
    pub const A_SHARP_4: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P4,
    };

    pub const D_FLAT_4: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P4,
    };
    pub const E_FLAT_4: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P4,
    };
    pub const G_FLAT_4: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P4,
    };
    pub const A_FLAT_4: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P4,
    };
    pub const B_FLAT_4: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P4,
    };

    pub const C_5: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P5,
    };
    pub const D_5: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P5,
    };
    pub const E_5: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P5,
    };
    pub const F_5: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P5,
    };
    pub const G_5: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P5,
    };
    pub const A_5: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P5,
    };
    pub const B_5: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P5,
    };

    pub const C_SHARP_5: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P5,
    };
    pub const D_SHARP_5: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P5,
    };
    pub const F_SHARP_5: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P5,
    };
    pub const G_SHARP_5: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P5,
    };
    pub const A_SHARP_5: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P5,
    };

    pub const D_FLAT_5: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P5,
    };
    pub const E_FLAT_5: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P5,
    };
    pub const G_FLAT_5: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P5,
    };
    pub const A_FLAT_5: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P5,
    };
    pub const B_FLAT_5: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P5,
    };

    pub const C_6: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P6,
    };
    pub const D_6: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P6,
    };
    pub const E_6: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P6,
    };
    pub const F_6: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P6,
    };
    pub const G_6: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P6,
    };
    pub const A_6: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P6,
    };
    pub const B_6: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P6,
    };

    pub const C_SHARP_6: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P6,
    };
    pub const D_SHARP_6: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P6,
    };
    pub const F_SHARP_6: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P6,
    };
    pub const G_SHARP_6: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P6,
    };
    pub const A_SHARP_6: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P6,
    };

    pub const D_FLAT_6: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P6,
    };
    pub const E_FLAT_6: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P6,
    };
    pub const G_FLAT_6: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P6,
    };
    pub const A_FLAT_6: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P6,
    };
    pub const B_FLAT_6: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P6,
    };

    pub const C_7: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P7,
    };
    pub const D_7: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P7,
    };
    pub const E_7: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P7,
    };
    pub const F_7: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P7,
    };
    pub const G_7: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P7,
    };
    pub const A_7: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P7,
    };
    pub const B_7: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P7,
    };

    pub const C_SHARP_7: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P7,
    };
    pub const D_SHARP_7: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P7,
    };
    pub const F_SHARP_7: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P7,
    };
    pub const G_SHARP_7: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P7,
    };
    pub const A_SHARP_7: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P7,
    };

    pub const D_FLAT_7: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P7,
    };
    pub const E_FLAT_7: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P7,
    };
    pub const G_FLAT_7: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P7,
    };
    pub const A_FLAT_7: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P7,
    };
    pub const B_FLAT_7: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P7,
    };

    pub const C_8: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P8,
    };
    pub const D_8: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P8,
    };
    pub const E_8: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P8,
    };
    pub const F_8: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P8,
    };
    pub const G_8: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P8,
    };
    pub const A_8: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P8,
    };
    pub const B_8: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P8,
    };

    pub const C_SHARP_8: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P8,
    };
    pub const D_SHARP_8: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P8,
    };
    pub const F_SHARP_8: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P8,
    };
    pub const G_SHARP_8: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P8,
    };
    pub const A_SHARP_8: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P8,
    };

    pub const D_FLAT_8: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P8,
    };
    pub const E_FLAT_8: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P8,
    };
    pub const G_FLAT_8: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P8,
    };
    pub const A_FLAT_8: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P8,
    };
    pub const B_FLAT_8: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P8,
    };

    pub const C_9: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P9,
    };
    pub const D_9: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P9,
    };
    pub const E_9: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P9,
    };
    pub const F_9: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P9,
    };
    pub const G_9: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P9,
    };
    pub const A_9: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P9,
    };
    pub const B_9: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P9,
    };

    pub const C_SHARP_9: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P9,
    };
    pub const D_SHARP_9: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P9,
    };
    pub const F_SHARP_9: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P9,
    };
    pub const G_SHARP_9: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P9,
    };
    pub const A_SHARP_9: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P9,
    };

    pub const D_FLAT_9: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P9,
    };
    pub const E_FLAT_9: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P9,
    };
    pub const G_FLAT_9: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P9,
    };
    pub const A_FLAT_9: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P9,
    };
    pub const B_FLAT_9: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P9,
    };
}

impl From<(Pitch, Octave)> for Note {
    fn from((pitch, octave): (Pitch, Octave)) -> Self {
        Self::new(pitch, octave)
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

impl From<Octave> for Semitones {
    fn from(v: Octave) -> Self {
        let v = match v {
            Octave::N1 => 12 * -1,
            Octave::P0 => 12 * 0,
            Octave::P1 => 12 * 1,
            Octave::P2 => 12 * 2,
            Octave::P3 => 12 * 3,
            Octave::P4 => 12 * 4,
            Octave::P5 => 12 * 5,
            Octave::P6 => 12 * 6,
            Octave::P7 => 12 * 7,
            Octave::P8 => 12 * 8,
            Octave::P9 => 12 * 9,
            Octave::P10 => 12 * 10,
        };
        Self::from(v)
    }
}

impl From<Note> for Semitones {
    fn from(v: Note) -> Self {
        let pitch_val = Semitones::from(v.pitch).0;
        let octave_val = Semitones::from(v.octave).0;
        Self(pitch_val + octave_val)
    }
}

impl From<Semitones> for Octave {
    fn from(v: Semitones) -> Self {
        if v.0 < 0 {
            return Octave::N1;
        }
        match v.0 / 12 {
            0 => Octave::P0,
            1 => Octave::P1,
            2 => Octave::P2,
            3 => Octave::P3,
            4 => Octave::P4,
            5 => Octave::P5,
            6 => Octave::P6,
            7 => Octave::P7,
            8 => Octave::P8,
            9 => Octave::P9,
            _ => Octave::P10,
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

impl From<Semitones> for Note {
    fn from(v: Semitones) -> Self {
        let pitch = Pitch::from(v);
        let octave = Octave::from(v);
        Note::new(pitch, octave)
    }
}

impl From<Solfege> for Note {
    fn from(v: Solfege) -> Self {
        Semitones::from(v).into()
    }
}

impl From<Note> for Solfege {
    fn from(v: Note) -> Self {
        Semitones::from(v).into()
    }
}

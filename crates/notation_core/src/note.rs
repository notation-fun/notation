use serde::{Serialize, Deserialize};

use crate::prelude::{Duration, Entry, EntryWrap};

// https://hellomusictheory.com/learn/
// http://openmusictheory.com/pitches.html
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum PitchName {
    C, D, E, F, G, A, B,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum PitchSign {
    Natural,
    Sharp, Flat,
    DoubleSharp, DoubleFlat,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Pitch {
    pub name: PitchName,
    pub sign: PitchSign,
}

impl Pitch {
    pub const C: Pitch = Self {name: PitchName::C, sign: PitchSign::Natural};
    pub const D: Pitch = Self {name: PitchName::D, sign: PitchSign::Natural};
    pub const E: Pitch = Self {name: PitchName::E, sign: PitchSign::Natural};
    pub const F: Pitch = Self {name: PitchName::F, sign: PitchSign::Natural};
    pub const G: Pitch = Self {name: PitchName::G, sign: PitchSign::Natural};
    pub const A: Pitch = Self {name: PitchName::A, sign: PitchSign::Natural};
    pub const B: Pitch = Self {name: PitchName::B, sign: PitchSign::Natural};

    pub const C_SHARP: Pitch = Self {name: PitchName::C, sign: PitchSign::Sharp};
    pub const D_SHARP: Pitch = Self {name: PitchName::D, sign: PitchSign::Sharp};
    pub const E_SHARP: Pitch = Self {name: PitchName::E, sign: PitchSign::Sharp};
    pub const F_SHARP: Pitch = Self {name: PitchName::F, sign: PitchSign::Sharp};
    pub const G_SHARP: Pitch = Self {name: PitchName::G, sign: PitchSign::Sharp};
    pub const A_SHARP: Pitch = Self {name: PitchName::A, sign: PitchSign::Sharp};
    pub const B_SHARP: Pitch = Self {name: PitchName::B, sign: PitchSign::Sharp};

    pub const C_FLAT: Pitch = Self {name: PitchName::C, sign: PitchSign::Flat};
    pub const D_FLAT: Pitch = Self {name: PitchName::D, sign: PitchSign::Flat};
    pub const E_FLAT: Pitch = Self {name: PitchName::E, sign: PitchSign::Flat};
    pub const F_FLAT: Pitch = Self {name: PitchName::F, sign: PitchSign::Flat};
    pub const G_FLAT: Pitch = Self {name: PitchName::G, sign: PitchSign::Flat};
    pub const A_FLAT: Pitch = Self {name: PitchName::A, sign: PitchSign::Flat};
    pub const B_FLAT: Pitch = Self {name: PitchName::B, sign: PitchSign::Flat};

    pub const C_DOUBLE_SHARP: Pitch = Self {name: PitchName::C, sign: PitchSign::DoubleSharp};
    pub const D_DOUBLE_SHARP: Pitch = Self {name: PitchName::D, sign: PitchSign::DoubleSharp};
    pub const E_DOUBLE_SHARP: Pitch = Self {name: PitchName::E, sign: PitchSign::DoubleSharp};
    pub const F_DOUBLE_SHARP: Pitch = Self {name: PitchName::F, sign: PitchSign::DoubleSharp};
    pub const G_DOUBLE_SHARP: Pitch = Self {name: PitchName::G, sign: PitchSign::DoubleSharp};
    pub const A_DOUBLE_SHARP: Pitch = Self {name: PitchName::A, sign: PitchSign::DoubleSharp};
    pub const B_DOUBLE_SHARP: Pitch = Self {name: PitchName::B, sign: PitchSign::DoubleSharp};

    pub const C_DOUBLE_FLAT: Pitch = Self {name: PitchName::C, sign: PitchSign::DoubleFlat};
    pub const D_DOUBLE_FLAT: Pitch = Self {name: PitchName::D, sign: PitchSign::DoubleFlat};
    pub const E_DOUBLE_FLAT: Pitch = Self {name: PitchName::E, sign: PitchSign::DoubleFlat};
    pub const F_DOUBLE_FLAT: Pitch = Self {name: PitchName::F, sign: PitchSign::DoubleFlat};
    pub const G_DOUBLE_FLAT: Pitch = Self {name: PitchName::G, sign: PitchSign::DoubleFlat};
    pub const A_DOUBLE_FLAT: Pitch = Self {name: PitchName::A, sign: PitchSign::DoubleFlat};
    pub const B_DOUBLE_FLAT: Pitch = Self {name: PitchName::B, sign: PitchSign::DoubleFlat};
}

// https://en.wikipedia.org/wiki/Scientific_pitch_notation
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Octave {
    N1,
    P0, P1, P2, P3, P4,
    P5, P6, P7, P8, P9,
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

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub octave: Octave,
}

impl Note {
    pub fn new(pitch: Pitch, octave: Octave) -> Self {
        Self {
            pitch,
            octave,
        }
    }
}

impl From<(Pitch, Octave)> for Note {
    fn from((pitch, octave) : (Pitch, Octave)) -> Self {
        Self::new(pitch, octave)
    }
}

pub type NoteEntry = EntryWrap<Note>;

pub type RestEntry = EntryWrap<()>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Semitones(pub i8);

impl From<i8> for Semitones {
    fn from(val: i8) -> Self {
        Self(val)
    }
}

impl From<PitchName> for Semitones {
    fn from(val: PitchName) -> Self {
        let v = match val {
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
    fn from(val: PitchSign) -> Self {
        let v = match val {
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
    fn from(val: Pitch) -> Self {
        let name_val = Semitones::from(val.name).0;
        let sign_val = Semitones::from(val.sign).0;
        Self::from(name_val + sign_val)
    }
}

impl From<Octave> for Semitones {
    fn from(val: Octave) -> Self {
        let v = match val {
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
    fn from(val: Note) -> Self {
        let pitch_val = Semitones::from(val.pitch).0;
        let octave_val = Semitones::from(val.octave).0;
        Self::from(pitch_val + octave_val)
    }
}

impl From<Semitones> for Octave {
    fn from(val: Semitones) -> Self {
        if val.0 < 0 {
            return Octave::N1;
        }
        match val.0 / 12 {
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
    fn from(val: Semitones) -> Self {
        let pos_val = if val.0 > 0 {val.0 % 12 } else {val.0 % 12 + 12};
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
    fn from(val: Semitones) -> Self {
        let pitch = Pitch::from(val);
        let octave = Octave::from(val);
        Note::new(pitch, octave)
    }
}

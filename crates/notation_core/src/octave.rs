use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::Semitones;

// https://en.wikipedia.org/wiki/Scientific_pitch_notation
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
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
            Octave::N1 => "N1",
            Octave::P0 => "P0",
            Octave::P1 => "P1",
            Octave::P2 => "P2",
            Octave::P3 => "P3",
            Octave::P4 => "P4",
            Octave::P5 => "P5",
            Octave::P6 => "P6",
            Octave::P7 => "P7",
            Octave::P8 => "P8",
            Octave::P9 => "P9",
            Octave::P10 => "P10",
        })
    }
}

impl Octave {
    pub fn to_ident(&self) -> String {
        format!("{}", self)
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "P0" => Self::P0,
            "P1" => Self::P1,
            "P2" => Self::P2,
            "P3" => Self::P3,
            "P4" => Self::P4,
            "P5" => Self::P5,
            "P6" => Self::P6,
            "P7" => Self::P7,
            "P8" => Self::P8,
            "P9" => Self::P9,
            "P10" => Self::P10,
            _ => Self::N1,
        }
    }
    pub fn get_higher(&self) -> Self {
        (Semitones::from(*self) + Semitones(12)).into()
    }
    pub fn get_lower(&self) -> Self {
        (Semitones::from(*self) - Semitones(12)).into()
    }
}

impl From<Octave> for Semitones {
    fn from(v: Octave) -> Self {
        let v = match v {
            Octave::N1 => -12,
            Octave::P0 => 0,
            Octave::P1 => 12,
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

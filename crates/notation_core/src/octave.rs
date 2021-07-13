use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::Semitones;

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

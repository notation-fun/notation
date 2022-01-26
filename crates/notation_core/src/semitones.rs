use std::fmt::Display;
use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

use crate::prelude::{Pitch, Octave};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Semitones(pub i8);

impl Display for Semitones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

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

impl From<(Pitch, Octave)> for Semitones {
    fn from(v: (Pitch, Octave)) -> Self {
        let octave_val = Semitones::from(v.0).0;
        let pitch_val = Semitones::from(v.1).0;
        Self(octave_val + pitch_val)
    }
}

impl Semitones {
    pub fn as_pitch_octave(&self) -> (Pitch, Octave) {
        let pitch = Pitch::from(*self);
        let octave = Octave::from(*self);
        (pitch, octave)
    }
}
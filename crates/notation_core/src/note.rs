use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::{Octave, Pitch, Syllable, Semitones};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Note {
    pub octave: Octave,
    pub pitch: Pitch,
    pub syllable: Syllable,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.octave, self.pitch, self.syllable)
    }
}

impl Note {
    pub fn new(octave: Octave, pitch: Pitch, syllable: Syllable) -> Self {
        Self { octave, pitch, syllable }
    }
}

impl From<(Octave, Pitch, Syllable)> for Note {
    fn from(v: (Octave, Pitch, Syllable)) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}

impl From<Note> for Semitones {
    fn from(v: Note) -> Self {
        let octave_val = Semitones::from(v.octave).0;
        let pitch_val = Semitones::from(v.pitch).0;
        Self(octave_val + pitch_val)
    }
}

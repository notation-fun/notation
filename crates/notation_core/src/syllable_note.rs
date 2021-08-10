use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::{Octave, Semitones, Syllable};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct SyllableNote {
    pub octave: Octave,
    pub syllable: Syllable,
}

impl Display for SyllableNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.octave, self.syllable)
    }
}

impl SyllableNote {
    pub fn new(octave: Octave, syllable: Syllable) -> Self {
        Self {
            octave,
            syllable,
        }
    }
}

impl From<(Octave, Syllable)> for SyllableNote {
    fn from(v: (Octave, Syllable)) -> Self {
        Self::new(v.0, v.1)
    }
}

impl From<SyllableNote> for Semitones {
    fn from(v: SyllableNote) -> Self {
        let octave_val = Semitones::from(v.octave).0;
        let syllable_val = Semitones::from(v.syllable).0;
        Self(octave_val + syllable_val)
    }
}

impl From<Semitones> for SyllableNote {
    fn from(v: Semitones) -> Self {
        let octave = Octave::from(v);
        let syllable = Syllable::from(v);
        SyllableNote::new(octave, syllable)
    }
}

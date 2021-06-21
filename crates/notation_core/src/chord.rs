use serde::{Serialize, Deserialize};

use crate::note::Pitch;
use crate::solfege::Syllable;

// https://hellomusictheory.com/learn/chord-inversions/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ChordInversion {
    RootPosition,
    FirstInversion,
    SecondInversion,
    ThirdInversion,
    SlashBass (Pitch),
}

// https://hellomusictheory.com/learn/types-of-chords/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ChordQuality {
    Major, Minor, Diminished, Augmented,
    Major7th, Minor7th, Dominant7th,
    Suspended2th, Suspended4th,
}
//Extended, Altered, Quartal and Quintal not supported

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Chord {
    pub pitch: Pitch,
    pub quality: ChordQuality,
    pub inversion: ChordInversion,
}

impl Chord {
    pub fn new(pitch: Pitch, quality: ChordQuality, inversion: ChordInversion) -> Self {
        Self {pitch, quality, inversion}
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Roman {
    pub syllable: Syllable,
    pub quality: ChordQuality,
    pub inversion: ChordInversion,
}

impl Roman {
    pub fn new(syllable: Syllable, quality: ChordQuality, inversion: ChordInversion) -> Self {
        Self {syllable, quality, inversion}
    }
}

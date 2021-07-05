use serde::{Deserialize, Serialize};

use crate::note::Pitch;
use crate::solfege::Syllable;

// https://hellomusictheory.com/learn/chord-inversions/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ChordInversion {
    RootPosition,
    FirstInversion,
    SecondInversion,
    ThirdInversion,
    SlashBass(Pitch),
}

// https://hellomusictheory.com/learn/types-of-chords/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Major7th,
    Minor7th,
    Dominant7th,
    Suspended2th,
    Suspended4th,
}
//Extended, Altered, Quartal and Quintal not supported

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Chord {
    pub pitch: Pitch,
    pub quality: ChordQuality,
    pub inversion: ChordInversion,
}

impl Chord {
    pub fn new(pitch: Pitch, quality: ChordQuality, inversion: ChordInversion) -> Self {
        Self {
            pitch,
            quality,
            inversion,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Roman {
    pub syllable: Syllable,
    pub quality: ChordQuality,
    pub inversion: ChordInversion,
}

impl Roman {
    pub fn new(syllable: Syllable, quality: ChordQuality, inversion: ChordInversion) -> Self {
        Self {
            syllable,
            quality,
            inversion,
        }
    }
}

impl Roman {
    pub const I_MAJOR: Self = Self {
        syllable: Syllable::Do,
        quality: ChordQuality::Major,
        inversion: ChordInversion::RootPosition,
    };
    pub const II_MINOR: Self = Self {
        syllable: Syllable::Re,
        quality: ChordQuality::Minor,
        inversion: ChordInversion::RootPosition,
    };
    pub const III_MINOR: Self = Self {
        syllable: Syllable::Mi,
        quality: ChordQuality::Minor,
        inversion: ChordInversion::RootPosition,
    };
    pub const IV_MAJOR: Self = Self {
        syllable: Syllable::Fa,
        quality: ChordQuality::Major,
        inversion: ChordInversion::RootPosition,
    };
    pub const V_MAJOR: Self = Self {
        syllable: Syllable::So,
        quality: ChordQuality::Major,
        inversion: ChordInversion::RootPosition,
    };
    pub const VI_MINOR: Self = Self {
        syllable: Syllable::La,
        quality: ChordQuality::Minor,
        inversion: ChordInversion::RootPosition,
    };
    pub const VII_DIMISHED: Self = Self {
        syllable: Syllable::Ti,
        quality: ChordQuality::Diminished,
        inversion: ChordInversion::RootPosition,
    };
}

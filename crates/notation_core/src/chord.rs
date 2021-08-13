use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::{Pitch, Syllable};

// https://hellomusictheory.com/learn/chord-inversions/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ChordInversion {
    RootPosition,
    FirstInversion,
    SecondInversion,
    ThirdInversion,
    SlashBass(Pitch),
}
impl Display for ChordInversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// https://hellomusictheory.com/learn/types-of-chords/
// https://www.earmaster.com/music-theory-online/ch05/chapter-5-4.html
// https://en.wikipedia.org/wiki/Chord_names_and_symbols_(popular_music)
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Suspended2th,
    Suspended4th,
    Major7th,
    Minor7th,
    Dominant7th,
    Diminished7th,
    HaldDiminished7th,
}
//Extended, Altered, Quartal and Quintal not supported
impl Display for ChordQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Chord {
    pub pitch: Pitch,
    pub quality: ChordQuality,
    pub inversion: ChordInversion,
    pub syllable: Option<Syllable>,
}
impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.pitch, self.quality, self.inversion)
    }
}

impl Chord {
    pub fn new(
        pitch: Pitch,
        quality: ChordQuality,
        inversion: ChordInversion,
        syllable: Option<Syllable>,
    ) -> Self {
        Self {
            pitch,
            quality,
            inversion,
            syllable,
        }
    }
}

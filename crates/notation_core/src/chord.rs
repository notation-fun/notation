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

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Chord {
    pub pitch: Pitch,
    pub quality: ChordQuality,
    pub inversion: ChordInversion,
    pub syllable: Option<Syllable>,
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

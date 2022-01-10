use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::{Chord, Key, Note, Octave, Pitch, Semitones, Syllable, SyllableNote};
use crate::tone::Tone;

// https://hellomusictheory.com/learn/music-scales-beginners-guide/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Scale {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}
impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for Scale {
    fn default() -> Self {
        Self::Ionian
    }
}
impl Scale {
    #[allow(non_upper_case_globals)]
    pub const Major: Scale = Scale::Ionian;
    #[allow(non_upper_case_globals)]
    pub const Minor: Scale = Scale::Aeolian;
    pub const ALL: [ Scale; 7 ] = [
        Scale::Ionian, Scale::Dorian, Scale::Phrygian, Scale::Lydian, Scale::Mixolydian, Scale::Aeolian, Scale::Locrian,
    ];

    pub fn to_ident(&self) -> String {
        format!("{}", self)
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "Major" => Self::Major,
            "Minor" => Self::Minor,
            "Ionian" => Self::Ionian,
            "Dorian" => Self::Dorian,
            "Phrygian" => Self::Phrygian,
            "Lydian" => Self::Lydian,
            "Mixolydian" => Self::Mixolydian,
            "Aeolian" => Self::Aeolian,
            "Locrian" => Self::Locrian,
            _ => Self::default(),
        }
    }
    pub fn get_syllables(&self) -> Vec<Syllable> {
        match self {
            Scale::Ionian => vec![
                Syllable::Do,
                Syllable::Re,
                Syllable::Mi,
                Syllable::Fa,
                Syllable::So,
                Syllable::La,
                Syllable::Ti,
            ],
            Scale::Dorian => vec![
                Syllable::Re,
                Syllable::Mi,
                Syllable::Fa,
                Syllable::So,
                Syllable::La,
                Syllable::Ti,
                Syllable::Do,
            ],
            Scale::Phrygian => vec![
                Syllable::Mi,
                Syllable::Fa,
                Syllable::So,
                Syllable::La,
                Syllable::Ti,
                Syllable::Do,
                Syllable::Re,
            ],
            Scale::Lydian => vec![
                Syllable::Fa,
                Syllable::So,
                Syllable::La,
                Syllable::Ti,
                Syllable::Do,
                Syllable::Re,
                Syllable::Mi,
            ],
            Scale::Mixolydian => vec![
                Syllable::So,
                Syllable::La,
                Syllable::Ti,
                Syllable::Do,
                Syllable::Re,
                Syllable::Mi,
                Syllable::Fa,
            ],
            Scale::Aeolian => vec![
                Syllable::La,
                Syllable::Ti,
                Syllable::Do,
                Syllable::Re,
                Syllable::Mi,
                Syllable::Fa,
                Syllable::So,
            ],
            Scale::Locrian => vec![
                Syllable::Ti,
                Syllable::Do,
                Syllable::Re,
                Syllable::Mi,
                Syllable::Fa,
                Syllable::So,
                Syllable::La,
            ],
        }
    }
}

impl Scale {
    pub fn calc_do_offset(&self) -> i8 {
        match self {
            Scale::Ionian => 0,
            Scale::Dorian => -2,
            Scale::Phrygian => -4,
            Scale::Lydian => -5,
            Scale::Mixolydian => 5,
            Scale::Aeolian => 3,
            Scale::Locrian => 1,
        }
    }
    pub fn calc_do_semitones(&self, key: &Key) -> Semitones {
        let semitones = Semitones::from(*key).0 + self.calc_do_offset();
        Semitones(semitones)
    }
    pub fn calc_root_syllable(&self) -> Syllable {
        Semitones(0 - self.calc_do_offset()).into()
    }
    pub fn calc_syllable_for_sort(&self, syllable: &Syllable) -> Syllable {
        let semitones = Semitones::from(*syllable).0 + self.calc_do_offset();
        Semitones(semitones).into()
    }
    pub fn calc_chord_for_sort(&self, chord: &Chord) -> Chord {
        if *self == Scale::Ionian {
            return chord.clone();
        }
        let root = self.calc_syllable_for_sort(&chord.root);
        Chord {
            root,
            intervals: chord.intervals.clone(),
            bass: chord.bass.clone(),
        }
    }
    pub fn calc_syllable(&self, key: &Key, pitch: &Pitch) -> Syllable {
        (Semitones::from(*pitch) - self.calc_do_semitones(key)).into()
    }
    pub fn calc_pitch(&self, key: &Key, syllable: &Syllable) -> Pitch {
        (Semitones::from(*syllable) + self.calc_do_semitones(key)).into()
    }
    pub fn calc_syllable_note(&self, key: &Key, note: &Note) -> SyllableNote {
        (Semitones::from(*note) - self.calc_do_semitones(key)).into()
    }
    pub fn calc_note(&self, key: &Key, syllable_note: &SyllableNote) -> Note {
        (Semitones::from(*syllable_note) + self.calc_do_semitones(key)).into()
    }
    pub fn calc_click_note(&self, key: &Key, octave: &Octave, syllable: &Syllable) -> Note {
        let pitch = self.calc_pitch(key, syllable);
        Note::new(*octave, pitch)
    }
    pub fn calc_click_tone(&self, key: &Key, octave: &Octave, syllable: &Syllable) -> Tone {
        Tone::Single(self.calc_click_note(key, octave, syllable))
    }
}

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
    pub fn calc_key_index(&self, key: Key) -> usize {
        let offset = Semitones::from(key) - Semitones::from(self.get_keys()[0]);
        let offset_val = if offset.0 >= 0 { offset.0 % 12 } else { offset.0 % 12 + 12 };
        match offset_val {
            0 => 0,
            1 => 7,
            2 => 2,
            3 => 9,
            4 => 4,
            5 => 11,
            6 => 6,
            7 => 1,
            8 => 8,
            9 => 3,
            10 => 10,
            11 => 5,
            _ => 0,
        }
    }
    //https://www.hooktheory.com/cheat-sheet
    pub fn get_keys(&self) -> [Key; 12] {
        match self {
            Scale::Ionian => [
                Key::C,
                Key::G,
                Key::D,
                Key::A,
                Key::E,
                Key::B,
                Key::F_SHARP,
                Key::D_FLAT,
                Key::A_FLAT,
                Key::E_FLAT,
                Key::B_FLAT,
                Key::F,
            ],
            Scale::Dorian => [
                Key::D,
                Key::A,
                Key::E,
                Key::B,
                Key::F_SHARP,
                Key::C_SHARP,
                Key::G_SHARP,
                Key::E_FLAT,
                Key::B_FLAT,
                Key::F,
                Key::C,
                Key::G,
            ],
            Scale::Phrygian => [
                Key::E,
                Key::B,
                Key::F_SHARP,
                Key::C_SHARP,
                Key::G_SHARP,
                Key::D_SHARP,
                Key::A_SHARP,
                Key::F,
                Key::C,
                Key::G,
                Key::D,
                Key::A,
            ],
            Scale::Lydian => [
                Key::F,
                Key::C,
                Key::G,
                Key::D,
                Key::A,
                Key::E,
                Key::B,
                Key::G_FLAT,
                Key::D_FLAT,
                Key::A_FLAT,
                Key::E_FLAT,
                Key::B_FLAT,
            ],
            Scale::Mixolydian => [
                Key::G,
                Key::D,
                Key::A,
                Key::E,
                Key::B,
                Key::F_SHARP,
                Key::C_SHARP,
                Key::A_FLAT,
                Key::E_FLAT,
                Key::B_FLAT,
                Key::F,
                Key::C,
            ],
            Scale::Aeolian => [
                Key::A,
                Key::E,
                Key::B,
                Key::F_SHARP,
                Key::C_SHARP,
                Key::G_SHARP,
                Key::D_SHARP,
                Key::B_FLAT,
                Key::F,
                Key::C,
                Key::G,
                Key::D,
            ],
            Scale::Locrian => [
                Key::B,
                Key::F_SHARP,
                Key::C_SHARP,
                Key::G_SHARP,
                Key::D_SHARP,
                Key::A_SHARP,
                Key::E_SHARP,
                Key::C,
                Key::G,
                Key::D,
                Key::A,
                Key::E,
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
        let key_index = self.calc_key_index(key.clone());
        if let Some(keys) = match syllable {
            Syllable::Do => Some(Scale::Ionian.get_keys()),
            Syllable::Re => Some(Scale::Dorian.get_keys()),
            Syllable::Mi => Some(Scale::Phrygian.get_keys()),
            Syllable::Fa => Some(Scale::Lydian.get_keys()),
            Syllable::So => Some(Scale::Mixolydian.get_keys()),
            Syllable::La => Some(Scale::Aeolian.get_keys()),
            Syllable::Ti => Some(Scale::Locrian.get_keys()),
            _ => None,
        } {
            keys[key_index].into()
        } else {
            (Semitones::from(*syllable) + self.calc_do_semitones(key)).into()
        }
    }
    pub fn calc_syllable_note(&self, key: &Key, note: &Note) -> SyllableNote {
        (Semitones::from(*note) - self.calc_do_semitones(key)).into()
    }
    pub fn calc_note(&self, key: &Key, syllable_note: &SyllableNote) -> Note {
        let pitch = self.calc_pitch(key, &syllable_note.syllable);
        Note::new(syllable_note.octave, pitch)
    }
    pub fn calc_click_note(&self, key: &Key, octave: &Octave, syllable: &Syllable) -> Note {
        let pitch = self.calc_pitch(key, syllable);
        Note::new(*octave, pitch)
    }
    pub fn calc_click_tone(&self, key: &Key, octave: &Octave, syllable: &Syllable) -> Tone {
        Tone::Single(self.calc_click_note(key, octave, syllable))
    }
}

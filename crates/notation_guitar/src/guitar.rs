use crate::tuning::GuitarTuning;
use notation_core::prelude::Note;
use notation_fretted::prelude::{Fretboard, FrettedEntry, HandShape};

pub const GUITAR_STRING_NUM: usize = 6;

pub const GUITAR_FRET_NUM_ACOUSTIC: usize = 20;
pub const GUITAR_FRET_NUM_CLASSICAL: usize = 19;
pub const GUITAR_FRET_NUM_ELECTRIC: usize = 22;

pub struct GuitarString();

impl GuitarString {
    pub const _1: u8 = 0;
    pub const _2: u8 = 1;
    pub const _3: u8 = 2;
    pub const _4: u8 = 3;
    pub const _5: u8 = 4;
    pub const _6: u8 = 5;
}

impl GuitarString {
    pub fn name_of(index: u8) -> &'static str {
        match index {
            Self::_1 => "1",
            Self::_2 => "2",
            Self::_3 => "3",
            Self::_4 => "4",
            Self::_5 => "5",
            Self::_6 => "6",
            _ => "?",
        }
    }
}

pub type GuitarFretboard = Fretboard<GUITAR_STRING_NUM>;

pub type GuitarStrings = [Note; GUITAR_STRING_NUM];
pub type GuitarHandShape = HandShape<GUITAR_STRING_NUM>;

pub type GuitarEntry = FrettedEntry<GUITAR_STRING_NUM>;

pub fn new_guitar_fretboard(total_fret_num: usize, strings: GuitarStrings) -> GuitarFretboard {
    GuitarFretboard {
        total_fret_num,
        string_notes: strings,
        capo: 0,
    }
}

pub fn new_acoustic_guitar_fretboard(tuning: GuitarTuning) -> GuitarFretboard {
    new_guitar_fretboard(GUITAR_FRET_NUM_ACOUSTIC, tuning.into())
}

pub fn new_classical_guitar_fretboard(tuning: GuitarTuning) -> GuitarFretboard {
    new_guitar_fretboard(GUITAR_FRET_NUM_CLASSICAL, tuning.into())
}

pub fn new_electric_guitar_fretboard(tuning: GuitarTuning) -> GuitarFretboard {
    new_guitar_fretboard(GUITAR_FRET_NUM_ELECTRIC, tuning.into())
}

use crate::tuning::GuitarTuning;
use notation_core::prelude::Note;
use notation_fretted::prelude::{Fretboard, FrettedEntry, HandShape};

pub const GUITAR_STRING_NUM: usize = 6;

pub const GUITAR_FRET_NUM_ACOUSTIC: usize = 20;
pub const GUITAR_FRET_NUM_CLASSICAL: usize = 19;
pub const GUITAR_FRET_NUM_ELECTRIC: usize = 22;

pub type GuitarFretboard = Fretboard<GUITAR_STRING_NUM>;

pub type GuitarStrings = [Note; GUITAR_STRING_NUM];
pub type GuitarHandShape = HandShape<GUITAR_STRING_NUM>;

pub type GuitarEntry = FrettedEntry<GUITAR_STRING_NUM>;

pub struct GuitarUtil();

impl GuitarUtil {
    pub fn new_guitar_fretboard(total_fret_num: usize, strings: GuitarStrings) -> GuitarFretboard {
        GuitarFretboard {
            total_fret_num,
            string_notes: strings,
            capo: 0,
        }
    }

    pub fn new_acoustic_guitar_fretboard(tuning: Option<GuitarTuning>) -> GuitarFretboard {
        Self::new_guitar_fretboard(GUITAR_FRET_NUM_ACOUSTIC, tuning.unwrap_or_default().into())
    }

    pub fn new_classical_guitar_fretboard(tuning: Option<GuitarTuning>) -> GuitarFretboard {
        Self::new_guitar_fretboard(GUITAR_FRET_NUM_CLASSICAL, tuning.unwrap_or_default().into())
    }

    pub fn new_electric_guitar_fretboard(tuning: Option<GuitarTuning>) -> GuitarFretboard {
        Self::new_guitar_fretboard(GUITAR_FRET_NUM_ELECTRIC, tuning.unwrap_or_default().into())
    }

    pub fn new_default_fretboard() -> GuitarFretboard {
        GuitarUtil::new_acoustic_guitar_fretboard(None)
    }
}

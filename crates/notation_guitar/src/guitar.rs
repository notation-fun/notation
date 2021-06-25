use notation_core::prelude::Note;
use notation_fretted::prelude::{Fretboard, FrettedEntry, HandShape, WithCapo};

pub const GUITAR_STRING_NUM: usize = 6;

pub const GUITAR_FRET_NUM_ACOUSTIC: usize = 20;
pub const GUITAR_FRET_NUM_CLASSICAL: usize = 19;
pub const GUITAR_FRET_NUM_ELECTRIC: usize = 22;

pub type GuitarFretboard = Fretboard<GUITAR_STRING_NUM>;
pub type GuitarWithCapo = WithCapo<GUITAR_STRING_NUM>;

pub type GuitarTuning = [Note; GUITAR_STRING_NUM];
pub type GuitarHandShape = HandShape<GUITAR_STRING_NUM>;

pub type GuitarEntry = FrettedEntry<GUITAR_STRING_NUM>;

use serde::{Serialize, Deserialize};

use notation_core::prelude::{Note, Semitones};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct HandShape<const S: usize> {
    #[serde(with = "serde_arrays")]
    pub frets: [Option<u8>; S],
}

impl<const S: usize> HandShape<S> {
    pub fn string_fret(&self, string: u8) -> Option<u8> {
        if string as usize >= self.frets.len() {
            None
        } else {
            self.frets[string as usize]
        }
    }
}

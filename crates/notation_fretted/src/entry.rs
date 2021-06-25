use serde::{Serialize, Deserialize};

use notation_core::prelude::{Duration, Entry};
use crate::prelude::{HandShape, Pick, Strum};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum FrettedEntry<const S: usize> {
    Shape (HandShape<S>, Duration),
    Pick (Pick, Duration),
    Strum (Strum, Duration),
}

impl<const S: usize> FrettedEntry<S> {
    pub fn duration(&self) -> Duration {
        match self {
            FrettedEntry::Shape(_, duration) => *duration,
            FrettedEntry::Pick(_, duration) => *duration,
            FrettedEntry::Strum(_, duration) => *duration,
        }
    }
}

impl<const S: usize> Entry for FrettedEntry<S> {
    fn duration(&self) -> Duration {
        self.duration()
    }
}

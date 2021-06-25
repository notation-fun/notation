use serde::{Serialize, Deserialize};

use notation_core::prelude::Note;
use crate::guitar::GuitarTuning;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Tuning {
    Standard,
    StandardFlat,
    DropD,
}

impl From<Tuning> for GuitarTuning {
    fn from(v: Tuning) -> Self {
        match v {
            Tuning::Standard =>
                [Note::E_2, Note::A_2, Note::D_3, Note::G_3, Note::B_3, Note::E_4],
            Tuning::StandardFlat =>
                [Note::E_FLAT_2, Note::A_FLAT_2, Note::D_FLAT_3, Note::G_FLAT_3, Note::B_FLAT_3, Note::E_FLAT_4],
            Tuning::DropD =>
                [Note::D_2, Note::A_2, Note::D_3, Note::G_3, Note::B_3, Note::E_4],
        }
    }
}
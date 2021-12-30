use serde::{Deserialize, Serialize};

use notation_core::prelude::Note;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum GuitarTuning {
    Standard,
    StandardFlat,
    DropD,
}

impl Default for GuitarTuning {
    fn default() -> Self {
        Self::Standard
    }
}

impl From<GuitarTuning> for [Note; 6] {
    fn from(v: GuitarTuning) -> Self {
        match v {
            GuitarTuning::Standard => [
                Note::E_4,
                Note::B_3,
                Note::G_3,
                Note::D_3,
                Note::A_2,
                Note::E_2,
            ],
            GuitarTuning::StandardFlat => [
                Note::E_FLAT_4,
                Note::B_FLAT_3,
                Note::G_FLAT_3,
                Note::D_FLAT_3,
                Note::A_FLAT_2,
                Note::E_FLAT_2,
            ],
            GuitarTuning::DropD => [
                Note::E_4,
                Note::B_3,
                Note::G_3,
                Note::D_3,
                Note::A_2,
                Note::D_2,
            ],
        }
    }
}

impl GuitarTuning {
    pub fn to_ident(&self) -> String {
        format!("{:?}", self)
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "StandardFlat" => Self::StandardFlat,
            "DropD" => Self::DropD,
            _ => Self::Standard,
        }
    }
}

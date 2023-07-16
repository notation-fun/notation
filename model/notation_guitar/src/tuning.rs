use serde::{Deserialize, Serialize};

use notation_core::prelude::{Semitones, Pitch, Octave};

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

impl From<GuitarTuning> for [Semitones; 6] {
    fn from(v: GuitarTuning) -> Self {
        match v {
            GuitarTuning::Standard => [
                (Pitch::E, Octave::P4).into(),
                (Pitch::B, Octave::P3).into(),
                (Pitch::G, Octave::P3).into(),
                (Pitch::D, Octave::P3).into(),
                (Pitch::A, Octave::P2).into(),
                (Pitch::E, Octave::P2).into(),
            ],
            GuitarTuning::StandardFlat => [
                (Pitch::E_FLAT, Octave::P4).into(),
                (Pitch::B_FLAT, Octave::P3).into(),
                (Pitch::G_FLAT, Octave::P3).into(),
                (Pitch::D_FLAT, Octave::P3).into(),
                (Pitch::A_FLAT, Octave::P2).into(),
                (Pitch::E_FLAT, Octave::P2).into(),
            ],
            GuitarTuning::DropD => [
                (Pitch::E, Octave::P4).into(),
                (Pitch::B, Octave::P3).into(),
                (Pitch::G, Octave::P3).into(),
                (Pitch::D, Octave::P3).into(),
                (Pitch::A, Octave::P2).into(),
                (Pitch::D, Octave::P2).into(),
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

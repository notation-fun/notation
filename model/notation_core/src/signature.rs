use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::{Unit, Units};

#[derive(Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct Beats(pub f32);

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Signature {
    pub beat_unit: Unit,
    pub bar_beats: u8,
}
impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.bar_beats, match self.beat_unit {
            Unit::Whole => 1,
            Unit::Half => 2,
            Unit::Quarter => 4,
            Unit::Eighth => 8,
            Unit::Sixteenth => 16,
            Unit::ThirtySecondth => 32,
        })
    }
}

impl Signature {
    pub fn new(beat_unit: Unit, bar_beats: u8) -> Self {
        Self {
            beat_unit,
            bar_beats,
        }
    }
}

impl Signature {
    pub const _4_4: Self = Self {
        beat_unit: Unit::Quarter,
        bar_beats: 4,
    };
    pub const _3_4: Self = Self {
        beat_unit: Unit::Quarter,
        bar_beats: 3,
    };
    pub const _2_4: Self = Self {
        beat_unit: Unit::Quarter,
        bar_beats: 2,
    };
    pub const _6_8: Self = Self {
        beat_unit: Unit::Eighth,
        bar_beats: 6,
    };
}

impl From<f32> for Beats {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

impl From<Signature> for Units {
    fn from(v: Signature) -> Self {
        Self::from(Units::from(v.beat_unit).0 * (v.bar_beats as f32))
    }
}

impl From<Signature> for Beats {
    fn from(v: Signature) -> Self {
        Self::from(v.bar_beats as f32)
    }
}

impl From<(Signature, Units)> for Beats {
    fn from((signature, units): (Signature, Units)) -> Self {
        Self::from(units.0 / Units::from(signature.beat_unit).0)
    }
}

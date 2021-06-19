use serde::{Serialize, Deserialize};

use crate::prelude::{EntryWrap, PitchName};

// https://hellomusictheory.com/learn/music-scales-beginners-guide/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Scale {
    Major, Minor,
}

pub type ScaleEntry = EntryWrap<Scale>;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Key {
    Natural(PitchName),
    Sharp(PitchName),
    Flat(PitchName),
}

pub type KeyEntry = EntryWrap<Key>;

impl Key {
    pub const A: Self = Self::Natural(PitchName::A);
    pub const B: Self = Self::Natural(PitchName::B);
    pub const C: Self = Self::Natural(PitchName::C);
    pub const D: Self = Self::Natural(PitchName::D);
    pub const E: Self = Self::Natural(PitchName::E);
    pub const F: Self = Self::Natural(PitchName::F);
    pub const G: Self = Self::Natural(PitchName::G);

    pub const A_SHARP: Self = Self::Sharp(PitchName::A);
    pub const C_SHARP: Self = Self::Sharp(PitchName::C);
    pub const D_SHARP: Self = Self::Sharp(PitchName::D);
    pub const F_SHARP: Self = Self::Sharp(PitchName::F);
    pub const G_SHARP: Self = Self::Sharp(PitchName::G);

    pub const A_FLAT: Self = Self::Flat(PitchName::A);
    pub const B_FLAT: Self = Self::Flat(PitchName::B);
    pub const D_FLAT: Self = Self::Flat(PitchName::D);
    pub const E_FLAT: Self = Self::Flat(PitchName::E);
    pub const G_FLAT: Self = Self::Flat(PitchName::G);
}
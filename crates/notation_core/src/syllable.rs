use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::Semitones;

// https://en.wikipedia.org/wiki/Solf%C3%A8ge
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Syllable {
    Do,
    Re,
    Mi,
    Fa,
    So,
    La,
    Ti,
    Di,
    Ri,
    Fi,
    Si,
    Li,
    Ra,
    Me,
    Se,
    Le,
    Te,
}

impl Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Syllable> for Semitones {
    fn from(v: Syllable) -> Self {
        match v {
            Syllable::Do => 0,
            Syllable::Re => 2,
            Syllable::Mi => 4,
            Syllable::Fa => 5,
            Syllable::So => 7,
            Syllable::La => 9,
            Syllable::Ti => 11,
            Syllable::Di => 1,
            Syllable::Ri => 3,
            Syllable::Fi => 6,
            Syllable::Si => 8,
            Syllable::Li => 10,
            Syllable::Ra => 1,
            Syllable::Me => 3,
            Syllable::Se => 6,
            Syllable::Le => 8,
            Syllable::Te => 10,
        }
        .into()
    }
}

impl From<Semitones> for Syllable {
    fn from(v: Semitones) -> Self {
        let pos_val = if v.0 >= 0 { v.0 % 12 } else { v.0 % 12 + 12 };
        match pos_val {
            0 => Syllable::Do,
            1 => Syllable::Di,
            2 => Syllable::Re,
            3 => Syllable::Ri,
            4 => Syllable::Mi,
            5 => Syllable::Fa,
            6 => Syllable::Fi,
            7 => Syllable::So,
            8 => Syllable::Si,
            9 => Syllable::La,
            10 => Syllable::Li,
            11 => Syllable::Ti,
            _ => Syllable::Do,
        }
    }
}

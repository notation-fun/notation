use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::prelude::Semitones;

// https://en.wikipedia.org/wiki/Solf%C3%A8ge
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Syllable {
    //Natural
    Do,
    Re,
    Mi,
    Fa,
    So,
    La,
    Ti,
    //Sharp
    Di,
    Ri,
    Fi,
    Si,
    Li,
    //Flat
    Ra,
    Me,
    Se,
    Le,
    Te,
}

impl Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ident())
    }
}

impl Default for Syllable {
    fn default() -> Self {
        Self::Do
    }
}

impl Syllable {
    pub fn to_text(&self) -> String {
        match self {
            //Natural
            Syllable::Do => "1",
            Syllable::Re => "2",
            Syllable::Mi => "3",
            Syllable::Fa => "4",
            Syllable::So => "5",
            Syllable::La => "6",
            Syllable::Ti => "7",
            //Sharp
            Syllable::Di => "#1",
            Syllable::Ri => "#2",
            Syllable::Fi => "#4",
            Syllable::Si => "#5",
            Syllable::Li => "#6",
            //Flat
            Syllable::Ra => "b2",
            Syllable::Me => "b3",
            Syllable::Se => "b5",
            Syllable::Le => "b6",
            Syllable::Te => "b7",
        }
        .to_owned()
    }
    pub fn from_text(text: &str) -> Self {
        match text {
            //Natural
            "1" => Syllable::Do,
            "2" => Syllable::Re,
            "3" => Syllable::Mi,
            "4" => Syllable::Fa,
            "5" => Syllable::So,
            "6" => Syllable::La,
            "7" => Syllable::Ti,
            //Sharp
            "#1" => Syllable::Di,
            "#2" => Syllable::Ri,
            "#4" => Syllable::Fi,
            "#5" => Syllable::Si,
            "#6" => Syllable::Li,
            //Flat
            "b2" => Syllable::Ra,
            "b3" => Syllable::Me,
            "b5" => Syllable::Se,
            "b6" => Syllable::Le,
            "b7" => Syllable::Te,
            _ => Syllable::Do,
        }
    }
}

impl Syllable {
    pub fn to_ident(&self) -> String {
        format!("{:?}", self)
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            //Natural
            "Do" => Syllable::Do,
            "Re" => Syllable::Re,
            "Mi" => Syllable::Mi,
            "Fa" => Syllable::Fa,
            "So" => Syllable::So,
            "La" => Syllable::La,
            "Ti" => Syllable::Ti,
            //Sharp
            "Di" => Syllable::Di,
            "Ri" => Syllable::Ri,
            "Fi" => Syllable::Fi,
            "Si" => Syllable::Si,
            "Li" => Syllable::Li,
            //Flat
            "Ra" => Syllable::Ra,
            "Me" => Syllable::Me,
            "Se" => Syllable::Se,
            "Le" => Syllable::Le,
            "Te" => Syllable::Te,
            _ => Syllable::Do,
        }
    }
}

impl From<Syllable> for Semitones {
    fn from(v: Syllable) -> Self {
        match v {
            //Natural
            Syllable::Do => 0,
            Syllable::Re => 2,
            Syllable::Mi => 4,
            Syllable::Fa => 5,
            Syllable::So => 7,
            Syllable::La => 9,
            Syllable::Ti => 11,
            //Sharp
            Syllable::Di => 1,
            Syllable::Ri => 3,
            Syllable::Fi => 6,
            Syllable::Si => 8,
            Syllable::Li => 10,
            //Flat
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

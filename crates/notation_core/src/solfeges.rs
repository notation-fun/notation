use serde::{Deserialize, Serialize};

use super::solfege::Solfege;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Solfeges {
    Double(Solfege, Solfege),
    Triple(Solfege, Solfege, Solfege),
    Tetra(Solfege, Solfege, Solfege, Solfege),
    Penta(Solfege, Solfege, Solfege, Solfege, Solfege),
    Hexa(Solfege, Solfege, Solfege, Solfege, Solfege, Solfege),
}

impl From<(Solfege, Solfege)> for Solfeges {
    fn from(v: (Solfege, Solfege)) -> Self {
        Solfeges::Double(v.0, v.1)
    }
}

impl From<(Solfege, Solfege, Solfege)> for Solfeges {
    fn from(v: (Solfege, Solfege, Solfege)) -> Self {
        Solfeges::Triple(v.0, v.1, v.2)
    }
}

impl From<(Solfege, Solfege, Solfege, Solfege)> for Solfeges {
    fn from(v: (Solfege, Solfege, Solfege, Solfege)) -> Self {
        Solfeges::Tetra(v.0, v.1, v.2, v.3)
    }
}

impl From<(Solfege, Solfege, Solfege, Solfege, Solfege)> for Solfeges {
    fn from(v: (Solfege, Solfege, Solfege, Solfege, Solfege)) -> Self {
        Solfeges::Penta(v.0, v.1, v.2, v.3, v.4)
    }
}

impl From<(Solfege, Solfege, Solfege, Solfege, Solfege, Solfege)> for Solfeges {
    fn from(v: (Solfege, Solfege, Solfege, Solfege, Solfege, Solfege)) -> Self {
        Solfeges::Hexa(v.0, v.1, v.2, v.3, v.4, v.5)
    }
}

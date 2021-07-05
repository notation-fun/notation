use serde::{Deserialize, Serialize};

use super::note::Note;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Notes {
    Double(Note, Note),
    Triple(Note, Note, Note),
    Tetra(Note, Note, Note, Note),
    Penta(Note, Note, Note, Note, Note),
    Hexa(Note, Note, Note, Note, Note, Note),
}

impl From<(Note, Note)> for Notes {
    fn from(v: (Note, Note)) -> Self {
        Notes::Double(v.0, v.1)
    }
}

impl From<(Note, Note, Note)> for Notes {
    fn from(v: (Note, Note, Note)) -> Self {
        Notes::Triple(v.0, v.1, v.2)
    }
}

impl From<(Note, Note, Note, Note)> for Notes {
    fn from(v: (Note, Note, Note, Note)) -> Self {
        Notes::Tetra(v.0, v.1, v.2, v.3)
    }
}

impl From<(Note, Note, Note, Note, Note)> for Notes {
    fn from(v: (Note, Note, Note, Note, Note)) -> Self {
        Notes::Penta(v.0, v.1, v.2, v.3, v.4)
    }
}

impl From<(Note, Note, Note, Note, Note, Note)> for Notes {
    fn from(v: (Note, Note, Note, Note, Note, Note)) -> Self {
        Notes::Hexa(v.0, v.1, v.2, v.3, v.4, v.5)
    }
}

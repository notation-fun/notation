use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::note::Note;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Tone {
    Single(Note),
    Double(Note, Note),
    Triple(Note, Note, Note),
    Tetra(Note, Note, Note, Note),
    Penta(Note, Note, Note, Note, Note),
    Hexa(Note, Note, Note, Note, Note, Note),
}
impl Display for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Tone::Single(a) => write!(f, "<Tone>({})", a),
            Tone::Double(a, b) => write!(f, "<Tone>({}, {})", a, b),
            Tone::Triple(a, b, c) => write!(f, "<Tone>({}, {}, {})", a, b, c),
            Tone::Tetra(a, b, c, d) => {
                write!(f, "<Tone>({}, {}, {}, {})", a, b, c, d)
            }
            Tone::Penta(a, b, c, d, e) => {
                write!(f, "<Tone>({}, {}, {}, {}, {})", a, b, c, d, e)
            }
            Tone::Hexa(a, b, c, d, e, g) => {
                write!(f, "<Tone>({}, {}, {}, {}, {}, {})", a, b, c, d, e, g)
            }
        }
    }
}

impl From<Note> for Tone {
    fn from(v: Note) -> Self {
        Tone::Single(v)
    }
}

impl From<(Note, Note)> for Tone {
    fn from(v: (Note, Note)) -> Self {
        Tone::Double(v.0, v.1)
    }
}

impl From<(Note, Note, Note)> for Tone {
    fn from(v: (Note, Note, Note)) -> Self {
        Tone::Triple(v.0, v.1, v.2)
    }
}

impl From<(Note, Note, Note, Note)> for Tone {
    fn from(v: (Note, Note, Note, Note)) -> Self {
        Tone::Tetra(v.0, v.1, v.2, v.3)
    }
}

impl From<(Note, Note, Note, Note, Note)> for Tone {
    fn from(v: (Note, Note, Note, Note, Note)) -> Self {
        Tone::Penta(v.0, v.1, v.2, v.3, v.4)
    }
}

impl From<(Note, Note, Note, Note, Note, Note)> for Tone {
    fn from(v: (Note, Note, Note, Note, Note, Note)) -> Self {
        Tone::Hexa(v.0, v.1, v.2, v.3, v.4, v.5)
    }
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::note::Note;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Tone {
    None,
    Single(Note),
    Double(Note, Note),
    Triple(Note, Note, Note),
    Tetra(Note, Note, Note, Note),
    Penta(Note, Note, Note, Note, Note),
    Hexa(Note, Note, Note, Note, Note, Note),
}

impl Tone {
    /// Returns `true` if the tone is [`None`].
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
impl Display for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tone::None => write!(f, "<Tone>()"),
            Tone::Single(n1) => write!(f, "<Tone>({})", n1),
            Tone::Double(n1, n2) => write!(f, "<Tone>({}, {})", n1, n2),
            Tone::Triple(n1, n2, n3) => write!(f, "<Tone>({}, {}, {})", n1, n2, n3),
            Tone::Tetra(n1, n2, n3, n4) => {
                write!(f, "<Tone>({}, {}, {}, {})", n1, n2, n3, n4)
            }
            Tone::Penta(n1, n2, n3, n4, n5) => {
                write!(f, "<Tone>({}, {}, {}, {}, {})", n1, n2, n3, n4, n5)
            }
            Tone::Hexa(n1, n2, n3, n4, n5, n6) => {
                write!(f, "<Tone>({}, {}, {}, {}, {}, {})", n1, n2, n3, n4, n5, n6)
            }
        }
    }
}

impl From<()> for Tone {
    fn from(_: ()) -> Self {
        Self::None
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

impl From<Vec<Note>> for Tone {
    fn from(v: Vec<Note>) -> Self {
        match v.len() {
            1 => Self::from(v[0]),
            2 => Self::from((v[0], v[1])),
            3 => Self::from((v[0], v[1], v[2])),
            4 => Self::from((v[0], v[1], v[2], v[3])),
            5 => Self::from((v[0], v[1], v[2], v[3], v[4])),
            _ => {
                println!("Unsupported Tones: {}", v.len());
                Self::None
            }
        }
    }
}

impl From<Vec<Option<Note>>> for Tone {
    fn from(v: Vec<Option<Note>>) -> Self {
        let notes = v
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<Note>>();
        notes.into()
    }
}

impl Tone {
    pub fn get_notes(&self) -> Vec<Note> {
        match *self {
            Self::None => vec![],
            Self::Single(n1) => vec![n1],
            Self::Double(n1, n2) => vec![n1, n2],
            Self::Triple(n1, n2, n3) => vec![n1, n2, n3],
            Self::Tetra(n1, n2, n3, n4) => vec![n1, n2, n3, n4],
            Self::Penta(n1, n2, n3, n4, n5) => vec![n1, n2, n3, n4, n5],
            Self::Hexa(n1, n2, n3, n4, n5, n6) => vec![n1, n2, n3, n4, n5, n6],
        }
    }
}

impl From<Tone> for Vec<Note> {
    fn from(v: Tone) -> Self {
        v.get_notes()
    }
}

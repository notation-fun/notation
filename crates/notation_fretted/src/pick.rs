use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::Finger;
use crate::strum::StrumDirection;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PickNote {
    pub string: u8,
    pub fret: Option<u8>,
    pub fret_finger: Option<Finger>,
    pub pick_finger: Option<Finger>,
    pub pick_direction: Option<StrumDirection>,
}
impl Display for PickNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.string,
            self.fret
                .map(|x| format!("@{}", x))
                .unwrap_or("".to_string()),
            self.fret_finger
                .map(|x| format!("_{}", x))
                .unwrap_or("".to_string()),
            self.pick_finger
                .map(|x| format!("^{}", x))
                .unwrap_or("".to_string()),
            self.pick_direction
                .map(|x| format!("*{}", x))
                .unwrap_or("".to_string()),
        )
    }
}
impl PickNote {
    pub fn new(
        string: u8,
        fret: Option<u8>,
        fret_finger: Option<Finger>,
        pick_finger: Option<Finger>,
        pick_direction: Option<StrumDirection>,
    ) -> Self {
        Self {
            string,
            fret,
            fret_finger,
            pick_finger,
            pick_direction,
        }
    }
    pub fn new_string(string: u8) -> Self {
        Self::new(string, None, None, None, None)
    }
    pub fn new_string_fret(string: u8, fret: u8) -> Self {
        Self::new(string, Some(fret), None, None, None)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Pick {
    None,
    Single(PickNote),
    Double(PickNote, PickNote),
    Triple(PickNote, PickNote, PickNote),
    Tetra(PickNote, PickNote, PickNote, PickNote),
    Penta(PickNote, PickNote, PickNote, PickNote, PickNote),
    Hexa(PickNote, PickNote, PickNote, PickNote, PickNote, PickNote),
}

impl Display for Pick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Pick::None => write!(f, "<Pick>()"),
            Pick::Single(p1) => write!(f, "<Pick>({})", p1),
            Pick::Double(p1, p2) => write!(f, "<Pick>({}, {})", p1, p2),
            Pick::Triple(p1, p2, p3) => write!(f, "<Pick>({}, {}, {})", p1, p2, p3),
            Pick::Tetra(p1, p2, p3, p4) => {
                write!(f, "<Pick>({}, {}, {}, {})", p1, p2, p3, p4)
            }
            Pick::Penta(p1, p2, p3, p4, p5) => {
                write!(f, "<Pick>({}, {}, {}, {}, {})", p1, p2, p3, p4, p5)
            }
            Pick::Hexa(p1, p2, p3, p4, p5, p6) => {
                write!(f, "<Pick>({}, {}, {}, {}, {}, {})", p1, p2, p3, p4, p5, p6)
            }
        }
    }
}

impl From<()> for Pick {
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl From<PickNote> for Pick {
    fn from(v: PickNote) -> Self {
        Self::Single(v)
    }
}

impl From<(PickNote, PickNote)> for Pick {
    fn from(v: (PickNote, PickNote)) -> Self {
        Self::Double(v.0, v.1)
    }
}

impl From<(PickNote, PickNote, PickNote)> for Pick {
    fn from(v: (PickNote, PickNote, PickNote)) -> Self {
        Self::Triple(v.0, v.1, v.2)
    }
}

impl From<(PickNote, PickNote, PickNote, PickNote)> for Pick {
    fn from(v: (PickNote, PickNote, PickNote, PickNote)) -> Self {
        Self::Tetra(v.0, v.1, v.2, v.3)
    }
}

impl From<(PickNote, PickNote, PickNote, PickNote, PickNote)> for Pick {
    fn from(v: (PickNote, PickNote, PickNote, PickNote, PickNote)) -> Self {
        Self::Penta(v.0, v.1, v.2, v.3, v.4)
    }
}

impl From<(PickNote, PickNote, PickNote, PickNote, PickNote, PickNote)> for Pick {
    fn from(v: (PickNote, PickNote, PickNote, PickNote, PickNote, PickNote)) -> Self {
        Self::Hexa(v.0, v.1, v.2, v.3, v.4, v.5)
    }
}

impl From<Vec<PickNote>> for Pick {
    fn from(v: Vec<PickNote>) -> Self {
        match v.len() {
            0 => Self::None,
            1 => Self::from(v[0]),
            2 => Self::from((v[0], v[1])),
            3 => Self::from((v[0], v[1], v[2])),
            4 => Self::from((v[0], v[1], v[2], v[3])),
            5 => Self::from((v[0], v[1], v[2], v[3], v[4])),
            6 => Self::from((v[0], v[1], v[2], v[3], v[4], v[5])),
            _ => {
                println!("PickNote lost: {}", v.len() - 6);
                Self::from((v[0], v[1], v[2], v[3], v[4], v[5]))
            }
        }
    }
}

impl Pick {
    pub fn get_notes(&self) -> Vec<PickNote> {
        match *self {
            Self::None => vec![],
            Self::Single(p1) => vec![p1],
            Self::Double(p1, p2) => vec![p1, p2],
            Self::Triple(p1, p2, p3) => vec![p1, p2, p3],
            Self::Tetra(p1, p2, p3, p4) => vec![p1, p2, p3, p4],
            Self::Penta(p1, p2, p3, p4, p5) => vec![p1, p2, p3, p4, p5],
            Self::Hexa(p1, p2, p3, p4, p5, p6) => vec![p1, p2, p3, p4, p5, p6],
        }
    }
}

impl From<Pick> for Vec<PickNote> {
    fn from(v: Pick) -> Self {
        v.get_notes()
    }
}

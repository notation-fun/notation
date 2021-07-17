use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Pick {
    None,
    Single(u8),
    Double(u8, u8),
    Triple(u8, u8, u8),
    Tetra(u8, u8, u8, u8),
    Penta(u8, u8, u8, u8, u8),
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
        }
    }
}

impl From<()> for Pick {
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl From<u8> for Pick {
    fn from(v: u8) -> Self {
        Self::Single(v)
    }
}

impl From<(u8, u8)> for Pick {
    fn from(v: (u8, u8)) -> Self {
        Self::Double(v.0, v.1)
    }
}

impl From<(u8, u8, u8)> for Pick {
    fn from(v: (u8, u8, u8)) -> Self {
        Self::Triple(v.0, v.1, v.2)
    }
}

impl From<(u8, u8, u8, u8)> for Pick {
    fn from(v: (u8, u8, u8, u8)) -> Self {
        Self::Tetra(v.0, v.1, v.2, v.3)
    }
}

impl From<(u8, u8, u8, u8, u8)> for Pick {
    fn from(v: (u8, u8, u8, u8, u8)) -> Self {
        Self::Penta(v.0, v.1, v.2, v.3, v.4)
    }
}

impl From<Vec<u8>> for Pick {
    fn from(v: Vec<u8>) -> Self {
        match v.len() {
            1 => Self::from(v[0]),
            2 => Self::from((v[0], v[1])),
            3 => Self::from((v[0], v[1], v[2])),
            4 => Self::from((v[0], v[1], v[2], v[3])),
            5 => Self::from((v[0], v[1], v[2], v[3], v[4])),
            _ => Self::None,
        }
    }
}

impl Pick {
    pub fn get_strings(&self) -> Vec<u8> {
        match *self {
            Self::None => vec![],
            Self::Single(p1) => vec![p1],
            Self::Double(p1, p2) => vec![p1, p2],
            Self::Triple(p1, p2, p3) => vec![p1, p2, p3],
            Self::Tetra(p1, p2, p3, p4) => vec![p1, p2, p3, p4],
            Self::Penta(p1, p2, p3, p4, p5) => vec![p1, p2, p3, p4, p5],
        }
    }
}

impl From<Pick> for Vec<u8> {
    fn from(v: Pick) -> Self {
        v.get_strings()
    }
}

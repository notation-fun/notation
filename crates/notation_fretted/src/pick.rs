use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Pick {
    Single(u8),
    Double(u8, u8),
    Triple(u8, u8, u8),
    Tetra(u8, u8, u8, u8),
    Penta(u8, u8, u8, u8, u8),
}

impl Display for Pick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Pick::Single(a) => write!(f, "<Pick>({})", a + 1),
            Pick::Double(a, b) => write!(f, "<Pick>({}, {})", a + 1, b + 1),
            Pick::Triple(a, b, c) => write!(f, "<Pick>({}, {}, {})", a + 1, b + 1, c + 1),
            Pick::Tetra(a, b, c, d) => {
                write!(f, "<Pick>({}, {}, {}, {})", a + 1, b + 1, c + 1, d + 1)
            }
            Pick::Penta(a, b, c, d, e) => write!(
                f,
                "<Pick>({}, {}, {}, {}, {})",
                a + 1,
                b + 1,
                c + 1,
                d + 1,
                e + 1
            ),
        }
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
            0 => Self::from(0),
            1 => Self::from(v[0]),
            2 => Self::from((v[0], v[1])),
            3 => Self::from((v[0], v[1], v[2])),
            4 => Self::from((v[0], v[1], v[2], v[3])),
            _ => Self::from((v[0], v[1], v[2], v[3], v[4])),
        }
    }
}

impl Pick {
    pub fn get_strings(&self) -> Vec<u8> {
        match self.clone() {
            Pick::Single(a) => vec![a],
            Pick::Double(a, b) => vec![a, b],
            Pick::Triple(a, b, c) => vec![a, b, c],
            Pick::Tetra(a, b, c, d) => vec![a, b, c, d],
            Pick::Penta(a, b, c, d, e) => vec![a, b, c, d, e],
        }
    }
}

impl From<Pick> for Vec<u8> {
    fn from(v: Pick) -> Self {
        v.get_strings()
    }
}

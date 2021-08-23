use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::interval::Interval;

//https://en.wikipedia.org/wiki/Chord_(music)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Intervals {
    Monad,
    Dyad(Interval),
    Triad(Interval, Interval),
    Tetrad(Interval, Interval, Interval),
    Pentad(Interval, Interval, Interval, Interval),
}
impl Display for Intervals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Intervals>({})", self.to_text())
    }
}
impl Intervals {
    fn to_text(&self) -> String {
        match self {
            Self::Monad => format!(""),
            Self::Dyad(n1) => format!("{}", n1),
            Self::Triad(n1, n2) => format!("{}, {}", n1, n2),
            Self::Tetrad(n1, n2, n3) => format!("{}, {}, {}", n1, n2, n3),
            Self::Pentad(n1, n2, n3, n4) => format!("{}, {}, {}, {}", n1, n2, n3, n4),
        }
    }
}
impl From<()> for Intervals {
    fn from(_: ()) -> Self {
        Self::Monad
    }
}

impl From<Interval> for Intervals {
    fn from(v: Interval) -> Self {
        Intervals::Dyad(v)
    }
}

impl From<(Interval, Interval)> for Intervals {
    fn from(v: (Interval, Interval)) -> Self {
        Intervals::Triad(v.0, v.1)
    }
}

impl From<(Interval, Interval, Interval)> for Intervals {
    fn from(v: (Interval, Interval, Interval)) -> Self {
        Intervals::Tetrad(v.0, v.1, v.2)
    }
}

impl From<(Interval, Interval, Interval, Interval)> for Intervals {
    fn from(v: (Interval, Interval, Interval, Interval)) -> Self {
        Intervals::Pentad(v.0, v.1, v.2, v.3)
    }
}

impl From<Vec<Interval>> for Intervals {
    fn from(v: Vec<Interval>) -> Self {
        match v.len() {
            1 => Self::from(v[0]),
            2 => Self::from((v[0], v[1])),
            3 => Self::from((v[0], v[1], v[2])),
            4 => Self::from((v[0], v[1], v[2], v[3])),
            _ => {
                println!("Unsupported Intervals: {}", v.len());
                Self::Monad
            }
        }
    }
}

impl From<Vec<Option<Interval>>> for Intervals {
    fn from(v: Vec<Option<Interval>>) -> Self {
        let notes = v
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<Interval>>();
        notes.into()
    }
}

impl Intervals {
    pub fn get_intervals(&self) -> Vec<Interval> {
        match *self {
            Self::Monad => vec![],
            Self::Dyad(n1) => vec![n1],
            Self::Triad(n1, n2) => vec![n1, n2],
            Self::Tetrad(n1, n2, n3) => vec![n1, n2, n3],
            Self::Pentad(n1, n2, n3, n4) => vec![n1, n2, n3, n4],
        }
    }
}

impl From<Intervals> for Vec<Interval> {
    fn from(v: Intervals) -> Self {
        v.get_intervals()
    }
}

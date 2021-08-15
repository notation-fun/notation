use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::interval::Interval;
use crate::prelude::{Intervals, Syllable};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Chord {
    pub root: Syllable,
    pub intervals: Intervals,
    pub base: Option<usize>,
}
impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = match self.base {
            Some(base) => format!(" /{}", base),
            None => "".to_owned(),
        };
        write!(f, "<Chord>({}: {}{})", self.root, self.intervals, base)
    }
}
impl Chord {
    pub fn new(root: Syllable, intervals: Intervals, base: Option<usize>) -> Self {
        Self {
            root,
            intervals,
            base,
        }
    }
}

impl From<(Syllable, Intervals)> for Chord {
    fn from(v: (Syllable, Intervals)) -> Self {
        Self::new(v.0, v.1, None)
    }
}

impl From<(Syllable, Vec<Interval>)> for Chord {
    fn from(v: (Syllable, Vec<Interval>)) -> Self {
        Self::new(v.0, v.1.into(), None)
    }
}

impl From<(Syllable, Intervals, usize)> for Chord {
    fn from(v: (Syllable, Intervals, usize)) -> Self {
        Self::new(v.0, v.1, Some(v.2))
    }
}

impl From<(Syllable, Vec<Interval>, usize)> for Chord {
    fn from(v: (Syllable, Vec<Interval>, usize)) -> Self {
        Self::new(v.0, v.1.into(), Some(v.2))
    }
}

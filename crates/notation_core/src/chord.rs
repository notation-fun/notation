use std::fmt::Display;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::interval::Interval;
use crate::prelude::{Intervals, Syllable};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Chord {
    pub root: Syllable,
    pub intervals: Intervals,
    pub base: Option<Interval>,
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
    pub fn new(root: Syllable, intervals: Intervals, base: Option<Interval>) -> Self {
        Self {
            root,
            intervals,
            base,
        }
    }
}
impl Hash for Chord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let string = self.to_string();
        string.hash(state);
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

impl From<(Syllable, Intervals, Interval)> for Chord {
    fn from(v: (Syllable, Intervals, Interval)) -> Self {
        Self::new(v.0, v.1, Some(v.2))
    }
}

impl From<(Syllable, Vec<Interval>, Interval)> for Chord {
    fn from(v: (Syllable, Vec<Interval>, Interval)) -> Self {
        Self::new(v.0, v.1.into(), Some(v.2))
    }
}

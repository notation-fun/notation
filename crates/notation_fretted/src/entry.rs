use serde::{Deserialize, Serialize};

use crate::prelude::{Fretboard, HandShape, Pick, Strum};
use notation_core::prelude::{Duration, Entry};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum FrettedEntry<const S: usize> {
    Pick(Pick, Duration),
    Strum(Strum, Duration),
    Shape(HandShape<S>, Duration),
    Fretboard(Fretboard<S>),
}

impl<const S: usize> FrettedEntry<S> {
    pub fn duration(&self) -> Duration {
        match self {
            FrettedEntry::Pick(_, duration) => *duration,
            FrettedEntry::Strum(_, duration) => *duration,
            FrettedEntry::Shape(_, duration) => *duration,
            FrettedEntry::Fretboard(_) => Duration::Zero,
        }
    }
    pub fn clone_<const S1: usize>(&self) -> FrettedEntry<S1> {
        if S != S1 {
            println!("FrettedEntry<{}> unsafe clone_: {}", S, S1);
        }
        match self {
            FrettedEntry::Pick(x, y) => FrettedEntry::<S1>::Pick(*x, *y),
            FrettedEntry::Strum(x, y) => FrettedEntry::<S1>::Strum(*x, *y),
            FrettedEntry::Shape(x, y) => FrettedEntry::<S1>::Shape(x.clone_::<S1>(), *y),
            FrettedEntry::Fretboard(x) => FrettedEntry::<S1>::Fretboard(x.clone_::<S1>()),
        }
    }
}

impl<const S: usize> FrettedEntry<S> {
    /// Returns `true` if the fretted_entry is [`Fretboard`].
    pub fn is_fretboard(&self) -> bool {
        matches!(self, Self::Fretboard(..))
    }

    /// Returns `true` if the fretted_entry is [`Shape`].
    pub fn is_shape(&self) -> bool {
        matches!(self, Self::Shape(..))
    }

    /// Returns `true` if the fretted_entry is [`Pick`].
    pub fn is_pick(&self) -> bool {
        matches!(self, Self::Pick(..))
    }

    /// Returns `true` if the fretted_entry is [`Strum`].
    pub fn is_strum(&self) -> bool {
        matches!(self, Self::Strum(..))
    }
}

impl<const S: usize> FrettedEntry<S> {
    pub fn as_pick(&self) -> Option<(&Pick, &Duration)> {
        if let Self::Pick(v, d) = self {
            Some((v, d))
        } else {
            None
        }
    }
    pub fn as_strum(&self) -> Option<(&Strum, &Duration)> {
        if let Self::Strum(v, d) = self {
            Some((v, d))
        } else {
            None
        }
    }
    pub fn as_shape(&self) -> Option<(&HandShape<S>, &Duration)> {
        if let Self::Shape(v, d) = self {
            Some((v, d))
        } else {
            None
        }
    }
    pub fn as_fretboard(&self) -> Option<&Fretboard<S>> {
        if let Self::Fretboard(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl<const S: usize> Entry for FrettedEntry<S> {
    fn duration(&self) -> Duration {
        self.duration()
    }
}

impl<const S: usize> From<(HandShape<S>, Duration)> for FrettedEntry<S> {
    fn from(v: (HandShape<S>, Duration)) -> Self {
        Self::Shape(v.0, v.1)
    }
}

impl<const S: usize> From<(Pick, Duration)> for FrettedEntry<S> {
    fn from(v: (Pick, Duration)) -> Self {
        Self::Pick(v.0, v.1)
    }
}

impl<const S: usize> From<(Strum, Duration)> for FrettedEntry<S> {
    fn from(v: (Strum, Duration)) -> Self {
        Self::Strum(v.0, v.1)
    }
}

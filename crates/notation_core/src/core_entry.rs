use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::{Chord, Duration, Entry, Tone};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum CoreEntry {
    Tie,
    Rest(Duration),
    Tone(Tone, Duration),
    Chord(Chord, Duration),
}
impl Display for CoreEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreEntry::Tie => write!(f, "Tie()"),
            CoreEntry::Rest(duration) => write!(f, "Rest({})", duration),
            CoreEntry::Tone(tone, duration) => write!(f, "Tone({}, {})", tone, duration),
            CoreEntry::Chord(chord, duration) => write!(f, "Chord({}, {})", chord, duration),
        }
    }
}
impl CoreEntry {
    pub fn duration(&self) -> Duration {
        match self {
            CoreEntry::Tie => Duration::Zero,
            CoreEntry::Rest(duration) => *duration,
            CoreEntry::Tone(_, duration) => *duration,
            CoreEntry::Chord(_, duration) => *duration,
        }
    }
}

impl Entry for CoreEntry {
    fn duration(&self) -> Duration {
        self.duration()
    }
}

impl CoreEntry {
    /// Returns `true` if the core_entry is [`Tie`].
    pub fn is_tie(&self) -> bool {
        matches!(self, Self::Tie)
    }

    /// Returns `true` if the entry is [`Rest`].
    pub fn is_rest(&self) -> bool {
        matches!(self, Self::Rest(..))
    }

    /// Returns `true` if the entry is [`Tone`].
    pub fn is_tone(&self) -> bool {
        matches!(self, Self::Tone(..))
    }

    /// Returns `true` if the entry is [`Chord`].
    pub fn is_chord(&self) -> bool {
        matches!(self, Self::Chord(..))
    }
}

impl CoreEntry {
    pub fn as_rest(&self) -> Option<&Duration> {
        if let Self::Rest(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_tone(&self) -> Option<&Tone> {
        if let Self::Tone(v, _) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_chord(&self) -> Option<&Chord> {
        if let Self::Chord(v, _) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<()> for CoreEntry {
    fn from(_: ()) -> Self {
        Self::Tie
    }
}

impl From<Duration> for CoreEntry {
    fn from(v: Duration) -> Self {
        Self::Rest(v)
    }
}

impl From<(Tone, Duration)> for CoreEntry {
    fn from(v: (Tone, Duration)) -> Self {
        Self::Tone(v.0, v.1)
    }
}

impl From<(Chord, Duration)> for CoreEntry {
    fn from(v: (Chord, Duration)) -> Self {
        Self::Chord(v.0, v.1)
    }
}

use serde::{Deserialize, Serialize};

use crate::prelude::{Chord, Duration, Signature, Tempo, Tone};

pub trait Entry {
    fn duration(&self) -> Duration {
        Duration::Zero
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum CoreEntry {
    Rest(Duration),
    Tone(Tone, Duration),
    Chord(Chord, Duration),
    Signature(Signature),
    Tempo(Tempo),
}

impl CoreEntry {
    pub fn duration(&self) -> Duration {
        match self {
            CoreEntry::Rest(duration) => *duration,
            CoreEntry::Tone(_, duration) => *duration,
            CoreEntry::Chord(_, duration) => *duration,
            CoreEntry::Signature(_) => Duration::Zero,
            CoreEntry::Tempo(_) => Duration::Zero,
        }
    }
}

impl Entry for CoreEntry {
    fn duration(&self) -> Duration {
        self.duration()
    }
}

impl CoreEntry {
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

    /// Returns `true` if the entry is [`Signature`].
    pub fn is_signature(&self) -> bool {
        matches!(self, Self::Signature(..))
    }

    /// Returns `true` if the entry is [`Tempo`].
    pub fn is_tempo(&self) -> bool {
        matches!(self, Self::Tempo(..))
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

impl From<Signature> for CoreEntry {
    fn from(v: Signature) -> Self {
        Self::Signature(v)
    }
}

impl From<Tempo> for CoreEntry {
    fn from(v: Tempo) -> Self {
        Self::Tempo(v)
    }
}

use serde::{Serialize, Deserialize};

use notation_core::prelude::{Duration, Note, Solfege, Chord, Roman, Signature, Tempo};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Entry {
    Rest (Duration),
    Note (Note, Duration),
    Solfege (Solfege, Duration),
    Chord (Chord, Duration),
    Roman (Roman, Duration),
    Signature (Signature),
    Tempo (Tempo),
}

impl Entry {
    pub fn duration(&self) -> Duration {
        match self {
            Entry::Rest(duration) => *duration,
            Entry::Note(_, duration) => *duration,
            Entry::Solfege(_, duration) => *duration,
            Entry::Chord(_, duration) => *duration,
            Entry::Roman(_, duration) => *duration,
            Entry::Signature(_) => Duration::Zero,
            Entry::Tempo(_) => Duration::Zero,
        }
    }

    /// Returns `true` if the entry is [`Rest`].
    pub fn is_rest(&self) -> bool {
        matches!(self, Self::Rest(..))
    }

    /// Returns `true` if the entry is [`Note`].
    pub fn is_note(&self) -> bool {
        matches!(self, Self::Note(..))
    }

    /// Returns `true` if the entry is [`Solfege`].
    pub fn is_solfege(&self) -> bool {
        matches!(self, Self::Solfege(..))
    }

    /// Returns `true` if the entry is [`Chord`].
    pub fn is_chord(&self) -> bool {
        matches!(self, Self::Chord(..))
    }

    /// Returns `true` if the entry is [`Roman`].
    pub fn is_roman(&self) -> bool {
        matches!(self, Self::Roman(..))
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

impl Entry {
    pub fn as_rest(&self) -> Option<&Duration> {
        if let Self::Rest(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_note(&self) -> Option<&Note> {
        if let Self::Note(v, _) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_solfege(&self) -> Option<&Solfege> {
        if let Self::Solfege(v, _) = self {
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

    pub fn as_roman(&self) -> Option<&Roman> {
        if let Self::Roman(v, _) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<Duration> for Entry {
    fn from(v: Duration) -> Self {
        Self::Rest(v)
    }
}

impl From<(Note, Duration)> for Entry {
    fn from(v: (Note, Duration)) -> Self {
        Self::Note(v.0, v.1)
    }
}

impl From<(Solfege, Duration)> for Entry {
    fn from(v: (Solfege, Duration)) -> Self {
        Self::Solfege(v.0, v.1)
    }
}

impl From<(Chord, Duration)> for Entry {
    fn from(v: (Chord, Duration)) -> Self {
        Self::Chord(v.0, v.1)
    }
}

impl From<(Roman, Duration)> for Entry {
    fn from(v: (Roman, Duration)) -> Self {
        Self::Roman(v.0, v.1)
    }
}

impl From<Signature> for Entry {
    fn from(v: Signature) -> Self {
        Self::Signature(v)
    }
}

impl From<Tempo> for Entry {
    fn from(v: Tempo) -> Self {
        Self::Tempo(v)
    }
}

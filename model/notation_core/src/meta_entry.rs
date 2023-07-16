use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::{Entry, Key, Scale, Signature, Tempo};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum MetaEntry {
    Signature(Signature),
    Tempo(Tempo),
    Scale(Scale),
    Key(Key),
}
impl Display for MetaEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaEntry::Signature(x) => write!(f, "Signature({})", x),
            MetaEntry::Tempo(x) => write!(f, "Tempo({})", x),
            MetaEntry::Scale(x) => write!(f, "Scale({})", x),
            MetaEntry::Key(x) => write!(f, "Key({})", x),
        }
    }
}

impl Entry for MetaEntry {}

impl MetaEntry {
    /// Returns `true` if the entry is [`Signature`].
    pub fn is_signature(&self) -> bool {
        matches!(self, Self::Signature(..))
    }

    /// Returns `true` if the entry is [`Tempo`].
    pub fn is_tempo(&self) -> bool {
        matches!(self, Self::Tempo(..))
    }
}

impl From<Signature> for MetaEntry {
    fn from(v: Signature) -> Self {
        Self::Signature(v)
    }
}

impl From<Tempo> for MetaEntry {
    fn from(v: Tempo) -> Self {
        Self::Tempo(v)
    }
}

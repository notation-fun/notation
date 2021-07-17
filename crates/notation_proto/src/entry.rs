use notation_fretted::prelude::FrettedEntry;
use serde::{Deserialize, Serialize};

use notation_core::prelude::{CoreEntry, Duration, Entry};
use notation_fretted::prelude::{Fretboard, HandShape};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ProtoEntry {
    Mark(String),
    Core(CoreEntry),
    FrettedSix(FrettedEntry<6>),
    FrettedFour(FrettedEntry<4>),
}

impl ProtoEntry {
    pub fn duration(&self) -> Duration {
        match self {
            ProtoEntry::Mark(_) => Duration::Zero,
            ProtoEntry::Core(entry) => entry.duration(),
            ProtoEntry::FrettedSix(entry) => entry.duration(),
            ProtoEntry::FrettedFour(entry) => entry.duration(),
        }
    }
    /// Returns `true` if the proto_entry is [`Mark`].
    pub fn is_mark(&self) -> bool {
        matches!(self, Self::Mark(..))
    }
    /// Returns `true` if the proto_entry is [`Core`].
    pub fn is_core(&self) -> bool {
        matches!(self, Self::Core(..))
    }
    pub fn as_mark(&self) -> Option<&String> {
        if let Self::Mark(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_core(&self) -> Option<&CoreEntry> {
        if let Self::Core(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn try_into_core(self) -> Result<CoreEntry, Self> {
        if let Self::Core(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
    /// Returns `true` if the proto_entry is [`FrettedSix`].
    pub fn is_fretted_six(&self) -> bool {
        matches!(self, Self::FrettedSix(..))
    }
    pub fn as_fretted_six(&self) -> Option<&FrettedEntry<6>> {
        if let Self::FrettedSix(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn try_into_fretted_six(self) -> Result<FrettedEntry<6>, Self> {
        if let Self::FrettedSix(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
    /// Returns `true` if the proto_entry is [`FrettedFour`].
    pub fn is_fretted_four(&self) -> bool {
        matches!(self, Self::FrettedFour(..))
    }
    pub fn as_fretted_four(&self) -> Option<&FrettedEntry<4>> {
        if let Self::FrettedFour(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn try_into_fretted_four(self) -> Result<FrettedEntry<4>, Self> {
        if let Self::FrettedFour(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
impl ProtoEntry {
    pub fn is_size_fretted<const S: usize>(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::FrettedSix(_) => S == 6,
            ProtoEntry::FrettedFour(_) => S == 4,
        }
    }
    pub fn cast_size_fretted<const S: usize>(&self) -> Option<FrettedEntry<S>> {
        match self {
            ProtoEntry::Mark(_) => None,
            ProtoEntry::Core(_) => None,
            ProtoEntry::FrettedSix(x) => Some(x.clone_::<S>()),
            ProtoEntry::FrettedFour(x) => Some(x.clone_::<S>()),
        }
    }
    pub fn cast_size_fretted_shape<const S: usize>(&self) -> Option<(HandShape<S>, Duration)> {
        if let Some(x) = self.cast_size_fretted() {
            if let FrettedEntry::Shape(y, z) = x {
                return Some((y, z));
            }
        }
        None
    }
    pub fn cast_size_fretted_fretboard<const S: usize>(&self) -> Option<Fretboard<S>> {
        if let Some(FrettedEntry::Fretboard(x)) = self.cast_size_fretted() {
            return Some(x);
        }
        None
    }
}
impl ProtoEntry {
    pub fn is_fretted(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::FrettedSix(_) => true,
            ProtoEntry::FrettedFour(_) => true,
        }
    }
    pub fn is_fretted_pick(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::FrettedSix(x) => x.is_pick(),
            ProtoEntry::FrettedFour(x) => x.is_pick(),
        }
    }
    pub fn is_fretted_strum(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::FrettedSix(x) => x.is_strum(),
            ProtoEntry::FrettedFour(x) => x.is_strum(),
        }
    }
    pub fn is_fretted_shape(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::FrettedSix(x) => x.is_shape(),
            ProtoEntry::FrettedFour(x) => x.is_shape(),
        }
    }
    pub fn is_fretted_fretboard(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::FrettedSix(x) => x.is_fretboard(),
            ProtoEntry::FrettedFour(x) => x.is_fretboard(),
        }
    }
}

impl Entry for ProtoEntry {
    fn duration(&self) -> Duration {
        self.duration()
    }
}

impl From<String> for ProtoEntry {
    fn from(v: String) -> Self {
        ProtoEntry::Mark(v)
    }
}

impl From<&str> for ProtoEntry {
    fn from(v: &str) -> Self {
        ProtoEntry::Mark(String::from(v))
    }
}

impl From<CoreEntry> for ProtoEntry {
    fn from(v: CoreEntry) -> Self {
        ProtoEntry::Core(v)
    }
}

impl From<FrettedEntry<6>> for ProtoEntry {
    fn from(v: FrettedEntry<6>) -> Self {
        ProtoEntry::FrettedSix(v)
    }
}

impl From<FrettedEntry<4>> for ProtoEntry {
    fn from(v: FrettedEntry<4>) -> Self {
        ProtoEntry::FrettedFour(v)
    }
}

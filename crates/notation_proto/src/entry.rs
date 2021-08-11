use notation_fretted::prelude::FrettedEntry;
use serde::{Deserialize, Serialize};

use notation_core::prelude::{CoreEntry, Duration, Entry};
use notation_fretted::prelude::{Fretboard, HandShape};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ProtoEntry {
    Mark(String),
    Core(CoreEntry),
    Word(String, Duration),
    FrettedSix(FrettedEntry<6>),
    FrettedFour(FrettedEntry<4>),
    Extra(String, String),
}

impl ProtoEntry {
    pub fn duration(&self) -> Duration {
        match self {
            ProtoEntry::Mark(_) => Duration::Zero,
            ProtoEntry::Core(entry) => entry.duration(),
            ProtoEntry::Word(_, duration) => *duration,
            ProtoEntry::FrettedSix(entry) => entry.duration(),
            ProtoEntry::FrettedFour(entry) => entry.duration(),
            ProtoEntry::Extra(_, _) => Duration::Zero,
        }
    }
    /// Returns `true` if the proto_entry is [`Mark`].
    pub fn is_mark(&self) -> bool {
        matches!(self, Self::Mark(..))
    }
    pub fn is_mark_string(&self, val: &String) -> bool {
        if let Self::Mark(v) = self {
            v == val
        } else {
            false
        }
    }
    pub fn is_mark_str(&self, val: &str) -> bool {
        if let Self::Mark(v) = self {
            v.as_str() == val
        } else {
            false
        }
    }
    /// Returns `true` if the proto_entry is [`Core`].
    pub fn is_core(&self) -> bool {
        matches!(self, Self::Core(..))
    }
    /// Returns `true` if the proto_entry is [`Word`].
    pub fn is_word(&self) -> bool {
        matches!(self, Self::Word(..))
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
    pub fn is_core_tie(&self) -> bool {
        self.as_core().map(|x| x.is_tie()).unwrap_or(false)
    }

}
impl ProtoEntry {
    pub fn is_size_fretted<const S: usize>(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::Word(_, _) => false,
            ProtoEntry::FrettedSix(_) => S == 6,
            ProtoEntry::FrettedFour(_) => S == 4,
            ProtoEntry::Extra(_, _) => false,
        }
    }
    pub fn cast_size_fretted<const S: usize>(&self) -> Option<FrettedEntry<S>> {
        match self {
            ProtoEntry::Mark(_) => None,
            ProtoEntry::Core(_) => None,
            ProtoEntry::Word(_, _) => None,
            ProtoEntry::FrettedSix(x) => Some(x.clone_::<S>()),
            ProtoEntry::FrettedFour(x) => Some(x.clone_::<S>()),
            ProtoEntry::Extra(_, _) => None,
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
            ProtoEntry::Word(_, _) => false,
            ProtoEntry::FrettedSix(_) => true,
            ProtoEntry::FrettedFour(_) => true,
            ProtoEntry::Extra(_, _) => false,
        }
    }
    pub fn is_fretted_pick(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::Word(_, _) => false,
            ProtoEntry::FrettedSix(x) => x.is_pick(),
            ProtoEntry::FrettedFour(x) => x.is_pick(),
            ProtoEntry::Extra(_, _) => false,
        }
    }
    pub fn is_fretted_strum(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::Word(_, _) => false,
            ProtoEntry::FrettedSix(x) => x.is_strum(),
            ProtoEntry::FrettedFour(x) => x.is_strum(),
            ProtoEntry::Extra(_, _) => false,
        }
    }
    pub fn is_fretted_shape(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::Word(_, _) => false,
            ProtoEntry::FrettedSix(x) => x.is_shape(),
            ProtoEntry::FrettedFour(x) => x.is_shape(),
            ProtoEntry::Extra(_, _) => false,
        }
    }
    pub fn is_fretted_fretboard(&self) -> bool {
        match self {
            ProtoEntry::Mark(_) => false,
            ProtoEntry::Core(_) => false,
            ProtoEntry::Word(_, _) => false,
            ProtoEntry::FrettedSix(x) => x.is_fretboard(),
            ProtoEntry::FrettedFour(x) => x.is_fretboard(),
            ProtoEntry::Extra(_, _) => false,
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

impl From<(String, Duration)> for ProtoEntry {
    fn from(v: (String, Duration)) -> Self {
        ProtoEntry::Word(v.0, v.1)
    }
}

impl From<(&str, Duration)> for ProtoEntry {
    fn from(v: (&str, Duration)) -> Self {
        ProtoEntry::Word(String::from(v.0), v.1)
    }
}

impl From<(String, String)> for ProtoEntry {
    fn from(v: (String, String)) -> Self {
        ProtoEntry::Extra(v.0, v.1)
    }
}

impl From<(&str, String)> for ProtoEntry {
    fn from(v: (&str, String)) -> Self {
        ProtoEntry::Extra(String::from(v.0), v.1)
    }
}

impl From<(String, &str)> for ProtoEntry {
    fn from(v: (String, &str)) -> Self {
        ProtoEntry::Extra(v.0, String::from(v.1))
    }
}

impl From<(&str, &str)> for ProtoEntry {
    fn from(v: (&str, &str)) -> Self {
        ProtoEntry::Extra(String::from(v.0), String::from(v.1))
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

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use notation_core::prelude::{CoreEntry, Duration, Entry, EntryPassMode, MetaEntry};
use notation_fretted::prelude::{FrettedEntry4, FrettedEntry6};

use crate::prelude::LyricEntry;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ProtoEntry {
    Mark(String),
    Meta(MetaEntry),
    Core(CoreEntry),
    Lyric(LyricEntry),
    Fretted6(FrettedEntry6),
    Fretted4(FrettedEntry4),
    Extra(String, String),
}
impl Display for ProtoEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtoEntry::Mark(x) => write!(f, "Mark({})", x),
            ProtoEntry::Meta(x) => write!(f, "Meta({})", x),
            ProtoEntry::Core(x) => write!(f, "Core({})", x),
            ProtoEntry::Lyric(x) => write!(f, "Lyric({})", x),
            ProtoEntry::Fretted6(x) => write!(f, "Fretted6({})", x),
            ProtoEntry::Fretted4(x) => write!(f, "Fretted4({})", x),
            ProtoEntry::Extra(x, y) => write!(f, "Extra({}, {})", x, y),
        }
    }
}
impl ProtoEntry {
    pub fn duration(&self) -> Duration {
        match self {
            ProtoEntry::Mark(_) => Duration::Zero,
            ProtoEntry::Meta(entry) => entry.duration(),
            ProtoEntry::Core(entry) => entry.duration(),
            ProtoEntry::Lyric(entry) => entry.duration(),
            ProtoEntry::Fretted6(entry) => entry.duration(),
            ProtoEntry::Fretted4(entry) => entry.duration(),
            ProtoEntry::Extra(_, _) => Duration::Zero,
        }
    }
    pub fn pass_mode(&self) -> EntryPassMode {
        match self {
            ProtoEntry::Mark(_) => EntryPassMode::Immediate,
            ProtoEntry::Meta(entry) => entry.pass_mode(),
            ProtoEntry::Core(entry) => entry.pass_mode(),
            ProtoEntry::Lyric(entry) => entry.pass_mode(),
            ProtoEntry::Fretted6(entry) => entry.pass_mode(),
            ProtoEntry::Fretted4(entry) => entry.pass_mode(),
            ProtoEntry::Extra(_, _) => EntryPassMode::Immediate,
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
    /// Returns `true` if the proto_entry is [`Meta`].
    pub fn is_meta(&self) -> bool {
        matches!(self, Self::Meta(..))
    }
    /// Returns `true` if the proto_entry is [`Core`].
    pub fn is_core(&self) -> bool {
        matches!(self, Self::Core(..))
    }
    /// Returns `true` if the proto_entry is [`Word`].
    pub fn is_word(&self) -> bool {
        matches!(self, Self::Lyric(..))
    }
    pub fn as_mark(&self) -> Option<&String> {
        if let Self::Mark(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_meta(&self) -> Option<&MetaEntry> {
        if let Self::Meta(v) = self {
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
    pub fn is_fretted6(&self) -> bool {
        matches!(self, Self::Fretted6(..))
    }
    pub fn as_fretted6(&self) -> Option<&FrettedEntry6> {
        if let Self::Fretted6(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn try_into_fretted6(self) -> Result<FrettedEntry6, Self> {
        if let Self::Fretted6(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
    /// Returns `true` if the proto_entry is [`FrettedFour`].
    pub fn is_fretted4(&self) -> bool {
        matches!(self, Self::Fretted4(..))
    }
    pub fn as_fretted4(&self) -> Option<&FrettedEntry4> {
        if let Self::Fretted4(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn try_into_fretted4(self) -> Result<FrettedEntry4, Self> {
        if let Self::Fretted4(v) = self {
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
impl Entry for ProtoEntry {
    fn duration(&self) -> Duration {
        self.duration()
    }
    fn pass_mode(&self) -> EntryPassMode {
        self.pass_mode()
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

impl From<MetaEntry> for ProtoEntry {
    fn from(v: MetaEntry) -> Self {
        ProtoEntry::Meta(v)
    }
}

impl From<CoreEntry> for ProtoEntry {
    fn from(v: CoreEntry) -> Self {
        ProtoEntry::Core(v)
    }
}

impl From<LyricEntry> for ProtoEntry {
    fn from(v: LyricEntry) -> Self {
        ProtoEntry::Lyric(v)
    }
}

impl From<FrettedEntry6> for ProtoEntry {
    fn from(v: FrettedEntry6) -> Self {
        ProtoEntry::Fretted6(v)
    }
}

impl From<FrettedEntry4> for ProtoEntry {
    fn from(v: FrettedEntry4) -> Self {
        ProtoEntry::Fretted4(v)
    }
}

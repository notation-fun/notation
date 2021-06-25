use serde::{Serialize, Deserialize};

use notation_core::prelude::{Duration, Entry, CoreEntry};
use notation_guitar::prelude::{GuitarEntry};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ProtoEntry {
    Core (CoreEntry),
    Guitar (GuitarEntry),
}

impl ProtoEntry {
    pub fn duration(&self) -> Duration {
        match self {
            ProtoEntry::Core(entry) => entry.duration(),
            ProtoEntry::Guitar(entry) => entry.duration(),
        }
    }
}

impl Entry for ProtoEntry {
    fn duration(&self) -> Duration {
        self.duration()
    }
}

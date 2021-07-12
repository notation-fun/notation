use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{ProtoEntry, TrackKind};

#[derive(Debug)]
pub struct Track {
    pub kind: TrackKind,
    pub name: String,
    pub entries: Vec<Arc<ProtoEntry>>,
}
impl Track {
    pub fn new(kind: TrackKind, name: String, entries: Vec<Arc<ProtoEntry>>) -> Self {
        Self {
            kind,
            name,
            entries,
        }
    }
}
impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>({:?}, {}, [{}])",
            stringify!(Track),
            self.kind,
            self.name,
            self.entries.len()
        )
    }
}

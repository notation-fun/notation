use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{ProtoEntry, TrackKind};

#[derive(Debug)]
pub struct Track {
    pub key: String,
    pub kind: TrackKind,
    pub entries: Vec<Arc<ProtoEntry>>,
}
impl Track {
    pub fn new(key: String, kind: TrackKind, entries: Vec<Arc<ProtoEntry>>) -> Self {
        Self {
            key,
            kind,
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
            self.key,
            self.entries.len()
        )
    }
}
impl From<notation_proto::prelude::Track> for Track {
    fn from(v: notation_proto::prelude::Track) -> Self {
        let entries: Vec<Arc<ProtoEntry>> = v.entries.into_iter().map(
            |entry| Arc::new(entry)
        ).collect();
        Self::new(v.key, v.kind, entries)
    }
}
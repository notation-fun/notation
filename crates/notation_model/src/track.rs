use std::fmt::Display;
use std::sync::Arc;

use crate::prelude::{ProtoEntry, TrackKind};

#[derive(Debug)]
pub struct Track {
    pub id: String,
    pub kind: TrackKind,
    pub entries: Vec<Arc<ProtoEntry>>,
}
impl Track {
    pub fn new(id: String, kind: TrackKind, entries: Vec<Arc<ProtoEntry>>) -> Self {
        Self { id, kind, entries }
    }
}
impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>({:?}, {}, [{}])",
            stringify!(Track),
            self.kind,
            self.id,
            self.entries.len()
        )
    }
}
impl From<notation_proto::prelude::Track> for Track {
    fn from(v: notation_proto::prelude::Track) -> Self {
        let entries: Vec<Arc<ProtoEntry>> = v.entries.into_iter().map(Arc::new).collect();
        Self::new(v.id, v.kind, entries)
    }
}

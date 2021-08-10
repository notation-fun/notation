use std::sync::{Arc, Weak};

use crate::prelude::Track;
use notation_proto::prelude::{Entry, ProtoEntry, TrackKind};

#[derive(Debug)]
pub struct ModelEntry {
    pub track: Weak<Track>,
    pub index: usize,
    pub value: Arc<ProtoEntry>,
}
impl ModelEntry {
    pub fn new(track: Weak<Track>, index: usize, value: Arc<ProtoEntry>) -> Self {
        Self {
            track,
            index,
            value,
        }
    }
}
impl Entry for ModelEntry {
    fn duration(&self) -> notation_proto::prelude::Duration {
        self.value.duration()
    }
}
impl ModelEntry {
    pub fn prev(&self) -> Option<Arc<ModelEntry>> {
        if let Some(track) = self.track.upgrade() {
            track.entries.get(self.index - 1).map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn next(&self) -> Option<Arc<ModelEntry>> {
        if let Some(track) = self.track.upgrade() {
            track.entries.get(self.index + 1).map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn prev_as_mark(&self) -> Option<String> {
        if let Some(entry) = self.prev() {
            entry.value.as_mark().map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn track_id(&self) -> String {
        if let Some(track) = self.track.upgrade() {
            track.id.clone()
        } else {
            "".to_owned()
        }
    }
    pub fn track_kind(&self) -> TrackKind {
        if let Some(track) = self.track.upgrade() {
            track.kind.clone()
        } else {
            TrackKind::Custom("".to_owned())
        }
    }
}

use std::sync::{Arc, Weak};

use crate::prelude::Track;
use notation_proto::prelude::{Entry, ProtoEntry, TrackKind, Duration, Units};

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
    fn prev_is_tie(&self) -> bool {
        self.prev()
            .map(|x| x.value.is_core_tie())
            .unwrap_or(false)
    }
    fn next_is_tie(&self) -> bool {
        self.next()
            .map(|x| x.value.is_core_tie())
            .unwrap_or(false)
    }
    fn tied_units(&self) -> Units {
        let self_units = Units::from(self.duration());
        self.get_tied_next()
            .map(|x| {
                self_units + x.tied_units()
            }).unwrap_or(self_units)
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
    pub fn get_tied_prev(&self) -> Option<Arc<ModelEntry>> {
        if let Some(track) = self.track.upgrade() {
            if let Some(prev) = track.entries.get(self.index - 1) {
                if prev.value.is_core_tie() {
                    for i in self.index - 2..=0 {
                        let entry = track.entries.get(i).unwrap();
                        if entry.duration() != Duration::Zero {
                            return Some(entry.clone());
                        }
                    }
                }
            }
        }
        None
    }
    pub fn get_tied_next(&self) -> Option<Arc<ModelEntry>> {
        if let Some(track) = self.track.upgrade() {
            if let Some(next) = track.entries.get(self.index + 1) {
                if next.value.is_core_tie() {
                    for i in self.index + 2..track.entries.len() {
                        let entry = track.entries.get(i).unwrap();
                        if entry.duration() != Duration::Zero {
                            return Some(entry.clone());
                        }
                    }
                }
            }
        }
        None
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

use std::sync::{Arc, Weak};

use crate::prelude::Track;
use notation_proto::prelude::{Duration, Entry, FrettedEntry6, FrettedEntry4, ProtoEntry, TrackKind, Units};

#[derive(Copy, Clone, Debug)]
pub struct ModelEntryProps {
    pub tied_units: Units,
}

#[derive(Debug)]
pub struct ModelEntry {
    pub track: Weak<Track>,
    pub index: usize,
    pub proto: Arc<ProtoEntry>,
    pub props: ModelEntryProps,
}
impl ModelEntry {
    pub fn new(
        track: Weak<Track>,
        index: usize,
        proto: Arc<ProtoEntry>,
        props: ModelEntryProps,
    ) -> Self {
        Self {
            track,
            index,
            proto,
            props,
        }
    }
}
impl Entry for ModelEntry {
    fn duration(&self) -> notation_proto::prelude::Duration {
        self.proto.duration()
    }
    fn prev_is_tie(&self) -> bool {
        self.prev().map(|x| x.proto.is_core_tie()).unwrap_or(false)
    }
    fn next_is_tie(&self) -> bool {
        self.next().map(|x| x.proto.is_core_tie()).unwrap_or(false)
    }
    fn tied_units(&self) -> Units {
        self.props.tied_units
    }
}
impl ModelEntry {
    pub fn as_fretted6(&self) -> Option<&FrettedEntry6> {
        self.proto.as_fretted6()
    }
    pub fn as_fretted4(&self) -> Option<&FrettedEntry4> {
        self.proto.as_fretted4()
    }
    pub fn prev(&self) -> Option<Arc<ModelEntry>> {
        if self.index == 0 {
            None
        } else if let Some(track) = self.track.upgrade() {
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
            entry.proto.as_mark().map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn get_tied_prev(&self) -> Option<Arc<ModelEntry>> {
        if self.index <= 1 {
            return None;
        }
        if let Some(track) = self.track.upgrade() {
            if let Some(prev) = track.entries.get(self.index - 1) {
                if prev.proto.is_core_tie() {
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
                if next.proto.is_core_tie() {
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
    pub fn get_track_entry<F: Fn(&ModelEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ModelEntry>> {
        if let Some(track) = self.track.upgrade() {
            track.get_entry(predicate)
        } else {
            None
        }
    }
}

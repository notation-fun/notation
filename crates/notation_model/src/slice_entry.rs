use std::sync::{Arc, Weak};

use crate::prelude::{ModelEntry, Slice};
use notation_proto::prelude::{Entry, ProtoEntry, TrackKind, Units};

#[derive(Copy, Clone, Debug)]
pub struct SliceEntryProps {
    pub in_bar_pos: Units,
}

#[derive(Debug)]
pub struct SliceEntry {
    pub slice: Weak<Slice>,
    pub index: usize,
    pub model: Arc<ModelEntry>,
    pub props: SliceEntryProps,
}
impl SliceEntry {
    pub fn new(
        slice: Weak<Slice>,
        index: usize,
        model: Arc<ModelEntry>,
        props: SliceEntryProps,
    ) -> Self {
        Self {
            slice,
            index,
            model,
            props,
        }
    }
}
impl Entry for SliceEntry {
    fn duration(&self) -> notation_proto::prelude::Duration {
        self.model.duration()
    }
    fn prev_is_tie(&self) -> bool {
        self.model.prev_is_tie()
    }
    fn next_is_tie(&self) -> bool {
        self.model.next_is_tie()
    }
    fn tied_units(&self) -> Units {
        self.model.tied_units()
    }
}
impl SliceEntry {
    pub fn model(&self) -> &ModelEntry {
        self.model.as_ref()
    }
    pub fn proto(&self) -> &ProtoEntry {
        self.model.proto.as_ref()
    }
    pub fn prev(&self) -> Option<Arc<SliceEntry>> {
        if self.index == 0 {
            None
        } else if let Some(slice) = self.slice.upgrade() {
            slice.entries.get(self.index - 1).map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn next(&self) -> Option<Arc<SliceEntry>> {
        if let Some(slice) = self.slice.upgrade() {
            slice.entries.get(self.index + 1).map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn prev_as_mark(&self) -> Option<String> {
        if let Some(entry) = self.prev() {
            entry.model.proto.as_mark().map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn track_id(&self) -> String {
        self.model.track_id()
    }
    pub fn track_kind(&self) -> TrackKind {
        self.model.track_kind()
    }
    pub fn get_slice_entry<F: Fn(&SliceEntry) -> bool>(
        &self,
        predicate: &F,
    ) -> Option<Arc<SliceEntry>> {
        if let Some(slice) = self.slice.upgrade() {
            slice.get_entry(predicate)
        } else {
            None
        }
    }
    pub fn get_track_entry<F: Fn(&ModelEntry) -> bool>(
        &self,
        predicate: &F,
    ) -> Option<Arc<ModelEntry>> {
        if let Some(slice) = self.slice.upgrade() {
            slice.track.get_entry(predicate)
        } else {
            None
        }
    }
}

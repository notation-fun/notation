use std::sync::{Arc, Weak};

use crate::prelude::{ModelEntry, BarLane};
use notation_proto::prelude::{Entry, ProtoEntry, TrackKind, Units};

#[derive(Copy, Clone, Debug)]
pub struct LaneEntryProps {
    pub in_bar_pos: Units,
}

#[derive(Debug)]
pub struct LaneEntry {
    pub lane: Weak<BarLane>,
    pub index: usize,
    pub model: Arc<ModelEntry>,
    pub props: LaneEntryProps,
}
impl LaneEntry {
    pub fn new(
        lane: Weak<BarLane>,
        index: usize,
        model: Arc<ModelEntry>,
        props: LaneEntryProps,
    ) -> Self {
        Self {
            lane,
            index,
            model,
            props,
        }
    }
}
impl Entry for LaneEntry {
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
impl LaneEntry {
    pub fn model(&self) -> &ModelEntry {
        self.model.as_ref()
    }
    pub fn proto(&self) -> &ProtoEntry {
        self.model.proto.as_ref()
    }
    pub fn prev(&self) -> Option<Arc<LaneEntry>> {
        if self.index == 0 {
            None
        } else if let Some(lane) = self.lane.upgrade() {
            lane.entries.get(self.index - 1).map(|x| x.clone())
        } else {
            None
        }
    }
    pub fn next(&self) -> Option<Arc<LaneEntry>> {
        if let Some(lane) = self.lane.upgrade() {
            lane.entries.get(self.index + 1).map(|x| x.clone())
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
    pub fn get_lane_entry<F: Fn(&LaneEntry) -> bool>(
        &self,
        predicate: &F,
    ) -> Option<Arc<LaneEntry>> {
        if let Some(lane) = self.lane.upgrade() {
            lane.get_entry(predicate)
        } else {
            None
        }
    }
    pub fn get_track_entry<F: Fn(&ModelEntry) -> bool>(
        &self,
        predicate: &F,
    ) -> Option<Arc<ModelEntry>> {
        if let Some(lane) = self.lane.upgrade() {
            lane.track.get_entry(predicate)
        } else {
            None
        }
    }
}

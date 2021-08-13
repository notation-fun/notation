use std::sync::Arc;

use crate::prelude::{Bar, BarLane, BarLayer, LaneEntry, ModelEntry, TabBar, Track};

pub fn get_track_entry<F: Fn(&ModelEntry) -> bool>(
    v: &[Arc<ModelEntry>],
    predicate: &F,
) -> Option<Arc<ModelEntry>> {
    for entry in v.iter() {
        if predicate(entry.as_ref()) {
            return Some(entry.clone());
        }
    }
    None
}
pub fn get_track_entry_<F: Fn(usize, &ModelEntry) -> bool>(
    v: &[Arc<ModelEntry>],
    predicate: &F,
) -> Option<Arc<ModelEntry>> {
    for (index, entry) in v.iter().enumerate() {
        if predicate(index, entry.as_ref()) {
            return Some(entry.clone());
        }
    }
    None
}
impl Track {
    pub fn get_entry<F: Fn(&ModelEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ModelEntry>> {
        get_track_entry(&self.entries, predicate)
    }
}
impl BarLayer {
    pub fn get_track_entry<F: Fn(&ModelEntry) -> bool>(
        &self,
        predicate: &F,
    ) -> Option<Arc<ModelEntry>> {
        self.track.get_entry(predicate)
    }
}
impl Bar {
    pub fn get_track_entry_in_layers<F: Fn(&ModelEntry) -> bool>(
        &self,
        predicate: &F,
    ) -> Option<Arc<ModelEntry>> {
        for layer in self.layers.iter() {
            if let Some(x) = layer.get_track_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}
pub fn get_lane_entry<F: Fn(&LaneEntry) -> bool>(
    v: &[Arc<LaneEntry>],
    predicate: &F,
) -> Option<Arc<LaneEntry>> {
    for entry in v.iter() {
        if predicate(entry.as_ref()) {
            return Some(entry.clone());
        }
    }
    None
}
pub fn get_lane_entry_<F: Fn(usize, &LaneEntry) -> bool>(
    v: &[Arc<LaneEntry>],
    predicate: &F,
) -> Option<Arc<LaneEntry>> {
    for (index, entry) in v.iter().enumerate() {
        if predicate(index, entry.as_ref()) {
            return Some(entry.clone());
        }
    }
    None
}
impl BarLane {
    pub fn get_entry<F: Fn(&LaneEntry) -> bool>(&self, predicate: &F) -> Option<Arc<LaneEntry>> {
        get_lane_entry(&self.entries, predicate)
    }
}
impl TabBar {
    pub fn get_entry_in_lanes<F: Fn(&LaneEntry) -> bool>(
        &self,
        predicate: &F,
    ) -> Option<Arc<LaneEntry>> {
        for lane in self.lanes.iter() {
            if let Some(x) = lane.get_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}

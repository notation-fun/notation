use std::sync::Arc;

use crate::prelude::{Bar, BarLane, BarLayer, LaneEntry, ModelEntry, TabBar, Track};

pub fn get_track_entry<T, F: Fn(&ModelEntry) -> Option<T>>(
    v: &[Arc<ModelEntry>],
    predicate: &F,
) -> Option<T> {
    for entry in v.iter() {
        if let Some(result) = predicate(entry.as_ref()) {
            return Some(result);
        }
    }
    None
}
pub fn get_track_entry_<T, F: Fn(usize, &ModelEntry) -> Option<T>>(
    v: &[Arc<ModelEntry>],
    predicate: &F,
) -> Option<T> {
    for (index, entry) in v.iter().enumerate() {
        if let Some(result) = predicate(index, entry.as_ref()) {
            return Some(result);
        }
    }
    None
}
impl Track {
    pub fn get_entry<T, F: Fn(&ModelEntry) -> Option<T>>(&self, predicate: &F) -> Option<T> {
        get_track_entry(&self.entries, predicate)
    }
}
impl BarLayer {
    pub fn get_track_entry<T, F: Fn(&ModelEntry) -> Option<T>>(&self, predicate: &F) -> Option<T> {
        self.track.get_entry(predicate)
    }
}
impl Bar {
    pub fn get_track_entry_in_layers<T, F: Fn(&ModelEntry) -> Option<T>>(
        &self,
        predicate: &F,
    ) -> Option<T> {
        for layer in self.layers.iter() {
            if let Some(x) = layer.get_track_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}
pub fn get_lane_entry<T, F: Fn(&LaneEntry) -> Option<T>>(
    v: &[Arc<LaneEntry>],
    predicate: &F,
) -> Option<T> {
    for entry in v.iter() {
        if let Some(result) = predicate(entry.as_ref()) {
            return Some(result);
        }
    }
    None
}
pub fn get_lane_entry_<T, F: Fn(usize, &LaneEntry) -> Option<T>>(
    v: &[Arc<LaneEntry>],
    predicate: &F,
) -> Option<T> {
    for (index, entry) in v.iter().enumerate() {
        if let Some(result) = predicate(index, entry.as_ref()) {
            return Some(result);
        }
    }
    None
}
impl BarLane {
    pub fn get_entry<T, F: Fn(&LaneEntry) -> Option<T>>(&self, predicate: &F) -> Option<T> {
        get_lane_entry(&self.entries, predicate)
    }
}
impl TabBar {
    pub fn get_entry_in_lanes<T, F: Fn(&LaneEntry) -> Option<T>>(
        &self,
        predicate: &F,
    ) -> Option<T> {
        for ((_k, _i), lane) in self.lanes.iter() {
            if let Some(x) = lane.get_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}

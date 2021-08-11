use std::sync::Arc;

use crate::prelude::{Bar, BarLane, BarLayer, ModelEntry, Slice, SliceEntry, Track};

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
pub fn get_slice_entry<F: Fn(&SliceEntry) -> bool>(
    v: &[Arc<SliceEntry>],
    predicate: &F,
) -> Option<Arc<SliceEntry>> {
    for entry in v.iter() {
        if predicate(entry.as_ref()) {
            return Some(entry.clone());
        }
    }
    None
}
pub fn get_slice_entry_<F: Fn(usize, &SliceEntry) -> bool>(
    v: &[Arc<SliceEntry>],
    predicate: &F,
) -> Option<Arc<SliceEntry>> {
    for (index, entry) in v.iter().enumerate() {
        if predicate(index, entry.as_ref()) {
            return Some(entry.clone());
        }
    }
    None
}

impl Slice {
    pub fn get_entry<F: Fn(&SliceEntry) -> bool>(&self, predicate: &F) -> Option<Arc<SliceEntry>> {
        get_slice_entry(&self.entries, predicate)
    }
}
impl BarLayer {
    pub fn get_entry<F: Fn(&SliceEntry) -> bool>(&self, predicate: &F) -> Option<Arc<SliceEntry>> {
        for slice in self.slices.iter() {
            if let Some(x) = slice.get_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}
impl BarLane {
    pub fn get_entry<F: Fn(&SliceEntry) -> bool>(&self, predicate: &F) -> Option<Arc<SliceEntry>> {
        self.slice.get_entry(predicate)
    }
}
impl Bar {
    pub fn get_entry_in_layers<F: Fn(&SliceEntry) -> bool>(
        &self,
        predicate: &F,
    ) -> Option<Arc<SliceEntry>> {
        for layer in self.layers.iter() {
            if let Some(x) = layer.get_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
    pub fn get_entry_in_lanes<F: Fn(&SliceEntry) -> bool>(
        &self,
        predicate: &F,
    ) -> Option<Arc<SliceEntry>> {
        for lane in self.lanes.iter() {
            if let Some(x) = lane.get_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}

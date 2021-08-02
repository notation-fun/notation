use std::sync::Arc;

use crate::prelude::{Bar, BarLayer, ModelEntry, Slice, Track};

pub fn get_entry<F: Fn(&ModelEntry) -> bool>(
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
pub fn get_entry_<F: Fn(usize, &ModelEntry) -> bool>(
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
impl Slice {
    pub fn get_entry<F: Fn(&ModelEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ModelEntry>> {
        get_entry(&self.entries, predicate)
    }
}
impl Track {
    pub fn get_entry<F: Fn(&ModelEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ModelEntry>> {
        get_entry(&self.entries, predicate)
    }
}
impl BarLayer {
    pub fn get_entry<F: Fn(&ModelEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ModelEntry>> {
        for slice in self.slices.iter() {
            if let Some(x) = slice.get_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}
impl Bar {
    pub fn get_entry<F: Fn(&ModelEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ModelEntry>> {
        for layer in self.layers.iter() {
            if let Some(x) = layer.get_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}

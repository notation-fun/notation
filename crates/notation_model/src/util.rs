use std::sync::Arc;

use crate::prelude::{Bar, BarLayer, Line, ProtoEntry, Slice, Track};

pub fn get_entry<F: Fn(&ProtoEntry) -> bool>(
    v: &[Arc<ProtoEntry>],
    predicate: &F,
) -> Option<Arc<ProtoEntry>> {
    for entry in v.iter() {
        if predicate(entry.as_ref()) {
            return Some(entry.clone());
        }
    }
    None
}
pub fn get_entry_<F: Fn(usize, &ProtoEntry) -> bool>(
    v: &[Arc<ProtoEntry>],
    predicate: &F,
) -> Option<Arc<ProtoEntry>> {
    for (index, entry) in v.iter().enumerate() {
        if predicate(index, entry.as_ref()) {
            return Some(entry.clone());
        }
    }
    None
}
impl Line {
    pub fn get_entry<F: Fn(&ProtoEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ProtoEntry>> {
        get_entry(&self.entries, predicate)
    }
}
impl Slice {
    pub fn get_entry<F: Fn(&ProtoEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ProtoEntry>> {
        get_entry_(&self.line.entries, &|index, entry| {
            index >= self.index && index < self.index + self.count && predicate(entry)
        })
    }
}
impl Track {
    pub fn get_entry<F: Fn(&ProtoEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ProtoEntry>> {
        get_entry(&self.entries, predicate)
    }
}
impl BarLayer {
    pub fn get_entry<F: Fn(&ProtoEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ProtoEntry>> {
        for slice in self.slices.iter() {
            if let Some(x) = slice.get_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}
impl Bar {
    pub fn get_entry<F: Fn(&ProtoEntry) -> bool>(&self, predicate: &F) -> Option<Arc<ProtoEntry>> {
        for layer in self.layers.iter() {
            if let Some(x) = layer.get_entry(predicate) {
                return Some(x);
            }
        }
        None
    }
}

use crate::fretted::fretted_layer_bundle::FrettedLayerBundle;
use crate::prelude::FrettedGrid;
use notation_model::prelude::{
    BarLayer, GuitarEntry, GuitarFretboard, GuitarTuning, GuitarUtil, ProtoEntry, TabBar, Track,
    GUITAR_STRING_NUM,
};
use std::sync::Arc;

pub type GuitarFrettedGrid = FrettedGrid<GUITAR_STRING_NUM>;
pub type GuitarLayerBundle = FrettedLayerBundle<GUITAR_STRING_NUM>;

fn as_fretted_entry(v: &ProtoEntry) -> Option<&GuitarEntry> {
    v.as_fretted_six()
}

fn new_default_fretboard() -> GuitarFretboard {
    GuitarUtil::new_acoustic_guitar_fretboard(GuitarTuning::Standard)
}

impl GuitarLayerBundle {
    pub fn new(bar: Arc<TabBar>, layer: Arc<BarLayer>, track: Arc<Track>) -> Self {
        FrettedLayerBundle::_new(bar, layer, track, &as_fretted_entry, &new_default_fretboard)
    }
}

use crate::{shapes::shapes_lane_bundle::ShapesLaneBundle, strings::strings_lane_bundle::StringsLaneBundle};
use crate::prelude::StringsGrid;
use notation_model::prelude::{
    GuitarEntry, GuitarFretboard, GuitarUtil, ModelEntry, Track, GUITAR_STRING_NUM,
};
use std::sync::Arc;

pub type GuitarShapesLaneBundle = ShapesLaneBundle<GUITAR_STRING_NUM>;

pub type GuitarStringsGrid = StringsGrid<GUITAR_STRING_NUM>;
pub type GuitarStringsLaneBundle = StringsLaneBundle<GUITAR_STRING_NUM>;


fn as_fretted_entry(v: &ModelEntry) -> Option<&GuitarEntry> {
    v.value.as_fretted_six()
}

fn new_default_fretboard() -> GuitarFretboard {
    GuitarUtil::new_acoustic_guitar_fretboard(None)
}

impl GuitarShapesLaneBundle {
    pub fn new(track: Arc<Track>) -> Self {
        ShapesLaneBundle::_new(track, &as_fretted_entry, &new_default_fretboard)
    }
}

impl GuitarStringsLaneBundle {
    pub fn new(track: Arc<Track>) -> Self {
        StringsLaneBundle::_new(track, &as_fretted_entry, &new_default_fretboard)
    }
}


use crate::prelude::StringsGrid;
use crate::shapes::shapes_lane_bundle::ShapesLaneBundle;
use crate::strings::strings_lane_bundle::StringsLaneBundle;
use notation_model::prelude::{
    GuitarEntry, GuitarFretboard, GuitarUtil, ModelEntry, Track, GUITAR_STRING_NUM,
};
use std::sync::Arc;

pub type GuitarShapesLaneBundle = ShapesLaneBundle<GUITAR_STRING_NUM>;

pub type GuitarStringsGrid = StringsGrid<GUITAR_STRING_NUM>;
pub type GuitarStringsLaneBundle = StringsLaneBundle<GUITAR_STRING_NUM>;

impl GuitarShapesLaneBundle {
    pub fn new(track: Arc<Track>) -> Self {
        ShapesLaneBundle::_new(track, &ModelEntry::as_fretted_six, &GuitarUtil::new_default_fretboard)
    }
}

impl GuitarStringsLaneBundle {
    pub fn new(track: Arc<Track>) -> Self {
        StringsLaneBundle::_new(track, &ModelEntry::as_fretted_six, &GuitarUtil::new_default_fretboard)
    }
}

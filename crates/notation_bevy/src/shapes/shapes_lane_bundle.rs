use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::{
    Fretboard4, Fretboard6, FrettedEntry4, FrettedEntry6, ModelEntry, Track,
};

macro_rules! impl_shapes_lane_bundle {
    ($type:ident, $fretted_entry:ident, $fretboard:ident, $get_fretboard:ident) => {
        #[derive(Bundle)]
        pub struct $type {
            fretboard: Option<$fretboard>,
        }

        impl $type {
            pub fn new(track: Arc<Track>) -> Self {
                let fretboard = track.$get_fretboard();
                Self { fretboard }
            }
        }
    };
}

impl_shapes_lane_bundle!(ShapesLaneBundle6, FrettedEntry6, Fretboard6, get_fretboard6);
impl_shapes_lane_bundle!(ShapesLaneBundle4, FrettedEntry4, Fretboard4, get_fretboard4);

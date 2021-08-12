use bevy::prelude::*;
use std::sync::Arc;

use crate::prelude::{StringsGrid6, StringsGrid4};
use notation_model::prelude::{Fretboard6, FrettedEntry6, Fretboard4, FrettedEntry4, ModelEntry, Track};

macro_rules! impl_strings_lane_bundle {
    ($type:ident, $fretted_entry:ident, $fretboard:ident, $get_fretboard:ident, $strings_grid:ident) => {
        #[derive(Bundle)]
        pub struct $type {
            fretboard: Option<$fretboard>,
            grid: $strings_grid,
        }

        impl $type {
            pub fn new(
                track: Arc<Track>,
            ) -> Self {
                let fretboard = track.$get_fretboard();
                Self {
                    fretboard,
                    grid: $strings_grid {},
                }
            }
        }
    }
}

impl_strings_lane_bundle!(StringsLaneBundle6, FrettedEntry6, Fretboard6, get_fretboard6, StringsGrid6);
impl_strings_lane_bundle!(StringsLaneBundle4, FrettedEntry4, Fretboard4, get_fretboard4, StringsGrid4);

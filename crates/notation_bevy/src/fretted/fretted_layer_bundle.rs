use bevy::prelude::*;
use std::sync::Arc;

use crate::prelude::FrettedGrid;
use notation_model::prelude::{Fretboard, FrettedEntry, ModelEntry, Track};

#[derive(Bundle)]
pub struct FrettedLayerBundle<const S: usize> {
    fretboard: Fretboard<S>,
    grid: FrettedGrid<S>,
}

impl<const S: usize> FrettedLayerBundle<S> {
    pub fn _new<F1, F2>(
        track: Arc<Track>,
        as_fretted_entry: &F1,
        new_default_fretboard: &F2,
    ) -> Self
    where
        F1: Fn(&ModelEntry) -> Option<&FrettedEntry<S>>,
        F2: Fn() -> Fretboard<S>,
    {
        let fretboard_entry = track.get_entry(&|x: &ModelEntry| {
            let fretted_entry = as_fretted_entry(x);
            fretted_entry.and_then(|y| y.as_fretboard()).is_some()
        });
        let fretboard = fretboard_entry
            .and_then(|x| {
                as_fretted_entry(x.as_ref()).and_then(|x| x.as_fretboard().map(|z| z.to_owned()))
            })
            .unwrap_or_else(|| new_default_fretboard());
        Self {
            fretboard,
            grid: FrettedGrid::<S> {},
        }
    }
}

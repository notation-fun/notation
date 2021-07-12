use bevy::prelude::*;

use notation_model::prelude::TabBar;
use std::sync::Arc;

use crate::prelude::{BevyConfig, FrettedPlugin, LyonShapeOp};
use notation_model::prelude::{Duration, Fretboard, HandShape, Pick, Units};

use super::pick_note::{PickNote, PickNoteData};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_pick_blocks::<6>.system())
        .with_system(create_pick_blocks::<4>.system())
}

fn create_pick_blocks<const S: usize>(
    mut commands: Commands,
    config: Res<BevyConfig>,
    query: Query<(&Parent, Entity, &Pick, &Duration, &Units), Added<Pick>>,
    layer_query: Query<(&Arc<TabBar>, &Fretboard<S>, &Children)>,
    shape_query: Query<&HandShape<S>>,
) {
    for (parent, entity, pick, duration, units) in query.iter() {
        if let Some((bar, fretboard, shape)) =
            FrettedPlugin::get_fretted_shape(&layer_query, &shape_query, parent.0, units)
        {
            for string in pick.get_strings() {
                if let Some(note) = fretboard.shape_note(&shape, string) {
                    let syllable = bar.calc_syllable(&note);
                    let data = PickNoteData::new(&bar, *duration, *units, string, syllable);
                    PickNote::create(&mut commands, entity, &config, data);
                }
            }
        }
    }
}

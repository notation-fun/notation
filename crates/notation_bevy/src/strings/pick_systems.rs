use bevy::prelude::*;

use notation_model::prelude::{BarLane, BarPosition, TabBar};
use std::sync::Arc;

use crate::prelude::{LyonShapeOp, NotationSettings, NotationTheme, StringsPlugin};
use notation_model::prelude::{Duration, Fretboard, HandShape, Pick};

use super::pick_note::{PickNoteData, PickNoteShape};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_pick_notes::<6>.system())
        .with_system(create_pick_notes::<4>.system())
}

fn create_pick_notes<const S: usize>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme: Res<NotationTheme>,
    settings: Res<NotationSettings>,
    query: Query<(Entity, &Pick, &Duration, &BarPosition), Added<Pick>>,
    lane_queries_0: Query<&Parent>,
    lane_queries_1: Query<&Children>,
    lane_queries_2: Query<&Arc<BarLane>>,
    shape_queries_0: Query<(&Arc<TabBar>, &Arc<BarLane>, &Fretboard<S>, &Children)>,
    shape_queries_1: Query<&HandShape<S>>,
) {
    for (entity, pick, duration, pos) in query.iter() {
        if let Some((bar, fretboard, shape)) = StringsPlugin::get_fretted_shape::<S>(
            entity,
            pos,
            (&lane_queries_0, &lane_queries_1, &lane_queries_2),
            (&shape_queries_0, &shape_queries_1),
        ) {
            let bar_units = bar.bar_units();
            for pick_note in pick.get_notes() {
                if let Some((fret, note)) = fretboard.shape_pick_fret_note(&shape, pick_note) {
                    let syllable = bar.calc_syllable(&note.pitch);
                    let data =
                        PickNoteData::new(bar_units, &bar, *duration, *pos, pick_note, syllable);
                    PickNoteShape::create_with_child(
                        &mut commands,
                        entity,
                        &theme,
                        data,
                        |child_commands| {
                            if settings.always_show_fret || pick_note.fret.is_some() {
                                theme
                                    .strings
                                    .insert_fret_text(child_commands, &asset_server, fret);
                            }
                        },
                    );
                }
            }
            let tone = fretboard.pick_tone(&shape, pick);
            if !tone.is_none() {
                commands.entity(entity).insert(tone);
            }
        }
    }
}

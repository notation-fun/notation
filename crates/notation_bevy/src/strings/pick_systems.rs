use bevy::prelude::*;

use notation_model::prelude::{BarLane, BarPosition, LaneEntry, TabBar, Entry};
use std::sync::Arc;

use crate::prelude::{LyonShapeOp, NotationSettings, NotationTheme, StringsPlugin};
use notation_model::prelude::{Duration, Fretboard4, Fretboard6, HandShape4, HandShape6, Pick};

use super::pick_note::{PickNoteData, PickNoteShape};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_pick_notes6.system())
        .with_system(create_pick_notes4.system())
}

macro_rules! impl_pick_system {
    ($create_pick_notes:ident, $fretboard:ident, $hand_shape:ident, $get_fretted_shape:ident
    ) => {
        fn $create_pick_notes(
            mut commands: Commands,
            asset_server: Res<AssetServer>,
            theme: Res<NotationTheme>,
            settings: Res<NotationSettings>,
            query: Query<
                (
                    &Parent,
                    Entity,
                    &Arc<LaneEntry>,
                    &Pick,
                    &Duration,
                    &BarPosition,
                ),
                Added<Pick>,
            >,
            lane_query: Query<&Arc<TabBar>>,
        ) {
            for (parent, entity, entry, pick, duration, pos) in query.iter() {
                if entry.as_ref().prev_is_tie() {
                    continue;
                }
                if let Ok(bar) = lane_query.get(parent.0) {
                    if let Some((fretboard, shape)) = bar.$get_fretted_shape(entry) {
                        let bar_units = bar.bar_units();
                        for pick_note in pick.get_notes() {
                            if let Some((fret, note)) =
                                fretboard.shape_pick_fret_note(&shape, pick_note)
                            {
                                let syllable = bar.calc_syllable(&note.pitch);
                                let data = PickNoteData::new(
                                    bar_units, &bar, *duration, *pos, pick_note, syllable,
                                );
                                PickNoteShape::create_with_child(
                                    &mut commands,
                                    entity,
                                    &theme,
                                    data,
                                    |child_commands| {
                                        if settings.always_show_fret || pick_note.fret.is_some() {
                                            theme.strings.insert_fret_text(
                                                child_commands,
                                                &asset_server,
                                                fret,
                                            );
                                        }
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    };
}

impl_pick_system!(
    create_pick_notes6,
    Fretboard6,
    HandShape6,
    get_fretted_shape6
);
impl_pick_system!(
    create_pick_notes4,
    Fretboard4,
    HandShape4,
    get_fretted_shape4
);

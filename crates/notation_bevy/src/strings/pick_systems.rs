use bevy::prelude::*;

use bevy_utils::prelude::BevyUtil;
use notation_model::prelude::LaneEntry;

use crate::prelude::{EntryPlaying, LyonShapeOp, NotationAssets, NotationSettings, NotationTheme};
use notation_model::prelude::Pick;

use super::pick_note::{PickNoteData, PickNoteShape, PickNoteValue};

pub fn on_entry_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &EntryPlaying, &Children), Changed<EntryPlaying>>,
    mut note_query: Query<(Entity, &mut PickNoteData, &Children)>,
    mut font_query: Query<&mut Text>,
) {
    for (_entity, playing, children) in query.iter() {
        for child in children.iter() {
            if let Ok((entity, mut data, note_children)) = note_query.get_mut(*child) {
                //println!("{:?} -> {:?} -> {:?}", name, data, playing)
                data.value.playing_state = playing.value;
                PickNoteShape::update(&mut commands, &theme, entity, &data);
                for child in note_children.iter() {
                    if let Ok(mut text) = font_query.get_mut(*child) {
                        BevyUtil::set_text_color(&mut text, data.calc_fret_color(&theme));
                    }
                }
            }
        }
    }
}

macro_rules! impl_pick_system {
    ($create_pick_notes:ident, $fretboard:ident, $hand_shape:ident, $get_fretted_shape:ident
    ) => {
        pub fn $create_pick_notes(
            commands: &mut Commands,
            assets: &NotationAssets,
            theme: &NotationTheme,
            settings: &NotationSettings,
            entity: Entity,
            entry: &LaneEntry,
            pick: &Pick,
        ) {
            /* TODO: check whether is the first bar in row
            if entry.as_ref().prev_is_tie() {
                continue;
            }
            */
            if let Some(bar) = entry.bar() {
                if let Some((fretboard, shape)) = bar.$get_fretted_shape(entry) {
                    for pick_note in pick.get_notes() {
                        if let Some((fret, note)) =
                            fretboard.shape_pick_fret_note(&shape, pick_note)
                        {
                            let syllable = bar.calc_syllable(&note.pitch);
                            let data =
                                PickNoteData::new(entry, PickNoteValue::new(pick_note, syllable));
                            let note_entity = PickNoteShape::create(commands, theme, entity, data);
                            if settings.always_show_fret || pick_note.fret.is_some() {
                                theme
                                    .strings
                                    .spawn_fret_text(commands, note_entity, &assets, fret);
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

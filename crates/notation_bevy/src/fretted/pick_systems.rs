use bevy::prelude::*;

use notation_midi::prelude::{PlayToneEvent, StopToneEvent};
use notation_model::prelude::{BarPosition, TabBar};
use std::sync::Arc;

use crate::prelude::{BevyConfig, EntryState, FrettedPlugin, LyonShapeOp};
use notation_model::prelude::{Duration, Fretboard, HandShape, Pick};

use super::pick_note::{PickNote, PickNoteData};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_pick_notes::<6>.system())
        .with_system(create_pick_notes::<4>.system())
        .with_system(play_pick_tone::<6>.system())
        .with_system(play_pick_tone::<4>.system())
}

fn create_pick_notes<const S: usize>(
    mut commands: Commands,
    config: Res<BevyConfig>,
    query: Query<(&Parent, Entity, &Pick, &Duration, &BarPosition), Added<Pick>>,
    layer_query: Query<(&Arc<TabBar>, &Fretboard<S>, &Children)>,
    shape_query: Query<&HandShape<S>>,
) {
    for (parent, entity, pick, duration, pos) in query.iter() {
        if let Some((bar, fretboard, shape)) =
            FrettedPlugin::get_fretted_shape(&layer_query, &shape_query, parent.0, pos)
        {
            for string in pick.get_strings() {
                if let Some(note) = fretboard.shape_note(&shape, string) {
                    let syllable = bar.calc_syllable(&note.pitch);
                    let data = PickNoteData::new(&bar, *duration, *pos, string, syllable);
                    PickNote::create(&mut commands, entity, &config, data);
                }
            }
        }
    }
}

fn play_pick_tone<const S: usize>(
    mut _commands: Commands,
    _config: Res<BevyConfig>,
    query: Query<
        (&Parent, &Pick, &BarPosition, &EntryState),
        Changed<EntryState>,
    >,
    layer_query: Query<(&Arc<TabBar>, &Fretboard<S>, &Children)>,
    shape_query: Query<&HandShape<S>>,
    mut play_note_evts: EventWriter<PlayToneEvent>,
    mut stop_note_evts: EventWriter<StopToneEvent>,
) {
    for (parent, pick, pos, state) in query.iter() {
        if !state.is_idle() {
            if let Some((_bar, fretboard, shape)) =
                FrettedPlugin::get_fretted_shape(&layer_query, &shape_query, parent.0, pos)
            {
                let tone = fretboard.pick_tone(&shape, pick);
                if !tone.is_none() {
                    if state.is_playing() {
                        play_note_evts.send(PlayToneEvent(tone));
                    } else if state.is_played() {
                        stop_note_evts.send(StopToneEvent(tone));
                    }
                }
            }
        }
    }
}

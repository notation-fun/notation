use std::sync::Arc;

use bevy::prelude::*;

use notation_model::prelude::{BarLane, Entry, LaneEntry, Tone};

use crate::prelude::{EntryPlaying, LyonShapeOp, NotationSettings, NotationTheme};

use super::tone_mode::ToneMode;
use super::tone_note::{ToneNoteData, ToneNoteShape, ToneNoteValue};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_tone_notes.system())
        .with_system(on_entry_playing_changed.system())
}

fn create_tone_notes(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    theme: Res<NotationTheme>,
    _settings: Res<NotationSettings>,
    query: Query<(&Parent, Entity, &Arc<LaneEntry>, &Tone), Added<Tone>>,
    lane_query: Query<(&Arc<BarLane>, &ToneMode)>,
) {
    for (parent, entity, entry, tone) in query.iter() {
        if entry.prev_is_tie() {
            continue;
        }
        if let Ok((lane, mode)) = lane_query.get(parent.0) {
            let bar = lane.bar().unwrap();
            for note in tone.get_notes() {
                let data = ToneNoteData::new(entry, ToneNoteValue::new(&bar, note, *mode));
                ToneNoteShape::create(&mut commands, entity, &theme, data);
            }
        }
    }
}

fn on_entry_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &EntryPlaying, &Children), Changed<EntryPlaying>>,
    mut note_query: Query<(Entity, &mut ToneNoteData)>,
) {
    for (_entity, playing, children) in query.iter() {
        for child in children.iter() {
            if let Ok((entity, mut data)) = note_query.get_mut(*child) {
                //println!("{:?} -> {:?} -> {:?}", name, data, playing)
                data.value.playing_state = playing.value;
                ToneNoteShape::update(&mut commands, &theme, entity, &data);
            }
        }
    }
}


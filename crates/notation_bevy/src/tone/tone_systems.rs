use std::sync::Arc;

use bevy::prelude::*;

use notation_model::prelude::{BarLane, LaneEntry, Tone};

use crate::prelude::{EntryPlaying, LyonShapeOp, NotationSettings, NotationTheme, NotationAssets};

use super::tone_mode::ToneMode;
use super::tone_note::{ToneNoteData, ToneNoteShape, ToneNoteValue};

pub fn create_tone_notes(
    mut commands: Commands,
    _assets: Res<NotationAssets>,
    theme: Res<NotationTheme>,
    _settings: Res<NotationSettings>,
    query: Query<(&Parent, Entity, &Arc<LaneEntry>, &Tone), Added<Tone>>,
    lane_query: Query<(&Arc<BarLane>, &ToneMode)>,
) {
    for (parent, entity, entry, tone) in query.iter() {
        /* TODO: check whether is the first on in row
        if entry.prev_is_tie() {
            continue;
        }
         */
        if let Ok((lane, mode)) = lane_query.get(parent.0) {
            let bar = lane.bar().unwrap();
            for note in tone.get_notes() {
                let data = ToneNoteData::new(entry, ToneNoteValue::new(&bar, note, *mode));
                ToneNoteShape::create(&mut commands, &theme, entity, data);
            }
        }
    }
}

pub fn on_entry_playing_changed(
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

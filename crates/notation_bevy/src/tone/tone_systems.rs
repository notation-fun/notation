use bevy::prelude::*;

use notation_bevy_utils::prelude::ShapeOp;
use notation_model::prelude::{LaneEntry, Tone};

use crate::prelude::{EntryPlaying, NotationAssets, NotationSettings, NotationTheme};

use super::tone_mode::ToneMode;
use super::tone_note::{ToneNoteData, ToneNoteValue};

pub fn create_tone_notes(
    commands: &mut Commands,
    _assets: &NotationAssets,
    theme: &NotationTheme,
    _settings: &NotationSettings,
    entity: Entity,
    entry: &LaneEntry,
    tone: &Tone,
) {
    /* TODO: check whether is the first on in row
    if entry.prev_is_tie() {
        continue;
    }
        */
    if let Some(lane) = entry.lane() {
        let mode: ToneMode = lane.kind.into();
        let bar = lane.bar().unwrap();
        for note in tone.get_notes() {
            let data = ToneNoteData::new(entry, ToneNoteValue::new(&bar, note, mode));
            data.create(commands, theme, entity);
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
                data.update(&mut commands, &theme, entity);
            }
        }
    }
}

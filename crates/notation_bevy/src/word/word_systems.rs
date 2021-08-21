use std::sync::Arc;

use bevy::prelude::*;

use notation_model::prelude::{Entry, LaneEntry};

use crate::prelude::{EntryPlaying, LyonShapeOp, NotationSettings, NotationTheme};

use super::word_text::{WordTextData, WordTextShape, WordTextValue};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(on_word_text.system())
        .with_system(on_entry_playing_changed.system())
}

fn on_word_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme: Res<NotationTheme>,
    _settings: Res<NotationSettings>,
    query: Query<(Entity, &Arc<LaneEntry>, &WordTextValue), Added<WordTextValue>>,
) {
    for (entity, entry, text) in query.iter() {
        if entry.as_ref().prev_is_tie() {
            continue;
        }
        let data = WordTextData::new(entry, text.clone());
        WordTextShape::create_with_child(&mut commands, &theme, entity, data, |child_commands| {
            if text.word.text != "" {
                theme
                    .lyrics
                    .insert_word_text(child_commands, &asset_server, text.word.text.as_str())
            }
        });
    }
}

fn on_entry_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &EntryPlaying, &Children), Changed<EntryPlaying>>,
    mut text_query: Query<(Entity, &mut WordTextData)>,
) {
    for (_entity, playing, children) in query.iter() {
        for child in children.iter() {
            if let Ok((entity, mut data)) = text_query.get_mut(*child) {
                //println!("{:?} -> {:?} -> {:?}", name, data, playing)
                data.value.playing_state = playing.value;
                WordTextShape::update(&mut commands, &theme, entity, &data);
            }
        }
    }
}

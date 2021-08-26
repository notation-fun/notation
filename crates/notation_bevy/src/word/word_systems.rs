use bevy::prelude::*;

use notation_model::prelude::LaneEntry;

use crate::prelude::{EntryPlaying, LyonShapeOp, NotationAssets, NotationSettings, NotationTheme};

use super::word_text::{WordTextData, WordTextShape, WordTextValue};

pub fn create_word_text(
    commands: &mut Commands,
    assets: &NotationAssets,
    theme: &NotationTheme,
    _settings: &NotationSettings,
    entity: Entity,
    entry: &LaneEntry,
    text: &WordTextValue,
) {
    /* TODO: check whether is the first on in row
    if entry.prev_is_tie() {
        continue;
    }
        */
    let data = WordTextData::new(entry, text.clone());
    WordTextShape::create_with_child(commands, theme, entity, data, |child_commands| {
        if text.word.text != "" {
            theme
                .lyrics
                .insert_word_text(child_commands, &assets, text.word.text.as_str())
        }
    });
}

pub fn on_entry_playing_changed(
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

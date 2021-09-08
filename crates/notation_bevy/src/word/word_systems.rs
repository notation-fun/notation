use bevy::prelude::*;

use bevy_utils::prelude::BevyUtil;
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
    let text_entity = WordTextShape::create(commands, theme, entity, data);
    if text.word.text != "" {
        theme
            .lyrics
            .spawn_word_text(commands, text_entity, &assets, text.word.text.as_str())
    }
}

pub fn on_entry_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &EntryPlaying, &Children), Changed<EntryPlaying>>,
    mut text_query: Query<(Entity, &mut WordTextData, &Children)>,
    mut font_query: Query<&mut Text>,
) {
    for (_entity, playing, children) in query.iter() {
        for child in children.iter() {
            if let Ok((entity, mut data, text_children)) = text_query.get_mut(*child) {
                //println!("{:?} -> {:?} -> {:?}", name, data, playing)
                data.value.playing_state = playing.value;
                WordTextShape::update(&mut commands, &theme, entity, &data);
                for child in text_children.iter() {
                    if let Ok(mut text) = font_query.get_mut(*child) {
                        BevyUtil::set_text_color(&mut text, data.calc_text_color(&theme));
                    }
                }
            }
        }
    }
}

use bevy::prelude::*;

use notation_bevy_utils::prelude::{BevyUtil, ShapeOp};
use notation_model::prelude::LaneEntry;

use crate::prelude::{EntryPlaying, NotationAssets, NotationSettings, NotationTheme};

use super::word_text::{WordTextData, WordTextValue};

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
    let text_entity = data.create(commands, theme, entity);
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
    mut text_query: QuerySet<(
        Query<(Entity, &mut WordTextData, &Children)>,
        Query<(Entity, &mut WordTextData)>,
    )>,
    mut font_query: Query<&mut Text>,
) {
    for (_entity, playing, children) in query.iter() {
        for child in children.iter() {
            if let Ok((entity, mut data, text_children)) = text_query.q0_mut().get_mut(*child) {
                data.value.playing_state = playing.value;
                data.update(&mut commands, &theme, entity);
                for child in text_children.iter() {
                    if let Ok(mut text) = font_query.get_mut(*child) {
                        BevyUtil::set_text_size_color(&mut text, data.calc_text_font_size(&theme), data.calc_text_color(&theme));
                    }
                }
            } else if let Ok((entity, mut data)) = text_query.q1_mut().get_mut(*child) {
                data.value.playing_state = playing.value;
                data.update(&mut commands, &theme, entity);
            }
        }
    }
}

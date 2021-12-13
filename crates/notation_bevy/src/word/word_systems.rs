use bevy::prelude::*;

use notation_bevy_utils::prelude::{ShapeOp};
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
            .texts.lyrics
            .spawn_word_text(commands, text_entity, &assets, text.word.text.as_str())
    }
}

/*
 Update font looks a bit weird, so not using it for now, leave the codes here in case want to bring it back.
pub fn on_entry_playing_changed_with_font(
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
                        let font_size = theme.texts.lyrics.word_font_size.of_state(&text.value.playing_state);
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
 */

pub fn on_entry_playing_changed(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &EntryPlaying, &Children), Changed<EntryPlaying>>,
    mut text_query: Query<(Entity, &mut WordTextData)>,
) {
    if theme._bypass_systems { return; }
    for (_entity, playing, children) in query.iter() {
        for child in children.iter() {
            if let Ok((entity, mut data)) = text_query.get_mut(*child) {
                data.value.playing_state = playing.value;
                data.update(&mut commands, &theme, entity);
            }
        }
    }
}

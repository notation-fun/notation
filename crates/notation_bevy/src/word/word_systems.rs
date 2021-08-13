use std::sync::Arc;

use bevy::prelude::*;

use notation_model::prelude::{Entry, LaneEntry, LyricWord};

use crate::prelude::{LyonShapeOp, NotationSettings, NotationTheme};

use super::word_text::{WordTextData, WordTextShape};

pub fn new_system_set() -> SystemSet {
    SystemSet::new().with_system(on_word_text.system())
}

fn on_word_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme: Res<NotationTheme>,
    _settings: Res<NotationSettings>,
    query: Query<(Entity, &Arc<LaneEntry>, &LyricWord), Added<LyricWord>>,
) {
    for (entity, entry, word) in query.iter() {
        if entry.as_ref().prev_is_tie() {
            continue;
        }
        let data = WordTextData::new(entry, word.clone());
        WordTextShape::create_with_child(&mut commands, entity, &theme, data, |child_commands| {
            if word.text != "" {
                theme
                    .lyrics
                    .insert_word_text(child_commands, &asset_server, word.text.as_str())
            }
        });
    }
}

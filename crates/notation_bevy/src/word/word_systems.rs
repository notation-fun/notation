use std::sync::Arc;

use bevy::prelude::*;

use notation_model::prelude::{BarPosition, Duration, TabBar};

use crate::{lyrics::lyrics_grid::LyricsGrid, prelude::{LyonShapeOp, NotationSettings, NotationTheme}};

use super::{word_bundle::WordText, word_text::{WordTextData, WordTextShape}};

pub fn new_system_set() -> SystemSet {
    SystemSet::new().with_system(create_word_text.system())
}

fn create_word_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme: Res<NotationTheme>,
    _settings: Res<NotationSettings>,
    query: Query<(&Parent, Entity, &WordText, &Duration, &BarPosition), Added<WordText>>,
    layer_query: Query<(&Arc<TabBar>, &LyricsGrid)>,
) {
    for (parent, entity, text, duration, pos) in query.iter() {
        if let Ok((bar, _grid)) = layer_query.get(parent.0) {
            let bar_units = bar.bar_units();
            let data = WordTextData::new(bar_units, &bar, *duration, *pos, text.0.clone());
            WordTextShape::create_with_child(&mut commands, entity, &theme, data, |child_commands| {
                if text.0 != "" {
                    theme.lyrics.insert_word_text(child_commands, &asset_server, text.0.as_str())
                }
            });
        }
    }
}

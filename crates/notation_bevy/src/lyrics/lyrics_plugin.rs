use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::{NotationAssets, NotationSettings, NotationTheme};
use crate::word::word_text::{WordText, WordTextValue};
use notation_model::prelude::{BarLane, LaneEntry, LyricEntry};

use super::lyrics_grid::LyricsGrid;

pub struct LyricsPlugin;

impl Plugin for LyricsPlugin {
    fn build(&self, _app: &mut AppBuilder) {}
}

impl LyricsPlugin {
    pub fn insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
        commands.insert(LyricsGrid::default());
    }
    pub fn insert_entry_extra(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        entry: &LaneEntry,
        lyric_entry: &LyricEntry,
    ) {
        match lyric_entry {
            LyricEntry::Word(word, _duration) => {
                let value = WordTextValue::new(word.clone());
                commands
                    .entity(entity)
                    .insert_bundle(WordText::from(value.clone()));
                crate::word::word_systems::create_word_text(
                    commands, assets, theme, settings, entity, entry, &value,
                );
            }
        }
    }
}

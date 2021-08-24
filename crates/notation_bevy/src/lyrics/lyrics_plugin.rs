use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::word::word_text::{WordText, WordTextValue};
use notation_model::prelude::{BarLane, LyricEntry};

use super::lyrics_grid::LyricsGrid;

pub struct LyricsPlugin;

impl Plugin for LyricsPlugin {
    fn build(&self, _app: &mut AppBuilder) {}
}

impl LyricsPlugin {
    pub fn insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
        commands.insert(LyricsGrid::default());
    }
    pub fn insert_entry_extra(commands: &mut EntityCommands, entry: &LyricEntry) {
        match entry {
            LyricEntry::Word(word, _duration) => {
                let value = WordTextValue::new(word.clone());
                commands.insert_bundle(WordText::from(value));
            }
        }
    }
}

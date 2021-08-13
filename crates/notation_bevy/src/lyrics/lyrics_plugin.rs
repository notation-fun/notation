use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::{LyonShapeOp, NotationTheme, WindowResizedEvent};
use crate::word::word_text::{WordText, WordTextData, WordTextShape};
use notation_model::prelude::{BarLane, LyricEntry};

use super::lyrics_grid::LyricsGrid;

pub struct LyricsPlugin;

impl Plugin for LyricsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed.system());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    tone_note_query: Query<(Entity, &WordTextData)>,
) {
    for _evt in evts.iter() {
        for (entity, data) in tone_note_query.iter() {
            WordTextShape::update(&mut commands, &theme, entity, data);
        }
    }
}

impl LyricsPlugin {
    pub fn insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
        commands.insert(LyricsGrid::default());
    }
    pub fn insert_entry_extra(commands: &mut EntityCommands, entry: &LyricEntry) {
        match entry {
            LyricEntry::Word(word, _duration) => {
                commands.insert_bundle(WordText::from(word.clone()));
            }
        }
    }
}

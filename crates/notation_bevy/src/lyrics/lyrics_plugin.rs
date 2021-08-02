use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::sync::Arc;

use crate::{prelude::{LyonShapeOp, LyricsLayerBundle, NotationTheme, WindowResizedEvent}, word::word_text::{WordTextData, WordTextShape}};
use notation_model::prelude::Track;

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
    pub fn insert_lyrics_layer_extra(commands: &mut EntityCommands, track: Arc<Track>) {
        commands.insert_bundle(LyricsLayerBundle::new(track));
    }
}

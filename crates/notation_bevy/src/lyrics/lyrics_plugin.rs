use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::sync::Arc;

use crate::prelude::LyricsLayerBundle;
use notation_model::prelude::Track;

pub struct LyricsPlugin;

impl Plugin for LyricsPlugin {
    fn build(&self, _app: &mut AppBuilder) {
        /*
        app
            .add_system(on_add_guitar_bar.system())
        ;
         */
    }
}

impl LyricsPlugin {
    pub fn insert_lyrics_layer_extra(commands: &mut EntityCommands, track: Arc<Track>) {
        commands.insert_bundle(LyricsLayerBundle::new(track));
    }
}

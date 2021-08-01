use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::sync::Arc;

use crate::prelude::MelodyLayerBundle;
use notation_model::prelude::{Track};

pub struct MelodyPlugin;

impl Plugin for MelodyPlugin {
    fn build(&self, _app: &mut AppBuilder) {
        /*
        app
            .add_system(on_add_guitar_bar.system())
        ;
         */
    }
}

impl MelodyPlugin {
    pub fn insert_melody_layer_extra(
        commands: &mut EntityCommands,
        track: Arc<Track>,
    ) {
        commands.insert_bundle(MelodyLayerBundle::new(track));
    }
}

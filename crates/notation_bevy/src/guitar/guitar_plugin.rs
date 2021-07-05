use bevy::prelude::*;
use std::sync::Arc;

use crate::prelude::GuitarLayerBundle;
use notation_proto::prelude::{BarLayer, TabBar, Track};

pub struct GuitarPlugin;

impl Plugin for GuitarPlugin {
    fn build(&self, _app: &mut AppBuilder) {
        /*
        app
            .add_system(on_add_guitar_bar.system())
        ;
         */
    }
}

impl GuitarPlugin {
    pub fn insert_guitar_layer_extra(
        commands: &mut bevy::ecs::system::EntityCommands,
        bar: Arc<TabBar>,
        layer: Arc<BarLayer>,
        track: Arc<Track>,
    ) {
        commands.insert_bundle(GuitarLayerBundle::new(bar, layer, track));
    }
}

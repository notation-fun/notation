use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::sync::Arc;

use crate::{prelude::{LyonShapeOp, MelodyLayerBundle, NotationTheme, WindowResizedEvent}, tone::tone_note::{ToneNoteData, ToneNoteShape}};
use notation_model::prelude::Track;

pub struct MelodyPlugin;

impl Plugin for MelodyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed.system());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    tone_note_query: Query<(Entity, &ToneNoteData)>,
) {
    for _evt in evts.iter() {
        for (entity, data) in tone_note_query.iter() {
            ToneNoteShape::update(&mut commands, &theme, entity, data);
        }
    }
}

impl MelodyPlugin {
    pub fn insert_melody_layer_extra(commands: &mut EntityCommands, track: Arc<Track>) {
        commands.insert_bundle(MelodyLayerBundle::new(track));
    }
}

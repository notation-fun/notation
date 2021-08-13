use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::{LyonShapeOp, MelodyGrid, NotationTheme, WindowResizedEvent};
use crate::tone::tone_mode::ToneMode;
use crate::tone::tone_note::{ToneNoteData, ToneNoteShape};
use notation_model::prelude::BarLane;

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
    pub fn insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
        commands.insert(MelodyGrid::default());
        commands.insert(ToneMode::Melody);
    }
}

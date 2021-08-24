use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::MelodyGrid;
use crate::tone::tone_mode::ToneMode;
use notation_model::prelude::BarLane;

pub struct MelodyPlugin;

impl Plugin for MelodyPlugin {
    fn build(&self, _app: &mut AppBuilder) {}
}

impl MelodyPlugin {
    pub fn insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
        commands.insert(MelodyGrid::default());
        commands.insert(ToneMode::Melody);
    }
}

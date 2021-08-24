use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::{LyonShapeOp, MelodyGrid, NotationTheme};
use crate::tone::tone_mode::ToneMode;
use crate::tone::tone_note::{ToneNoteData, ToneNoteShape};
use notation_model::prelude::BarLane;

pub struct MelodyPlugin;

impl Plugin for MelodyPlugin {
    fn build(&self, app: &mut AppBuilder) {
    }
}

impl MelodyPlugin {
    pub fn insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
        commands.insert(MelodyGrid::default());
        commands.insert(ToneMode::Melody);
    }
}

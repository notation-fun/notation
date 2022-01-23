use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::HarmonyGrid;
use notation_model::prelude::BarLane;

pub struct HarmonyPlugin;

impl Plugin for HarmonyPlugin {
    fn build(&self, _app: &mut App) {}
}

impl HarmonyPlugin {
    pub fn insert_lane_extra(commands: &mut EntityCommands, _lane: &BarLane) {
        commands.insert(HarmonyGrid::default());
    }
}

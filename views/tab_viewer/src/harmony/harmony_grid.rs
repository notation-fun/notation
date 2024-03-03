use edger_bevy::bevy_prelude::*;

use crate::{prelude::{NotationTheme, NotationSettings}, tone::tone_mode::ToneMode};
use notation_model::prelude::*;

use crate::tone::tone_line::ToneLineData;

#[derive(Debug, Default, Component)]
pub struct HarmonyGrid();

impl HarmonyGrid {
    pub fn add_lines(
        &self,
        commands: &mut Commands,
        theme: &NotationTheme,
        _settings: &NotationSettings,
        entity: Entity,
        lane: &BarLane,
    ) {
        ToneLineData::add_lines(commands, theme, entity, lane, ToneMode::Harmony);
    }
}

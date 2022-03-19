use bevy::prelude::*;
use notation_bevy_utils::prelude::ShapeOp;

use crate::{prelude::{NotationTheme, NotationSettings}, tone::tone_mode::ToneMode};
use notation_model::{bar_lane::BarLane, prelude::{Semitones}};

use crate::tone::tone_line::{ToneLineValue, ToneLineData};

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

use bevy::prelude::*;
use notation_bevy_utils::prelude::ShapeOp;

use crate::{prelude::NotationTheme, settings::notation_settings::NotationSettings, tone::{tone_line::{ToneLineData, ToneLineValue}, tone_mode::ToneMode}};
use notation_model::{prelude::{Semitones}, bar_lane::BarLane};

#[derive(Debug, Default, Component)]
pub struct MelodyGrid();

impl MelodyGrid {
    pub fn add_lines(
        &self,
        commands: &mut Commands,
        theme: &NotationTheme,
        _settings: &NotationSettings,
        entity: Entity,
        lane: &BarLane,
    ) {
        ToneLineData::add_lines(commands, theme, entity, lane, ToneMode::Melody);
    }
}

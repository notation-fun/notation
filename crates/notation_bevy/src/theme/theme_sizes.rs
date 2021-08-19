use notation_model::prelude::PlayingState;
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::mini::mini_map::MiniMap;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct PlayingSize {
    pub idle: f32,
    pub current: f32,
    pub played: f32,
}
impl PlayingSize {
    pub fn new(idle: f32, current: f32, played: f32) -> Self {
        Self {
            idle,
            current,
            played,
        }
    }
    pub fn of_state(&self, state: &PlayingState) -> f32 {
        match state {
            PlayingState::Idle => self.idle,
            PlayingState::Current => self.current,
            PlayingState::Played => self.played,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ThemeSizes {
    pub mini_map: MiniMapSizes,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MiniMapSizes {
    pub min_bar_width: f32,
    pub max_bar_width: f32,
    pub bar_height: f32,
    pub bar_outline: PlayingSize,
    pub section_separator: f32,
    pub margin: f32,
}

impl Default for MiniMapSizes {
    fn default() -> Self {
        Self {
            min_bar_width: 4.0,
            max_bar_width: 128.0,
            bar_height: 24.0,
            bar_outline: PlayingSize::new(0.5, 2.0, 0.5),
            section_separator: 2.0,
            margin: 2.0,
        }
    }
}
impl MiniMapSizes {
    pub fn bar_height_with_margin(&self) -> f32 {
        self.bar_height + self.margin
    }
}

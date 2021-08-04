use notation_model::prelude::{Tab, TabPosition};
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::prelude::{BarLayout, NotationSettings};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct GridTheme {
    pub margin: f32,
    pub bar_size: f32,
    pub semitone_size: f32,
    pub header_height: f32,
    pub bar_height: f32,
    pub bar_separator_size: f32,
    pub bar_separator_top: f32,
    pub bar_separator_bottom: f32,
    pub bar_beat_top: f32,
    pub bar_beat_bottom: f32,
    pub pos_indicator_size: f32,
    pub pos_indicator_top: f32,
    pub pos_indicator_bottom: f32,
}

impl Default for GridTheme {
    fn default() -> Self {
        Self {
            margin: 20.0,
            bar_size: 240.0,
            semitone_size: 10.0,
            header_height: 64.0,
            bar_height: 200.0,
            bar_separator_size: 3.0,
            bar_separator_top: 100.0,
            bar_separator_bottom: -90.0,
            bar_beat_top: 90.0,
            bar_beat_bottom: -80.0,
            pos_indicator_size: 2.0,
            pos_indicator_top: 100.0,
            pos_indicator_bottom: -90.0,
        }
    }
}

impl GridTheme {
    pub fn resize(&mut self, settings: &NotationSettings) {
        let bar_size = (settings.window_width - self.margin * 2.0) / settings.layout.bars_in_window as f32;
        self.bar_size = bar_size;
    }
    pub fn calc_tab_transform(&self, settings: &NotationSettings) -> Transform {
        let x = (self.bar_size * settings.layout.bars_in_window as f32) * -0.5;
        let y = settings.window_height / 2.0 - self.margin - self.header_height;
        Transform::from_xyz(x, y, 0.0)
    }
    pub fn calc_bar_transform(&self, layout: &BarLayout) -> Transform {
        let x = self.bar_size * layout.col as f32;
        let y = -1.0 * self.bar_height * layout.row as f32;
        Transform::from_xyz(x, y, 0.0)
    }
    pub fn calc_pos_transform(
        &self,
        settings: &NotationSettings,
        tab: &Tab,
        pos: TabPosition,
    ) -> Transform {
        let bar_layout = settings.layout.calc_pos_layout(tab, pos);
        let bar_x = self.bar_size * bar_layout.col as f32;
        let bars = bar_layout.row * settings.layout.bars_in_window as usize + bar_layout.col;
        let bar_units = tab.bar_units();
        let offset_units = pos.in_tab_pos.0 - bar_units.0 * bars as f32;
        let x = bar_x + offset_units * self.bar_size / bar_units.0;
        let y = -1.0 * self.bar_height * bar_layout.row as f32;
        Transform::from_xyz(x, y, 0.0)
    }
}

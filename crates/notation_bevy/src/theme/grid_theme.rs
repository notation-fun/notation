use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::mini::mini_bar::MiniBarValue;
use crate::mini::mini_map::MiniMapBackData;
use crate::prelude::{BarLayout, NotationAppState, NotationSettings};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct GridTheme {
    pub margin: f32,
    pub bar_size: f32,
    pub lane_back_margin: bool,
    pub lane_back_color: Color,
    pub semitone_size: f32,
    pub header_height: f32,
    pub bar_height: f32,
    pub bar_separator_size: f32,
    pub bar_separator_extra: f32,
    pub bar_beat_extra: f32,
    pub pos_indicator_size: f32,
    pub pos_indicator_extra: f32,
}

impl Default for GridTheme {
    fn default() -> Self {
        Self {
            margin: 20.0,
            bar_size: 240.0,
            lane_back_margin: false,
            lane_back_color: Color::hex("33333333").unwrap(),
            semitone_size: 10.0,
            header_height: 16.0,
            bar_height: 280.0,
            bar_separator_size: 3.0,
            bar_separator_extra: 8.0,
            bar_beat_extra: 0.0,
            pos_indicator_size: 2.0,
            pos_indicator_extra: 8.0,
        }
    }
}

impl GridTheme {
    pub fn resize(&mut self, app_state: &NotationAppState, settings: &NotationSettings) {
        let bar_size =
            (app_state.window_width - self.margin * 2.0) / settings.layout.bars_in_window as f32;
        self.bar_size = bar_size;
    }
    pub fn calc_tab_transform(
        &self,
        app_state: &NotationAppState,
        settings: &NotationSettings,
    ) -> Transform {
        let x = (self.bar_size * settings.layout.bars_in_window as f32) * -0.5;
        let y = app_state.window_height / 2.0 - self.margin - self.header_height;
        Transform::from_xyz(x, y, 0.0)
    }
    pub fn calc_mini_map_transform(
        &self,
        app_state: &NotationAppState,
        mini_bar_value: &MiniBarValue,
    ) -> (Transform, MiniMapBackData) {
        let space = app_state.window_width - mini_bar_value.size * mini_bar_value.cols as f32;
        let x = space / 2.0 - self.margin;
        let mut y =
            -app_state.window_height + self.margin + self.header_height + mini_bar_value.margin;
        if mini_bar_value.rows > 1 {
            let size_and_margin = mini_bar_value.size + mini_bar_value.margin;
            y += (mini_bar_value.rows - 1) as f32 * size_and_margin;
        }
        let transform = Transform::from_xyz(x, y, 0.0);
        let height = mini_bar_value.rows as f32 * mini_bar_value.size
            + (mini_bar_value.rows + 1) as f32 * mini_bar_value.margin;
        let back_data = MiniMapBackData {
            x: -space / 2.0,
            y: mini_bar_value.size + mini_bar_value.margin,
            width: app_state.window_width,
            height,
        };
        (transform, back_data)
    }
    pub fn calc_bar_transform(&self, layout: &BarLayout) -> Transform {
        let x = self.bar_size * layout.data.col as f32;
        let y = layout.offset;
        Transform::from_xyz(x, y, 0.0)
    }
}

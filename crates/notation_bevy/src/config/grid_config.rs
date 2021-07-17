use notation_model::prelude::{Tab, TabBar, TabPosition, Units};
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GridRow(pub usize);

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GridCol(pub usize);

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct GridConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub margin: f32,
    pub bar_size: f32,
    pub semitone_size: f32,
    pub note_height: f32,
    pub note_outline: f32,
    pub bars_in_row: u8,
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

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            window_width: 1280.0,
            window_height: 720.0,
            margin: 20.0,
            bar_size: 240.0,
            semitone_size: 10.0,
            note_height: 10.0,
            note_outline: 1.0,
            bars_in_row: 4,
            header_height: 20.0,
            bar_height: 200.0,
            bar_separator_size: 4.0,
            bar_separator_top: 0.0,
            bar_separator_bottom: -140.0,
            bar_beat_top: -10.0,
            bar_beat_bottom: -130.0,
            pos_indicator_size: 8.0,
            pos_indicator_top: 10.0,
            pos_indicator_bottom: -150.0,
        }
    }
}

impl GridConfig {
    pub fn resize(&mut self, width: f32, height: f32) {
        let bar_size = (width - self.margin * 2.0) / self.bars_in_row as f32;
        println!(
            "GridConfig.resize({}, {} -> {}, {}), unit_size: {} -> {}",
            self.window_width, self.window_height, width, height, self.bar_size, bar_size,
        );
        self.window_width = width;
        self.window_height = height;
        self.bar_size = bar_size;
    }
    pub fn calc_tab_transform(&self) -> Transform {
        let x = (self.bar_size * self.bars_in_row as f32) * -0.5;
        let y = self.window_height / 2.0 - self.margin - self.header_height;
        Transform::from_xyz(x, y, 0.0)
    }
    pub fn calc_row_col(&self, index: usize) -> (GridRow, GridCol) {
        let row = GridRow(index / self.bars_in_row as usize);
        let col = GridCol(index % self.bars_in_row as usize);
        (row, col)
    }
    pub fn calc_bar_row_col(&self, bar: &TabBar) -> (GridRow, GridCol) {
        self.calc_row_col(bar.bar_ordinal - 1)
    }
    pub fn calc_bar_transform(&self, row: &GridRow, col: &GridCol) -> Transform {
        let x = self.bar_size * col.0 as f32;
        let y = -1.0 * self.bar_height * row.0 as f32;
        Transform::from_xyz(x, y, 0.0)
    }
    pub fn calc_pos_row_col(&self, tab: &Tab, pos: TabPosition) -> (GridRow, GridCol) {
        let bar_units = tab.bar_units();
        let mut index = (pos.in_tab_pos.0 / bar_units.0) as usize;
        if index >= tab.bars.len() {
            index = tab.bars.len() - 1;
        }
        self.calc_row_col(index)
    }
    pub fn calc_pos_transform(&self, tab: &Tab, pos: TabPosition) -> Transform {
        let (row, col) = self.calc_pos_row_col(tab, pos);
        let bar_x = self.bar_size * col.0 as f32;
        let bars = row.0 * self.bars_in_row as usize + col.0;
        let bar_units = tab.bar_units();
        let offset_units = pos.in_tab_pos.0 - bar_units.0 * bars as f32;
        let x = bar_x + offset_units * self.bar_size / bar_units.0;
        let y = -1.0 * self.bar_height * row.0 as f32;
        Transform::from_xyz(x, y, 0.0)
    }
}

use notation_core::prelude::{Signature, Units};
use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GridRow(pub usize);

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GridCol(pub usize);

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Inspectable)]
pub struct GridConfig {
    pub margin: f32,
    pub unit_size: f32,
    pub separator_size: f32,
    pub semitone_size: f32,
    pub note_height: f32,
    pub note_outline: f32,
    pub bars_in_row: u8,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            margin: 20.0,
            unit_size: 240.0,
            separator_size: 4.0,
            semitone_size: 10.0,
            note_height: 10.0,
            note_outline: 1.0,
            bars_in_row: 4,
        }
    }
}

impl GridConfig {
    pub fn resize(&mut self, width: f32, height: f32) {
        let unit_size = self.unit_size;
        self.unit_size = (width - self.margin * 2.0) / self.bars_in_row as f32;
        println!(
            "GridConfig.resize({}, {}), {} -> {}",
            width, height, unit_size, self.unit_size
        );
    }
    pub fn calc_tab_transform(&self, signature: &Signature) -> Transform {
        let x = (self.unit_size * Units::from(*signature).0 * self.bars_in_row as f32) * -0.5;
        let y = 100.0;
        Transform::from_xyz(x, y, 0.0)
    }
    pub fn calc_bar_transform(&self, bar_units: Units, row: &GridRow, col: &GridCol) -> Transform {
        let x = self.unit_size * bar_units.0 * col.0 as f32;
        let y = self.semitone_size * 18.0 * row.0 as f32;
        Transform::from_xyz(x, y, 0.0)
    }
}

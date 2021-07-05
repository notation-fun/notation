use serde::{Deserialize, Serialize};

use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GridRow(pub usize);

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GridCol(pub usize);

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Inspectable)]
pub struct GridConfig {
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
            unit_size: 240.0,
            separator_size: 4.0,
            semitone_size: 10.0,
            note_height: 10.0,
            note_outline: 1.0,
            bars_in_row: 4,
        }
    }
}

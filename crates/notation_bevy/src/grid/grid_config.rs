use serde::{Serialize, Deserialize};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use notation_proto::prelude::{Units, ArcLine};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Inspectable)]
pub struct GridConfig {
    #[inspectable(min = 40.0, max = 800.0)]
    pub unit_size: f32,
    #[inspectable(min = 4.0, max = 80.0)]
    pub semitone_size: f32,
    #[inspectable(min = 4.0, max = 80.0)]
    pub note_height: f32,
    #[inspectable(min = 0.5, max = 10.0)]
    pub note_outline: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            unit_size: 240.0,
            semitone_size: 16.0,
            note_height: 16.0,
            note_outline: 2.0,
        }
    }
}
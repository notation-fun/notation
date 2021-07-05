use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Inspectable)]
pub struct FrettedTheme {
    pub string_color: Color,
    pub string_space: f32,
    pub string_z: f32,
    pub pick_z: f32,
}

impl Default for FrettedTheme {
    fn default() -> Self {
        Self {
            string_color: Color::hex("D3B59C").unwrap(),
            string_space: 20.0,
            string_z: 1.0,
            pick_z: 10.0,
        }
    }
}

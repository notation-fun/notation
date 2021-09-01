use serde::{Deserialize, Serialize};

use notation_model::prelude::GUITAR_STRING_NUM;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct GuitarTheme {
    #[serde(with = "serde_arrays")]
    pub string_widthes: [f32; GUITAR_STRING_NUM],
    pub image_size: (f32, f32),
    pub string_x_factor: f32,
    pub string_y_factor: f32,
}

impl Default for GuitarTheme {
    fn default() -> Self {
        Self {
            string_widthes: [1.0, 1.2, 1.4, 2.0, 2.2, 2.4],
            image_size: (100.0, 877.0),
            string_x_factor: 0.115,
            string_y_factor: 0.337,
        }
    }
}

impl GuitarTheme {
    pub fn get_string_width(&self, string: u8) -> f32 {
        self.string_widthes[string as usize - 1]
    }
}

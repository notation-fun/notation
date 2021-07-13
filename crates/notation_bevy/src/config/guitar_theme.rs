use serde::{Deserialize, Serialize};

use bevy_inspector_egui::Inspectable;

use notation_model::prelude::GUITAR_STRING_NUM;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Inspectable)]
pub struct GuitarTheme {
    #[serde(with = "serde_arrays")]
    pub string_widthes: [f32; GUITAR_STRING_NUM],
}

impl Default for GuitarTheme {
    fn default() -> Self {
        Self {
            string_widthes: [2.3, 2.5, 2.7, 3.3, 3.6, 3.9],
        }
    }
}

impl GuitarTheme {
    pub fn get_string_width(&self, string: u8) -> f32 {
        self.string_widthes[string as usize - 1]
    }
}

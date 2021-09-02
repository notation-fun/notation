use serde::{Deserialize, Serialize};

use notation_model::prelude::{GUITAR_STRING_NUM};

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
    pub fret_finger_radius: f32,
    pub fret_y_factors: [f32; 23],
}

impl Default for GuitarTheme {
    fn default() -> Self {
        Self {
            string_widthes: [1.0, 1.2, 1.4, 2.0, 2.2, 2.4],
            image_size: (100.0, 877.0),
            string_x_factor: 0.115,
            string_y_factor: 0.337,
            fret_finger_radius: 4.0,
            fret_y_factors: [
                //0
                0.347, 0.303, 0.255, 0.209, 0.164,
                //5
                0.124, 0.085, 0.049, 0.015, -0.017,
                //10
                -0.049, -0.078, -0.107, -0.130, -0.155,
                //15
                -0.178, -0.200, -0.220, -0.239, -0.257,
                //20
                -0.274, -0.292, -0.308,
            ],
        }
    }
}

impl GuitarTheme {
    pub fn get_string_width(&self, string: u8) -> f32 {
        let index = if string < 1 {
            0
        } else if string > 6 {
            5
        } else {
            (string - 1) as usize
        };
        self.string_widthes[index]
    }
    pub fn calc_string_x(&self, string: u8, guitar_width: f32) -> f32 {
        -1.0 * (string as f32 - 3.5) * guitar_width * self.string_x_factor
    }
    pub fn calc_fret_y(&self, fret: u8, guitar_height: f32) -> f32 {
        let index = if fret as usize >= self.fret_y_factors.len() {
            self.fret_y_factors.len() - 1
        } else {
            fret as usize
        };
        self.fret_y_factors[index] * guitar_height
    }
}

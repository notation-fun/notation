use serde::{Deserialize, Serialize};

use edger_bevy_app::bevy_prelude::*;
use notation_model::prelude::GUITAR_STRING_NUM;

use super::theme_texts::NoteTexts;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GuitarTheme {
    #[serde(with = "serde_arrays")]
    pub string_widthes: [f32; GUITAR_STRING_NUM],
    pub current_extra_width: f32,
    pub hit_outline: f32,
    pub image_size: (f32, f32),
    pub string_x_factor: f32,
    pub string_y_factor: f32,
    pub fret_y_factors: [f32; 23],
    pub capo_width_factor: f32,
    pub capo_height_factor: f32,
    pub barre_width_factor: f32,
    pub barre_height_factor: f32,
    pub guitar_width: f32,
    pub hit_string_seconds_range: (f32, f32),
    pub syllable_text: NoteTexts,
    pub syllable_base_width: f32,
}

impl Default for GuitarTheme {
    fn default() -> Self {
        Self {
            string_widthes: [1.0, 1.2, 1.4, 2.0, 2.2, 2.4],
            current_extra_width: 0.5,
            hit_outline: 0.5,
            image_size: (100.0, 750.0),
            string_x_factor: 0.115,
            string_y_factor: 0.5, //0.466,
            fret_y_factors: [
                //0
                0.476, 0.425, 0.369, 0.315, 0.264, //5
                0.217, 0.172, 0.130, 0.090, 0.051, //10
                0.016, -0.018, -0.051, -0.080, -0.108, //15
                -0.135, -0.1605, -0.184, -0.206, -0.227, //20
                -0.247, -0.267, -0.286,
            ],
            capo_width_factor: 0.78,
            capo_height_factor: 0.02,
            barre_width_factor: 0.72,
            barre_height_factor: 0.016,
            guitar_width: 128.0,
            hit_string_seconds_range: (0.05, 0.15),
            syllable_text: NoteTexts {
                text_x: 0.0,
                text_y: 24.0,
                text_z: 1.0,
                horizontal_center: true,
                syllable_font_size: 18.0,
                syllable_font_color: Color::hex("FFFFFF").unwrap(),
            },
            syllable_base_width: 256.0,
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
    pub fn calc_scale(&self, guitar_width: f32) -> f32 {
        guitar_width / self.syllable_base_width
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

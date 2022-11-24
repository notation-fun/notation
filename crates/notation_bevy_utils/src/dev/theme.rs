use rand::Rng;
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

pub fn color_of_hex(hex: &str) -> Color {
    let color = Color::hex(hex).unwrap();
    color.as_rgba_linear()
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default, Resource)]
pub struct BevyUtilsTheme {
    pub layout: LayoutTheme,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct LayoutTheme {
    pub pivot_color: Color,
    pub anchor_color: Color,
    pub border_color: Color,
    pub pivot_radius: f32,
    pub anchor_radius: f32,
    pub border_line_width: f32,
}

impl Default for LayoutTheme {
    fn default() -> Self {
        Self {
            pivot_color: color_of_hex("FF0000"),
            anchor_color: color_of_hex("0000FF"),
            border_color: color_of_hex("00FF00"),
            pivot_radius: 16.0,
            anchor_radius: 16.0,
            border_line_width: 4.0,
        }
    }
}

impl LayoutTheme {
    pub fn get_view_color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let hue = rng.gen_range(0.0..360.0);
        Color::Hsla {
            hue,
            saturation: 0.5,
            lightness: 0.5,
            alpha: 0.5,
        }
    }
}

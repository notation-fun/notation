use bevy::prelude::*;
use bevy_egui::egui::color::Hsva;

use super::BevyUtil;

impl BevyUtil {
    pub fn rgb_to_egui(color: &Color) -> Hsva {
        Hsva::from_rgb([color.r(), color.g(), color.b()])
    }
}

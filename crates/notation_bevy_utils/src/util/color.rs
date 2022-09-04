#[cfg(feature = "with_egui")]
use bevy::prelude::*;

#[cfg(feature = "with_egui")]
use bevy_egui::egui::color::Hsva;

use super::BevyUtil;

impl BevyUtil {
    #[cfg(feature = "with_egui")]
    pub fn rgb_to_egui(color: &Color) -> Hsva {
        let v = color.as_linear_rgba_f32();
        Hsva::from_rgb([v[0], v[1], v[2]])
    }
}

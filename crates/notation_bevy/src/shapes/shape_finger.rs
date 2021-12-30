use bevy::prelude::*;
use notation_bevy_utils::prelude::{FillCircle, ShapeOp};
use notation_model::prelude::Finger;

use crate::prelude::NotationTheme;

#[derive(Clone, Debug)]
pub struct ShapeFingerData {
    pub string: u8,
    pub fret: Option<u8>,
    pub finger: Option<Finger>,
}

impl ShapeFingerData {
    pub fn new(string: u8, fret: Option<u8>, finger: Option<Finger>) -> Self {
        ShapeFingerData {
            string,
            fret,
            finger,
        }
    }
}

impl ShapeOp<NotationTheme, FillCircle> for ShapeFingerData {
    fn get_shape(&self, theme: &NotationTheme) -> FillCircle {
        let shapes = theme.shapes;
        let color = if self.fret.is_none() {
            shapes.shape_finger_mute_color
        } else {
            shapes.shape_finger_color
        };
        let x = shapes.shape_finger_offset_x - shapes.shape_string_space * self.string as f32;
        let y =
            shapes.shape_finger_offset_y - shapes.shape_fret_space * self.fret.unwrap_or(0) as f32;
        FillCircle {
            radius: shapes.shape_finger_radius,
            color,
            offset: Vec3::new(x, y, shapes.shape_text_z),
        }
    }
}

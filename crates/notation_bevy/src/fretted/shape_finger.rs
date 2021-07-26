use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::Finger;

use crate::config::bevy_config::BevyConfig;
use crate::prelude::{LyonShape, LyonShapeOp};

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
pub struct ShapeFingerShape<'a> {
    config: &'a BevyConfig,
    data: ShapeFingerData,
}

impl<'a> LyonShape<shapes::Circle> for ShapeFingerShape<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{:?}{:?}",
            self.data.string, self.data.fret, self.data.finger,
        )
    }
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            radius: self.config.theme.fretted.shape_finger_radius,
            center: Vec2::ZERO,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.config.theme.fretted.shape_finger_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        let fretted = self.config.theme.fretted;
        let x = fretted.shape_finger_offset_x - fretted.shape_string_space * self.data.string as f32;
        let y = fretted.shape_finger_offset_y - fretted.shape_fret_space * self.data.fret.unwrap_or(0) as f32;
        Transform::from_xyz(x, y, self.config.theme.fretted.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, ShapeFingerData, shapes::Circle, ShapeFingerShape<'a>>
    for ShapeFingerShape<'a>
{
    fn new_shape(config: &'a BevyConfig, data: ShapeFingerData) -> ShapeFingerShape<'a> {
        ShapeFingerShape::<'a> { config, data }
    }
}

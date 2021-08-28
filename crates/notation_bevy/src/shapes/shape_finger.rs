use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::Finger;

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

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
    theme: &'a NotationTheme,
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
            radius: self.theme.shapes.shape_finger_radius,
            center: Vec2::ZERO,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.shapes.shape_finger_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        let shapes = self.theme.shapes;
        let x = shapes.shape_finger_offset_x - shapes.shape_string_space * self.data.string as f32;
        let y = shapes.shape_finger_offset_y
            - shapes.shape_fret_space * self.data.fret.unwrap_or(0) as f32;
        Transform::from_xyz(x, y, self.theme.shapes.shape_text_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, ShapeFingerData, shapes::Circle, ShapeFingerShape<'a>>
    for ShapeFingerShape<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: ShapeFingerData) -> ShapeFingerShape<'a> {
        ShapeFingerShape::<'a> { theme, data }
    }
}

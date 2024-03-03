use edger_bevy::bevy_prelude::*;
use edger_bevy::bevy_prototype_lyon;
use edger_bevy::prelude::{FillRectangle, ShapeOp};

use crate::prelude::NotationTheme;

#[derive(Clone, Debug, Component)]
pub struct ShapeBarreData {
    pub barre: u8,
}

impl ShapeBarreData {
    pub fn new(barre: u8) -> Self {
        ShapeBarreData { barre }
    }
}

impl ShapeOp<NotationTheme, FillRectangle> for ShapeBarreData {
    fn get_shape(&self, theme: &NotationTheme) -> FillRectangle {
        let shapes = theme.shapes;
        let color = shapes.shape_finger_color;
        let x = shapes.shape_barre_offset_x;
        let y = shapes.shape_barre_offset_y;
        FillRectangle {
            width: shapes.shape_barre_width,
            height: shapes.shape_barre_height,
            origin: bevy_prototype_lyon::prelude::shapes::RectangleOrigin::Center,
            color,
            offset: Vec3::new(x, y, theme.shapes.shape_text_z),
        }
    }
}

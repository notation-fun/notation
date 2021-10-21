use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::shape::Shape;

#[derive(Clone, Debug)]
pub struct StrokeLine {
    pub from: Vec2,
    pub to: Vec2,
    pub line_width: f32,
    pub color: Color,
    pub offset: Vec3,
}

impl Shape<shapes::Line> for StrokeLine {
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(self.from, self.to)
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Stroke(StrokeOptions::default().with_line_width(self.line_width))
    }
    fn get_transform(&self) -> Transform {
        Transform::from_xyz(
            self.offset.x,
            self.offset.y,
            self.offset.z,
        )
    }
}
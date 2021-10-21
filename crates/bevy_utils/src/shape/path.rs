use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::shape::Shape;

#[derive(Clone, Debug)]
pub struct StrokePath {
    pub size: Vec2,
    pub path: String,
    pub color: Color,
    pub line_width: f32,
    pub offset: Vec3,
    pub scale: f32,
    pub angle: f32,
}

impl Shape<shapes::SvgPathShape> for StrokePath {
    fn get_shape(&self) -> shapes::SvgPathShape {
        shapes::SvgPathShape {
            svg_doc_size_in_px: self.size,
            svg_path_string: self.path.clone(),
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Stroke(
            StrokeOptions::default().with_line_width(self.line_width),
        )
    }
    fn get_transform(&self) -> Transform {
        Transform {
            translation: self.offset,
            rotation: Quat::from_rotation_z(self.angle),
            scale: Vec3::new(self.scale, self.scale, 1.0),
        }
    }
}
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::shape::{Shape, SingleShape};

#[derive(Clone, Debug)]
pub struct StrokeLine {
    pub from: Vec2,
    pub to: Vec2,
    pub line_width: f32,
    pub color: Color,
    pub offset: Vec3,
}

impl Shape for StrokeLine {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl SingleShape<shapes::Line> for StrokeLine {
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(self.from, self.to)
    }
    fn get_stroke(&self) -> Option<Stroke> {
        Some(Stroke::new(self.color, self.line_width))
    }
    fn get_transform(&self) -> Transform {
        Transform::from_xyz(self.offset.x, self.offset.y, self.offset.z)
    }
}

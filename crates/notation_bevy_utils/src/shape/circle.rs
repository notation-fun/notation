use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::BevyUtil;

use super::shape::{Shape, SingleShape};

#[derive(Clone, Debug)]
pub struct FillCircle {
    pub radius: f32,
    pub color: Color,
    pub offset: Vec3,
}

impl Shape for FillCircle {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl SingleShape<shapes::Circle> for FillCircle {
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            center: Vec2::ZERO,
            radius: self.radius,
        }
    }
    fn get_fill(&self) -> Option<Fill>{
        Some(Fill::color(self.color))
    }
    fn get_transform(&self) -> Transform {
        if self.radius <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        Transform::from_xyz(self.offset.x, self.offset.y, self.offset.z)
    }
}

#[derive(Clone, Debug)]
pub struct StrokeCircle {
    pub radius: f32,
    pub line_width: f32,
    pub color: Color,
    pub offset: Vec3,
}

impl Shape for StrokeCircle {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl SingleShape<shapes::Circle> for StrokeCircle {
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            center: Vec2::ZERO,
            radius: self.radius,
        }
    }
    fn get_stroke(&self) -> Option<Stroke> {
        Some(Stroke::new(self.color, self.line_width))
    }
    fn get_transform(&self) -> Transform {
        if self.radius <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        Transform::from_xyz(self.offset.x, self.offset.y, self.offset.z)
    }
}

#[derive(Clone, Debug)]
pub struct OutlineCircle {
    pub radius: f32,
    pub color: Color,
    pub outline_width: f32,
    pub outline_color: Color,
    pub offset: Vec3,
}

impl Shape for OutlineCircle {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl SingleShape<shapes::Circle> for OutlineCircle {
    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            center: Vec2::ZERO,
            radius: self.radius,
        }
    }
    fn get_fill(&self) -> Option<Fill> {
        Some(Fill::color(self.color))
    }
    fn get_stroke(&self) -> Option<Stroke> {
        if self.outline_width > 0.0 {
            Some(Stroke::new(self.outline_color, self.outline_width))
        } else {
            None
        }
    }
    fn get_transform(&self) -> Transform {
        if self.radius <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        Transform::from_xyz(self.offset.x, self.offset.y, self.offset.z)
    }
}

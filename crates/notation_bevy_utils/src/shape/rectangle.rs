use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::BevyUtil;

use super::shape::{Shape, SingleShape};

#[derive(Clone, Debug)]
pub struct FillRectangle {
    pub width: f32,
    pub height: f32,
    pub origin: shapes::RectangleOrigin,
    pub color: Color,
    pub offset: Vec3,
}

impl Shape for FillRectangle {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl SingleShape<shapes::Rectangle> for FillRectangle {
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            extents: Vec2::new(self.width, self.height),
            origin: self.origin,
        }
    }
    fn get_fill(&self) -> Option<Fill> {
        Some(Fill::color(self.color))
    }
    fn get_transform(&self) -> Transform {
        if self.width <= 0.0 || self.height <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        Transform::from_xyz(self.offset.x, self.offset.y, self.offset.z)
    }
}

#[derive(Clone, Debug)]
pub struct StrokeRectangle {
    pub width: f32,
    pub height: f32,
    pub origin: shapes::RectangleOrigin,
    pub line_width: f32,
    pub color: Color,
    pub offset: Vec3,
}

impl Shape for StrokeRectangle {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl SingleShape<shapes::Rectangle> for StrokeRectangle {
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            extents: Vec2::new(self.width, self.height),
            origin: self.origin,
        }
    }
    fn get_stroke(&self) -> Option<Stroke> {
        Some(Stroke::new(self.color, self.line_width))
    }
    fn get_transform(&self) -> Transform {
        if self.width <= 0.0 || self.height <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        Transform::from_xyz(self.offset.x, self.offset.y, self.offset.z)
    }
}

#[derive(Clone, Debug)]
pub struct OutlineRectangle {
    pub width: f32,
    pub height: f32,
    pub origin: shapes::RectangleOrigin,
    pub color: Color,
    pub outline_width: f32,
    pub outline_color: Color,
    pub offset: Vec3,
}

impl Shape for OutlineRectangle {
    fn _create(&self, commands: &mut Commands, entity: Entity) {
        self._do_create(commands, entity);
    }
}
impl SingleShape<shapes::Rectangle> for OutlineRectangle {
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            extents: Vec2::new(self.width, self.height),
            origin: self.origin,
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
        if self.width <= 0.0 || self.height <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        Transform::from_xyz(self.offset.x, self.offset.y, self.offset.z)
    }
}

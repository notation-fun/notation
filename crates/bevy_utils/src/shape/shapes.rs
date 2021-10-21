use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct Shapes {
    pub geometries: Vec<Box<dyn Geometry>>,
    pub colors: ShapeColors,
    pub draw_mode: DrawMode,
    pub transform: Transform,
}


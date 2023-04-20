use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::shape::Shape;

pub trait DoubleShape<T1: Geometry, T2: Geometry>: Shape {
    fn get_shape1(&self) -> T1;
    fn get_shape2(&self) -> T2;
    fn get_fill(&self) -> Option<Fill> {
        None
    }
    fn get_stroke(&self) -> Option<Stroke> {
        None
    }
    fn get_transform(&self) -> Transform;
    fn _do_create(&self, commands: &mut Commands, entity: Entity) {
        let shape1 = self.get_shape1();
        let shape2 = self.get_shape2();
        let multishape = GeometryBuilder::new()
            .add(&shape1)
            .add(&shape2);
        let mut op = commands.entity(entity);
        op
            .insert(ShapeBundle{
                path: multishape.build(),
                transform: self.get_transform(),
                ..default()
            });
        if let Some(fill) = self.get_fill() {
            op.insert(fill);
        }
        if let Some(stroke) = self.get_stroke() {
            op.insert(stroke);
        }
    }
}

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::shape::Shape;

pub trait DoubleShape<T1: Geometry, T2: Geometry>: Shape {
    fn get_shape1(&self) -> T1;
    fn get_shape2(&self) -> T2;
    fn get_draw_mode(&self) -> DrawMode;
    fn get_transform(&self) -> Transform;
    fn _do_create(&self, commands: &mut Commands, entity: Entity) {
        let shape1 = self.get_shape1();
        let shape2 = self.get_shape2();
        let draw_mode = self.get_draw_mode();
        let transform = self.get_transform();
        let multishape = GeometryBuilder::new()
            .add(&shape1)
            .add(&shape2);
        commands
            .entity(entity)
            .insert(multishape.build(draw_mode, transform));
    }
}

use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

pub trait Shape {
    fn _create(&self, commands: &mut Commands, entity: Entity);
    fn create(&self, commands: &mut Commands, parent: Entity) -> Entity {
        let entity = commands.spawn_empty().id();
        commands.entity(parent).push_children(&[entity]);
        self._create(commands, entity);
        entity
    }
    fn update(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<ShapeBundle>();
        self._create(commands, entity);
    }
}

pub trait SingleShape<T: Geometry>: Shape {
    fn get_shape(&self) -> T;
    fn get_fill(&self) -> Option<Fill> {
        None
    }
    fn get_stroke(&self) -> Option<Stroke> {
        None
    }
    fn get_transform(&self) -> Transform;
    fn _do_create(&self, commands: &mut Commands, entity: Entity) {
        let shape = self.get_shape();
        let mut op = commands.entity(entity);
        op
            .insert(ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
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

pub trait ShapeOp<Theme, S: Shape>: Clone + Component {
    fn get_shape(&self, theme: &Theme) -> S;
    fn create(&self, commands: &mut Commands, theme: &Theme, parent: Entity) -> Entity {
        let shape = self.get_shape(theme);
        let shape_entity = shape.create(commands, parent);
        commands.entity(shape_entity).insert(self.clone());
        shape_entity
    }
    fn update(&self, commands: &mut Commands, theme: &Theme, entity: Entity) {
        let shape = self.get_shape(theme);
        shape.update(commands, entity);
    }
}

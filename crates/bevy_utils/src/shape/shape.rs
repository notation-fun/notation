use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;

pub trait Shape<T: Geometry> {
    fn get_shape(&self) -> T;
    fn get_colors(&self) -> ShapeColors;
    fn get_draw_mode(&self) -> DrawMode;
    fn get_transform(&self) -> Transform;
    fn _create(&self, entity_commands: &mut EntityCommands) {
        let shape = self.get_shape();
        let colors = self.get_colors();
        let draw_mode = self.get_draw_mode();
        let transform = self.get_transform();
        entity_commands.insert_bundle(GeometryBuilder::build_as(
            &shape, colors, draw_mode, transform,
        ));
    }
    fn create(&self, commands: &mut Commands, parent: Entity) -> Entity {
        let mut entity_commands = commands.spawn();
        self._create(&mut entity_commands);
        let shape_entity = entity_commands.id();
        commands.entity(parent).push_children(&[shape_entity]);
        shape_entity
    }
    fn update(&self, commands: &mut Commands, entity: Entity) {
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove_bundle::<ShapeBundle>();
        self._create(&mut entity_commands);
    }
}

pub trait ShapeOp<Theme, T: Geometry, S: Shape<T>> : Clone + Send + Sync + 'static {
    fn get_shape(&self, theme: &Theme) -> S;
    fn create(&self, commands: &mut Commands, theme: &Theme, entity: Entity) -> Entity {
        let shape = self.get_shape(theme);
        let shape_entity = shape.create(commands, entity);
        commands.entity(shape_entity).insert(self.clone());
        shape_entity
    }
    fn update(&self, commands: &mut Commands, theme: &Theme, entity: Entity) {
        let shape = self.get_shape(theme);
        shape.update(commands, entity);
    }
}

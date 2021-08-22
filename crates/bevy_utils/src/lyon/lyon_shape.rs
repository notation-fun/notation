use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

pub trait LyonShape<T: Geometry> {
    fn get_name(&self) -> String;
    fn get_shape(&self) -> T;
    fn get_colors(&self) -> ShapeColors;
    fn get_draw_mode(&self) -> DrawMode;
    fn get_transform(&self) -> Transform;
    fn _insert_lyon(&self, entity_commands: &mut EntityCommands) {
        let shape = self.get_shape();
        let colors = self.get_colors();
        let draw_mode = self.get_draw_mode();
        let transform = self.get_transform();
        entity_commands.insert_bundle(GeometryBuilder::build_as(
            &shape, colors, draw_mode, transform,
        ));
    }
    fn insert_lyon<F>(&self, commands: &mut Commands, entity: Entity, extra: F) -> Entity
    where
        F: Fn(&mut EntityCommands),
    {
        let mut entity_commands = commands.spawn();
        entity_commands.insert(Name::from(self.get_name().as_str()));
        self._insert_lyon(&mut entity_commands);
        extra(&mut entity_commands);
        let line_entity = entity_commands.id();
        commands.entity(entity).push_children(&[line_entity]);
        line_entity
    }
    fn update_lyon(&self, commands: &mut Commands, entity: Entity) {
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove_bundle::<ShapeBundle>();
        self._insert_lyon(&mut entity_commands);
    }
}

pub trait LyonShapeOp<'a, Theme, Data: Clone + Send + Sync + 'static, T: Geometry, Op: LyonShape<T>>
{
    fn new_shape(theme: &'a Theme, data: Data) -> Op;
    fn create(commands: &mut Commands, theme: &'a Theme, entity: Entity, data: Data) -> Entity {
        let shape = Self::new_shape(theme, data.clone());
        shape.insert_lyon(commands, entity, |entity_commands| {
            entity_commands.insert(data.clone());
        })
    }
    fn create_with_child<F>(
        commands: &mut Commands,
        theme: &'a Theme,
        entity: Entity,
        data: Data,
        setup_child: F,
    ) -> Entity
    where
        F: Fn(&mut EntityCommands),
    {
        let entity = Self::create(commands, theme, entity, data);
        let mut child_commands = commands.spawn();
        setup_child(&mut child_commands);
        let child_entity = child_commands.id();
        commands.entity(entity).push_children(&[child_entity]);
        entity
    }
    fn update(commands: &mut Commands, theme: &'a Theme, entity: Entity, data: &Data) {
        let shape = Self::new_shape(theme, data.clone());
        shape.update_lyon(commands, entity);
    }
}

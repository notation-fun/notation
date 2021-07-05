use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::sync::Arc;

use crate::prelude::BevyConfig;
use notation_proto::prelude::{TabBar, Units};

pub struct FrettedGrid<const S: usize> {}

impl<const S: usize> FrettedGrid<S> {
    pub fn add_string(
        commands: &mut Commands,
        config: &BevyConfig,
        parent: Entity,
        width: Units,
        string: usize,
    ) -> Entity {
        let shape = shapes::Line(Vec2::ZERO, Vec2::new(config.grid.unit_size * width.0, 0.0));
        let color = config.theme.fretted.string_color;
        let line_width = config.theme.guitar.string_widthes[string];
        let y = string as f32 * -1.0 * config.theme.fretted.string_space;
        let string_entity = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(color),
                DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width)),
                Transform::from_xyz(0.0, y, config.theme.fretted.string_z),
            ))
            .insert(Name::from(format!("String {}", string).as_str()))
            .id();
        commands.entity(parent).push_children(&[string_entity]);
        string_entity
    }
    pub fn add_strings(
        &self,
        commands: &mut Commands,
        config: &BevyConfig,
        entity: Entity,
        tab_bar: &Arc<TabBar>,
    ) -> () {
        let width = tab_bar.units();
        for string_index in 0..S {
            Self::add_string(commands, config, entity, width, string_index);
        }
    }
}

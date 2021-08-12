use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use notation_model::prelude::{BarLane, BarPosition};
use std::sync::Arc;

use super::hand_bundles::HandShapeBundle6;
use super::hand_bundles::HandShapeBundle4;
use super::shape_diagram::{ShapeDiagramData6, ShapeDiagram6};
use super::shape_diagram::{ShapeDiagramData4, ShapeDiagram4};
use crate::prelude::{LyonShapeOp, NotationTheme, WindowResizedEvent};
use notation_model::prelude::{Fretboard6, FrettedEntry6, HandShape6, TabBar};
use notation_model::prelude::{Fretboard4, FrettedEntry4, HandShape4};

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed6.system());
        app.add_system(on_config_changed4.system());
        app.add_system_set(super::hand_systems::new_system_set());
    }
}

macro_rules! impl_shapes_plugin {
    ($on_config:ident, $insert_entry_extra:ident,
        $fretted_entry:ident, $fretboard:ident, $hand_shape:ident,
        $diagram:ident, $diagram_data:ident, $hand_shape_bundle:ident
    ) => {
        fn $on_config(
            mut commands: Commands,
            mut evts: EventReader<WindowResizedEvent>,
            theme: Res<NotationTheme>,
            shapes_query: Query<(Entity, &$diagram_data)>,
        ) {
            for _evt in evts.iter() {
                for (entity, data) in shapes_query.iter() {
                    $diagram::update(&mut commands, &theme, entity, data);
                }
            }
        }

        impl ShapesPlugin {
            pub fn $insert_entry_extra(
                commands: &mut EntityCommands,
                entry: &$fretted_entry,
            ) {
                match entry {
                    $fretted_entry::Shape(shape, _) => {
                        commands.insert_bundle($hand_shape_bundle::from(*shape));
                    }
                    _ => (),
                }
            }
        }
    }
}

impl_shapes_plugin!(on_config_changed6, insert_entry_extra6,
    FrettedEntry6, Fretboard6, HandShape6, ShapeDiagram6, ShapeDiagramData6, HandShapeBundle6);
impl_shapes_plugin!(on_config_changed4, insert_entry_extra4,
    FrettedEntry4, Fretboard4, HandShape4, ShapeDiagram4, ShapeDiagramData4, HandShapeBundle4);
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::NotationAssetsStates;

use super::hand_bundles::{HandShapeBundle4, HandShapeBundle6};

use notation_model::prelude::{BarLane, FrettedEntry4, FrettedEntry6};

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(NotationAssetsStates::Loaded)
            .with_system(super::hand_systems::on_add_hand_shape6.system())
            .with_system(super::hand_systems::on_add_hand_shape4.system())
        );
    }
}

impl ShapesPlugin {
    pub fn insert_lane_extra(_commands: &mut EntityCommands, _lane: &BarLane) {}
}

macro_rules! impl_shapes_plugin {
    ($insert_entry_extra:ident,
        $fretted_entry:ident, $fretboard:ident, $hand_shape:ident,
        $diagram:ident, $diagram_data:ident, $hand_shape_bundle:ident
    ) => {
        impl ShapesPlugin {
            pub fn $insert_entry_extra(commands: &mut EntityCommands, entry: &$fretted_entry) {
                match entry {
                    $fretted_entry::Shape(shape, _) => {
                        commands.insert_bundle($hand_shape_bundle::from(*shape));
                    }
                    _ => (),
                }
            }
        }
    };
}

impl_shapes_plugin!(
    insert_entry_extra6,
    FrettedEntry6,
    Fretboard6,
    HandShape6,
    ShapeDiagram6,
    ShapeDiagramData6,
    HandShapeBundle6
);
impl_shapes_plugin!(
    insert_entry_extra4,
    FrettedEntry4,
    Fretboard4,
    HandShape4,
    ShapeDiagram4,
    ShapeDiagramData4,
    HandShapeBundle4
);

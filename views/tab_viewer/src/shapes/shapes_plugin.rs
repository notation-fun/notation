use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::{NotationAssets, NotationSettings, NotationTheme};

use super::hand_bundles::{HandShapeBundle4, HandShapeBundle6};

use notation_model::prelude::{BarLane, FrettedEntry4, FrettedEntry6, LaneEntry};

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, _app: &mut App) {
        /*
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
        );
        */
    }
}

impl ShapesPlugin {
    pub fn insert_lane_extra(_commands: &mut EntityCommands, _lane: &BarLane) {}
}

macro_rules! impl_shapes_plugin {
    ($insert_entry_extra:ident, $create_hand_shape:ident,
        $fretted_entry:ident, $fretboard:ident, $hand_shape:ident,
        $diagram:ident, $diagram_data:ident, $hand_shape_bundle:ident
    ) => {
        impl ShapesPlugin {
            pub fn $insert_entry_extra(
                commands: &mut Commands,
                assets: &NotationAssets,
                theme: &NotationTheme,
                settings: &NotationSettings,
                entity: Entity,
                entry: &LaneEntry,
                fretted_entry: &$fretted_entry,
            ) {
                match fretted_entry {
                    $fretted_entry::Shape(shape, _) => {
                        commands
                            .entity(entity)
                            .insert($hand_shape_bundle::from(*shape));
                        super::hand_systems::$create_hand_shape(
                            commands, assets, theme, settings, entity, entry, shape,
                        );
                    }
                    _ => (),
                }
            }
        }
    };
}

impl_shapes_plugin!(
    insert_entry_extra6,
    create_hand_shape6,
    FrettedEntry6,
    Fretboard6,
    HandShape6,
    ShapeDiagram6,
    ShapeDiagramData6,
    HandShapeBundle6
);
impl_shapes_plugin!(
    insert_entry_extra4,
    create_hand_shape4,
    FrettedEntry4,
    Fretboard4,
    HandShape4,
    ShapeDiagram4,
    ShapeDiagramData4,
    HandShapeBundle4
);

use bevy::prelude::*;

use notation_model::prelude::LaneEntry;

use super::shape_diagram::{ShapeDiagram4, ShapeDiagram6, ShapeDiagramData4, ShapeDiagramData6};
use super::shape_finger::{ShapeFingerData, ShapeFingerShape};
use crate::prelude::{LyonShapeOp, NotationAssets, NotationSettings, NotationTheme};
use notation_model::prelude::{HandShape4, HandShape6};

macro_rules! impl_hand_system {
    ($type:ident, $hand_shape:ident, $diagram:ident, $diagram_data:ident) => {
        pub fn $type(
            commands: &mut Commands,
            assets: &NotationAssets,
            theme: &NotationTheme,
            _settings: &NotationSettings,
            entity: Entity,
            entry: &LaneEntry,
            shape: &$hand_shape,
        ) {
            let data = $diagram_data::from((entry, *shape));
            let diagram_entity =
                $diagram::create_with_child(commands, theme, entity, data, |child_commands| {
                    if let Some(mark) = entry.model().prev_as_mark() {
                        theme
                            .shapes
                            .insert_shape_text(child_commands, &assets, &mark);
                    }
                });
            for (string, fret) in shape.frets.iter().enumerate() {
                if fret.is_none() || fret.unwrap() > 0 {
                    let finger_data = ShapeFingerData::new(string as u8, *fret, None);
                    ShapeFingerShape::create(commands, theme, diagram_entity, finger_data);
                }
            }
        }
    };
}

impl_hand_system!(
    create_hand_shape6,
    HandShape6,
    ShapeDiagram6,
    ShapeDiagramData6
);
impl_hand_system!(
    create_hand_shape4,
    HandShape4,
    ShapeDiagram4,
    ShapeDiagramData4
);

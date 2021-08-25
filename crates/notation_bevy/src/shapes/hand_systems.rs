use bevy::prelude::*;

use notation_model::prelude::LaneEntry;
use std::sync::Arc;

use super::shape_diagram::{ShapeDiagram4, ShapeDiagram6, ShapeDiagramData4, ShapeDiagramData6};
use super::shape_finger::{ShapeFingerData, ShapeFingerShape};
use crate::prelude::{LyonShapeOp, NotationTheme, NotationAssets};
use notation_model::prelude::{HandShape4, HandShape6};

macro_rules! impl_on_add_hand_shape {
    ($type:ident, $hand_shape:ident, $diagram:ident, $diagram_data:ident) => {
        pub fn $type(
            mut commands: Commands,
            theme: Res<NotationTheme>,
            assets: Res<NotationAssets>,
            query: Query<(Entity, &Arc<LaneEntry>, &$hand_shape), Added<$hand_shape>>,
        ) {
            for (entity, entry, shape) in query.iter() {
                let data = $diagram_data::from((entry.as_ref(), *shape));
                let diagram_entity = $diagram::create_with_child(
                    &mut commands,
                    &theme,
                    entity,
                    data,
                    |child_commands| {
                        if let Some(mark) = entry.model().prev_as_mark() {
                            theme
                                .shapes
                                .insert_shape_text(child_commands, &assets, &mark);
                        }
                    },
                );
                for (string, fret) in shape.frets.iter().enumerate() {
                    if fret.is_none() || fret.unwrap() > 0 {
                        let finger_data = ShapeFingerData::new(string as u8, *fret, None);
                        ShapeFingerShape::create(
                            &mut commands,
                            &theme,
                            diagram_entity,
                            finger_data,
                        );
                    }
                }
            }
        }
    };
}

impl_on_add_hand_shape!(
    on_add_hand_shape6,
    HandShape6,
    ShapeDiagram6,
    ShapeDiagramData6
);
impl_on_add_hand_shape!(
    on_add_hand_shape4,
    HandShape4,
    ShapeDiagram4,
    ShapeDiagramData4
);

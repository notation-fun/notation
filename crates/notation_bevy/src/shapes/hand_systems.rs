use bevy::prelude::*;

use notation_model::prelude::{BarPosition, Duration, LaneEntry};
use std::sync::Arc;

use super::shape_diagram::{ShapeDiagram4, ShapeDiagram6, ShapeDiagramData4, ShapeDiagramData6};
use super::shape_finger::{ShapeFingerData, ShapeFingerShape};
use crate::prelude::{LyonShapeOp, NotationTheme};
use notation_model::prelude::{HandShape4, HandShape6, TabBar};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(on_add_hand_shape6.system())
        .with_system(on_add_hand_shape4.system())
}

macro_rules! impl_on_add_hand_shape {
    ($type:ident, $hand_shape:ident, $diagram:ident, $diagram_data:ident) => {
        fn $type(
            mut commands: Commands,
            asset_server: Res<AssetServer>,
            theme: Res<NotationTheme>,
            query: Query<
                (
                    &Parent,
                    Entity,
                    &Arc<LaneEntry>,
                    &$hand_shape,
                    &Duration,
                    &BarPosition,
                ),
                Added<$hand_shape>,
            >,
            parent_query: Query<&Arc<TabBar>>,
        ) {
            for (parent, entity, entry, shape, duration, position) in query.iter() {
                if let Ok(bar) = parent_query.get(parent.0) {
                    let data =
                        $diagram_data::new(bar.bar_units(), &bar, *duration, *position, *shape);
                    let diagram_entity = $diagram::create_with_child(
                        &mut commands,
                        entity,
                        &theme,
                        data,
                        |child_commands| {
                            if let Some(mark) = entry.model().prev_as_mark() {
                                theme.shapes.insert_shape_text(
                                    child_commands,
                                    &asset_server,
                                    &mark,
                                );
                            }
                        },
                    );
                    for (string, fret) in shape.frets.iter().enumerate() {
                        if fret.is_none() || fret.unwrap() > 0 {
                            let finger_data = ShapeFingerData::new(string as u8, *fret, None);
                            ShapeFingerShape::create(
                                &mut commands,
                                diagram_entity,
                                &theme,
                                finger_data,
                            );
                        }
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

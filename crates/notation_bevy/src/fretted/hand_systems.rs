use bevy::prelude::*;

use notation_model::prelude::{BarPosition, Duration};
use std::sync::Arc;

use super::shape_diagram::{ShapeDiagramData, ShapeDiagramShape};
use super::shape_finger::{ShapeFingerData, ShapeFingerShape};
use crate::prelude::{LyonShapeOp, NotationTheme};
use notation_model::prelude::{HandShape, TabBar};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(on_add_shape_diagram::<6>.system())
        .with_system(on_add_shape_diagram::<4>.system())
}

fn on_add_shape_diagram<const S: usize>(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(&Parent, Entity, &HandShape<S>, &Duration, &BarPosition), Added<HandShape<S>>>,
    parent_query: Query<&Arc<TabBar>>,
) {
    for (parent, entity, shape, duration, position) in query.iter() {
        if let Ok(bar) = parent_query.get(parent.0) {
            let data =
                ShapeDiagramData::<S>::new(bar.bar_units(), &bar, *duration, *position, *shape);
            let diagram_entity =
                ShapeDiagramShape::<S>::create(&mut commands, entity, &theme, data);
            for (string, fret) in shape.frets.iter().enumerate() {
                if fret.is_none() || fret.unwrap() > 0 {
                    let finger_data = ShapeFingerData::new(string as u8, *fret, None);
                    ShapeFingerShape::create(&mut commands, diagram_entity, &theme, finger_data);
                }
            }
        }
    }
}

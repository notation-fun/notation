use edger_bevy::bevy_prelude::*;

use notation_model::prelude::LaneEntry;

use edger_bevy::prelude::ShapeOp;

use super::shape_barre::ShapeBarreData;
use super::shape_diagram::{ShapeDiagramData4, ShapeDiagramData6};
use super::shape_finger::ShapeFingerData;
use crate::prelude::{NotationAssets, NotationSettings, NotationTheme};
use notation_model::prelude::{HandShape4, HandShape6};

macro_rules! impl_hand_system {
    ($type:ident, $hand_shape:ident, $diagram_data:ident) => {
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
            let diagram_entity = data.create(commands, theme, entity);
            if let Some(mark) = entry.model().prev_as_mark() {
                theme
                    .shapes
                    .insert_shape_text(commands, &assets, diagram_entity, &mark);
            }
            let barre = shape.barre.unwrap_or(0);
            if barre > 0 {
                let barre_data = ShapeBarreData::new(barre);
                barre_data.create(commands, theme, diagram_entity);
                theme
                    .shapes
                    .insert_barre_text(commands, &assets, diagram_entity, barre);
            }
            for (string, fret) in shape.frets.iter().enumerate() {
                if fret.is_none() || fret.unwrap() > 0 {
                    let finger_data = ShapeFingerData::new(string as u8, *fret, None);
                    finger_data.create(commands, theme, diagram_entity);
                }
            }
        }
    };
}

impl_hand_system!(create_hand_shape6, HandShape6, ShapeDiagramData6);
impl_hand_system!(create_hand_shape4, HandShape4, ShapeDiagramData4);

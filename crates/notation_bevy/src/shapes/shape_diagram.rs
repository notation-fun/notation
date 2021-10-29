use bevy::prelude::*;

use crate::prelude::{EntryData, NotationTheme};
use bevy_utils::prelude::{BevyUtil, ShapeOp, StrokePath};
use notation_model::prelude::{HandShape4, HandShape6, LaneEntry};

macro_rules! impl_shape_diagram {
    ($hand_shape:ident, $diagram_data:ident, $diagram_value:ident) => {
        #[derive(Clone, Debug)]
        pub struct $diagram_value {
            pub shape: $hand_shape,
            pub bar_size: f32,
        }
        pub type $diagram_data = EntryData<$diagram_value>;
        impl From<(&LaneEntry, $hand_shape)> for $diagram_data {
            fn from(v: (&LaneEntry, $hand_shape)) -> Self {
                Self::new(v.0, $diagram_value{
                    shape: v.1,
                    bar_size: 0.0,
                })
            }
        }
        impl ShapeOp<NotationTheme, StrokePath> for $diagram_data {
            fn get_shape(&self, theme: &NotationTheme) -> StrokePath {
                let x = if self.value.bar_size <= 0.0 {
                    BevyUtil::offscreen_offset().x
                } else {
                    self.value.bar_size / self.bar_props.bar_units.0 * self.entry_props.in_bar_pos.0
                        + theme.shapes.shape_x
                };
                StrokePath {
                    size: Vec2::new(32.0, 32.0),
                    path: "m 7.4069823,6.6 34.9746887,7.2e-6 M 7.4069823,55.4 H 42.596149 M 42.661112,6.2 V 55.8 M 35.958389,6.2 V 55.8 M 28.920469,6.2 V 55.8 M 21.88255,6.2 V 55.8 M 14.84487,6.2 V 55.8 M 7.8069823,6.2 v 49.6 m -0.4,-12.46488 H 42.381671 m -34.9746887,-12.4 H 42.381671 m -34.9746887,-12.4 H 42.381671 M 7.4067902,6.2 H 42.60679 V 55.8 H 7.4067902 Z m 0,0 H 42.60679 V 55.8 H 7.4067902 Z".to_owned(),
                    color: theme.shapes.shape_color,
                    line_width: theme.shapes.shape_line_width,
                    offset: Vec3::new(x, theme.shapes.shape_y, theme.shapes.shape_z),
                    scale: theme.shapes.shape_scale,
                    angle: 0.0,
                }
            }
        }
    }
}

impl_shape_diagram!(
    HandShape6,
    ShapeDiagramData6,
    ShapeDiagramValue6
);
impl_shape_diagram!(
    HandShape4,
    ShapeDiagramData4,
    ShapeDiagramValue4
);

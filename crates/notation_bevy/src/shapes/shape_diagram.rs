use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use bevy_utils::prelude::BevyUtil;
use notation_model::prelude::{HandShape4, HandShape6, LaneEntry};
use crate::prelude::{EntryData, LyonShape, LyonShapeOp, NotationTheme};

macro_rules! impl_shape_diagram {
    ($hand_shape:ident, $diagram:ident, $diagram_data:ident, $diagram_value:ident) => {
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
        pub struct $diagram<'a> {
            theme: &'a NotationTheme,
            data: $diagram_data,
        }

        impl<'a> LyonShape<shapes::SvgPathShape> for $diagram<'a> {
            fn get_name(&self) -> String {
                format!("{}:{}", self.data.bar_props.bar_ordinal, self.data.value.shape)
            }
            fn get_shape(&self) -> shapes::SvgPathShape {
                shapes::SvgPathShape {
                    svg_doc_size_in_px: Vec2::new(32.0, 32.0),
                    svg_path_string: "m 7.4069823,6.6 34.9746887,7.2e-6 M 7.4069823,55.4 H 42.596149 M 42.661112,6.2 V 55.8 M 35.958389,6.2 V 55.8 M 28.920469,6.2 V 55.8 M 21.88255,6.2 V 55.8 M 14.84487,6.2 V 55.8 M 7.8069823,6.2 v 49.6 m -0.4,-12.46488 H 42.381671 m -34.9746887,-12.4 H 42.381671 m -34.9746887,-12.4 H 42.381671 M 7.4067902,6.2 H 42.60679 V 55.8 H 7.4067902 Z m 0,0 H 42.60679 V 55.8 H 7.4067902 Z".to_owned(),
                }
            }
            fn get_colors(&self) -> ShapeColors {
                ShapeColors::new(self.theme.shapes.shape_color)
            }
            fn get_draw_mode(&self) -> DrawMode {
                DrawMode::Stroke(
                    StrokeOptions::default().with_line_width(self.theme.shapes.shape_line_width),
                )
            }
            fn get_transform(&self) -> Transform {
                if self.data.value.bar_size <= 0.0 {
                    return BevyUtil::offscreen_transform();
                }
                let x = self.data.value.bar_size / self.data.bar_props.bar_units.0 * self.data.entry_props.in_bar_pos.0
                    + self.theme.shapes.shape_x;
                let mut trans =
                    Transform::from_xyz(x, self.theme.shapes.shape_y, self.theme.shapes.shape_z);
                trans.scale = Vec3::new(
                    self.theme.shapes.shape_scale,
                    self.theme.shapes.shape_scale,
                    1.0,
                );
                trans
            }
        }

        impl<'a>
            LyonShapeOp<'a, NotationTheme, $diagram_data, shapes::SvgPathShape, $diagram<'a>>
            for $diagram<'a>
        {
            fn new_shape(theme: &'a NotationTheme, data: $diagram_data) -> $diagram<'a> {
                $diagram::<'a> { theme, data }
            }
        }
    }
}

impl_shape_diagram!(
    HandShape6,
    ShapeDiagram6,
    ShapeDiagramData6,
    ShapeDiagramValue6
);
impl_shape_diagram!(
    HandShape4,
    ShapeDiagram4,
    ShapeDiagramData4,
    ShapeDiagramValue4
);

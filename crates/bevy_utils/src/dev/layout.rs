use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::lyon::lyon_shape::{LyonShape, LyonShapeOp};
use crate::prelude::LayoutData;

use super::theme::BevyUtilsTheme;

pub struct LayoutShape<'a> {
    pub theme: &'a BevyUtilsTheme,
    pub data: LayoutData,
}

impl<'a> LyonShape<shapes::Rectangle> for LayoutShape<'a> {
    fn get_name(&self) -> String {
        format!("{}", self.data)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.data.size.width,
            height: self.data.size.height,
            origin: shapes::RectangleOrigin::from(self.data),
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::outlined(
            self.theme.layout.get_view_color(),
            self.theme.layout.border_color,
        )
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default()
                .with_line_width(self.theme.layout.border_line_width),
        }
    }
    fn get_transform(&self) -> Transform {
        Transform::from_xyz(self.data.offset.x, self.data.offset.y, 2.0)
    }
}

impl<'a> LyonShapeOp<'a, BevyUtilsTheme, LayoutData, shapes::Rectangle, LayoutShape<'a>>
    for LayoutShape<'a>
{
    fn new_shape(theme: &'a BevyUtilsTheme, data: LayoutData) -> LayoutShape<'a> {
        LayoutShape::<'a> { theme, data }
    }
}

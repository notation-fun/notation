use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::LayoutSize;
use notation_model::prelude::{Position, Units};

use crate::prelude::{BarLayoutData, LyonShape, LyonShapeOp, NotationSettings, NotationTheme};

#[derive(Clone, Debug)]
pub struct BarIndicatorData {
    pub bar_offset: Vec2,
    pub bar_size: LayoutSize,
}

impl BarIndicatorData {
    pub fn new() -> Self {
        BarIndicatorData {
            bar_offset: Vec2::ZERO,
            bar_size: LayoutSize::ZERO,
        }
    }
}

pub struct BarIndicator<'a> {
    pub theme: &'a NotationTheme,
    pub data: BarIndicatorData,
}

impl<'a> LyonShape<shapes::Rectangle> for BarIndicator<'a> {
    fn get_name(&self) -> String {
        "Current Bar".to_string()
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.data.bar_size.width,
            height: self.data.bar_size.height + self.theme.grid.bar_separator_extra * 2.0,
            origin: shapes::RectangleOrigin::TopLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.colors.mini_map.bar_outline.current)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.grid.pos_indicator_size;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let x = self.data.bar_offset.x;
        let y = self.data.bar_offset.y + self.theme.grid.bar_separator_extra;
        Transform::from_xyz(x, y, self.theme.core.bar_indicator_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, BarIndicatorData, shapes::Rectangle, BarIndicator<'a>>
    for BarIndicator<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: BarIndicatorData) -> BarIndicator<'a> {
        BarIndicator::<'a> { theme, data }
    }
}

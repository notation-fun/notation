use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::LayoutSize;

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct BarSeparatorValue {
    pub is_begin: bool,
    pub bar_size: LayoutSize,
}
pub type BarSeparatorData = BarData<BarSeparatorValue>;

impl BarSeparatorValue {
    pub fn new(is_begin: bool) -> Self {
        Self {
            is_begin,
            bar_size: LayoutSize::ZERO,
        }
    }
}
pub struct BarSeparator<'a> {
    theme: &'a NotationTheme,
    data: BarSeparatorData,
}

impl<'a> LyonShape<shapes::Line> for BarSeparator<'a> {
    fn get_name(&self) -> String {
        if self.data.value.is_begin {
            format!("| {}", self.data.bar_props.bar_ordinal)
        } else {
            format!("{} |", self.data.bar_props.bar_ordinal)
        }
    }
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(
            Vec2::new(0.0, self.theme.grid.bar_separator_extra),
            Vec2::new(
                0.0,
                -self.data.value.bar_size.height - self.theme.grid.bar_separator_extra,
            ),
        )
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.core.bar_separator_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.grid.bar_separator_size;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let x = if self.data.value.is_begin {
            0.0
        } else {
            self.data.value.bar_size.width
        };
        Transform::from_xyz(x, 0.0, self.theme.core.bar_separator_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, BarSeparatorData, shapes::Line, BarSeparator<'a>>
    for BarSeparator<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: BarSeparatorData) -> BarSeparator<'a> {
        BarSeparator::<'a> { theme, data }
    }
}

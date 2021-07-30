use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};
use notation_model::prelude::{TabBar, Units};

#[derive(Clone, Debug)]
pub struct BarSeparatorData {
    pub bar_ordinal: usize,
    pub bar_units: Units,
    pub is_begin: bool,
}

impl BarSeparatorData {
    pub fn new(tab_bar: &Arc<TabBar>, is_begin: bool) -> Self {
        let bar_ordinal = tab_bar.bar_ordinal;
        let bar_units = tab_bar.bar_units();
        BarSeparatorData {
            bar_ordinal,
            bar_units,
            is_begin,
        }
    }
}

pub struct BarSeparator<'a> {
    theme: &'a NotationTheme,
    data: BarSeparatorData,
}

impl<'a> LyonShape<shapes::Line> for BarSeparator<'a> {
    fn get_name(&self) -> String {
        if self.data.is_begin {
            format!("| {}", self.data.bar_ordinal)
        } else {
            format!("{} |", self.data.bar_ordinal)
        }
    }
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(
            Vec2::new(0.0, self.theme.grid.bar_separator_top),
            Vec2::new(0.0, self.theme.grid.bar_separator_bottom),
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
        let x = if self.data.is_begin {
            0.0
        } else {
            self.theme.grid.bar_size
        };
        Transform::from_xyz(x, 0.0, self.theme.core.bar_separator_z)
    }
}

impl<'a> LyonShapeOp<'a, BarSeparatorData, shapes::Line, BarSeparator<'a>> for BarSeparator<'a> {
    fn new_shape(theme: &'a NotationTheme, data: BarSeparatorData) -> BarSeparator<'a> {
        BarSeparator::<'a> { theme, data }
    }
}

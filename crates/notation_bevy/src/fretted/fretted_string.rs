use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::Units;

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

#[derive(Clone, Debug)]
pub struct FrettedStringData {
    pub bar_ordinal: usize,
    pub bar_units: Units,
    pub string: u8,
}

impl FrettedStringData {
    pub fn new(tab_bar: &Arc<TabBar>, string: u8) -> Self {
        let bar_ordinal = tab_bar.bar_ordinal;
        let bar_units = tab_bar.bar_units();
        FrettedStringData {
            bar_ordinal,
            bar_units,
            string,
        }
    }
}
pub struct FrettedString<'a> {
    theme: &'a NotationTheme,
    data: FrettedStringData,
}

impl<'a> LyonShape<shapes::Line> for FrettedString<'a> {
    fn get_name(&self) -> String {
        format!("{}:String {}", self.data.bar_ordinal, self.data.string)
    }
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(Vec2::ZERO, Vec2::new(self.theme.grid.bar_size, 0.0))
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.fretted.string_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.guitar.get_string_width(self.data.string);
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let y = -1.0 * self.data.string as f32 * self.theme.fretted.string_space;
        Transform::from_xyz(0.0, y, self.theme.fretted.string_z)
    }
}

impl<'a> LyonShapeOp<'a, FrettedStringData, shapes::Line, FrettedString<'a>> for FrettedString<'a> {
    fn new_shape(theme: &'a NotationTheme, data: FrettedStringData) -> FrettedString<'a> {
        FrettedString::<'a> { theme, data }
    }
}

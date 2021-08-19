use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

use super::mini_bar::MiniBarValue;

#[derive(Clone, Debug)]
pub struct MiniSectionSeparatorValue {
    pub bar: MiniBarValue,
}
impl MiniSectionSeparatorValue {
    pub fn new(bar: MiniBarValue) -> Self {
        Self { bar }
    }
}

pub type MiniSectionSeparatorData = BarData<MiniSectionSeparatorValue>;

pub struct MiniSectionSeparator<'a> {
    theme: &'a NotationTheme,
    data: MiniSectionSeparatorData,
}

impl<'a> LyonShape<shapes::Line> for MiniSectionSeparator<'a> {
    fn get_name(&self) -> String {
        format!("| {}", self.data.bar_props.section_ordinal)
    }
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, self.data.value.bar.size),
        )
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.colors.mini_section_separator_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.grid.bar_separator_size;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let (x, y) = self.data.value.bar.calc_xy(self.data.bar_props.bar_ordinal);
        Transform::from_xyz(x, y, self.theme.core.mini_bar_z + 1.0)
    }
}

impl<'a> LyonShapeOp<'a, MiniSectionSeparatorData, shapes::Line, MiniSectionSeparator<'a>>
    for MiniSectionSeparator<'a>
{
    fn new_shape(
        theme: &'a NotationTheme,
        data: MiniSectionSeparatorData,
    ) -> MiniSectionSeparator<'a> {
        MiniSectionSeparator::<'a> { theme, data }
    }
}

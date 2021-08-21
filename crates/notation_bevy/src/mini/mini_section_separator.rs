use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

use super::mini_bar::{MiniBarLayout};

#[derive(Clone, Debug)]
pub struct MiniSectionSeparatorValue {
    pub layout: MiniBarLayout,
}
impl MiniSectionSeparatorValue {
    pub fn new(bar: MiniBarLayout) -> Self {
        Self { layout: bar }
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
            Vec2::new(0.0, self.theme.sizes.mini_map.bar_height_with_margin()),
        )
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.colors.of_section(self.data.bar_props.section_index))
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.sizes.mini_map.section_separator;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let (x, y) = self.data.value.layout.calc_xy(&self.theme.sizes.mini_map, self.data.bar_props.bar_ordinal);
        Transform::from_xyz(x, y, self.theme.core.mini_bar_z + 1.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, MiniSectionSeparatorData, shapes::Line, MiniSectionSeparator<'a>>
    for MiniSectionSeparator<'a>
{
    fn new_shape(
        theme: &'a NotationTheme,
        data: MiniSectionSeparatorData,
    ) -> MiniSectionSeparator<'a> {
        MiniSectionSeparator::<'a> { theme, data }
    }
}

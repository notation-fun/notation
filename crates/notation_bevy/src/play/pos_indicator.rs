use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct PosIndicatorData {}

impl Default for PosIndicatorData {
    fn default() -> Self {
        PosIndicatorData {}
    }
}

pub struct PosIndicator<'a> {
    pub theme: &'a NotationTheme,
    pub data: PosIndicatorData,
}

impl<'a> LyonShape<shapes::Line> for PosIndicator<'a> {
    fn get_name(&self) -> String {
        "Pos Indicator".to_string()
    }
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(
            Vec2::new(0.0, self.theme.grid.pos_indicator_top),
            Vec2::new(0.0, self.theme.grid.pos_indicator_bottom),
        )
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.core.pos_indicator_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.grid.pos_indicator_size;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        Transform::from_xyz(0.0, 0.0, self.theme.core.pos_indicator_z)
    }
}

impl<'a> LyonShapeOp<'a, PosIndicatorData, shapes::Line, PosIndicator<'a>> for PosIndicator<'a> {
    fn new_shape(theme: &'a NotationTheme, data: PosIndicatorData) -> PosIndicator<'a> {
        PosIndicator::<'a> { theme, data }
    }
}

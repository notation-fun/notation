use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::LayoutSize;
use notation_model::prelude::{Position, Units};

use crate::prelude::{BarLayoutData, LyonShape, LyonShapeOp, NotationSettings, NotationTheme};

#[derive(Clone, Debug)]
pub struct PosIndicatorData {
    pub bar_offset: Vec2,
    pub bar_size: LayoutSize,
    pub bar_units: Units,
    pub in_bar_pos: Units,
}

impl PosIndicatorData {
    pub fn new(bar_units: Units) -> Self {
        PosIndicatorData {
            bar_offset: Vec2::ZERO,
            bar_size: LayoutSize::ZERO,
            bar_units,
            in_bar_pos: Units(0.0),
        }
    }
}

pub struct PosIndicator<'a> {
    pub theme: &'a NotationTheme,
    pub data: PosIndicatorData,
}

impl<'a> LyonShape<shapes::Line> for PosIndicator<'a> {
    fn get_name(&self) -> String {
        "Current Pos".to_string()
    }
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(
            Vec2::ZERO,
            Vec2::new(
                0.0,
                -self.data.bar_size.height,
            ),
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
        let mut x = self.data.bar_offset.x;
        let y = self.data.bar_offset.y;
        x += self.data.bar_size.width
            * self.data.in_bar_pos.0 / self.data.bar_units.0;
        Transform::from_xyz(x, y + self.theme.grid.bar_separator_extra, self.theme.core.pos_indicator_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, PosIndicatorData, shapes::Line, PosIndicator<'a>>
    for PosIndicator<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: PosIndicatorData) -> PosIndicator<'a> {
        PosIndicator::<'a> { theme, data }
    }
}

impl<'a> PosIndicator<'a> {
    pub fn update_pos(
        commands: &mut Commands,
        theme: &'a NotationTheme,
        pos_indicator_query: &mut Query<(Entity, &mut PosIndicatorData)>,
        pos: Position,
    ) {
        if let Ok((entity, mut data)) = pos_indicator_query.single_mut() {
            data.in_bar_pos = pos.bar.in_bar_pos;
            Self::update(commands, theme, entity, &data);
        }
    }
}

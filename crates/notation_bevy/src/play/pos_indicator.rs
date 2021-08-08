use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{Position, Units};

use crate::prelude::{BarLayout, LyonShape, LyonShapeOp, NotationSettings, NotationTheme};

#[derive(Clone, Debug)]
pub struct PosIndicatorData {
    bar_units: Units,
    bar_col: usize,
    bar_offset: f32,
    bar_height: f32,
    in_bar_pos: Units,
}

impl PosIndicatorData {
    pub fn new(bar_units: Units, bar_layout: &BarLayout) -> Self {
        PosIndicatorData {
            bar_units,
            bar_col: bar_layout.data.col,
            bar_offset: bar_layout.offset,
            bar_height: bar_layout.height,
            in_bar_pos: Units(0.0),
        }
    }
    pub fn update(&mut self, bar_layout: &BarLayout, in_bar_pos: Units) {
        self.bar_col = bar_layout.data.col;
        self.bar_offset = bar_layout.offset;
        self.bar_height = bar_layout.height;
        self.in_bar_pos = in_bar_pos;
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
            Vec2::new(0.0, self.theme.grid.pos_indicator_extra),
            Vec2::new(0.0, -self.data.bar_height - self.theme.grid.pos_indicator_extra),
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
        let x = self.theme.grid.bar_size
            * (self.data.bar_col as f32 + self.data.in_bar_pos.0 / self.data.bar_units.0);
        let y = self.data.bar_offset;
        Transform::from_xyz(x, y, self.theme.core.pos_indicator_z)
    }
}

impl<'a> LyonShapeOp<'a, PosIndicatorData, shapes::Line, PosIndicator<'a>> for PosIndicator<'a> {
    fn new_shape(theme: &'a NotationTheme, data: PosIndicatorData) -> PosIndicator<'a> {
        PosIndicator::<'a> { theme, data }
    }
}

impl<'a> PosIndicator<'a> {
    pub fn update_pos(
        commands: &mut Commands,
        theme: &'a NotationTheme,
        children: &Children,
        settings: &NotationSettings,
        pos_indicator_query: &mut Query<&mut PosIndicatorData>,
        bar_layouts: &Arc<Vec<BarLayout>>,
        pos: Position,
    ) {
        settings.layout.bar_layout_of_pos(bar_layouts, pos)
            .map(|bar_layout| {
                for &child in children.iter() {
                    if let Ok(mut data) = pos_indicator_query.get_mut(child) {
                        PosIndicatorData::update(&mut data, &bar_layout, pos.bar.in_bar_pos);
                        Self::update(commands, theme, child, &data);
                    }
                }
            });
    }
}

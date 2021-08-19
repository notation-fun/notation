use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{Position, Units};

use crate::prelude::{BarLayout, LyonShape, LyonShapeOp, NotationSettings, NotationTheme};

#[derive(Clone, Debug)]
pub struct BarIndicatorData {
    bar_units: Units,
    bar_col: usize,
    bar_offset: f32,
    bar_height: f32,
}

impl BarIndicatorData {
    pub fn new(bar_units: Units, bar_layout: &BarLayout) -> Self {
        BarIndicatorData {
            bar_units,
            bar_col: bar_layout.data.col,
            bar_offset: bar_layout.offset,
            bar_height: bar_layout.height,
        }
    }
    pub fn update(&mut self, bar_layout: &BarLayout) {
        self.bar_col = bar_layout.data.col;
        self.bar_offset = bar_layout.offset;
        self.bar_height = bar_layout.height;
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
            width: self.theme.grid.bar_size,
            height: self.data.bar_height + self.theme.grid.bar_separator_extra * 2.0,
            origin: shapes::RectangleOrigin::TopLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.colors.mini_bar_current_outline)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.grid.pos_indicator_size;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let x = self.theme.grid.bar_size * self.data.bar_col as f32;
        let y = self.data.bar_offset + self.theme.grid.bar_separator_extra;
        Transform::from_xyz(x, y, self.theme.core.bar_indicator_z)
    }
}

impl<'a> LyonShapeOp<'a, BarIndicatorData, shapes::Rectangle, BarIndicator<'a>>
    for BarIndicator<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: BarIndicatorData) -> BarIndicator<'a> {
        BarIndicator::<'a> { theme, data }
    }
}

impl<'a> BarIndicator<'a> {
    pub fn update_pos(
        commands: &mut Commands,
        theme: &'a NotationTheme,
        settings: &NotationSettings,
        pos_indicator_query: &mut Query<(Entity, &mut BarIndicatorData)>,
        bar_layouts: &Arc<Vec<BarLayout>>,
        pos: Position,
    ) {
        settings
            .layout
            .bar_layout_of_pos(bar_layouts, pos)
            .map(|bar_layout| {
                if let Ok((entity, mut data)) = pos_indicator_query.single_mut() {
                    BarIndicatorData::update(&mut data, &bar_layout);
                    Self::update(commands, theme, entity, &data);
                }
            });
    }
}

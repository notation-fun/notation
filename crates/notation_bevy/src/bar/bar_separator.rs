use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::config::bevy_config::BevyConfig;
use crate::prelude::{LyonShape, LyonShapeOp};
use notation_model::prelude::{TabBar, Units};

#[derive(Clone, Debug)]
pub struct BarSeparatorData {
    pub bar_ordinal: usize,
    pub bar_units: Units,
    pub top: f32,
    pub bottom: f32,
    pub is_begin: bool,
}

impl BarSeparatorData {
    pub fn new(tab_bar: &Arc<TabBar>, top: f32, bottom: f32, is_begin: bool) -> Self {
        let bar_ordinal = tab_bar.bar_ordinal;
        let bar_units = tab_bar.bar_units();
        BarSeparatorData {
            bar_ordinal,
            bar_units,
            top,
            bottom,
            is_begin,
        }
    }
}

pub struct BarSeparator<'a> {
    config: &'a BevyConfig,
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
            Vec2::new(0.0, self.data.top),
            Vec2::new(0.0, self.data.bottom),
        )
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.config.theme.core.bar_separator_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.config.grid.separator_size;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let x = if self.data.is_begin {
            0.0
        } else {
            self.config.grid.unit_size * self.data.bar_units.0
        };
        Transform::from_xyz(x, 0.0, self.config.theme.core.bar_separator_z)
    }
}

impl<'a> LyonShapeOp<'a, BarSeparatorData, shapes::Line, BarSeparator<'a>> for BarSeparator<'a> {
    fn new_shape(config: &'a BevyConfig, data: BarSeparatorData) -> BarSeparator<'a> {
        BarSeparator::<'a> { config, data }
    }
}

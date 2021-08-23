use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme, LaneLayoutData};
use notation_model::prelude::TabBar;

#[derive(Clone, Debug)]
pub struct LaneBackData {
    pub bar_ordinal: usize,
    pub height: f32,
    pub margin: f32,
    pub bar_size: f32,
}

impl LaneBackData {
    pub fn new(tab_bar: &Arc<TabBar>, lane_layout: &LaneLayoutData) -> Self {
        let bar_ordinal = tab_bar.props.bar_ordinal;
        let height = lane_layout.height;
        let margin = lane_layout.margin;
        LaneBackData {
            bar_ordinal,
            height,
            margin,
            bar_size: 0.0,
        }
    }
}

pub struct LaneBack<'a> {
    theme: &'a NotationTheme,
    data: LaneBackData,
}

impl<'a> LyonShape<shapes::Rectangle> for LaneBack<'a> {
    fn get_name(&self) -> String {
        format!("{}:{}", self.data.bar_ordinal, self.data.height)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        let height = if self.theme.grid.lane_back_margin {
            self.data.margin
        } else {
            self.data.height
        };
        shapes::Rectangle {
            width: self.data.bar_size,
            height,
            origin: shapes::RectangleOrigin::TopLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = self.theme.grid.lane_back_color;
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        let y = if self.theme.grid.lane_back_margin {
            -self.data.height
        } else {
            0.0
        };
        Transform::from_xyz(0.0, y, 0.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, LaneBackData, shapes::Rectangle, LaneBack<'a>>
    for LaneBack<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: LaneBackData) -> LaneBack<'a> {
        LaneBack::<'a> { theme, data }
    }
}

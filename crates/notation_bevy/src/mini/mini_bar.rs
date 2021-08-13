use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use notation_model::prelude::{Syllable, TabBar};

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct MiniBarValue {
    pub rows: usize,
    pub cols: usize,
    pub size: f32,
    pub margin: f32,
}
impl MiniBarValue {
    pub fn new(rows: usize, cols: usize, size: f32, margin: f32) -> Self {
        Self {
            rows,
            cols,
            size,
            margin,
        }
    }
}

pub type MiniBarData = BarData<MiniBarValue>;

pub struct MiniBarShape<'a> {
    theme: &'a NotationTheme,
    data: MiniBarData,
}

impl<'a> LyonShape<shapes::Rectangle> for MiniBarShape<'a> {
    fn get_name(&self) -> String {
        format!("{}: {:?}", self.data.bar_props.bar_ordinal, self.data.value)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.data.value.size,
            height: self.data.value.size,
            origin: shapes::RectangleOrigin::BottomLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let index = self.data.bar_props.section_index % self.theme.colors.sections.len();
        let color = self.theme.colors.sections[index];
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        let index = self.data.bar_props.bar_ordinal - 1;
        let mut row = index / self.data.value.cols;
        let col = index % self.data.value.cols;
        let x = col as f32 * self.data.value.size;
        if row > self.data.value.rows {
            row = self.data.value.rows;
        }
        let size_and_margin = self.data.value.size + self.data.value.margin;
        let y = -1.0 * row as f32 * size_and_margin;
        Transform::from_xyz(x, y, self.theme.core.mini_bar_z)
    }
}

impl<'a> LyonShapeOp<'a, MiniBarData, shapes::Rectangle, MiniBarShape<'a>> for MiniBarShape<'a> {
    fn new_shape(theme: &'a NotationTheme, data: MiniBarData) -> MiniBarShape<'a> {
        MiniBarShape::<'a> { theme, data }
    }
}


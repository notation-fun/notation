use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::PlayingState;

use crate::prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct MiniBarValue {
    pub rows: usize,
    pub cols: usize,
    pub size: f32,
    pub margin: f32,
    pub playing_state: PlayingState,
}
impl MiniBarValue {
    pub fn new(rows: usize, cols: usize, size: f32, margin: f32) -> Self {
        Self {
            rows,
            cols,
            size,
            margin,
            playing_state: PlayingState::Idle,
        }
    }
    pub fn calc_xy(&self, bar_ordinal: usize) -> (f32, f32) {
        let index = bar_ordinal - 1;
        let mut row = index / self.cols;
        let col = index % self.cols;
        let x = col as f32 * self.size;
        if row > self.rows {
            row = self.rows;
        }
        let size_and_margin = self.size + self.margin;
        let y = -1.0 * row as f32 * size_and_margin;
        (x, y)
    }
}

pub type MiniBarData = BarData<MiniBarValue>;

pub struct MiniBarShape<'a> {
    theme: &'a NotationTheme,
    data: MiniBarData,
}

const OUTLINE_SIZE: f32 = 2.0;

impl<'a> LyonShape<shapes::Rectangle> for MiniBarShape<'a> {
    fn get_name(&self) -> String {
        format!("{}: {:?}", self.data.bar_props.bar_ordinal, self.data.value)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        let (mut width, mut height) = (self.data.value.size, self.data.value.size);
        if self.data.value.playing_state.is_current() {
            width -= OUTLINE_SIZE;
            height -= OUTLINE_SIZE;
        }
        shapes::Rectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::Center,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let index = self.data.bar_props.section_index % self.theme.colors.sections.len();
        let fill = self.theme.colors.sections[index];
        if self.data.value.playing_state.is_current() {
            let outline = self.theme.colors.mini_bar_current_outline;
            ShapeColors::outlined(fill, outline)
        } else {
            ShapeColors::new(fill)
        }
    }
    fn get_draw_mode(&self) -> DrawMode {
        if self.data.value.playing_state.is_current() {
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(OUTLINE_SIZE),
            }
        } else {
            DrawMode::Fill(FillOptions::default())
        }
    }
    fn get_transform(&self) -> Transform {
        let (x, y) = self.data.value.calc_xy(self.data.bar_props.bar_ordinal);
        let mut z = self.theme.core.mini_bar_z;
        if self.data.value.playing_state.is_current() {
            z += 2.0;
        }
        Transform::from_xyz(
            self.data.value.size / 2.0 + x,
            self.data.value.size / 2.0 + y,
            z,
        )
    }
}

impl<'a> LyonShapeOp<'a, MiniBarData, shapes::Rectangle, MiniBarShape<'a>> for MiniBarShape<'a> {
    fn new_shape(theme: &'a NotationTheme, data: MiniBarData) -> MiniBarShape<'a> {
        MiniBarShape::<'a> { theme, data }
    }
}

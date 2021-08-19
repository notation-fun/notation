use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{PlayingState, Syllable};

use crate::{prelude::{BarData, LyonShape, LyonShapeOp, NotationTheme}, theme::theme_sizes::MiniMapSizes};

#[derive(Clone, Debug)]
pub struct MiniBarLayout {
    pub rows: usize,
    pub cols: usize,
    pub width: f32,
}
impl MiniBarLayout {
    pub fn new(rows: usize, cols: usize, width: f32) -> Self {
        Self {
            rows,
            cols,
            width,
        }
    }
    pub fn calc_xy(&self, sizes: &MiniMapSizes, bar_ordinal: usize) -> (f32, f32) {
        let index = bar_ordinal - 1;
        let mut row = index / self.cols;
        let col = index % self.cols;
        let x = col as f32 * self.width;
        if row > self.rows {
            row = self.rows - 1;
        }
        let y = -1.0 * row as f32 * sizes.bar_height_with_margin();
        (x, y)
    }
}

#[derive(Clone, Debug)]
pub struct MiniBarValue {
    pub layout: MiniBarLayout,
    pub syllable: Syllable,
    pub playing_state: PlayingState,
}
impl MiniBarValue {
    pub fn new(layout: MiniBarLayout, syllable: Syllable) -> Self {
        Self {
            layout,
            syllable,
            playing_state: PlayingState::Idle,
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
        let (mut width, mut height) = (self.data.value.layout.width, self.theme.sizes.mini_map.bar_height);
        let outline = self.theme.sizes.mini_map.bar_outline.of_state(&self.data.value.playing_state);
        if self.data.value.playing_state.is_current() {
            width += outline;
            height += outline;
        } else {
            width -= outline;
            height -= outline;
        }
        shapes::Rectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::Center,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let fill = self
            .theme
            .colors
            .of_syllable(self.data.value.syllable);

        let outline = self.theme.colors.mini_map.bar_outline.of_state(&self.data.value.playing_state);
        ShapeColors::outlined(fill, outline)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(self.theme.sizes.mini_map.bar_outline.of_state(&self.data.value.playing_state)),
        }
    }
    fn get_transform(&self) -> Transform {
        let (x, y) = self.data.value.layout.calc_xy(&self.theme.sizes.mini_map, self.data.bar_props.bar_ordinal);
        let mut z = self.theme.core.mini_bar_z;
        if self.data.value.playing_state.is_current() {
            z += 2.0;
        }
        Transform::from_xyz(
            self.data.value.layout.width / 2.0 + x,
            self.theme.sizes.mini_map.bar_height / 2.0 + y,
            z,
        )
    }
}

impl<'a> LyonShapeOp<'a, MiniBarData, shapes::Rectangle, MiniBarShape<'a>> for MiniBarShape<'a> {
    fn new_shape(theme: &'a NotationTheme, data: MiniBarData) -> MiniBarShape<'a> {
        MiniBarShape::<'a> { theme, data }
    }
}

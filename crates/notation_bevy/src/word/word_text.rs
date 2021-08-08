use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{BarPosition, Duration, Units};

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

#[derive(Clone, Debug)]
pub struct WordTextData {
    pub bar_units: Units,
    pub bar_ordinal: usize,
    pub duration: Duration,
    pub position: BarPosition,
    pub text: String,
}

impl WordTextData {
    pub fn new(
        bar_units: Units,
        tab_bar: &Arc<TabBar>,
        duration: Duration,
        position: BarPosition,
        text: String,
    ) -> Self {
        let bar_ordinal = tab_bar.bar_ordinal;
        WordTextData {
            bar_units,
            bar_ordinal,
            duration,
            position,
            text,
        }
    }
}
pub struct WordTextShape<'a> {
    theme: &'a NotationTheme,
    data: WordTextData,
}

impl<'a> LyonShape<shapes::Line> for WordTextShape<'a> {
    fn get_name(&self) -> String {
        format!("{}:{}", self.data.bar_ordinal, self.data.text)
    }
    fn get_shape(&self) -> shapes::Line {
        let width = self.theme.grid.bar_size / self.data.bar_units.0
            * Units::from(self.data.duration).0
            - self.theme.lyrics.word_gap;
        shapes::Line(Vec2::ZERO, Vec2::new(width, 0.0))
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.lyrics.line_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.lyrics.line_size;
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        let x = self.theme.grid.bar_size / self.data.bar_units.0 * self.data.position.in_bar_pos.0;
        let y = 0.0;
        Transform::from_xyz(x, y, self.theme.strings.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, WordTextData, shapes::Line, WordTextShape<'a>> for WordTextShape<'a> {
    fn new_shape(theme: &'a NotationTheme, data: WordTextData) -> WordTextShape<'a> {
        WordTextShape::<'a> { theme, data }
    }
}

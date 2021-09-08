use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::BevyUtil;
use notation_model::prelude::{LyricWord, PlayingState};
use std::fmt::Display;

use crate::prelude::{EntryData, LyonShape, LyonShapeOp, NotationTheme, SingleBundle};

#[derive(Clone, Debug)]
pub struct WordTextValue {
    pub word: LyricWord,
    pub playing_state: PlayingState,
    pub bar_size: f32,
}
impl WordTextValue {
    pub fn new(word: LyricWord) -> Self {
        Self {
            word,
            playing_state: PlayingState::Idle,
            bar_size: 0.0,
        }
    }
}
impl Display for WordTextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type WordText = SingleBundle<WordTextValue>;

pub type WordTextData = EntryData<WordTextValue>;

pub struct WordTextShape<'a> {
    theme: &'a NotationTheme,
    data: WordTextData,
}

impl WordTextData {
    pub fn calc_text_color(&self, theme: &NotationTheme) -> Color {
        theme.colors.lyrics.line.of_state(&self.value.playing_state)
    }
}

impl<'a> LyonShape<shapes::Line> for WordTextShape<'a> {
    fn get_name(&self) -> String {
        format!("{}:{}", self.data.bar_props.bar_ordinal, self.data.value)
    }
    fn get_shape(&self) -> shapes::Line {
        let width = self.data.value.bar_size / self.data.bar_props.bar_units.0
            * self.data.entry_props.tied_units.0
            - self.theme.lyrics.word_gap;
        shapes::Line(Vec2::ZERO, Vec2::new(width, 0.0))
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.data.calc_text_color(self.theme))
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self
            .theme
            .sizes
            .lyrics
            .line_height
            .of_state(&self.data.value.playing_state);
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        if self.data.value.bar_size <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let x = self.data.value.bar_size / self.data.bar_props.bar_units.0
            * self.data.entry_props.in_bar_pos.0;
        let y = 0.0;
        Transform::from_xyz(x, y, self.theme.strings.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, WordTextData, shapes::Line, WordTextShape<'a>>
    for WordTextShape<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: WordTextData) -> WordTextShape<'a> {
        WordTextShape::<'a> { theme, data }
    }
}

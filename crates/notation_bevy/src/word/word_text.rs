use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, ShapeOp, StrokeLine};
use notation_model::prelude::{LyricWord, PlayingState};
use std::fmt::Display;

use crate::prelude::{EntryData, NotationTheme, SingleBundle};

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

impl ShapeOp<NotationTheme, shapes::Line, StrokeLine> for WordTextData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeLine {
        let width = self.value.bar_size / self.bar_props.bar_units.0
            * self.entry_props.tied_units.0
            - theme.lyrics.word_gap;
        let line_width = theme
            .sizes
            .lyrics
            .line_height
            .of_state(&self.value.playing_state);
        let offset = if self.value.bar_size <= 0.0 {
            BevyUtil::offscreen_offset()
        } else {
            let x = self.value.bar_size / self.bar_props.bar_units.0
                * self.entry_props.in_bar_pos.0;
            let y = 0.0;
            Vec3::new(x, y, theme.strings.pick_z)
        };
        StrokeLine {
            from: Vec2::ZERO,
            to: Vec2::new(width, 0.0),
            line_width,
            color: self.calc_text_color(theme),
            offset,
        }
    }
}

impl WordTextData {
    pub fn calc_text_color(&self, theme: &NotationTheme) -> Color {
        theme.colors.lyrics.line.of_state(&self.value.playing_state)
    }
    pub fn calc_text_font_size(&self, theme: &NotationTheme) -> f32 {
        theme.lyrics.word_font_size.of_state(&self.value.playing_state)
    }
}
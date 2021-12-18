use bevy::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, ShapeOp, StrokeLine};
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

impl ShapeOp<NotationTheme, StrokeLine> for WordTextData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeLine {
        let width = self.value.bar_size / self.bar_props.bar_units.0
            * self.entry_props.tied_units.0
            - theme.sizes.lyrics.word_gap;
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
            Vec3::new(x, y, theme.z.word)
        };
        let y = -1.0 * theme.sizes.lyrics.layout_height() / 2.0;
        StrokeLine {
            from: Vec2::new(0.0, y),
            to: Vec2::new(width, y),
            line_width,
            color: theme.colors.lyrics.line.of_state(&self.value.playing_state),
            offset,
        }
    }
}
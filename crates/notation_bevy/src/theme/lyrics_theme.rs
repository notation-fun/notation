use bevy::prelude::*;
use notation_bevy_utils::prelude::BevyUtil;
use notation_model::prelude::PlayingState;
use serde::{Deserialize, Serialize};

use crate::prelude::NotationAssets;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use super::theme_sizes::PlayingSize;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LyricsTheme {
    pub text_x: f32,
    pub text_y: f32,
    pub text_z: f32,
    pub line_size: f32,
    pub line_color: Color,
    pub word_gap: f32,
    pub word_font_size: PlayingSize,
    pub word_font_color: Color,
}

impl Default for LyricsTheme {
    fn default() -> Self {
        Self {
            text_x: 4.0,
            text_y: -10.0,
            text_z: 1.0,
            line_size: 2.0,
            line_color: Color::hex("555555").unwrap(),
            word_gap: 1.0,
            word_font_size: PlayingSize::new(20.0, 22.0, 20.0),
            word_font_color: Color::hex("000000").unwrap(),
        }
    }
}

impl LyricsTheme {
    pub fn spawn_word_text(
        &self,
        commands: &mut Commands,
        entity: Entity,
        assets: &NotationAssets,
        text: &str,
    ) {
        //NOTE: not sure why, using HorizontalAlign::Right here got the left behaviour
        BevyUtil::spawn_text(
            commands,
            entity,
            text,
            assets.cn_font.clone(),
            self.word_font_size.of_state(&PlayingState::Idle),
            self.word_font_color,
            HorizontalAlign::Right,
            VerticalAlign::Center,
            self.text_x,
            self.text_y,
            self.text_z,
        );
    }
}

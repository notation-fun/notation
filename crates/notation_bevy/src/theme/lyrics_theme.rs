use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::NotationAssets;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LyricsTheme {
    pub text_x: f32,
    pub text_y: f32,
    pub text_z: f32,
    pub line_size: f32,
    pub line_color: Color,
    pub word_gap: f32,
    pub word_font_size: f32,
    pub word_font_color: Color,
}

impl Default for LyricsTheme {
    fn default() -> Self {
        Self {
            text_x: 4.0,
            text_y: -8.0,
            text_z: 1.0,
            line_size: 2.0,
            line_color: Color::hex("555555").unwrap(),
            word_gap: 1.0,
            word_font_size: 20.0,
            word_font_color: Color::hex("000000").unwrap(),
        }
    }
}

impl LyricsTheme {
    pub fn insert_word_text(
        &self,
        entity_commands: &mut EntityCommands,
        assets: &NotationAssets,
        text: &str,
    ) {
        let font = assets.cn_font.clone();
        let style = TextStyle {
            font,
            font_size: self.word_font_size,
            color: self.word_font_color,
        };
        let alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Right,
        };
        entity_commands.insert_bundle(Text2dBundle {
            text: Text::with_section(text, style, alignment),
            transform: Transform::from_xyz(self.text_x, self.text_y, self.text_z),
            ..Default::default()
        });
    }
}

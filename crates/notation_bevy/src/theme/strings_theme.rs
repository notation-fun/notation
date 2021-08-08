use serde::{Deserialize, Serialize};

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct StringsTheme {
    pub string_color: Color,
    pub string_space: f32,
    pub string_z: f32,
    pub pick_z: f32,
    pub note_height: f32,
    pub note_outline: f32,
    pub note_outline_color: Color,
    pub fret_font_size: f32,
    pub fret_font_color: Color,
    pub fret_text_x: f32,
    pub fret_text_y: f32,
    pub fret_text_z: f32,
}

impl Default for StringsTheme {
    fn default() -> Self {
        Self {
            string_color: Color::hex("D3B59C").unwrap(),
            string_space: 12.0,
            string_z: 1.0,
            pick_z: 10.0,
            note_height: 6.0,
            note_outline: 1.0,
            note_outline_color: Color::hex("AAAAAA").unwrap(),
            fret_font_size: 18.0,
            fret_font_color: Color::hex("000000").unwrap(),
            fret_text_x: 4.0,
            fret_text_y: 4.0,
            fret_text_z: 1.0,
        }
    }
}

impl StringsTheme {
    pub fn insert_fret_text(
        &self,
        entity_commands: &mut EntityCommands,
        asset_server: &AssetServer,
        fret: u8,
    ) {
        let font = asset_server.load("fonts/FiraMono-Medium.ttf");
        let style = TextStyle {
            font,
            font_size: self.fret_font_size,
            color: self.fret_font_color,
        };
        let alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Right,
        };
        entity_commands.insert_bundle(Text2dBundle {
            text: Text::with_section(format!("{}", fret).as_str(), style, alignment),
            transform: Transform::from_xyz(self.fret_text_x, self.fret_text_y, self.fret_text_z),
            ..Default::default()
        });
    }
}

use serde::{Deserialize, Serialize};

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::prelude::NotationAssets;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ShapesTheme {
    pub shape_x: f32,
    pub shape_y: f32,
    pub shape_z: f32,
    pub shape_scale: f32,
    pub shape_color: Color,
    pub shape_line_width: f32,
    pub shape_finger_radius: f32,
    pub shape_finger_color: Color,
    pub shape_string_space: f32,
    pub shape_fret_space: f32,
    pub shape_finger_offset_x: f32,
    pub shape_finger_offset_y: f32,
    pub shape_font_size: f32,
    pub shape_font_color: Color,
    pub shape_text_x: f32,
    pub shape_text_y: f32,
    pub shape_text_z: f32,
}

impl Default for ShapesTheme {
    fn default() -> Self {
        Self {
            shape_x: 12.0,
            shape_y: -12.0,
            shape_z: 11.0,
            shape_scale: 0.75,
            shape_color: Color::hex("F27D7A").unwrap(),
            shape_line_width: 1.5,
            shape_finger_radius: 3.5,
            shape_finger_color: Color::hex("F27D7A").unwrap(),
            shape_string_space: 7.0,
            shape_fret_space: 12.0,
            shape_finger_offset_x: 27.0,
            shape_finger_offset_y: 14.0,
            shape_font_size: 24.0,
            shape_font_color: Color::hex("F27D7A").unwrap(),
            shape_text_x: 36.0,
            shape_text_y: -12.0,
            shape_text_z: 1.0,
        }
    }
}

impl ShapesTheme {
    pub fn insert_shape_text(
        &self,
        entity_commands: &mut EntityCommands,
        assets: &NotationAssets,
        text: &String,
    ) {
        let font = assets.en_font.clone();
        let style = TextStyle {
            font,
            font_size: self.shape_font_size,
            color: self.shape_font_color,
        };
        let alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Right,
        };
        entity_commands.insert_bundle(Text2dBundle {
            text: Text::with_section(text.as_str(), style, alignment),
            transform: Transform::from_xyz(self.shape_text_x, self.shape_text_y, self.shape_text_z),
            ..Default::default()
        });
    }
}

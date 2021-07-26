use serde::{Deserialize, Serialize};

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct FrettedTheme {
    pub string_color: Color,
    pub string_space: f32,
    pub string_z: f32,
    pub pick_z: f32,
    pub always_show_fret: bool,
    pub shape_color: Color,
    pub shape_line_width: f32,
    pub shape_finger_radius: f32,
    pub shape_finger_color: Color,
    pub shape_string_space: f32,
    pub shape_fret_space: f32,
    pub shape_finger_offset_x: f32,
    pub shape_finger_offset_y: f32,
}

impl Default for FrettedTheme {
    fn default() -> Self {
        Self {
            string_color: Color::hex("D3B59C").unwrap(),
            string_space: 20.0,
            string_z: 1.0,
            pick_z: 10.0,
            always_show_fret: false,
            shape_color: Color::hex("F27D7A").unwrap(),
            shape_line_width: 1.5,
            shape_finger_radius: 3.5,
            shape_finger_color: Color::hex("F27D7A").unwrap(),
            shape_string_space: 7.0,
            shape_fret_space: 12.0,
            shape_finger_offset_x: 27.0,
            shape_finger_offset_y: 14.0,
        }
    }
}

impl FrettedTheme {
    pub fn insert_fret_text(
        &self,
        entity_commands: &mut EntityCommands,
        asset_server: &AssetServer,
        fret: u8,
    ) {
        let font = asset_server.load("fonts/FiraMono-Medium.ttf");
        let style = TextStyle {
            font,
            font_size: 24.0,
            color: Color::BLACK,
        };
        let alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        };
        entity_commands.insert_bundle(Text2dBundle {
            text: Text::with_section(format!("{}", fret).as_str(), style, alignment),
            transform: Transform::from_xyz(12.0, 5.0, 1.0),
            ..Default::default()
        });
    }
}

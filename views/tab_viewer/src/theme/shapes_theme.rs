use notation_model::prelude::ProtoEntry;
use serde::{Deserialize, Serialize};

use edger_bevy_app::bevy::{prelude::*, sprite::Anchor};

use crate::prelude::NotationAssets;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ShapesTheme {
    pub shape_x: f32,
    pub shape_y: f32,
    pub shape_z: f32,
    pub shape_scale: f32,
    pub shape_color: Color,
    pub shape_line_width: f32,
    pub shape_barre_width: f32,
    pub shape_barre_height: f32,
    pub shape_barre_offset_x: f32,
    pub shape_barre_offset_y: f32,
    pub shape_finger_radius: f32,
    pub shape_finger_color: Color,
    pub shape_finger_mute_color: Color,
    pub shape_string_space: f32,
    pub shape_fret_space: f32,
    pub shape_finger_offset_x: f32,
    pub shape_finger_offset_y: f32,
    pub shape_font_size: f32,
    pub shape_font_color: Color,
    pub shape_text_x: f32,
    pub shape_text_y: f32,
    pub shape_text_z: f32,
    pub barre_font_size: f32,
    pub barre_font_color: Color,
    pub barre_text_x: f32,
    pub barre_text_y: f32,
    pub barre_text_z: f32,
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
            shape_barre_width: 44.0,
            shape_barre_height: 6.0,
            shape_barre_offset_x: 10.0,
            shape_barre_offset_y: 14.0,
            shape_finger_radius: 3.5,
            shape_finger_color: Color::hex("F27D7A").unwrap(),
            shape_finger_mute_color: Color::hex("000000").unwrap(),
            shape_string_space: 7.0,
            shape_fret_space: 12.0,
            shape_finger_offset_x: 27.0,
            shape_finger_offset_y: 14.0,
            shape_font_size: 24.0,
            shape_font_color: Color::hex("F27D7A").unwrap(),
            shape_text_x: 36.0,
            shape_text_y: -28.0,
            shape_text_z: 1.0,
            barre_font_size: 20.0,
            barre_font_color: Color::hex("F27D7A").unwrap(),
            barre_text_x: 36.0,
            barre_text_y: 6.0,
            barre_text_z: 1.0,
        }
    }
}

impl ShapesTheme {
    pub fn insert_shape_text(
        &self,
        commands: &mut Commands,
        assets: &NotationAssets,
        entity: Entity,
        text: &String,
    ) {
        let mut entity_commands = commands.spawn_empty();
        let font = assets.latin_font.clone();
        let style = TextStyle {
            font,
            font_size: self.shape_font_size,
            color: self.shape_font_color,
        };
        let justify = JustifyText::Left;
        let anchor = Anchor::Center;
        let shape_text = ProtoEntry::trim_comments(text);
        entity_commands.insert(Text2dBundle {
            text: Text::from_section(shape_text.as_str(), style).with_justify(justify),
            text_anchor: anchor,
            transform: Transform::from_xyz(self.shape_text_x, self.shape_text_y, self.shape_text_z),
            ..Default::default()
        });
        let text_entity = entity_commands.id();
        commands.entity(entity).push_children(&[text_entity]);
    }
    pub fn insert_barre_text(
        &self,
        commands: &mut Commands,
        assets: &NotationAssets,
        entity: Entity,
        barre: u8,
    ) {
        let mut entity_commands = commands.spawn_empty();
        let font = assets.latin_font.clone();
        let style = TextStyle {
            font,
            font_size: self.barre_font_size,
            color: self.barre_font_color,
        };
        let justify = JustifyText::Left;
        let anchor = Anchor::Center;
        entity_commands.insert(Text2dBundle {
            text: Text::from_section(barre.to_string(), style).with_justify(justify),
            transform: Transform::from_xyz(self.barre_text_x, self.barre_text_y, self.barre_text_z),
            text_anchor: anchor,
            ..Default::default()
        });
        let text_entity = entity_commands.id();
        commands.entity(entity).push_children(&[text_entity]);
    }
}

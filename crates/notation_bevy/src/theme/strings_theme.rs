use notation_bevy_utils::prelude::BevyUtil;
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::prelude::NotationAssets;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct StringsTheme {
    pub string_color: Color,
    pub string_z: f32,
    pub pick_z: f32,
    pub fret_font_size: f32,
    pub fret_font_color: Color,
    pub fret_text_x: f32,
    pub fret_text_y: f32,
    pub fret_text_z: f32,
    pub hit_string_seconds_range: (f32, f32),
}

impl Default for StringsTheme {
    fn default() -> Self {
        Self {
            string_color: Color::hex("D3B59C").unwrap(),
            string_z: 1.0,
            pick_z: 10.0,
            fret_font_size: 18.0,
            fret_font_color: Color::hex("000000").unwrap(),
            fret_text_x: 2.0,
            fret_text_y: -2.0,
            fret_text_z: 1.0,
            hit_string_seconds_range: (0.05, 0.15),
        }
    }
}

impl StringsTheme {
    pub fn spawn_fret_text(
        &self,
        commands: &mut Commands,
        entity: Entity,
        assets: &NotationAssets,
        fret: u8,
    ) {
        let text = format!("{}", fret);
        let x = self.fret_text_x;
        let y = self.fret_text_y;
        //NOTE: not sure why, using HorizontalAlign::Right here got the left behaviour
        BevyUtil::spawn_text(
            commands,
            entity,
            text.as_str(),
            assets.en_font.clone(),
            self.fret_font_size,
            self.fret_font_color,
            HorizontalAlign::Right,
            VerticalAlign::Center,
            x,
            y,
            self.fret_text_z,
        );
    }
}

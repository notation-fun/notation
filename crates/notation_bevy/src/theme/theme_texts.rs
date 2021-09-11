use bevy_utils::prelude::BevyUtil;
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::prelude::{NotationAssets, ThemeColors};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ThemeTexts {
    pub rhythm: RhythmTexts,
    pub mini_map: MiniMapTexts,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct RhythmTexts {
    pub bar_font_size: f32,
    pub bar_font_color: Color,
    pub bar_y: f32,
}
impl Default for RhythmTexts {
    fn default() -> Self {
        Self {
            bar_font_size: 40.0,
            bar_font_color: ThemeColors::color_of_hex("FFFFFF"),
            bar_y: 2.0,
        }
    }
}
impl RhythmTexts {
    pub fn spawn_bar_text(
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
            self.bar_font_size,
            self.bar_font_color,
            HorizontalAlign::Center,
            VerticalAlign::Center,
            0.0,
            self.bar_y,
            3.0,
        );
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MiniMapTexts {
    pub bar_font_size: f32,
    pub bar_font_color: Color,
}
impl Default for MiniMapTexts {
    fn default() -> Self {
        Self {
            bar_font_size: 16.0,
            bar_font_color: ThemeColors::color_of_hex("FFFFFF"),
        }
    }
}
impl MiniMapTexts {
    pub fn spawn_bar_text(
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
            self.bar_font_size,
            self.bar_font_color,
            HorizontalAlign::Center,
            VerticalAlign::Center,
            0.0,
            0.0,
            1.0,
        );
    }
}

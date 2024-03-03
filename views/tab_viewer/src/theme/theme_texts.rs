use edger_bevy::prelude::{text, LayoutData};
use notation_model::prelude::{Syllable, Scale, Key};
use serde::{Deserialize, Serialize};

use edger_bevy::bevy::{prelude::*, sprite::Anchor};

use crate::prelude::{NotationAssets, NotationSettings, ThemeColors};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ThemeTexts {
    pub tab: TabTexts,
    pub chord: ChordTexts,
    pub rhythm: RhythmTexts,
    pub lyrics: LyricsTexts,
    pub melody: NoteTexts,
    pub harmony: NoteTexts,
    pub strings: StringsTexts,
    pub mini_map: MiniMapTexts,
}

impl Default for ThemeTexts {
    fn default() -> Self {
        Self {
            tab: Default::default(),
            chord: Default::default(),
            rhythm: Default::default(),
            lyrics: Default::default(),
            melody: Default::default(),
            harmony: NoteTexts::default_harmony(),
            strings: Default::default(),
            mini_map: Default::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct TabTexts {
    pub bar_font_size: f32,
    pub bar_font_color: Color,
    pub bar_x: f32,
    pub bar_y: f32,
}
impl Default for TabTexts {
    fn default() -> Self {
        Self {
            bar_font_size: 18.0,
            bar_font_color: ThemeColors::hex_linear("00000066"),
            bar_x: -6.0,
            bar_y: -6.0,
        }
    }
}
impl TabTexts {
    pub fn spawn_bar_number(
        &self,
        commands: &mut Commands,
        assets: &NotationAssets,
        entity: Entity,
        text: &str,
    ) {
        text::spawn(
            commands,
            entity,
            text,
            assets.latin_font.clone(),
            self.bar_font_size,
            self.bar_font_color,
            JustifyText::Right,
            Anchor::Center,
            self.bar_x,
            self.bar_y,
            3.0,
        );
    }
    pub fn update_bar_number_x(&self, transform: &mut Transform, bar_width: f32) {
        transform.translation.x = bar_width + self.bar_x;
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ChordTexts {
    pub bars_font_size: f32,
    pub bars_font_color: Color,
    pub bars_x: f32,
    pub bars_y: f32,
}
impl Default for ChordTexts {
    fn default() -> Self {
        Self {
            bars_font_size: 16.0,
            bars_font_color: ThemeColors::hex_linear("FFFFFF"),
            bars_x: 2.0,
            bars_y: -2.0,
        }
    }
}
impl ChordTexts {
    pub fn spawn_bars_text(
        &self,
        commands: &mut Commands,
        assets: &NotationAssets,
        entity: Entity,
        text: &str,
        z: f32,
    ) {
        text::spawn(
            commands,
            entity,
            text,
            assets.latin_font.clone(),
            self.bars_font_size,
            self.bars_font_color,
            JustifyText::Left,
            Anchor::TopCenter,
            self.bars_x,
            self.bars_y,
            z,
        );
    }
    pub fn update_bars_xy(&self, transform: &mut Transform, layout: &LayoutData) {
        transform.translation.x = -layout.size.width / 2.0 + self.bars_x;
        transform.translation.y = layout.size.height / 2.0 + self.bars_y;
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct RhythmTexts {
    pub bar_font_size: f32,
    pub bar_font_color: Color,
    pub bar_y: f32,
}
impl Default for RhythmTexts {
    fn default() -> Self {
        Self {
            bar_font_size: 40.0,
            bar_font_color: ThemeColors::hex_linear("FFFFFF"),
            bar_y: 2.0,
        }
    }
}
impl RhythmTexts {
    pub fn spawn_bar_text(
        &self,
        commands: &mut Commands,
        assets: &NotationAssets,
        entity: Entity,
        text: &str,
    ) {
        text::spawn(
            commands,
            entity,
            text,
            assets.lyrics_font.clone(),
            self.bar_font_size,
            self.bar_font_color,
            JustifyText::Center,
            Anchor::Center,
            0.0,
            self.bar_y,
            3.0,
        );
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct LyricsTexts {
    pub text_x: f32,
    pub text_y: f32,
    pub text_z: f32,
    pub word_font_size: f32,
    pub word_font_color: Color,
}
impl Default for LyricsTexts {
    fn default() -> Self {
        Self {
            text_x: 4.0,
            text_y: -8.0,
            text_z: 1.0,
            word_font_size: 20.0,
            word_font_color: Color::hex("000000").unwrap(),
        }
    }
}
impl LyricsTexts {
    pub fn spawn_word_text(
        &self,
        commands: &mut Commands,
        entity: Entity,
        assets: &NotationAssets,
        text: &str,
    ) {
        text::spawn(
            commands,
            entity,
            text,
            assets.lyrics_font.clone(),
            self.word_font_size,
            self.word_font_color,
            JustifyText::Left,
            Anchor::Center,
            self.text_x,
            self.text_y,
            self.text_z,
        );
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct NoteTexts {
    pub text_x: f32,
    pub text_y: f32,
    pub text_z: f32,
    pub horizontal_center: bool,
    pub syllable_font_size: f32,
    pub syllable_font_color: Color,
}
impl Default for NoteTexts {
    fn default() -> Self {
        Self {
            text_x: 4.0,
            text_y: -14.0,
            text_z: 1.0,
            horizontal_center: false,
            syllable_font_size: 16.0,
            syllable_font_color: Color::hex("000000").unwrap(),
        }
    }
}
impl NoteTexts {
    pub fn default_harmony() -> Self {
        Self {
            text_y: 7.0,
            syllable_font_size: 18.0,
            ..Default::default()
        }
    }
    pub fn spawn_note_text(
        &self,
        commands: &mut Commands,
        entity: Entity,
        assets: &NotationAssets,
        settings: &NotationSettings,
        scale: &Scale,
        key: &Key,
        syllable: &Syllable,
    ) {
        self.spawn_scaled_note_text(commands, entity, assets, settings, scale, key, syllable, 1.0);
    }
    pub fn spawn_scaled_note_text(
        &self,
        commands: &mut Commands,
        entity: Entity,
        assets: &NotationAssets,
        settings: &NotationSettings,
        scale: &Scale,
        key: &Key,
        syllable: &Syllable,
        size_scale: f32,
    ) {
        let text = Self::calc_text(settings, scale, key, syllable);
        text::spawn(
            commands,
            entity,
            text.as_str(),
            assets.syllable_font.clone(),
            self.syllable_font_size * size_scale,
            self.syllable_font_color,
            if self.horizontal_center {
                JustifyText::Center
            } else {
                JustifyText::Left
            },
            Anchor::Center,
            self.text_x * size_scale,
            self.text_y * size_scale,
            self.text_z,
        );
    }
    pub fn calc_text(
        settings: &NotationSettings,
        scale: &Scale,
        key: &Key,
        syllable: &Syllable,
    ) -> String {
        let pitch_text = if settings.show_note_pitch {
            Some(scale.calc_pitch(key, syllable).to_text())
        } else {
            None
        };
        let syllable_text = if settings.show_note_syllable {
            Some(if settings.show_syllable_as_num {
                syllable.to_text()
            } else {
                syllable.to_ident()
            })
        } else {
            None
        };
        format!("{}{}{}",
            pitch_text.unwrap_or("".to_owned()),
            if settings.show_note_pitch && settings.show_note_syllable { " " } else { "" },
            syllable_text.unwrap_or("".to_owned())
        )
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct StringsTexts {
    pub text_x: f32,
    pub text_y: f32,
    pub text_z: f32,
    pub fret_font_size: f32,
    pub fret_font_color: Color,
}
impl Default for StringsTexts {
    fn default() -> Self {
        Self {
            text_x: 3.0,
            text_y: -2.0,
            text_z: 1.0,
            fret_font_size: 18.0,
            fret_font_color: super::theme_colors::hex_linear("000000"),
        }
    }
}
impl StringsTexts {
    pub fn spawn_fret_text(
        &self,
        commands: &mut Commands,
        entity: Entity,
        assets: &NotationAssets,
        fret: u8,
    ) {
        let text = format!("{}", fret);
        text::spawn(
            commands,
            entity,
            text.as_str(),
            assets.fret_font.clone(),
            self.fret_font_size,
            self.fret_font_color,
            JustifyText::Left,
            Anchor::Center,
            self.text_x,
            self.text_y,
            self.text_z,
        );
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MiniMapTexts {
    pub bar_font_size: f32,
    pub bar_font_color: Color,
}
impl Default for MiniMapTexts {
    fn default() -> Self {
        Self {
            bar_font_size: 16.0,
            bar_font_color: ThemeColors::hex_linear("FFFFFF"),
        }
    }
}
impl MiniMapTexts {
    pub fn spawn_bar_text(
        &self,
        commands: &mut Commands,
        assets: &NotationAssets,
        entity: Entity,
        text: &str,
    ) {
        text::spawn(
            commands,
            entity,
            text,
            assets.lyrics_font.clone(),
            self.bar_font_size,
            self.bar_font_color,
            JustifyText::Center,
            Anchor::Center,
            0.0,
            0.0,
            5.0,
        );
    }
    pub fn spawn_debug_text(
        &self,
        commands: &mut Commands,
        entity: Entity,
        assets: &NotationAssets,
        text: &str,
    ) {
        text::spawn(
            commands,
            entity,
            text,
            assets.latin_font.clone(),
            24.0,
            ThemeColors::hex_linear("000000"),
            JustifyText::Center,
            Anchor::Center,
            0.0,
            0.0,
            10.0,
        );
    }
}

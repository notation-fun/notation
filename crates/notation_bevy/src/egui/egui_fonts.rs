use std::sync::RwLock;

use bevy::prelude::*;
use crate::bevy_egui::{egui::{FontDefinitions, FontData, FontFamily, FontId, Style, TextStyle}, EguiContext};

use crate::prelude::{ExtraAssets, NotationSettings};

lazy_static! {
    static ref EGUI_FONT: RwLock<EguiFont> = RwLock::new(EguiFont::default());
}

#[derive(Clone, Debug, Default)]
pub struct EguiFont {
    pub name: Option<String>,
    pub data: Option<Vec<u8>>,
}
impl EguiFont {
    pub fn has_data() -> bool {
        EGUI_FONT.read().unwrap().data.is_some()
    }
    pub fn set_font(name: String, data: Vec<u8>) {
        let mut font = EGUI_FONT.write().unwrap();
        font.name = Some(name);
        font.data = Some(data);
    }
    pub fn get_font() -> Option<(String, Vec<u8>)> {
        let font = EGUI_FONT.read().unwrap();
        if font.name.is_some() && font.data.is_some() {
            Some((font.name.clone().unwrap(), font.data.clone().unwrap()))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EguiFontSizes {
    pub small: f32,
    pub body: f32,
    pub button: f32,
    pub heading: f32,
    pub mono: f32,
}

impl Default for EguiFontSizes {
    fn default() -> Self {
        Self {
            small: 14.0,
            body: 16.0,
            button: 18.0,
            heading: 24.0,
            mono: 16.0,
        }
    }
}

impl EguiFontSizes {
    pub const BIGGER: Self = Self {
        small: 16.0,
        body: 18.0,
        button: 20.0,
        heading: 26.0,
        mono: 18.0,
    };
}

pub fn get_font_definitions(
    font: Option<(String, Vec<u8>)>,
) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    if let Some((name, data)) = font {
        let font_data = FontData::from_owned(data);
        fonts.font_data.insert(name.clone(), font_data);
        fonts.families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, name.clone());
        fonts.families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, name.clone());
    }
    fonts
}

pub fn set_style_font_sizes(style: &mut Style, sizes: EguiFontSizes) {
    style.text_styles.insert(
        TextStyle::Small,
        FontId::new(sizes.small, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Body,
        FontId::new(sizes.body, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(sizes.button, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(sizes.heading, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(sizes.mono, FontFamily::Monospace),
    );
}

pub fn setup_egui_fonts<A: ExtraAssets>(
    settings: Res<NotationSettings>,
    extra_assets: Res<A>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    println!("setup_egui_fonts() ---------------------------------------");
    let fonts = get_font_definitions(EguiFont::get_font());
    let mut ctx = egui_ctx.ctx_mut();
    ctx.set_fonts(fonts);
    let mut style: Style = (*ctx.style()).clone();
    set_style_font_sizes(&mut style, extra_assets.get_egui_font_sizes(&settings));
    ctx.set_style(style);
}

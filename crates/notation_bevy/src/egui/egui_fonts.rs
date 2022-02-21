use std::sync::RwLock;

use bevy::prelude::*;
use bevy_egui::{egui::{FontDefinitions, FontData, FontFamily, TextStyle}, EguiContext};

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
    sizes: EguiFontSizes,
) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    if let Some((name, data)) = font {
        let font_data = FontData::from_owned(data);
        fonts.font_data.insert(name.clone(), font_data);
        fonts.fonts_for_family
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, name.clone());
        fonts.fonts_for_family
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, name.clone());
    }
    fonts.family_and_size.insert(
        TextStyle::Small,
        (FontFamily::Proportional, sizes.small),
    );
    fonts.family_and_size.insert(
        TextStyle::Body,
        (FontFamily::Proportional, sizes.body),
    );
    fonts.family_and_size.insert(
        TextStyle::Button,
        (FontFamily::Proportional, sizes.button),
    );
    fonts.family_and_size.insert(
        TextStyle::Heading,
        (FontFamily::Proportional, sizes.heading),
    );
    fonts.family_and_size.insert(
        TextStyle::Monospace,
        (FontFamily::Monospace, sizes.mono),
    );
    fonts
}

pub fn setup_egui_fonts<A: ExtraAssets>(
    settings: Res<NotationSettings>,
    extra_assets: Res<A>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    println!("setup_egui_fonts() ---------------------------------------");
    let fonts = get_font_definitions(
        EguiFont::get_font(),
        extra_assets.get_egui_font_sizes(&settings));
    egui_ctx.ctx_mut()
        .set_fonts(fonts);
}

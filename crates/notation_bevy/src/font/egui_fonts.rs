use std::collections::BTreeMap;

use bevy_egui::egui::{FontDefinitions, TextStyle, FontFamily};

pub type FontData = std::borrow::Cow<'static, [u8]>;

pub fn embedded_fonts(egui_scale_factor: f32) -> FontDefinitions {
    #[allow(unused)]
    let mut font_data: BTreeMap<String, FontData> = BTreeMap::new();
    let mut fonts_for_family = BTreeMap::new();
    let mut family_and_size = BTreeMap::new();

    let mut add_font = |name: &str, bytes: FontData, small: f32, body: f32, button: f32, heading: f32, mono: f32| {
        font_data.insert(
            name.to_owned(),
            bytes,
        );

        fonts_for_family.insert(
            FontFamily::Monospace,
            vec![
                name.to_owned(),
            ],
        );
        fonts_for_family.insert(
            FontFamily::Proportional,
            vec![
                name.to_owned(),
            ],
        );
        family_and_size.insert(TextStyle::Small, (FontFamily::Proportional, small / egui_scale_factor));
        family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, body / egui_scale_factor));
        family_and_size.insert(TextStyle::Button, (FontFamily::Proportional, button / egui_scale_factor));
        family_and_size.insert(TextStyle::Heading, (FontFamily::Proportional, heading / egui_scale_factor));
        family_and_size.insert(TextStyle::Monospace, (FontFamily::Monospace, mono / egui_scale_factor));
    };


    #[cfg(feature = "chinese")]
    {
        add_font("NotoSansSC", std::borrow::Cow::Borrowed(include_bytes!("../../fonts/NotoSansSC-Medium.otf")), 16.0, 18.0, 20.0, 26.0, 18.0);
    }
    #[cfg(not(feature = "chinese"))]
    {
        add_font("FiraMono", std::borrow::Cow::Borrowed(include_bytes!("../../fonts/FiraMono-Medium.ttf")), 14.0, 16.0, 18.0, 24.0, 16.0);
    }

    FontDefinitions {
        font_data,
        fonts_for_family,
        family_and_size,
    }
}
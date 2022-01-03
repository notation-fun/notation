use std::collections::BTreeMap;

use bevy_egui::egui::{FontDefinitions, FontData, FontFamily, TextStyle};

pub fn embedded_fonts(egui_scale_factor: f32) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    let mut add_font = |name: &str,
                        font_data: FontData,
                        small: f32,
                        body: f32,
                        button: f32,
                        heading: f32,
                        mono: f32| {
        fonts.font_data.insert(name.to_owned(), font_data);

        fonts.fonts_for_family
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, name.to_owned());
        fonts.fonts_for_family
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, name.to_owned());
        fonts.family_and_size.insert(
            TextStyle::Small,
            (FontFamily::Proportional, small / egui_scale_factor),
        );
        fonts.family_and_size.insert(
            TextStyle::Body,
            (FontFamily::Proportional, body / egui_scale_factor),
        );
        fonts.family_and_size.insert(
            TextStyle::Button,
            (FontFamily::Proportional, button / egui_scale_factor),
        );
        fonts.family_and_size.insert(
            TextStyle::Heading,
            (FontFamily::Proportional, heading / egui_scale_factor),
        );
        fonts.family_and_size.insert(
            TextStyle::Monospace,
            (FontFamily::Monospace, mono / egui_scale_factor),
        );
    };

    #[cfg(feature = "chinese")]
    {
        add_font(
            "NotoSansSC",
            FontData::from_static(include_bytes!("../../fonts/NotoSansSC-Medium.otf")),
            16.0,
            18.0,
            20.0,
            26.0,
            18.0,
        );
    }
    #[cfg(not(feature = "chinese"))]
    {
        add_font(
            "FiraMono",
            FontData::from_static(include_bytes!("../../fonts/FiraMono-Medium.ttf")),
            14.0,
            16.0,
            18.0,
            24.0,
            16.0,
        );
    }
    fonts
}

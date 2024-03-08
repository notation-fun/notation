use edger_bevy::bevy_prelude::*;
use bevy_asset_loader::prelude::*;
use edger_bevy::prelude::*;
use unic_langid::LanguageIdentifier;

use crate::prelude::{NotationArgs, NotationSettings};

#[derive(AssetCollection, Resource)]
pub struct NotationAssets {
    pub lang: LanguageIdentifier,

    #[asset(key = "syllable_font")]
    pub syllable_font: Handle<Font>,

    #[asset(key = "fret_font")]
    pub fret_font: Handle<Font>,

    #[asset(key = "latin_font")]
    pub latin_font: Handle<Font>,

    #[asset(key = "lyrics_font")]
    pub lyrics_font: Handle<Font>,

    #[asset(key = "fretboard_image")]
    pub fretboard: Handle<Image>,
}

impl FromWorld for NotationAssets {
    fn from_world(world: &mut World) -> Self {
        let args = world.get_resource::<NotationArgs>().unwrap();
        Self {
            lang: NotationSettings::parse_lang(&args.lang),
            syllable_font: default(),
            fret_font: default(),
            latin_font: default(),
            lyrics_font: default(),
            fretboard: default(),
        }
    }
}

impl NotationAssets {
    fn get_syllable_font(&self) -> String {
        "fonts/Sofia_Handwritten.otf".to_owned()
    }
    fn get_fret_font(&self) -> String {
        "fonts/Bitter-Bold.ttf".to_owned()
    }
    fn get_latin_font(&self) -> String {
        "fonts/FiraMono-Medium.ttf".to_owned()
    }
    fn get_lyrics_font(&self) -> String {
        if self.lang == NotationSettings::ZH_CN {
            #[cfg(feature = "with_egui")]
            return "fonts/zh-CN/NotoSansSC-Medium.otf.egui".to_owned();

            return "fonts/zh-CN/NotoSansSC-Medium.otf".to_owned();
        }

        #[cfg(feature = "with_egui")]
        return "fonts/en-US/FiraMono-Medium.ttf.egui".to_owned();

        return "fonts/en-US/FiraMono-Medium.ttf".to_owned();
    }

    fn get_fretboard_image(&self) -> String {
        "png/fretboard.png".to_owned()
    }

    #[cfg(feature = "with_egui")]
    fn get_egui_font_sizes(&self) -> EguiFontSizes {
        if self.lang == NotationSettings::ZH_CN {
            return EguiFontSizes::BIGGER;
        }
        EguiFontSizes::default()
    }

    #[cfg(feature = "with_egui")]
    pub fn setup_egui_context(
        assets: Res<NotationAssets>,
        mut egui_ctx: EguiContexts,
    ) {
        assets.get_egui_font_sizes().apply_context(&mut egui_ctx);
    }
}

impl PreloadAssets for NotationAssets {
    fn setup_keys(&self, asset_keys: &mut DynamicAssets) {
        register_file_asset(asset_keys, "syllable_font", self.get_syllable_font());
        register_file_asset(asset_keys, "fret_font", self.get_fret_font());
        register_file_asset(asset_keys, "latin_font", self.get_latin_font());
        register_file_asset(asset_keys, "lyrics_font", self.get_lyrics_font());
        register_file_asset(asset_keys, "fretboard_image", self.get_fretboard_image());
    }
}
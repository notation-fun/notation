use std::path::PathBuf;
use bevy::{prelude::*, asset::{AssetPath, HandleId, Asset}};
use bevy_asset_loader::{AssetCollection, AssetKeys};

use crate::{egui::egui_fonts::EguiFontSizes, settings::notation_settings::NotationSettings};

#[derive(AssetCollection)]
pub struct NotationAssets {
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

    //Not using the folder way, which is not supported under wasm
    //#[asset(folder = "extra")]
    pub extra: Vec<HandleUntyped>,
}

pub trait ExtraAssets : AssetCollection {
    fn get_assets(&self) -> Vec<HandleUntyped>;
    fn get_syllable_font(_settings: &NotationSettings) -> &'static str {
        "fonts/uchiyama.ttf"
    }
    fn get_fret_font(_settings: &NotationSettings) -> &'static str {
        "fonts/Bitter-Bold.ttf"
    }
    fn get_latin_font(_settings: &NotationSettings) -> &'static str {
        "fonts/FiraMono-Medium.ttf"
    }
    fn get_lyrics_font(settings: &NotationSettings) -> &'static str {
        if settings.lang() == NotationSettings::ZH_CN {
            return "fonts/zh-CN/NotoSansSC-Medium.otf.egui"
        }
        "fonts/en-US/FiraMono-Medium.ttf.egui"
    }
    fn get_fretboard_image(_settings: &NotationSettings) -> &'static str {
        "png/fretboard.png"
    }
    fn get_egui_font_sizes(&self, settings: &NotationSettings) -> EguiFontSizes {
        if settings.lang() == NotationSettings::ZH_CN {
            return EguiFontSizes::BIGGER;
        }
        EguiFontSizes::default()
    }
    fn setup_extra_keys(settings: &NotationSettings, asset_keys: &mut AssetKeys);
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum NotationAssetsStates {
    Loading,
    Loaded,
}

impl NotationAssets {
    pub fn setup_keys<A: ExtraAssets>(
        settings: Res<NotationSettings>,
        mut asset_keys: ResMut<AssetKeys>,
    ) {
        asset_keys.set_asset_key("syllable_font", A::get_syllable_font(&settings));
        asset_keys.set_asset_key("fret_font", A::get_fret_font(&settings));
        asset_keys.set_asset_key("latin_font", A::get_latin_font(&settings));
        asset_keys.set_asset_key("lyrics_font", A::get_lyrics_font(&settings));
        asset_keys.set_asset_key("fretboard_image", A::get_fretboard_image(&settings));
        A::setup_extra_keys(&settings, &mut asset_keys);
    }
    pub fn get_extra<A: Asset>(&self, path: PathBuf) -> Option<Handle<A>> {
        let handle_id = HandleId::from(AssetPath::new(path, None));
        let mut handle = None;
        for asset in self.extra.iter() {
            if asset.id == handle_id {
                handle = Some(asset.clone().typed::<A>());
                break;
            }
        }
        handle
    }
    pub fn add_extra(&mut self, handle: HandleUntyped) {
        self.extra.push(handle)
    }
    pub fn add_extra_assets<A: ExtraAssets>(
        extra: Res<A>,
        mut assets: ResMut<NotationAssets>,
    ) {
        for asset in extra.get_assets().iter() {
            assets.add_extra(asset.clone());
        }
    }
}

#[derive(AssetCollection)]
pub struct NoExtraAssets {
}

impl ExtraAssets for NoExtraAssets {
    fn get_assets(&self) -> Vec<HandleUntyped> {
        Vec::new()
    }
    fn setup_extra_keys(_settings: &NotationSettings, _asset_keys: &mut AssetKeys) {
    }
}
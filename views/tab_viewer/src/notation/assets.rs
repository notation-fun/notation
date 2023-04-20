use std::path::PathBuf;
use bevy::{prelude::*, asset::{AssetPath, HandleId, Asset}};
use bevy_asset_loader::prelude::*;

#[cfg(feature = "with_egui")]
use crate::egui::egui_fonts::EguiFontSizes;

use crate::settings::notation_settings::NotationSettings;

#[derive(AssetCollection, Resource)]
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
    fn get_syllable_font(_settings: &NotationSettings) -> String {
        "fonts/Sofia_Handwritten.otf".to_owned()
    }
    fn get_fret_font(_settings: &NotationSettings) -> String {
        "fonts/Bitter-Bold.ttf".to_owned()
    }
    fn get_latin_font(_settings: &NotationSettings) -> String {
        "fonts/FiraMono-Medium.ttf".to_owned()
    }
    fn get_lyrics_font(settings: &NotationSettings) -> String {
        if settings.lang() == NotationSettings::ZH_CN {
            #[cfg(feature = "with_egui")]
            return "fonts/zh-CN/NotoSansSC-Medium.otf.egui".to_owned();

            #[cfg(not(feature = "with_egui"))]
            return "fonts/zh-CN/NotoSansSC-Medium.otf".to_owned();
        }
        #[cfg(feature = "with_egui")]
        return "fonts/en-US/FiraMono-Medium.ttf.egui".to_owned();

        #[cfg(not(feature = "with_egui"))]
        return "fonts/en-US/FiraMono-Medium.ttf".to_owned();
    }
    fn get_fretboard_image(_settings: &NotationSettings) -> String {
        "png/fretboard.png".to_owned()
    }
    #[cfg(feature = "with_egui")]
    fn get_egui_font_sizes(&self, settings: &NotationSettings) -> EguiFontSizes {
        if settings.lang() == NotationSettings::ZH_CN {
            return EguiFontSizes::BIGGER;
        }
        EguiFontSizes::default()
    }
    fn setup_extra_keys(settings: &NotationSettings, asset_keys: &mut DynamicAssets);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum NotationAssetsStates {
    #[default]
    Init,
    Loading,
    Loaded,
}

impl NotationAssets {
    pub fn setup_keys<A: ExtraAssets>(
        settings: Res<NotationSettings>,
        mut asset_keys: ResMut<DynamicAssets>,
        mut state: ResMut<NextState<NotationAssetsStates>>,
    ) {
        asset_keys.register_asset("syllable_font", Box::new(StandardDynamicAsset::File {
            path: A::get_syllable_font(&settings)
        }));
        asset_keys.register_asset("fret_font", Box::new(StandardDynamicAsset::File {
            path: A::get_fret_font(&settings)
        }));
        asset_keys.register_asset("latin_font", Box::new(StandardDynamicAsset::File {
            path: A::get_latin_font(&settings)
        }));
        asset_keys.register_asset("lyrics_font", Box::new(StandardDynamicAsset::File {
            path: A::get_lyrics_font(&settings)
        }));
        asset_keys.register_asset("fretboard_image", Box::new(StandardDynamicAsset::File {
            path: A::get_fretboard_image(&settings)
        }));
        A::setup_extra_keys(&settings, &mut asset_keys);
        state.set(NotationAssetsStates::Loading);
    }
    pub fn get_extra<A: Asset>(&self, path: PathBuf) -> Option<Handle<A>> {
        let handle_id = HandleId::from(AssetPath::new(path, None));
        let mut handle = None;
        for asset in self.extra.iter() {
            if asset.id() == handle_id {
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

#[derive(AssetCollection, Resource)]
pub struct NoExtraAssets {
}

impl ExtraAssets for NoExtraAssets {
    fn get_assets(&self) -> Vec<HandleUntyped> {
        Vec::new()
    }
    fn setup_extra_keys(_settings: &NotationSettings, _asset_keys: &mut DynamicAssets) {
    }
}
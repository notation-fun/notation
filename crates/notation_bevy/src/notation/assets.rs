use std::path::PathBuf;
use bevy::{prelude::*, asset::{AssetPath, HandleId, Asset}};
use bevy_asset_loader::{AssetCollection, AssetKeys};

use crate::egui::egui_fonts::EguiFontSizes;

#[derive(AssetCollection)]
pub struct NotationAssets {
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
    fn get_latin_font() -> &'static str {
        "fonts/FiraMono-Medium.ttf"
    }
    fn get_lyrics_font() -> &'static str {
        "fonts/FiraMono-Medium.ttf"
    }
    fn get_fretboard_image() -> &'static str {
        "png/fretboard.png"
    }
    fn get_egui_font_sizes(&self) -> EguiFontSizes {
        EguiFontSizes::default()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum NotationAssetsStates {
    Loading,
    Loaded,
}

impl NotationAssets {
    pub fn setup_keys<A: ExtraAssets>(
        mut asset_keys: ResMut<AssetKeys>,
    ) {
        asset_keys.set_asset_key("latin_font", A::get_latin_font());
        asset_keys.set_asset_key("lyrics_font", A::get_lyrics_font());
        asset_keys.set_asset_key("fretboard_image", A::get_fretboard_image());
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
}
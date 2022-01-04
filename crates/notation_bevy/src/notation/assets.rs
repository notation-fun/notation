use std::path::PathBuf;
use bevy::{prelude::*, asset::{AssetPath, HandleId, Asset}};
use bevy_asset_loader::{AssetCollection};
use notation_bevy_utils::asset::markdown_asset::MarkDownAsset;

#[derive(AssetCollection)]
pub struct NotationAssets {
    #[asset(path = "fonts/FiraMono-Medium.font")]
    pub latin_font: Handle<Font>,

    #[cfg(feature = "chinese")]
    #[asset(path = "fonts/NotoSansSC-Medium.font")]
    pub lyrics_font: Handle<Font>,

    #[cfg(not(feature = "chinese"))]
    #[asset(path = "fonts/FiraMono-Medium.font")]
    pub lyrics_font: Handle<Font>,

    //#[asset(path = "gltf/guitar.gltf#Scene0")]
    //pub guitar: Handle<Scene>,
    #[asset(path = "png/fretboard.png")]
    pub fretboard: Handle<Texture>,

    //Not using the folder way, which is not supported under wasm
    //#[asset(folder = "extra")]
    pub extra: Vec<HandleUntyped>,
}

pub trait ExtraAssets : AssetCollection {
    fn get_assets(&self) -> Vec<HandleUntyped>;
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum NotationAssetsStates {
    Loading,
    Loaded,
}

impl NotationAssets {
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
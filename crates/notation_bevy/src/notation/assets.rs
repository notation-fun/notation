use std::path::PathBuf;
use bevy::{prelude::*, asset::{AssetPath, HandleId}};
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

    #[asset(folder = "kb")]
    pub kb: Vec<HandleUntyped>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum NotationAssetsStates {
    Loading,
    Loaded,
}

impl NotationAssets {
    pub fn get_kb(&self, path: PathBuf) -> Option<Handle<MarkDownAsset>> {
        let handle_id = HandleId::from(AssetPath::new(path, None));
        let mut handle = None;
        for asset in self.kb.iter() {
            if asset.id == handle_id {
                handle = Some(asset.clone().typed::<MarkDownAsset>());
                break;
            }
        }
        handle
    }
}
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
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
    #[asset(path = "help/welcome.md")]
    pub help_welcome: Handle<MarkDownAsset>,
    #[asset(path = "help/usage.md")]
    pub help_usage: Handle<MarkDownAsset>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum NotationAssetsStates {
    Loading,
    Loaded,
}

use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct NotationAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub en_font: Handle<Font>,
    #[asset(path = "fonts/NotoSansSC-Medium.otf")]
    pub cn_font: Handle<Font>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum NotationAssetsStates {
    Loading,
    Loaded,
}

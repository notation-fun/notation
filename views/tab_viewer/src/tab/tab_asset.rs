use thiserror::Error;
use notation_model::parse::ParseError;
use edger_bevy_app::bevy_prelude::*;
use edger_bevy_app::bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext, io::Reader};
use edger_bevy_app::bevy::reflect::TypePath;
use edger_bevy_app::bevy::utils::BoxedFuture;

use notation_model::prelude::ProtoTab;

#[derive(Clone, Debug, Asset, TypePath)]
pub struct TabAsset {
    pub tab: Result<ProtoTab, TabError>,
}

#[derive(Resource)]
pub struct TabAssetHandle(pub Handle<TabAsset>);

#[derive(Clone, Error, Debug)]
pub enum TabError {
    #[error("decode ron failed")]
    DecodeRonFailed(ron::error::SpannedError),
    #[error("get tab failed")]
    GetTabFailed(String),
    #[error("parse tab failed")]
    ParseFailed(ParseError),
}

impl From<ProtoTab> for TabAsset {
    fn from(v: ProtoTab) -> Self {
        Self { tab: Ok(v) }
    }
}

impl From<TabError> for TabAsset {
    fn from(v: TabError) -> Self {
        Self { tab: Err(v) }
    }
}

impl TabAsset {
    #[cfg(feature = "dsl")]
    pub const EXTENSIONS: [&'static str; 2] = ["rs", "ron"];
    #[cfg(not(feature = "dsl"))]
    pub const EXTENSIONS: [&'static str; 1] = ["ron"];
}

#[derive(Default)]
pub struct TabAssetLoader;

pub type LoadError = anyhow::Error;
pub type LoadResult = anyhow::Result<TabAsset, LoadError>;

impl AssetLoader for TabAssetLoader {
    type Asset = TabAsset;
    type Settings = ();
    type Error = LoadError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, LoadResult> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let tab_asset = match ron::de::from_bytes::<ProtoTab>(&bytes) {
                Ok(tab) => TabAsset::from(tab),
                Err(err) => TabAsset::from(TabError::DecodeRonFailed(err)),
            };
            Ok(tab_asset)
        })
    }
    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

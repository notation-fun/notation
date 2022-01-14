use thiserror::Error;
use notation_model::parse::ParseError;
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;
use bevy::utils::BoxedFuture;

use notation_model::prelude::ProtoTab;

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "52bcea66-eb44-4ad6-85bf-240b79494499"]
pub struct TabAsset {
    pub tab: Result<ProtoTab, TabError>,
}

#[derive(Clone, Error, Debug)]
pub enum TabError {
    #[error("decode ron failed")]
    DecodeRonFailed(ron::Error),
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

impl AssetLoader for TabAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let tab_asset = match ron::de::from_bytes::<ProtoTab>(bytes) {
                Ok(tab) => TabAsset::from(tab),
                Err(err) => TabAsset::from(TabError::DecodeRonFailed(err)),
            };
            load_context.set_default_asset(LoadedAsset::new(tab_asset));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

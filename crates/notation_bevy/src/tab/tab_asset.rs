use std::sync::Arc;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

use notation_model::prelude::{ProtoTab};

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "52bcea66-eb44-4ad6-85bf-240b79494499"]
pub struct TabAsset {
    pub tab: ProtoTab,
}

impl From<ProtoTab> for TabAsset {
    fn from(v: ProtoTab) -> Self {
        Self { tab: v }
    }
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
            let tab = ron::de::from_bytes::<ProtoTab>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(TabAsset::from(tab)));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

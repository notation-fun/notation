use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;
use bevy::utils::BoxedFuture;
use serde::Deserialize;

use notation_dsl::prelude::parse_get_tab;
use notation_model::prelude::ProtoTab;

use crate::tab::tab_asset::TabAsset;

#[derive(Default)]
pub struct GetTabAssetLoader;

impl AssetLoader for GetTabAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let text = String::from_utf8(bytes.to_vec())?;
            let tab = parse_get_tab(&text)?;
            load_context.set_default_asset(LoadedAsset::new(TabAsset::from(tab)));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["rs"]
    }
}

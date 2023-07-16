use edger_bevy_app::bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use edger_bevy_app::bevy::utils::BoxedFuture;

use notation_dsl::prelude::parse_get_tab;

use crate::tab::tab_asset::{TabAsset, TabError};

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
            let tab_asset = match parse_get_tab(&text) {
                Ok(tab) => TabAsset::from(tab),
                Err(err) => TabAsset::from(TabError::GetTabFailed(err.to_string())),
            };
            load_context.set_default_asset(LoadedAsset::new(tab_asset));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["rs"]
    }
}

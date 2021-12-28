use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;
use bevy::utils::BoxedFuture;
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "ae3cb724-f08b-4ceb-a7dd-c7d6781ba49b"]
pub struct MarkDownAsset {
    pub text: String,
}

impl From<String> for MarkDownAsset {
    fn from(v: String) -> Self {
        Self { text: v }
    }
}

impl From<&str> for MarkDownAsset {
    fn from(v: &str) -> Self {
        Self { text: String::from(v) }
    }
}

#[derive(Default)]
pub struct MarkDownAssetLoader;

impl AssetLoader for MarkDownAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let text = String::from_utf8(bytes.to_vec())?;
            load_context.set_default_asset(LoadedAsset::new(MarkDownAsset::from(text)));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["md"]
    }
}

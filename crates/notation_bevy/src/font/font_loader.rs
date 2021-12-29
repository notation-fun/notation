use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::utils::BoxedFuture;

#[derive(Default)]
pub struct EmbeddedFontAssetLoader;

impl AssetLoader for EmbeddedFontAssetLoader {
    fn load<'a>(
        &'a self,
        _bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let filename = load_context.path().file_name().and_then(|x| x.to_str());
            let mut bytes = None;
            if let Some(filename) = filename {
                match filename {
                    #[cfg(feature = "chinese")]
                    "NotoSansSC-Medium.font" => {
                        bytes = Some(include_bytes!("../../fonts/NotoSansSC-Medium.otf").to_vec())
                    },
                    "FiraMono-Medium.font" => {
                        bytes = Some(include_bytes!("../../fonts/FiraMono-Medium.ttf").to_vec())
                    },
                    _ => {}
                }
            }
            if bytes.is_none() {
                println!("EmbeddedFontAssetLoader: not found: path = {:?}", load_context.path());
            }
            let bytes = bytes.unwrap_or(include_bytes!("../../fonts/FiraMono-Medium.ttf").to_vec());
            let font = Font::try_from_bytes(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(font));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["font"]
    }
}

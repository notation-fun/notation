use edger_bevy_app::bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use edger_bevy_app::bevy_prelude::*;
use edger_bevy_app::bevy::utils::BoxedFuture;

use super::egui_fonts::EguiFont;

#[derive(Default)]
pub struct EguiFontAssetLoader;

impl AssetLoader for EguiFontAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            if !EguiFont::has_data() {
                let filename = load_context.path().file_name()
                .and_then(|x| x.to_str())
                .map(|x| x.replace(".font", ""))
                .unwrap_or("bevy_egui_font".to_owned());
                let data = bytes.to_vec();
                EguiFont::set_font(filename.to_owned(), data);
            }
            let font = Font::try_from_bytes(bytes.to_vec())?;
            load_context.set_default_asset(LoadedAsset::new(font));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["egui"]
    }
}

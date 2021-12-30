use bevy::prelude::*;

use super::font_loader::EmbeddedFontAssetLoader;

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_asset_loader::<EmbeddedFontAssetLoader>();
    }
}

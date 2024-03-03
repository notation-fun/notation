use edger_bevy::bevy_prelude::*;

use super::egui_font_loader::EguiFontAssetLoader;

pub struct EguiPlugin;

impl Plugin for EguiPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<EguiFontAssetLoader>();
    }
}

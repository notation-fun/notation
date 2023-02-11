use tab_viewer::bevy::prelude::*;
use tab_viewer::bevy_asset_loader::prelude::*;
use tab_viewer::prelude::*;

#[derive(AssetCollection)]
pub struct FretsAssets {
}

impl ExtraAssets for FretsAssets {
    fn get_assets(&self) -> Vec<HandleUntyped> {
        vec![
        ]
    }
    fn setup_extra_keys(settings: &NotationSettings, asset_keys: &mut DynamicAssets) {
    }
}
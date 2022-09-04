use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_asset_loader::prelude::*;
use notation_bevy::prelude::*;

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
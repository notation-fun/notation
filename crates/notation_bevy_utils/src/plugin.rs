use bevy::prelude::*;

use crate::asset::markdown_asset::{MarkDownAsset, MarkDownAssetLoader};

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<MarkDownAsset>();
        app.init_asset_loader::<MarkDownAssetLoader>();
    }
}

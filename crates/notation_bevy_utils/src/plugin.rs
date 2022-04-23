use bevy::prelude::*;

use crate::asset::markdown_asset::{MarkDownAsset, MarkDownAssetLoader};

#[cfg(feature = "with_egui")]
use crate::prelude::EasyLinkEvent;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<MarkDownAsset>();
        app.init_asset_loader::<MarkDownAssetLoader>();

        #[cfg(feature = "with_egui")]
        app.add_event::<EasyLinkEvent>();
    }
}

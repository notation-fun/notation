use notation_bevy::bevy::prelude::*;
use notation_bevy::prelude::{bevy_asset_loader, AssetCollection, MarkDownAsset, ExtraAssets};

#[derive(AssetCollection)]
pub struct NotationViewerAssets {
    #[asset(path = "kb/welcome.md")]
    pub kb_welcome: Handle<MarkDownAsset>,

    #[asset(path = "kb/usage.md")]
    pub kb_usage: Handle<MarkDownAsset>,
}

impl ExtraAssets for NotationViewerAssets {
    fn get_assets(&self) -> Vec<HandleUntyped> {
        vec![
            self.kb_welcome.clone_untyped(),
            self.kb_usage.clone_untyped(),
        ]
    }
}
use notation_bevy::bevy::prelude::*;
use notation_bevy::prelude::{bevy_asset_loader, AssetCollection, MarkDownAsset, ExtraAssets};

#[derive(AssetCollection)]
pub struct NotationKnowledgeBaseAssets {
    #[asset(path = "kb/welcome.md")]
    pub kb_welcome: Handle<MarkDownAsset>,

    #[asset(path = "kb/harmonics.md")]
    pub kb_harmonics: Handle<MarkDownAsset>,
}

impl ExtraAssets for NotationKnowledgeBaseAssets {
    fn get_assets(&self) -> Vec<HandleUntyped> {
        vec![
            self.kb_welcome.clone_untyped(),
            self.kb_harmonics.clone_untyped(),
        ]
    }
}
use notation_bevy::bevy::prelude::*;
use notation_bevy::prelude::{bevy_asset_loader, AssetCollection, MarkDownAsset, ExtraAssets};

#[derive(AssetCollection)]
pub struct NotationKnowledgeBaseAssets {
    #[asset(path = "kb/welcome.md")]
    pub kb_welcome: Handle<MarkDownAsset>,

    #[asset(path = "kb/sound.md")]
    pub kb_sound: Handle<MarkDownAsset>,

    #[asset(path = "kb/scale.md")]
    pub kb_scale: Handle<MarkDownAsset>,
}

impl ExtraAssets for NotationKnowledgeBaseAssets {
    fn get_assets(&self) -> Vec<HandleUntyped> {
        vec![
            self.kb_welcome.clone_untyped(),
            self.kb_sound.clone_untyped(),
            self.kb_scale.clone_untyped(),
        ]
    }
    fn get_latin_font() -> &'static str {
        "fonts/FiraMono-Medium.ttf.egui"
    }
    fn get_lyrics_font() -> &'static str {
        "fonts/FiraMono-Medium.ttf.egui"
    }
}
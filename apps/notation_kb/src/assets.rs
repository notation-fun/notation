use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_asset_loader::AssetKeys;
use notation_bevy::prelude::{bevy_asset_loader, AssetCollection, MarkDownAsset, ExtraAssets, NotationSettings};

#[derive(AssetCollection)]
pub struct NotationKnowledgeBaseAssets {
    #[asset(key = "kb_welcome")]
    pub kb_welcome: Handle<MarkDownAsset>,

    #[asset(key = "kb_sound")]
    pub kb_sound: Handle<MarkDownAsset>,

    #[asset(key = "kb_scale")]
    pub kb_scale: Handle<MarkDownAsset>,

    #[asset(key = "kb_guitar")]
    pub kb_guitar: Handle<MarkDownAsset>,
}

impl NotationKnowledgeBaseAssets {
    pub fn get_welcome_path(settings: &NotationSettings) -> String {
        let lang = settings.lang();
        format!("kb/{}/welcome.md", lang)
    }
    pub fn get_sound_path(settings: &NotationSettings) -> String {
        let lang = settings.lang();
        format!("kb/{}/sound.md", lang)
    }
    pub fn get_scale_path(settings: &NotationSettings) -> String {
        let lang = settings.lang();
        format!("kb/{}/scale.md", lang)
    }
    pub fn get_guitar_path(settings: &NotationSettings) -> String {
        let lang = settings.lang();
        format!("kb/{}/guitar.md", lang)
    }
}

impl ExtraAssets for NotationKnowledgeBaseAssets {
    fn get_assets(&self) -> Vec<HandleUntyped> {
        vec![
            self.kb_welcome.clone_untyped(),
            self.kb_sound.clone_untyped(),
            self.kb_scale.clone_untyped(),
            self.kb_guitar.clone_untyped(),
        ]
    }
    fn setup_extra_keys(settings: &NotationSettings, asset_keys: &mut AssetKeys) {
        asset_keys.set_asset_key("kb_welcome", Self::get_welcome_path(settings).as_str());
        asset_keys.set_asset_key("kb_sound", Self::get_sound_path(settings).as_str());
        asset_keys.set_asset_key("kb_scale", Self::get_scale_path(settings).as_str());
        asset_keys.set_asset_key("kb_guitar", Self::get_guitar_path(settings).as_str());
    }
}
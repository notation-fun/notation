use bevy::prelude::*;
use tab_viewer::bevy_asset_loader::prelude::*;
use tab_viewer::prelude::*;

#[derive(AssetCollection, Resource)]
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
    fn get_assets(&self) -> Vec<UntypedHandle> {
        vec![
            self.kb_welcome.clone().untyped(),
            self.kb_sound.clone().untyped(),
            self.kb_scale.clone().untyped(),
            self.kb_guitar.clone().untyped(),
        ]
    }
    fn setup_extra_keys(settings: &NotationSettings, asset_keys: &mut DynamicAssets) {
        asset_keys.register_asset("kb_welcome", Box::new(StandardDynamicAsset::File {
            path: Self::get_welcome_path(settings)
        }));
        asset_keys.register_asset("kb_sound", Box::new(StandardDynamicAsset::File {
            path: Self::get_sound_path(settings)
        }));
        asset_keys.register_asset("kb_scale", Box::new(StandardDynamicAsset::File {
            path: Self::get_scale_path(settings)
        }));
        asset_keys.register_asset("kb_guitar", Box::new(StandardDynamicAsset::File {
            path: Self::get_guitar_path(settings)
        }));
    }
}
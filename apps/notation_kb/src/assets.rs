use bevy::prelude::*;
use tab_viewer::edger_bevy::prelude::PreloadAssets;
use tab_viewer::bevy_asset_loader::prelude::*;
use tab_viewer::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct NotationKnowledgeBaseAssets {
    pub lang: LanguageIdentifier,

    #[asset(key = "kb_welcome")]
    pub kb_welcome: Handle<MarkDownAsset>,

    #[asset(key = "kb_sound")]
    pub kb_sound: Handle<MarkDownAsset>,

    #[asset(key = "kb_scale")]
    pub kb_scale: Handle<MarkDownAsset>,

    #[asset(key = "kb_guitar")]
    pub kb_guitar: Handle<MarkDownAsset>,
}

impl FromWorld for NotationKnowledgeBaseAssets {
    fn from_world(world: &mut World) -> Self {
        let args = world.get_resource::<NotationArgs>().unwrap();
        Self {
            lang: NotationSettings::parse_lang(&args.lang),
            kb_welcome: default(),
            kb_sound: default(),
            kb_scale: default(),
            kb_guitar: default(),
        }
    }
}

impl NotationKnowledgeBaseAssets {
    pub fn get_welcome_path(&self) -> String {
        format!("kb/{}/welcome.md", self.lang)
    }
    pub fn get_sound_path(&self) -> String {
        format!("kb/{}/sound.md", self.lang)
    }
    pub fn get_scale_path(&self) -> String {
        format!("kb/{}/scale.md", self.lang)
    }
    pub fn get_guitar_path(&self) -> String {
        format!("kb/{}/guitar.md", self.lang)
    }
}

impl PreloadAssets for NotationKnowledgeBaseAssets {
    fn app_assets(&self) -> Vec<UntypedHandle> {
        vec![
            self.kb_welcome.clone().untyped(),
            self.kb_sound.clone().untyped(),
            self.kb_scale.clone().untyped(),
            self.kb_guitar.clone().untyped(),
        ]
    }
    fn setup_keys(&self, asset_keys: &mut DynamicAssets) {
        register_file_asset(asset_keys, "kb_welcome", self.get_welcome_path());
        register_file_asset(asset_keys, "kb_sound", self.get_sound_path());
        register_file_asset(asset_keys, "kb_scale", self.get_scale_path());
        register_file_asset(asset_keys, "kb_guitar", self.get_guitar_path());
    }
}
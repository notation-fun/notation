use bevy::prelude::*;
use tab_viewer::edger_bevy::prelude::PreloadAssets;
use tab_viewer::bevy_asset_loader::prelude::*;
use tab_viewer::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct NotationViewerAssets {
    pub lang: LanguageIdentifier,

    #[asset(key = "kb_welcome")]
    pub kb_welcome: Handle<MarkDownAsset>,

    #[asset(key = "kb_usage")]
    pub kb_usage: Handle<MarkDownAsset>,
}

impl FromWorld for NotationViewerAssets {
    fn from_world(world: &mut World) -> Self {
        let args = world.get_resource::<NotationArgs>().unwrap();
        Self {
            lang: NotationSettings::parse_lang(&args.lang),
            kb_welcome: default(),
            kb_usage: default(),
        }
    }
}

impl NotationViewerAssets {
    pub fn get_welcome_path(&self) -> String {
        format!("kb/{}/welcome.md", self.lang)
    }
    pub fn get_usage_path(&self) -> String {
        format!("kb/{}/usage.md", self.lang)
    }
}

impl PreloadAssets for NotationViewerAssets {
    fn app_assets(&self) -> Vec<UntypedHandle> {
        vec![
            self.kb_welcome.clone().untyped(),
            self.kb_usage.clone().untyped(),
        ]
    }
    fn setup_keys(&self, asset_keys: &mut DynamicAssets) {
        register_file_asset(asset_keys, "kb_welcome", self.get_welcome_path());
        register_file_asset(asset_keys, "kb_usage", self.get_usage_path());
    }
}
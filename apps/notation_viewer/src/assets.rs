use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_asset_loader::prelude::*;
use notation_bevy::prelude::*;

#[derive(AssetCollection)]
pub struct NotationViewerAssets {
    #[asset(key = "kb_welcome")]
    pub kb_welcome: Handle<MarkDownAsset>,

    #[asset(key = "kb_usage")]
    pub kb_usage: Handle<MarkDownAsset>,
}
impl NotationViewerAssets {
    pub fn get_welcome_path(settings: &NotationSettings) -> String {
        let lang = settings.lang();
        format!("kb/{}/welcome.md", lang)
    }
    pub fn get_usage_path(settings: &NotationSettings) -> String {
        let lang = settings.lang();
        format!("kb/{}/usage.md", lang)
    }
}

impl ExtraAssets for NotationViewerAssets {
    fn get_assets(&self) -> Vec<HandleUntyped> {
        vec![
            self.kb_welcome.clone_untyped(),
            self.kb_usage.clone_untyped(),
        ]
    }
    fn setup_extra_keys(settings: &NotationSettings, asset_keys: &mut DynamicAssets) {
        asset_keys.register_asset("kb_welcome", Box::new(StandardDynamicAsset::File {
            path: Self::get_welcome_path(settings)
        }));
        asset_keys.register_asset("kb_usage", Box::new(StandardDynamicAsset::File {
            path: Self::get_usage_path(settings)
        }));
    }
}
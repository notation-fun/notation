use notation_bevy::bevy::prelude::*;
use notation_bevy::prelude::{bevy_asset_loader, AssetCollection, MarkDownAsset, ExtraAssets, EguiFontSizes};

#[bevy_main]
fn main() {
    #[cfg(target_arch = "wasm32")]
    let tabs = vec![notation_bevy::prelude::NotationApp::get_tab_from_url()
        .unwrap_or("tabs/long_juan_feng.ron".to_owned())];

    #[cfg(not(target_arch = "wasm32"))]
    let tabs = vec![
        "tabs/test.ron".to_owned(),
        "tabs/long_juan_feng.ron".to_owned(),
    ];
    notation_viewer::viewer::NotationViewer::run::<NotationViewerAssets>(tabs);
}

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
    fn get_lyrics_font() -> &'static str {
        "fonts/NotoSansSC-Medium.otf.egui"
    }
    fn get_egui_font_sizes(&self) -> EguiFontSizes {
        EguiFontSizes::BIGGER
    }
}
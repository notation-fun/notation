use notation_bevy::bevy::prelude::bevy_main;
use notation_viewer::assets::NotationViewerAssets;

#[bevy_main]
fn main() {
    #[cfg(target_arch = "wasm32")]
    let tabs = vec![notation_bevy::prelude::NotationApp::get_tab_from_url()
        .unwrap_or("tabs/scarborough_fair.ron".to_owned())];

    #[cfg(not(target_arch = "wasm32"))]
    let tabs = vec![
        "tabs/test.ron".to_owned(),
        "tabs/scarborough_fair.ron".to_owned(),
    ];
    notation_viewer::viewer::NotationViewer::run::<NotationViewerAssets>(tabs);
}

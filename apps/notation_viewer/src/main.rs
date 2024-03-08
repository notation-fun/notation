use tab_viewer::edger_bevy::bevy_prelude::bevy_main;
use tab_viewer::prelude::NotationArgs;
use notation_viewer::assets::NotationViewerAssets;

#[bevy_main]
fn main() {
    let args = NotationArgs::parse_args();
    notation_viewer::viewer::NotationViewer::run(args);
}

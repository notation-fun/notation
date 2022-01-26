use notation_bevy::{bevy::prelude::bevy_main, prelude::NotationArgs};
use notation_viewer::assets::NotationViewerAssets;

#[bevy_main]
fn main() {
    let args = NotationArgs::parse_args();
    notation_viewer::viewer::NotationViewer::run::<NotationViewerAssets>(args);
}

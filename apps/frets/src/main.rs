use notation_bevy::{bevy::prelude::bevy_main, prelude::NotationArgs};
use frets::assets::FretsAssets;

#[bevy_main]
fn main() {
    let args = NotationArgs::parse_args();
    frets::app::FretsApp::run::<FretsAssets>(args);
}

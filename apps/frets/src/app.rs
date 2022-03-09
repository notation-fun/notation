use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};

use notation_bevy::prelude::*;
use notation_bevy::settings::layout_settings::LayoutMode;

pub struct FretsApp();

impl FretsApp {
    fn extra(app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
        );
    }
    pub fn run<A: ExtraAssets>(args: NotationArgs) {
        notation_bevy::prelude::NotationApp::run_with_extra::<A, _>(args, Self::extra);
    }
}

impl FretsApp {
}
use tab_viewer::bevy::prelude::*;
use tab_viewer::bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};

use tab_viewer::prelude::*;
use tab_viewer::settings::layout_settings::LayoutMode;

pub struct FretsApp();

impl FretsApp {
    fn extra(app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
        );
    }
    pub fn run<A: ExtraAssets>(args: NotationArgs) {
        tab_viewer::prelude::NotationApp::run_with_extra::<A, _>(args, Self::extra);
    }
}

impl FretsApp {
}
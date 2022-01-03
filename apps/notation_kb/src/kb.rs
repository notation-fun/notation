use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy::prelude::AppBuilder;
//use notation_bevy::bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};

use notation_bevy::prelude::*;

use crate::index_panel::IndexPanel;

pub struct NotationKnowledgeBase();

impl NotationKnowledgeBase {
    fn extra(app: &mut AppBuilder) {
        app.init_resource::<IndexPanel>();
        app.add_startup_system(Self::setup_state.system());
        TabPlugin::setup_mouse_input(app);
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(IndexPanel::index_ui.system())
                .with_system(IndexPanel::handle_link_evts.system())
        );
    }
    pub fn run() {
        notation_bevy::prelude::NotationApp::run_with_extra(vec![], Self::extra);
    }
    fn setup_state(
        mut state: ResMut<NotationState>,
    ) {
        state.show_kb = true;
    }
}


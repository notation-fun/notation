use std::sync::Arc;

use notation_bevy::bevy::prelude::*;
//use notation_bevy::bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};

use notation_bevy::prelude::*;

use crate::assets::NotationKnowledgeBaseAssets;
use crate::index_panel::IndexPanel;

pub struct NotationKnowledgeBase();

impl NotationKnowledgeBase {
    fn extra(app: &mut App) {
        app.init_resource::<IndexPanel>();
        app.add_startup_system(Self::setup_state);
        TabPlugin::setup_mouse_input(app);
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(IndexPanel::hack_settings)
                .with_system(IndexPanel::check_reload)
                .with_system(IndexPanel::index_ui)
                .with_system(IndexPanel::index_audio)
                .with_system(IndexPanel::handle_link_evts)
                .with_system(Self::load_tab)
                .with_system(Self::on_window_resized)
        );
    }
    pub fn run() {
        notation_bevy::prelude::NotationApp::run_with_extra::<NotationKnowledgeBaseAssets, _>(vec![], Self::extra);
    }
    fn setup_state(
        mut state: ResMut<NotationState>,
    ) {
        state.show_kb = true;
    }
    fn load_tab(
        mut commands: Commands,
        time: Res<Time>,
        mut windows: ResMut<Windows>,
        mut state: ResMut<NotationState>,
        mut theme: ResMut<NotationTheme>,
        mut evts: EventWriter<AddTabEvent>,
        entities: Query<Entity, With<GlobalTransform>>,
        viewer_query: Query<(Entity, &TabViewer), With<TabViewer>>,
        index: Res<IndexPanel>,
    ) {
        NotationApp::load_tab(&mut commands, &time, &mut windows, &mut state, &mut theme, &mut evts, &entities, &viewer_query, |tab_path| {
            index.make_tab(tab_path)
        })
    }
    fn on_window_resized(
        mut state: ResMut<NotationState>,
        mut theme: ResMut<NotationTheme>,
        mut window_resized_evts: EventReader<WindowResizedEvent>,
    ) {
        let mut need_reload = false;
        for evt in window_resized_evts.iter() {
            if state.window_width > state.window_height {
                if evt.last_width <= evt.last_height {
                    need_reload = true;
                }
            } else {
                if evt.last_width > evt.last_height {
                    need_reload = true;
                }
            }
        }
        if need_reload {
            Control::reload_tab(&mut state, &mut theme);
        }
    }
}


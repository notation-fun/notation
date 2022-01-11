use std::sync::Arc;

use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy::prelude::AppBuilder;
//use notation_bevy::bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};

use notation_bevy::prelude::*;

use crate::assets::NotationKnowledgeBaseAssets;
use crate::index_panel::IndexPanel;

pub struct NotationKnowledgeBase();

impl NotationKnowledgeBase {
    fn extra(app: &mut AppBuilder) {
        app.init_resource::<IndexPanel>();
        app.add_startup_system(Self::setup.system());
        TabPlugin::setup_mouse_input(app);
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(IndexPanel::index_ui.system())
                .with_system(IndexPanel::index_audio.system())
                .with_system(IndexPanel::handle_link_evts.system())
                .with_system(Self::load_tab.system())
        );
    }
    pub fn run() {
        notation_bevy::prelude::NotationApp::run_with_extra::<NotationKnowledgeBaseAssets, _>(vec![], Self::extra);
    }
    fn setup(
        mut state: ResMut<NotationState>,
        mut settings: ResMut<NotationSettings>,
    ) {
        state.show_kb = true;
        settings.hide_guitar_view = true;
        settings.hide_chords_view = true;
        settings.hide_mini_map = true;
    }
    fn load_tab(
        mut commands: Commands,
        time: Res<Time>,
        mut windows: ResMut<Windows>,
        mut state: ResMut<NotationState>,
        mut theme: ResMut<NotationTheme>,
        mut evts: EventWriter<AddTabEvent>,
        entities: Query<Entity, With<GlobalTransform>>,
        viewer_query: Query<(Entity, &Arc<TabViewer>), With<Arc<TabViewer>>>,
        index: Res<IndexPanel>,
    ) {
        NotationApp::load_tab(&mut commands, &time, &mut windows, &mut state, &mut theme, &mut evts, &entities, &viewer_query, |tab_path| {
            index.make_tab(tab_path)
        })
    }
}


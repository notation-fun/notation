use bevy::window::PrimaryWindow;
use tab_viewer::bevy::prelude::*;
//use tab_viewer::bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};

use tab_viewer::prelude::*;

use crate::index_panel::IndexPanel;
use notation_viewer::viewer::NotationViewer;

pub struct NotationKnowledgeBase();

impl NotationKnowledgeBase {
    fn extra(app: &mut App) {
        app.init_resource::<IndexPanel>();
        app.add_systems(Startup, Self::setup_state);
        TabPlugin::setup_mouse_input(app);
        #[cfg(target_arch = "wasm32")]
        tab_viewer::prelude::StereoStream::init_streaming(app, true);
        app.add_systems(Update, (
            IndexPanel::hack_settings,
            IndexPanel::check_reload,
            IndexPanel::index_ui,
            IndexPanel::index_audio,
            IndexPanel::handle_link_evts,
            NotationViewer::handle_keyboard_inputs,
            NotationViewer::handle_mouse_inputs,
            NotationViewer::handle_touch_inputs,
            Self::load_tab,
            Self::on_window_resized,
        ).run_if(in_state(NotationAssetsStates::Loaded)));
    }
    pub fn run<A: ExtraAssets>(args: NotationArgs) {
        tab_viewer::prelude::NotationApp::run_with_extra::<A, _>(args, Self::extra);
    }
    fn setup_state(
        mut state: ResMut<NotationState>,
    ) {
        state.show_kb = true;
    }
    fn load_tab(
        mut commands: Commands,
        time: Res<Time>,
        mut window_query: Query<&mut Window, With<PrimaryWindow>>,
        mut state: ResMut<NotationState>,
        mut theme: ResMut<NotationTheme>,
        mut settings: ResMut<NotationSettings>,
        mut evts: EventWriter<AddTabEvent>,
        entities: Query<Entity, With<GlobalTransform>>,
        viewer_query: Query<(Entity, &TabViewer), With<TabViewer>>,
        index: Res<IndexPanel>,
    ) {
        settings.add_ready_section = false;
        NotationApp::load_tab(&mut commands, &time, &mut window_query, &mut state, &mut theme, &settings, &mut evts, &entities, &viewer_query, |tab_path| {
            Some(TabAsset::from(index.make_tab(tab_path)))
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


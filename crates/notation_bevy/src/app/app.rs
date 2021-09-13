use std::sync::Arc;

use bevy::app::PluginGroupBuilder;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::WindowResized;

use bevy_asset_loader::AssetLoader;

use crate::prelude::*;
use crate::ui::viewer::TabViewerPlugin;
use crate::viewer::control::ControlView;

use notation_midi::prelude::{MidiPlugin, MidiState, PlayControlEvent};
use notation_model::prelude::*;

use super::app_state::{NotationAppState, TabPathes};

pub struct NotationPlugins;
impl PluginGroup for NotationPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(EntryPlugin);
        group.add(MelodyPlugin);
        group.add(LyricsPlugin);
        group.add(BarPlugin);
        group.add(StringsPlugin);
        group.add(ShapesPlugin);
        group.add(MiniPlugin);
        group.add(TabPlugin);
        group.add(PlayPlugin);
        group.add(TabViewerPlugin);
        //crates plugins
        group.add(MidiPlugin);
        //external plugins
        group.add(bevy_prototype_lyon::prelude::ShapePlugin);
        //group.add(bevy_svg::prelude::SvgPlugin);
    }
}

pub struct NotationApp;

impl NotationApp {
    pub fn new_builder(title: &str) -> AppBuilder {
        let mut app = App::build();
        AssetLoader::new(NotationAssetsStates::Loading, NotationAssetsStates::Loaded)
            .with_collection::<NotationAssets>()
            .build(&mut app);
        app.add_state(NotationAssetsStates::Loading);
        insert_window_descriptor(&mut app, String::from(title));
        super::app_events::add_notation_app_events(&mut app);

        app.insert_resource(Msaa { samples: 1 });
        app.add_plugins(DefaultPlugins);
        app.insert_resource(ClearColor(CoreTheme::default().background_color));
        app.add_plugin(bevy_easings::EasingsPlugin);

        app.init_resource::<NotationTheme>();
        app.init_resource::<NotationSettings>();
        app.add_plugins(NotationPlugins);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        // When building for WASM, print panics to the browser console
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(crate::wasm::bevy_web_fullscreen::FullViewportPlugin);

        app.add_plugin(bevy_egui::EguiPlugin);
        app.add_plugin(NotationUiPlugin);

        #[cfg(feature = "dev")]
        app.add_plugins(crate::dev::NotationDevPlugins);

        #[cfg(feature = "inspector")]
        app.add_plugins(crate::inspector::NotationInspectorPlugins);

        app
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_tab_from_url() -> Result<String, String> {
        web_sys::window()
            .ok_or("No_Window".to_owned())
            .and_then(|x| x.document().ok_or("No_Document".to_owned()))
            .and_then(|x| x.location().ok_or("No_Location".to_owned()))
            .and_then(|x| x.search().map_err(|e| format!("No_Search:{:?}", e)))
            .map(|x| x.trim_start_matches('?').to_owned())
    }

    pub fn run<F>(title: &str, tab_pathes: Vec<String>, extra: F)
    where
        F: Fn(&mut AppBuilder),
    {
        let mut app = NotationApp::new_builder(title);

        app.insert_resource(TabPathes(tab_pathes));
        app.init_resource::<NotationAppState>();

        app.add_startup_system(setup_camera.system());

        app.add_system_set(
            SystemSet::on_enter(NotationAssetsStates::Loaded)
                .with_system(setup_window_size.system())
        );
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(on_window_resized.system())
                .with_system(handle_inputs.system())
                .with_system(load_tab.system())
        );

        extra(&mut app);
        app.run();
    }
}

fn insert_window_descriptor(app: &mut AppBuilder, title: String) {
    app.insert_resource(WindowDescriptor {
        title,
        ..WindowDescriptor::default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn load_tab(
    mut state: ResMut<NotationAppState>,
    assets: ResMut<Assets<TabAsset>>,
    mut evts: EventWriter<AddTabEvent>,
) {
    if state.window_width > 0.0 && state.window_height > 0.0 && state.tab.is_none() && state.parse_error.is_none() {
        if let Some(asset) = assets.get(&state.tab_asset) {
            match Tab::try_parse_arc(asset.tab.clone()) {
                Ok(tab) => {
                    state.tab = Some(tab.clone());
                    evts.send(AddTabEvent(tab));
                }
                Err(err) => {
                    println!("Parse Tab Failed: {:?}", err);
                    state.parse_error = Some(err);
                }
            }
        }
    }
}

fn handle_inputs(
    mut commands: Commands,
    windows: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut state: ResMut<NotationAppState>,
    mut settings: ResMut<NotationSettings>,
    mut midi_state: ResMut<MidiState>,
    mut play_control_evts: EventWriter<PlayControlEvent>,
    mut mouse_clicked: EventWriter<MouseClickedEvent>,
    mut mouse_dragged: EventWriter<MouseDraggedEvent>,
    mut window_resized_evts: EventWriter<WindowResizedEvent>,
    viewer_query: Query<(Entity, &Arc<NotationViewer>), With<Arc<NotationViewer>>>,
) {
    if keyboard_input.pressed(KeyCode::LControl) {
        settings.mouse_dragged_panning = true;
    } else if keyboard_input.just_released(KeyCode::LControl) {
        settings.mouse_dragged_panning = false;
    } else if keyboard_input.just_released(KeyCode::Tab) {
        state.hide_control = !state.hide_control;
        if !ControlView::HUD_MODE {
            window_resized_evts.send(WindowResizedEvent());
        }
    } else if keyboard_input.just_released(KeyCode::Space) {
        crate::viewer::control::ControlView::play_or_pause(&mut midi_state, &mut play_control_evts);
    } else if keyboard_input.just_released(KeyCode::Return) {
        crate::viewer::control::ControlView::play_or_stop(&mut midi_state, &mut play_control_evts);
    } else if keyboard_input.just_released(KeyCode::Backslash) {
        crate::viewer::control::ControlView::toggle_layout_mode(&mut commands, &mut state, &mut settings, &viewer_query);
    } else if mouse_input.just_released(MouseButton::Left) {
        windows
            .get_primary()
            .and_then(|x| x.cursor_position())
            .map(|cursor_position| {
                //println!("handle_inputs() -> MouseClickedEvent({:?})", cursor_position);
                mouse_clicked.send(MouseClickedEvent { cursor_position });
            });
    } else {
        if mouse_input.pressed(MouseButton::Left) {
            for event in mouse_motion_events.iter() {
                //println!("handle_inputs() -> MouseDraggedEvent({:?})", event.delta);
                mouse_dragged.send(MouseDraggedEvent { delta: event.delta });
            }
        }
    }
}

fn setup_window_size(
    window: Res<WindowDescriptor>,
    mut app_state: ResMut<NotationAppState>,
) {
    #[cfg(target_arch = "wasm32")]
    let (width, height) = crate::wasm::bevy_web_fullscreen::get_viewport_size();

    #[cfg(not(target_arch = "wasm32"))]
    let (width, height) = (window.width, window.height);

    println!("setup_window_size(): {} {} ", width, height);
    app_state.window_width = width;
    app_state.window_height = height;
}

fn on_window_resized(
    mut window: ResMut<WindowDescriptor>,
    mut evts: EventReader<WindowResized>,
    mut app_state: ResMut<NotationAppState>,
    mut window_resized_evts: EventWriter<WindowResizedEvent>,
) {
    for evt in evts.iter() {
        if evt.width as usize != window.width as usize
            || evt.height as usize != window.height as usize
        {
            println!("on_window_resized(): {} {} -> {} {} ", window.width, window.height, evt.width, evt.height);
            window.width = evt.width;
            window.height = evt.height;
            app_state.window_width = evt.width;
            app_state.window_height = evt.height;
            window_resized_evts.send(WindowResizedEvent());
        }
    }
}

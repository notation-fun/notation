use bevy::app::PluginGroupBuilder;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;
use bevy::window::WindowResized;

use crate::prelude::*;
use crate::settings::layout_settings::LayoutSettings;
use notation_midi::prelude::MidiPlugin;
use notation_model::prelude::*;

use super::notation_app_state::{NotationAppState, TabPathes};
use super::top_panel;

pub struct NotationPlugins;
impl PluginGroup for NotationPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(EntryPlugin);
        group.add(MelodyPlugin);
        group.add(LyricsPlugin);
        group.add(LanePlugin);
        group.add(BarPlugin);
        group.add(StringsPlugin);
        group.add(ShapesPlugin);
        group.add(GuitarPlugin);
        group.add(TabPlugin);
        group.add(PlayPlugin);
        //crates plugins
        group.add(MidiPlugin);
        //external plugins
        group.add(bevy_prototype_lyon::prelude::ShapePlugin);
    }
}

pub struct NotationApp;

impl NotationApp {
    pub fn new_builder(title: &str) -> AppBuilder {
        let mut app = App::build();
        insert_window_descriptor(&mut app, String::from(title));
        super::notation_app_events::add_notation_app_events(&mut app);

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
        app.add_plugin(crate::ext::bevy_web_fullscreen::FullViewportPlugin);

        app.add_plugin(bevy_egui::EguiPlugin);
        app.add_plugin(NotationUiPlugin);

        #[cfg(feature = "dev")]
        app.add_plugins(crate::dev::NotationDevPlugins);

        #[cfg(feature = "inspector")]
        app.add_plugins(crate::inspector::NotationInspectorPlugins);

        app
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_tab_from_url() -> Result<Tone, Tone> {
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
        app.add_startup_system(setup_camera.system());

        app.insert_resource(TabPathes(tab_pathes));
        app.init_resource::<NotationAppState>();

        app.add_startup_system(setup_window_size.system());
        app.add_system(on_window_resized.system());

        app.add_system(update_camera.system());

        app.add_system(load_tab.system());

        app.add_system(top_panel::top_panel_ui.system());

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
    if state.tab.is_none() && state.parse_error.is_none() {
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

fn update_camera(
    _keyboard_input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut state: ResMut<NotationAppState>,
    settings: Res<NotationSettings>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &OrthographicProjection)>,
) {
    if keyboard_input.just_released(KeyCode::Space) {
        state.camera_panning = !state.camera_panning;
    }

    if state.camera_panning {
        for event in mouse_motion_events.iter() {
            if mouse_input.pressed(MouseButton::Left) {
                settings
                    .layout
                    .pan_camera(&mut camera_query, event.delta.x, event.delta.y);
            }
        }
    }
}

fn setup_window_size(
    window: Res<WindowDescriptor>,
    mut app_state: ResMut<NotationAppState>,
    settings: Res<NotationSettings>,
    mut theme: ResMut<NotationTheme>,
) {
    app_state.window_width = window.width;
    app_state.window_height = window.height;
    theme.grid.resize(&app_state, &settings);
}

fn on_window_resized(
    mut window: ResMut<WindowDescriptor>,
    mut evts: EventReader<WindowResized>,
    mut app_state: ResMut<NotationAppState>,
    settings: Res<NotationSettings>,
    mut theme: ResMut<NotationTheme>,
    mut config_evts: EventWriter<WindowResizedEvent>,
) {
    for evt in evts.iter() {
        if evt.width as usize != window.width as usize
            || evt.height as usize != window.height as usize
        {
            window.width = evt.width;
            window.height = evt.height;
            app_state.window_width = evt.width;
            app_state.window_height = evt.height;
            theme.grid.resize(&app_state, &settings);
            config_evts.send(WindowResizedEvent());
        }
    }
}

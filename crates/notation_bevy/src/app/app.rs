use std::sync::Arc;

use bevy::app::PluginGroupBuilder;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::WindowResized;

use bevy_asset_loader::AssetLoader;

use crate::theme::theme_colors::UiColors;
use crate::{prelude::*, settings::layout_settings::LayoutMode};
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
        app.insert_resource(ClearColor(UiColors::default().app_background));
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
                .with_system(handle_keyboard_inputs.system())
                .with_system(handle_mouse_inputs.system())
                .with_system(handle_touch_inputs.system())
                .with_system(load_tab.system())
        );

        extra(&mut app);
        app.run();
    }
}

fn insert_window_descriptor(app: &mut AppBuilder, title: String) {
    app.insert_resource(WindowDescriptor {
        title,
        width: 1920.,
        height: 1080.,
        ..WindowDescriptor::default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn load_tab(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<NotationAppState>,
    mut theme: ResMut<NotationTheme>,
    entities: Query<Entity, With<GlobalTransform>>,
    assets: ResMut<Assets<TabAsset>>,
    mut evts: EventWriter<AddTabEvent>,
    viewer_query: Query<(Entity, &Arc<NotationViewer>), With<Arc<NotationViewer>>>,
) {
    if state.window_width > 0.0 && state.window_height > 0.0 && state.tab.is_none() && state.parse_error.is_none() {
        let mut count = 0;
        for _ in entities.iter() {
            count += 1;
        }
        //A bit hacky to make sure despawning finished, otherwise might got panic with "Entity not exist"
        if count > 1 {
            if state._despawn_delay_seconds > 0.0 {
                state._despawn_delay_seconds -= time.delta_seconds();
                println!("load_tab(): Waiting to despawn: {} -> {}", count, state._despawn_delay_seconds);
                return;
            }
            let mut despawn_count = 0;
            for (entity, _viewer) in viewer_query.iter() {
                commands.entity(entity).despawn_recursive();
                despawn_count += 1;
            }
            if despawn_count > 0 {
                println!("load_tab(): Despawning viewers: {} {}", despawn_count, count);
            } else {
                println!("load_tab(): Waiting for entities to be despawned: {}", count);
            }
            return;
        }
        if state._load_tab_delay_seconds > 0.0 {
            state._load_tab_delay_seconds -= time.delta_seconds();
            println!("load_tab(): Waiting to Load tab: -> {}", state._load_tab_delay_seconds);
            return;
        }
        println!("\nload_tab(): Loading: {}", state.tab_path);
        if let Some(asset) = assets.get(&state.tab_asset) {
            match Tab::try_parse_arc(asset.tab.clone()) {
                Ok(tab) => {
                    state.tab = Some(tab.clone());
                    theme._bypass_systems = false;
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

fn handle_keyboard_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NotationAppState>,
    mut settings: ResMut<NotationSettings>,
    mut theme: ResMut<NotationTheme>,
    mut midi_state: ResMut<MidiState>,
    mut play_control_evts: EventWriter<PlayControlEvent>,
    mut window_resized_evts: EventWriter<WindowResizedEvent>,
) {
    if app_state.tab.is_none() {
        return;
    }
    if keyboard_input.just_released(KeyCode::LControl) {
        app_state.hide_control = !app_state.hide_control;
        if !ControlView::HUD_MODE {
            window_resized_evts.send(WindowResizedEvent());
        }
    } else if keyboard_input.just_released(KeyCode::Space) {
        crate::viewer::control::ControlView::play_or_pause(&mut midi_state, &mut play_control_evts);
    } else if keyboard_input.just_released(KeyCode::Return) {
        crate::viewer::control::ControlView::stop(&mut midi_state, &mut play_control_evts);
    } else if keyboard_input.just_released(KeyCode::Backslash) {
        crate::viewer::control::ControlView::toggle_layout_mode(&mut app_state, &mut settings, &mut theme);
    } else if keyboard_input.just_released(KeyCode::M) {
        crate::viewer::control::ControlView::toggle_show_melody_syllable(&mut app_state, &mut settings, &mut theme);
    } else if keyboard_input.just_released(KeyCode::F) {
        crate::viewer::control::ControlView::toggle_always_show_fret(&mut app_state, &mut settings, &mut theme);
    } else if keyboard_input.just_released(KeyCode::L) {
        settings.should_loop = !settings.should_loop;
        crate::viewer::control::ControlView::sync_should_loop(
            &settings,
            &mut midi_state,
            &mut play_control_evts,
        );
    } else if keyboard_input.just_released(KeyCode::A) {
        crate::viewer::control::ControlView::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts);
    } else if keyboard_input.just_released(KeyCode::B) {
        crate::viewer::control::ControlView::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts);
    } else if keyboard_input.just_released(KeyCode::C) {
        crate::viewer::control::ControlView::clear_begin_end(&mut midi_state, &mut play_control_evts);
    }
}

fn handle_mouse_inputs(
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    app_state: Res<NotationAppState>,
    settings: Res<NotationSettings>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_input: EventReader<bevy::input::mouse::MouseWheel>,
    mut mouse_clicked: EventWriter<MouseClickedEvent>,
    mut mouse_dragged: EventWriter<MouseDraggedEvent>,
) {
    if app_state.tab.is_none() {
        return;
    }
    if mouse_input.just_released(MouseButton::Left) {
        windows
            .get_primary()
            .and_then(|x| x.cursor_position())
            .map(|cursor_position| {
                //println!("handle_inputs() -> MouseClickedEvent({:?})", cursor_position);
                mouse_clicked.send(MouseClickedEvent { cursor_position });
            });
    } else if mouse_input.just_pressed(MouseButton::Right) {
    } else if mouse_input.just_released(MouseButton::Right) {
    } else if mouse_input.pressed(MouseButton::Right) {
        for event in mouse_motion_events.iter() {
            //println!("handle_inputs() -> MouseDraggedEvent({:?})", event.delta);
            mouse_dragged.send(MouseDraggedEvent { delta: event.delta });
        }
    } else {
        for event in mouse_wheel_input.iter() {
            let mut delta = match event.unit {
                    bevy::input::mouse::MouseScrollUnit::Line =>
                        Vec2::new(event.x * settings.panning_line_size, event.y * settings.panning_line_size),
                    bevy::input::mouse::MouseScrollUnit::Pixel =>
                        Vec2::new(event.x, event.y),
                };
            if settings.layout.mode == LayoutMode::Line {
                delta = Vec2::new(delta.y, delta.x);
            }
            mouse_dragged.send(MouseDraggedEvent { delta: delta });
        }
    }
}

fn handle_touch_inputs(
    windows: Res<Windows>,
    touch_input: Res<Touches>,
    mut app_state: ResMut<NotationAppState>,
    mut mouse_clicked: EventWriter<MouseClickedEvent>,
    //mut mouse_dragged: EventWriter<MouseDraggedEvent>,
) {
    if app_state.tab.is_none() {
        return;
    }
    for (_index, finger) in touch_input.iter().enumerate() {
        if touch_input.just_pressed(finger.id()) {
            windows
                .get_primary()
                .map(|w| (w.physical_width() as f32, w.physical_height() as f32))
                .map(| (physical_width, physical_height) | {
                    /*
                    Super hacky way to get the touch input in mobile browsers (WASM).
                    winit not support it yet, using a pull request version, which seems to have some issues
                    as well, also the touch event triggering is very unreliable during my test, but at least
                    it's better than no touch at all.
                    */
                    let dpi_x = physical_width / app_state.window_width;
                    let dpi_y = physical_height / app_state.window_height;
                    let x = finger.position().x * dpi_x;
                    let y = app_state.window_height - finger.position().y * dpi_y;
                    app_state.debug_str = Some(format!("Touch: {} {:?} -> {} {}", _index, finger.position(), x, y));
                    mouse_clicked.send(MouseClickedEvent { cursor_position: Vec2::new(x, y) });
                });
        } else if touch_input.just_released(finger.id()) {
            app_state.debug_str = None;
        } else {
            app_state.debug_str = Some(format!("Touch: {} - {:?}", _index, finger.position()));
            /*
            let delta = finger.position() - finger.previous_position();
            app_state.debug_str = Some(format!("Dragged: {}, {:?}", _index, delta));
            mouse_dragged.send(MouseDraggedEvent { delta: delta });
             */
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
    if app_state.tab.is_none() {
        return;
    }
    for evt in evts.iter() {
        if evt.width as usize != window.width as usize
            || evt.height as usize != window.height as usize
        {
            println!("on_window_resized(): {} {} -> {} {} ", window.width, window.height, evt.width, evt.height);
            window.width = evt.width;
            window.height = evt.height;
            app_state.window_width = evt.width;
            app_state.window_height = evt.height;
            app_state.scale_factor_override = window.scale_factor_override;
            window_resized_evts.send(WindowResizedEvent());
        }
    }
}

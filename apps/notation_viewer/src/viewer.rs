use std::sync::Arc;

use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy::prelude::AppBuilder;
use notation_bevy::bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};

use notation_bevy::prelude::*;
use notation_bevy::settings::layout_settings::LayoutMode;
use notation_bevy::notation_midi::prelude::*;

use crate::help_panel::HelpPanel;

pub struct NotationViewer();

impl NotationViewer {
    fn extra(app: &mut AppBuilder) {
        app.init_resource::<HelpPanel>();
        TabPlugin::setup_mouse_input(app);
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(Self::handle_keyboard_inputs.system())
                .with_system(Self::handle_mouse_inputs.system())
                .with_system(Self::handle_touch_inputs.system())
                .with_system(Self::load_tab.system())
                .with_system(HelpPanel::help_ui.system())
        );
    }
    pub fn run(tabs: Vec<String>) {
        notation_bevy::prelude::NotationApp::run_with_extra(tabs, Self::extra);
    }
}

impl NotationViewer {
    fn load_tab(
        mut commands: Commands,
        time: Res<Time>,
        asset_server: Res<AssetServer>,
        mut windows: ResMut<Windows>,
        mut state: ResMut<NotationState>,
        mut theme: ResMut<NotationTheme>,
        entities: Query<Entity, With<GlobalTransform>>,
        assets: ResMut<Assets<TabAsset>>,
        mut evts: EventWriter<AddTabEvent>,
        viewer_query: Query<(Entity, &Arc<TabViewer>), With<Arc<TabViewer>>>,
    ) {
        if state.window_width > 0.0
            && state.window_height > 0.0
            && state.tab.is_none()
            && state.parse_error.is_none()
        {
            let mut count = 0;
            for _ in entities.iter() {
                count += 1;
            }
            //A bit hacky to make sure despawning finished, otherwise might got panic with "Entity not exist"
            if count > 1 {
                if state._despawn_delay_seconds > 0.0 {
                    state._despawn_delay_seconds -= time.delta_seconds();
                    println!(
                        "load_tab(): Waiting to despawn: {} -> {}",
                        count, state._despawn_delay_seconds
                    );
                    return;
                }
                let mut despawn_count = 0;
                for (entity, _viewer) in viewer_query.iter() {
                    commands.entity(entity).despawn_recursive();
                    despawn_count += 1;
                }
                if despawn_count > 0 {
                    println!(
                        "load_tab(): Despawning viewers: {} {}",
                        despawn_count, count
                    );
                } else {
                    println!(
                        "load_tab(): Waiting for entities to be despawned: {}",
                        count
                    );
                }
                return;
            }
            asset_server.free_unused_assets();
            if state._load_tab_delay_seconds > 0.0 {
                state._load_tab_delay_seconds -= time.delta_seconds();
                println!(
                    "load_tab(): Waiting to Load tab: -> {}",
                    state._load_tab_delay_seconds
                );
                return;
            }
            println!("\nload_tab(): Loading: {}", state.tab_path);
            let tab_asset: Handle<TabAsset> = asset_server.load(state.tab_path.as_str());
            if let Some(asset) = assets.get(&tab_asset) {
                match Tab::try_parse_arc(asset.tab.clone()) {
                    Ok(tab) => {
                        state.tab = Some(tab.clone());
                        if let Some(window) = windows.get_primary_mut() {
                            let title = format!("{} - {}", NotationApp::TITLE, state.tab_path);
                            window.set_title(title);
                        }
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
        mut app_state: ResMut<NotationState>,
        mut settings: ResMut<NotationSettings>,
        mut theme: ResMut<NotationTheme>,
        midi_settings: Res<MidiSettings>,
        mut midi_state: ResMut<MidiState>,
        mut play_control_evts: EventWriter<PlayControlEvent>,
        mut window_resized_evts: EventWriter<WindowResizedEvent>,
        mut jump_to_bar_evts: EventWriter<JumpToBarEvent>,
    ) {
        if app_state.tab.is_none() {
            return;
        }
        if keyboard_input.just_released(KeyCode::Tab) {
            app_state.show_control = !app_state.show_control;
            if !ControlPanel::HUD_MODE {
                window_resized_evts.send(WindowResizedEvent());
            }
        } else if keyboard_input.just_released(KeyCode::F1) || keyboard_input.just_released(KeyCode::H)
        {
            app_state.show_kb = !app_state.show_kb;
        } else if keyboard_input.just_released(KeyCode::F5) || keyboard_input.just_released(KeyCode::R)
        {
            Control::reload_tab(&mut app_state, &mut theme);
        } else if keyboard_input.just_released(KeyCode::Space) {
            Control::play_or_pause(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Return) {
            Control::stop(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Home) {
            Control::jump_to_section_start(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::End) {
            Control::jump_to_section_end(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::PageUp) {
            Control::jump_to_prev_section(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::PageDown) {
            Control::jump_to_next_section(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::Left) {
            Control::jump_to_prev_bar(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::Right) {
            Control::jump_to_next_bar(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::Down) {
            Control::seek_forward(&midi_settings, &mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Backslash) {
            Control::toggle_layout_mode(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::G) {
            Control::toggle_show_guitar_syllable(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::M) {
            Control::toggle_show_melody_syllable(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::F) {
            Control::toggle_always_show_fret(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::L) {
            settings.should_loop = !settings.should_loop;
            Control::sync_should_loop(&settings, &mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::A) {
            Control::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::B) {
            Control::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::C) {
            Control::clear_begin_end(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::D) {
            Control::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts);
            Control::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::E) {
            Control::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts);
            Control::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Key1) {
            Control::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 0.25);
        } else if keyboard_input.just_released(KeyCode::Key2) {
            Control::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 0.5);
        } else if keyboard_input.just_released(KeyCode::Key3) {
            Control::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 0.75);
        } else if keyboard_input.just_released(KeyCode::Key4) {
            Control::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 1.0);
        }
    }

    fn handle_mouse_inputs(
        windows: Res<Windows>,
        mouse_input: Res<Input<MouseButton>>,
        app_state: Res<NotationState>,
        settings: Res<NotationSettings>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        mut mouse_wheel_input: EventReader<MouseWheel>,
        mut mouse_clicked: EventWriter<MouseClickedEvent>,
        mut mouse_dragged: EventWriter<MouseDraggedEvent>,
    ) {
        if app_state.tab.is_none() {
            return;
        }
        let cursor_position = windows.get_primary().and_then(|x| x.cursor_position());
        if cursor_position.is_none() {
            return;
        }
        let cursor_position = cursor_position.unwrap();
        if mouse_input.just_released(MouseButton::Left) {
            mouse_clicked.send(MouseClickedEvent { cursor_position });
        } else if mouse_input.just_pressed(MouseButton::Right) {
        } else if mouse_input.just_released(MouseButton::Right) {
        } else if mouse_input.pressed(MouseButton::Right) {
            for event in mouse_motion_events.iter() {
                //println!("handle_inputs() -> MouseDraggedEvent({:?})", event.delta);
                mouse_dragged.send(MouseDraggedEvent {
                    cursor_position,
                    delta: event.delta,
                });
            }
        } else {
            for event in mouse_wheel_input.iter() {
                let mut delta = match event.unit {
                    MouseScrollUnit::Line => Vec2::new(
                        event.x * settings.panning_line_size,
                        event.y * settings.panning_line_size,
                    ),
                    MouseScrollUnit::Pixel => Vec2::new(event.x, event.y),
                };
                if settings.layout.mode == LayoutMode::Line {
                    delta = Vec2::new(delta.y, delta.x);
                }
                mouse_dragged.send(MouseDraggedEvent {
                    cursor_position,
                    delta: delta,
                });
            }
        }
    }

    fn handle_touch_inputs(
        windows: Res<Windows>,
        touch_input: Res<Touches>,
        mut app_state: ResMut<NotationState>,
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
                    .map(|(physical_width, physical_height)| {
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
                        app_state.debug_str = Some(format!(
                            "Touch: {} {:?} -> {} {}",
                            _index,
                            finger.position(),
                            x,
                            y
                        ));
                        mouse_clicked.send(MouseClickedEvent {
                            cursor_position: Vec2::new(x, y),
                        });
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
}
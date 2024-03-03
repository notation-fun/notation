use bevy::input::keyboard::KeyboardInput;
use bevy::window::PrimaryWindow;
use tab_viewer::edger_bevy_app::bevy_prelude::*;
use tab_viewer::edger_bevy_app::bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};

use tab_viewer::edger_bevy_app::bevy_egui::EguiContext;
use tab_viewer::prelude::*;
use tab_viewer::settings::layout_settings::LayoutMode;

use crate::help_panel::HelpPanel;

pub struct NotationViewer();

impl NotationViewer {
    fn extra(app: &mut App) {
        app.init_resource::<HelpPanel>();
        TabPlugin::setup_mouse_input(app);
        app.add_systems(Update, (
            Self::handle_keyboard_inputs,
            Self::handle_mouse_inputs,
            Self::handle_touch_inputs,
            Self::load_tab,
            HelpPanel::help_ui,
            HelpPanel::handle_link_evts,
        ).run_if(in_state(NotationAssetsStates::Loaded)));
    }
    pub fn run<A: ExtraAssets>(args: NotationArgs) {
        tab_viewer::prelude::NotationApp::run_with_extra::<A, _>(args, Self::extra);
    }
}

impl NotationViewer {
    fn load_tab(
        mut commands: Commands,
        time: Res<Time>,
        mut window_query: Query<&mut Window, With<PrimaryWindow>>,
        mut state: ResMut<NotationState>,
        mut theme: ResMut<NotationTheme>,
        settings: Res<NotationSettings>,
        mut evts: EventWriter<AddTabEvent>,
        entities: Query<Entity, With<GlobalTransform>>,
        viewer_query: Query<(Entity, &TabViewer), With<TabViewer>>,
        asset_server: Res<AssetServer>,
        assets: Res<Assets<TabAsset>>,
    ) {
        NotationApp::load_tab(&mut commands, &time, &mut window_query, &mut state, &mut theme, &settings, &mut evts, &entities, &viewer_query, |commands: &mut Commands, tab_path| {
            NotationApp::load_tab_from_assets(commands, &asset_server, &assets, tab_path)
        })
    }
    pub fn handle_keyboard_inputs(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut egui_ctx: EguiContexts,
        mut app_state: ResMut<NotationState>,
        mut settings: ResMut<NotationSettings>,
        mut theme: ResMut<NotationTheme>,
        midi_settings: Res<MidiSettings>,
        mut midi_state: ResMut<MidiState>,
        mut play_control_evts: EventWriter<PlayControlEvent>,
        mut window_resized_evts: EventWriter<WindowResizedEvent>,
        mut jump_to_bar_evts: EventWriter<JumpToBarEvent>,
        tab_bars_query: Query<(&TabBars, &GridData), With<TabBars>>,
    ) {
        if egui_ctx.ctx_mut().wants_keyboard_input() {
            return;
        }
        if keyboard_input.just_released(KeyCode::F10) || keyboard_input.just_released(KeyCode::Backslash) {
            app_state.show_control = !app_state.show_control;
            if !EguiControlPanel::HUD_MODE {
                window_resized_evts.send(WindowResizedEvent::new(&app_state));
            }
        } else if keyboard_input.just_released(KeyCode::F1) || keyboard_input.just_released(KeyCode::KeyH)
        {
            app_state.show_kb = !app_state.show_kb;
        } else if keyboard_input.just_released(KeyCode::F2)
        {
            Control::toggle_hide_guitar_view(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::F3)
        {
            Control::toggle_hide_chords_view(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::F4)
        {
            Control::toggle_hide_mini_map(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::F5) || keyboard_input.just_released(KeyCode::KeyR)
        {
            app_state.bars_range = None;
            Control::reload_tab(&mut app_state, &mut theme);
        } else if keyboard_input.just_released(KeyCode::Space) {
            MidiControl::play_or_pause(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Enter) {
            MidiControl::stop(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Home) {
            MidiControl::jump_to_section_start(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::End) {
            MidiControl::jump_to_section_end(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::PageUp) {
            MidiControl::jump_to_prev_section(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::PageDown) {
            MidiControl::jump_to_next_section(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::ArrowLeft) {
            MidiControl::jump_to_prev_bar(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::ArrowRight) {
            MidiControl::jump_to_next_bar(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::ArrowUp) {
            MidiControl::jump_to_prev_row(&midi_state, &mut jump_to_bar_evts, &tab_bars_query);
        } else if keyboard_input.just_released(KeyCode::ArrowDown) {
            MidiControl::jump_to_next_row(&midi_state, &mut jump_to_bar_evts, &tab_bars_query);
        } else if keyboard_input.just_released(KeyCode::Backquote) {
            MidiControl::seek_forward(&midi_settings, &mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Minus) {
            Control::toggle_layout_mode(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::KeyS) {
            Control::toggle_show_note_syllable(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::KeyP) {
            Control::toggle_show_note_pitch(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::KeyF) {
            Control::toggle_always_show_fret(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::KeyL) {
            settings.should_loop = !settings.should_loop;
            MidiControl::sync_should_loop(&settings, &mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::KeyA) {
            MidiControl::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::KeyB) {
            MidiControl::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::KeyC) {
            MidiControl::clear_begin_end(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::KeyD) {
            MidiControl::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts);
            MidiControl::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::KeyE) {
            MidiControl::set_begin_end_to_section(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Digit1) {
            MidiControl::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 0.25);
        } else if keyboard_input.just_released(KeyCode::Digit2) {
            MidiControl::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 0.5);
        } else if keyboard_input.just_released(KeyCode::Digit3) {
            MidiControl::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 0.75);
        } else if keyboard_input.just_released(KeyCode::Digit4) {
            MidiControl::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 1.0);
        }
    }

    pub fn handle_mouse_inputs(
        window_query: Query<&Window, With<PrimaryWindow>>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        mut egui_ctx: EguiContexts,
        mut app_state: ResMut<NotationState>,
        settings: Res<NotationSettings>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        mut mouse_wheel_input: EventReader<MouseWheel>,
        mut mouse_clicked: EventWriter<MouseClickedEvent>,
        mut mouse_dragged: EventWriter<MouseDraggedEvent>,
    ) {
        if app_state.tab.is_none() {
            return;
        }
        if egui_ctx.ctx_mut().is_pointer_over_area() {
            return;
        }
        let Ok(window) = window_query.get_single() else {
            return;
        };
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        if mouse_input.just_released(MouseButton::Left) {
            mouse_clicked.send(MouseClickedEvent { cursor_position });
        } else if mouse_input.just_pressed(MouseButton::Right) {
        } else if mouse_input.just_released(MouseButton::Right) {
        } else if mouse_input.pressed(MouseButton::Right) {
            for event in mouse_motion_events.read() {
                //println!("handle_inputs() -> MouseDraggedEvent({:?})", event.delta);
                mouse_dragged.send(MouseDraggedEvent {
                    cursor_position,
                    delta: event.delta,
                });
            }
        } else {
            for event in mouse_wheel_input.read() {
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

    pub fn handle_touch_inputs(
        window_query: Query<&Window, With<PrimaryWindow>>,
        touch_input: Res<Touches>,
        mut egui_ctx: EguiContexts,
        mut app_state: ResMut<NotationState>,
        mut mouse_clicked: EventWriter<MouseClickedEvent>,
        //mut mouse_dragged: EventWriter<MouseDraggedEvent>,
    ) {
        if app_state.tab.is_none() {
            return;
        }
        if egui_ctx.ctx_mut().wants_pointer_input() {
            return;
        }
        let Ok(window) = window_query.get_single() else {
            return;
        };
        for (_index, finger) in touch_input.iter().enumerate() {
            if touch_input.just_pressed(finger.id()) {
                app_state.show_kb = false;
                mouse_clicked.send(MouseClickedEvent {
                    cursor_position: finger.position(),
                });
            } else if touch_input.just_released(finger.id()) {
                app_state.debug_str = None;
            } else {
                /*
                app_state.debug_str = Some(format!("Touch: {} - {:?}", _index, finger.position()));
                let delta = finger.position() - finger.previous_position();
                app_state.debug_str = Some(format!("Dragged: {}, {:?}", _index, delta));
                mouse_dragged.send(MouseDraggedEvent { delta: delta });
                */
            }
        }
    }
}
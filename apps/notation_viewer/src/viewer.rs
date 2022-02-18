use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy::input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit};

use notation_bevy::bevy_egui::EguiContext;
use notation_bevy::prelude::*;
use notation_bevy::settings::layout_settings::LayoutMode;

use crate::help_panel::HelpPanel;

pub struct NotationViewer();

impl NotationViewer {
    fn extra(app: &mut App) {
        app.init_resource::<HelpPanel>();
        TabPlugin::setup_mouse_input(app);
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(Self::handle_keyboard_inputs)
                .with_system(Self::handle_mouse_inputs)
                .with_system(Self::handle_touch_inputs)
                .with_system(Self::load_tab)
                .with_system(HelpPanel::help_ui)
                .with_system(HelpPanel::handle_link_evts)
        );
    }
    pub fn run<A: ExtraAssets>(args: NotationArgs) {
        notation_bevy::prelude::NotationApp::run_with_extra::<A, _>(args, Self::extra);
    }
}

impl NotationViewer {
    fn load_tab(
        mut commands: Commands,
        time: Res<Time>,
        mut windows: ResMut<Windows>,
        mut state: ResMut<NotationState>,
        mut theme: ResMut<NotationTheme>,
        settings: Res<NotationSettings>,
        mut evts: EventWriter<AddTabEvent>,
        entities: Query<Entity, With<GlobalTransform>>,
        viewer_query: Query<(Entity, &TabViewer), With<TabViewer>>,
        asset_server: Res<AssetServer>,
        assets: Res<Assets<TabAsset>>,
    ) {
        NotationApp::load_tab(&mut commands, &time, &mut windows, &mut state, &mut theme, &settings, &mut evts, &entities, &viewer_query, |tab_path| {
            NotationApp::load_tab_from_assets(&asset_server, &assets, tab_path)
        })
    }
    pub fn handle_keyboard_inputs(
        keyboard_input: Res<Input<KeyCode>>,
        egui_ctx: Res<EguiContext>,
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
        if egui_ctx.ctx().wants_keyboard_input() {
            return;
        }
        if keyboard_input.just_released(KeyCode::F10) || keyboard_input.just_released(KeyCode::Backslash) {
            app_state.show_control = !app_state.show_control;
            if !ControlPanel::HUD_MODE {
                window_resized_evts.send(WindowResizedEvent::new(&app_state));
            }
        } else if keyboard_input.just_released(KeyCode::F1) || keyboard_input.just_released(KeyCode::H)
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
        } else if keyboard_input.just_released(KeyCode::F5) || keyboard_input.just_released(KeyCode::R)
        {
            app_state.bars_range = None;
            Control::reload_tab(&mut app_state, &mut theme);
        } else if keyboard_input.just_released(KeyCode::Space) {
            MidiControl::play_or_pause(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Return) {
            MidiControl::stop(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Home) {
            MidiControl::jump_to_section_start(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::End) {
            MidiControl::jump_to_section_end(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::PageUp) {
            MidiControl::jump_to_prev_section(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::PageDown) {
            MidiControl::jump_to_next_section(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::Left) {
            MidiControl::jump_to_prev_bar(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::Right) {
            MidiControl::jump_to_next_bar(&midi_state, &mut jump_to_bar_evts);
        } else if keyboard_input.just_released(KeyCode::Up) {
            MidiControl::jump_to_prev_row(&midi_state, &mut jump_to_bar_evts, &tab_bars_query);
        } else if keyboard_input.just_released(KeyCode::Down) {
            MidiControl::jump_to_next_row(&midi_state, &mut jump_to_bar_evts, &tab_bars_query);
        } else if keyboard_input.just_released(KeyCode::Grave) {
            MidiControl::seek_forward(&midi_settings, &mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Minus) {
            Control::toggle_layout_mode(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::S) {
            Control::toggle_show_note_syllable(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::P) {
            Control::toggle_show_note_pitch(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::F) {
            Control::toggle_always_show_fret(&mut app_state, &mut settings, &mut theme);
        } else if keyboard_input.just_released(KeyCode::L) {
            settings.should_loop = !settings.should_loop;
            MidiControl::sync_should_loop(&settings, &mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::A) {
            MidiControl::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::B) {
            MidiControl::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::C) {
            MidiControl::clear_begin_end(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::D) {
            MidiControl::set_begin_bar_ordinal(&mut midi_state, &mut play_control_evts);
            MidiControl::set_end_bar_ordinal(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::E) {
            MidiControl::set_begin_end_to_section(&mut midi_state, &mut play_control_evts);
        } else if keyboard_input.just_released(KeyCode::Key1) {
            MidiControl::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 0.25);
        } else if keyboard_input.just_released(KeyCode::Key2) {
            MidiControl::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 0.5);
        } else if keyboard_input.just_released(KeyCode::Key3) {
            MidiControl::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 0.75);
        } else if keyboard_input.just_released(KeyCode::Key4) {
            MidiControl::set_speed_factor(&mut settings, &mut midi_state, &mut play_control_evts, 1.0);
        }
    }

    pub fn handle_mouse_inputs(
        windows: Res<Windows>,
        mouse_input: Res<Input<MouseButton>>,
        egui_ctx: Res<EguiContext>,
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
        if egui_ctx.ctx().is_pointer_over_area() {
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

    pub fn handle_touch_inputs(
        windows: Res<Windows>,
        touch_input: Res<Touches>,
        egui_ctx: Res<EguiContext>,
        mut app_state: ResMut<NotationState>,
        mut mouse_clicked: EventWriter<MouseClickedEvent>,
        //mut mouse_dragged: EventWriter<MouseDraggedEvent>,
    ) {
        if app_state.tab.is_none() {
            return;
        }
        if egui_ctx.ctx().wants_pointer_input() {
            /* bevy_egui not supporting touch properly yet
            app_state.debug_str = Some(format!(
                "Touch: egui",
            ));
            return;
            */
        }
        for (_index, finger) in touch_input.iter().enumerate() {
            if touch_input.just_pressed(finger.id()) {
                windows
                    .get_primary()
                    .map(|w| (w.physical_width() as f32, w.physical_height() as f32))
                    .map(|(physical_width, physical_height)| {
                        /* bevy_egui not supporting touch properly yet */
                        app_state.show_kb = false;
                        /*
                        app_state.debug_str = Some(format!(
                            "Touch: {} {:?}",
                            _index,
                            finger.position(),
                        ));
                        */
                        mouse_clicked.send(MouseClickedEvent {
                            cursor_position: finger.position(),
                        });
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
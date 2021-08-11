use std::sync::Arc;

use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;
use bevy_egui::egui::{self, Slider};
use bevy_egui::EguiContext;
use float_eq::float_ne;
use notation_midi::prelude::{MidiState, PlayControlEvt};
use notation_model::play::play_control::TickResult;
use notation_model::prelude::Tab;

use crate::prelude::NotationSettings;

use super::notation_app_state::{NotationAppState, TabPathes};

pub fn reload_tab(
    commands: &mut Commands,
    app_state: &mut NotationAppState,
    tab_query: &Query<Entity, With<Arc<Tab>>>,
    get_cam: &mut Query<(&mut Transform, &mut OrthographicProjection)>,
) {
    for tab in tab_query.iter() {
        commands.entity(tab).despawn_recursive();
    }
    app_state.tab = None;
    let (mut cam, _) = get_cam.single_mut().unwrap();
    let trans = cam.translation;
    *cam = Transform::from_xyz(0.0, 0.0, trans.z);
}

pub fn sync_play_speed(
    settings: &NotationSettings,
    midi_state: &mut MidiState,
    play_control_evts: &mut EventWriter<PlayControlEvt>,
) {
    midi_state.play_control.play_speed = settings.play_speed;
    play_control_evts.send(PlayControlEvt::on_play_speed(settings.play_speed));
}

pub fn send_play_state_evt(
    play_control_evts: &mut EventWriter<PlayControlEvt>,
    midi_state: &MidiState,
) {
    play_control_evts.send(PlayControlEvt::on_play_state(
        midi_state.play_control.play_state,
    ));
    let tick_result = TickResult {
        changed: true,
        end_passed: false,
        stopped: midi_state.play_control.play_state.is_stopped(),
    };
    play_control_evts.send(PlayControlEvt::on_tick(
        midi_state.play_control.position,
        tick_result,
    ));
}

pub fn top_panel_ui(
    mut commands: Commands,
    egui_ctx: Res<EguiContext>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NotationAppState>,
    mut settings: ResMut<NotationSettings>,
    mut get_cam: Query<(&mut Transform, &mut OrthographicProjection)>,
    tab_query: Query<Entity, With<Arc<Tab>>>,
    tab_pathes: Res<TabPathes>,
    mut midi_state: ResMut<MidiState>,
    mut play_control_evts: EventWriter<PlayControlEvt>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        ui.horizontal(|ui| {
            ui.checkbox(&mut app_state.camera_panning, "Panning");
            let play_title = if midi_state.play_control.play_state.is_playing() {
                "Pause"
            } else {
                "Play"
            };
            if ui.button(play_title).clicked() {
                if midi_state.play_control.play_state.is_playing() {
                    if midi_state.play_control.pause() {
                        send_play_state_evt(&mut play_control_evts, &midi_state);
                    }
                } else {
                    if midi_state.play_control.play() {
                        send_play_state_evt(&mut play_control_evts, &midi_state);
                    }
                }
            }
            if !midi_state.play_control.play_state.is_stopped() {
                if ui.button("Stop").clicked() {
                    if midi_state.play_control.stop() {
                        send_play_state_evt(&mut play_control_evts, &midi_state);
                    }
                }
            }
            let play_speed = settings.play_speed;
            ui.add(Slider::new(&mut settings.play_speed, 0.1..=2.0).text("Play Speed"));
            if float_ne!(play_speed, settings.play_speed, abs <= 0.01) {
                sync_play_speed(&settings, &mut &mut midi_state, &mut play_control_evts)
            }
            let always_show_fret = settings.always_show_fret;
            ui.checkbox(&mut settings.always_show_fret, "Always Show Fret");
            if always_show_fret != settings.always_show_fret {
                reload_tab(&mut commands, &mut app_state, &tab_query, &mut get_cam);
            }
            if ui.button("Reload Tab").clicked() {
                reload_tab(&mut commands, &mut app_state, &tab_query, &mut get_cam);
            }
            if tab_pathes.0.len() > 1 {
                egui::ComboBox::from_label("Select Tab:")
                    .selected_text(app_state.tab_path.clone())
                    .show_ui(ui, |ui| {
                        for path in tab_pathes.0.iter() {
                            if ui
                                .selectable_label(*path == app_state.tab_path, path)
                                .clicked()
                            {
                                for tab in tab_query.iter() {
                                    commands.entity(tab).despawn_recursive();
                                }
                                app_state.change_tab(&asset_server, path.clone());
                            }
                        }
                    });
            }
        });
    });
}

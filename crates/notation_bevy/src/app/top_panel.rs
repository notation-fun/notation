use std::sync::Arc;

use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;
use bevy_egui::egui::{self, Slider};
use bevy_egui::EguiContext;
use float_eq::float_ne;
use notation_model::prelude::Tab;

use crate::prelude::{NotationSettings, TabState};

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
    _commands: &mut Commands,
    settings: &NotationSettings,
    tab_state_query: &mut Query<(Entity, &mut TabState)>,
) {
    for (_, mut tab_state) in tab_state_query.iter_mut() {
        tab_state.play_speed = settings.play_speed;
    }
}

pub fn top_panel_ui(
    mut commands: Commands,
    egui_ctx: Res<EguiContext>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NotationAppState>,
    mut settings: ResMut<NotationSettings>,
    mut tab_state_query: Query<(Entity, &mut TabState)>,
    mut get_cam: Query<(&mut Transform, &mut OrthographicProjection)>,
    tab_query: Query<Entity, With<Arc<Tab>>>,
    tab_pathes: Res<TabPathes>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        ui.horizontal(|ui| {
            ui.checkbox(&mut app_state.camera_panning, "Panning");
            if let Ok((tab_state_entity, mut tab_state)) = tab_state_query.single_mut() {
                let play_title = if tab_state.play_state.is_playing() {
                    "Pause"
                } else {
                    "Play"
                };
                if ui.button(play_title).clicked() {
                    if tab_state.play_state.is_playing() {
                        tab_state.pause(&mut commands, tab_state_entity);
                    } else {
                        tab_state.play(&mut commands, tab_state_entity);
                    }
                }
                if !tab_state.play_state.is_stopped() {
                    if ui.button("Stop").clicked() {
                        tab_state.stop(&mut commands, tab_state_entity);
                    }
                }
            }
            let play_speed = settings.play_speed;
            ui.add(Slider::new(&mut settings.play_speed, 0.1..=2.0).text("Play Speed"));
            if float_ne!(play_speed, settings.play_speed, abs <= 0.01) {
                sync_play_speed(&mut commands, &settings, &mut tab_state_query)
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

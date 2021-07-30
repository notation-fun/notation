use std::sync::Arc;

use bevy::prelude::*;
use bevy_egui::egui::{self, Slider};
use bevy_egui::EguiContext;
use notation_model::prelude::Tab;

use crate::prelude::{NotationSettings, TabState};

use super::notation_app_state::{NotationAppState, TabPathes};

pub fn top_panel_ui(
    mut commands: Commands,
    egui_ctx: Res<EguiContext>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NotationAppState>,
    mut settings: ResMut<NotationSettings>,
    mut query: Query<&mut TabState>,
    tab_query: Query<Entity, With<Arc<Tab>>>,
    tab_pathes: Res<TabPathes>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        ui.horizontal(|ui| {
            ui.checkbox(&mut app_state.camera_panning, "Panning");
            if let Ok(mut tab_state) = query.single_mut() {
                let play_title = if tab_state.play_state.is_playing() {
                    "Pause"
                } else {
                    "Play"
                };
                if ui.button(play_title).clicked() {
                    if tab_state.play_state.is_playing() {
                        tab_state.pause();
                    } else {
                        tab_state.play();
                    }
                }
                if !tab_state.play_state.is_stopped() {
                    if ui.button("Stop").clicked() {
                        tab_state.stop();
                    }
                }
            }
            ui.add(Slider::new(&mut settings.play_speed, 0.1..=2.0).text("Play Speed"));
            ui.checkbox(&mut settings.always_show_fret, "Always Show Fret");
            if ui.button("Reload Tab").clicked() {
                for tab in tab_query.iter() {
                    commands.entity(tab).despawn_recursive();
                }
                app_state.tab = None;
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

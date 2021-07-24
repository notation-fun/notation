use std::sync::Arc;

use bevy::prelude::*;
use bevy_egui::{EguiContext, egui::{self, Slider}};
use float_eq::float_ne;
use notation_model::prelude::Tab;

use crate::{config::bevy_config::{BevyConfig, BevyConfigAccessor}, prelude::TabState};

use super::app::AppState;

pub struct TopPanelState {
    playing: bool,
    stopped: bool,
    play_speed: f32,
    always_show_fret: bool,
}
impl TopPanelState {
    pub fn default() -> Self {
        Self {
            playing: false,
            stopped: false,
            play_speed: 1.0,
            always_show_fret: false,
        }
    }
}
impl FromWorld for TopPanelState {
    fn from_world(world: &mut World) -> Self {
        //TODO: get it from user preference, TabState is NOT a resource
        world.get_resource::<BevyConfig>()
            .map(|config|
                Self {
                    playing: false,
                    stopped: false,
                    play_speed: 1.0,
                    always_show_fret: config.theme.fretted.always_show_fret,
                }
            ).unwrap_or_else(TopPanelState::default)
    }
}

impl TopPanelState {
    pub fn sync_to_world(
        &self,
        config: &mut ResMut<BevyConfig>,
        query: &mut Query<&mut TabState>,
    ) {
        config.theme.fretted.always_show_fret = self.always_show_fret;
        for mut tab_state in query.iter_mut() {
            if self.stopped {
                tab_state.stop();
            } else if self.playing {
                tab_state.play();
            } else {
                tab_state.pause();
            }
            tab_state.play_speed = self.play_speed;
        }
    }
}

pub fn top_panel_ui(
    mut commands: Commands,
    egui_ctx: Res<EguiContext>,
    _assets: Res<AssetServer>,
    mut app_state: ResMut<AppState>,
    mut state: ResMut<TopPanelState>,
    mut config: ResMut<BevyConfig>,
    mut query: Query<&mut TabState>,
    tab_query: Query<Entity, With<Arc<Tab>>>,
) {
    let mut changed = false;
    let play_speed = state.play_speed;
    let always_show_fret = state.always_show_fret;
    egui::TopBottomPanel::top("top_panel")
        .show(egui_ctx.ctx(), |ui| {
            ui.horizontal(|ui| {
                let play_title = if state.stopped || !state.playing { "Play" } else { "Pause" };
                if ui.button(play_title).clicked() {
                    state.playing = state.stopped || !state.playing;
                    state.stopped = false;
                    changed = true;
                }
                if ui.button("Stop").clicked() {
                    state.stopped = true;
                    changed = true;
                }
                ui.add(
                    Slider::new(&mut state.play_speed, 0.1..=2.0)
                        .text("Play Speed"),
                );
                ui.checkbox(&mut state.always_show_fret, "Always Show Fret");
                if ui.button("Reload Tab").clicked() {
                    for tab in tab_query.iter() {
                        commands.entity(tab).despawn_recursive();
                    }
                    app_state.tab = None;
                }
            });
        });
    if !changed {
        if always_show_fret != state.always_show_fret
            || float_ne!(play_speed, state.play_speed, abs<=0.01) {
            changed = true;
        }
    }
    if changed {
        state.sync_to_world(&mut config, &mut query);
    }
}

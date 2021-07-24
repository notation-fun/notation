use bevy::prelude::*;
use bevy_egui::{EguiContext, egui::{self, Slider}};
use float_eq::float_ne;

use crate::prelude::TabState;

pub struct TopPanelState {
    playing: bool,
    stopped: bool,
    play_speed: f32,
}
impl TopPanelState {
    pub fn default() -> Self {
        Self {
            playing: false,
            stopped: false,
            play_speed: 1.0,
        }
    }
}
impl FromWorld for TopPanelState {
    fn from_world(world: &mut World) -> Self {
        world.get_resource::<TabState>()
            .map(|state|
                Self {
                    playing: state.play_state.is_playing(),
                    stopped: state.play_state.is_stopped(),
                    play_speed: state.play_speed,
                }

            ).unwrap_or_else(TopPanelState::default)
    }
}

impl TopPanelState {
    pub fn sync_tab_state(
        &self,
        query: &mut Query<&mut TabState>,
    ) {
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
    egui_ctx: Res<EguiContext>,
    _assets: Res<AssetServer>,
    mut state: ResMut<TopPanelState>,
    mut query: Query<&mut TabState>,
) {
    let mut changed = false;
    let play_speed = state.play_speed;
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
                    Slider::new(&mut state.play_speed, 0.1..=1.0)
                        .text("Play Speed"),
                );
            });
        });
    if !changed && float_ne!(play_speed, state.play_speed, abs<=0.01) {
        changed = true;
    }
    if changed {
        state.sync_tab_state(&mut query);
    }
}

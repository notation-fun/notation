use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::prelude::TabState;

pub struct TopPanelState {
    playing: bool,
    stopped: bool,
}
impl Default for TopPanelState {
    fn default() -> Self {
        Self {
            playing: false,
            stopped: false,
        }
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
            });
        });
    if changed {
        state.sync_tab_state(&mut query);
    }
}

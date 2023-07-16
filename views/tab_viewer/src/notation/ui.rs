//use edger_bevy_app::bevy::app::StartupStage;
use edger_bevy_app::bevy::{prelude::*, window::PrimaryWindow};
//use crate::bevy_egui::{EguiSettings};

use crate::prelude::{NotationState};

pub struct NotationUiPlugin;

impl Plugin for NotationUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_window_scale_factor);
    }
}

pub fn update_window_scale_factor(
    //mut _egui_settings: ResMut<EguiSettings>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut app_state: ResMut<NotationState>,
) {
    if let Ok(window) = window_query.get_single() {
        let scale_factor = window.scale_factor();
        if scale_factor != app_state.window_scale_factor {
            println!(
                "scale_factor changed:() {} -> {}",
                app_state.window_scale_factor, scale_factor
            );
            app_state.window_scale_factor = scale_factor;
            /*
             * egui_settings.scale_factor = 1.0 / scale_factor;
             */
        }
    }
}
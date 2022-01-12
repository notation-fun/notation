use bevy::app::StartupStage;
use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiSettings};

use super::state::NotationState;

pub struct NotationUiPlugin;

impl Plugin for NotationUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_window_scale_factor);
        app.add_startup_system_to_stage(StartupStage::Startup, setup_egui_fonts);
    }
}

pub fn update_window_scale_factor(
    mut _egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
    mut app_state: ResMut<NotationState>,
) {
    if let Some(window) = windows.get_primary() {
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

pub fn setup_egui_fonts(egui_ctx: Res<EguiContext>) {
    println!("setup_egui_fonts() ---------------------------------------");
    egui_ctx
        .ctx()
        .set_fonts(crate::font::egui_fonts::embedded_fonts(1.0));
}

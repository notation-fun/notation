use std::ops::Add;

use bevy::prelude::*;
use bevy_egui::{EguiSettings, EguiContext};

use crate::prelude::NotationAppState;

pub mod layout;
pub mod viewer;
pub struct NotationUiPlugin;

impl Plugin for NotationUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_window_scale_factor.system());
        app.add_system(update_egui_fonts.system());
    }
}

pub fn update_window_scale_factor(
    mut _egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
    mut app_state: ResMut<NotationAppState>,
) {
    if let Some(window) = windows.get_primary() {
        let scale_factor = window.scale_factor();
        if scale_factor != app_state.window_scale_factor {
            println!("scale_factor changed:() {} -> {}", app_state.window_scale_factor, scale_factor);
            app_state.window_scale_factor = scale_factor;
            /*
             * egui_settings.scale_factor = 1.0 / scale_factor;
             *
             * Seems the scale factor was taken care by bevy_egui? didn't check
             * source code yet, though it seems working right now.
             * One issue is that each time the scale factor changed (with 2 monitors * with different DPI, can trigger this when move window across them),
             * the newly created context is not having the updated fonts, so use a
             * hacky trick to apply it again.
             * Not that can't call set_font every frame either (even if there is no * performance penalty), will crash in case of egui context recreated.
             * there is still chances to crash in this case, current workaround is to hide
             * egui panels when need to set font, the drawback is a flicking for one frame
             * when scale_factor changed.
             */
            app_state._egui_needs_set_fonts = true;
        }
    }
}

pub fn update_egui_fonts(
    egui_ctx: Res<EguiContext>,
    mut app_state: ResMut<NotationAppState>,
) {
    if app_state._egui_needs_set_fonts {
        println!("update_egui_fonts() -----------------------");
        app_state._egui_needs_set_fonts = false;
        egui_ctx.ctx().set_fonts(crate::font::egui_fonts::embedded_fonts(1.0));
    }
}

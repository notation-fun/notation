use bevy::prelude::*;
use bevy_egui::EguiSettings;

pub mod layout;
pub mod viewer;
pub mod guitar;

pub struct NotationUiPlugin;

impl Plugin for NotationUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_ui_scale_factor.system());
    }
}

pub fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(_window) = windows.get_primary() {
        //setting scale_factor like this will make the app crash on windows for some reason
        //egui_settings.scale_factor = window.scale_factor();
        egui_settings.scale_factor = 1.0;
    }
}

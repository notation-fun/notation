use bevy::prelude::*;

use super::bevy_theme::BevyTheme;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<super::bevy_config::BevyConfig>()
            .insert_resource(ClearColor(BevyTheme::default().background_color));
    }
}

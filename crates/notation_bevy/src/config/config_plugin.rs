use bevy::prelude::*;

use super::core_theme::CoreTheme;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<super::bevy_config::BevyConfig>()
            .insert_resource(ClearColor(CoreTheme::default().background_color));
    }
}

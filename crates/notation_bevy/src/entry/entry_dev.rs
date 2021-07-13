use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;

pub struct EntryDevPlugin;

impl Plugin for EntryDevPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<crate::prelude::BevyConfig>::new())
            .add_startup_system(crate::tone::tone_dev::register_inspectors.system());
    }
}

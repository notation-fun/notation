use edger_bevy_app::bevy::app::{PluginGroup, PluginGroupBuilder};
use edger_bevy_app::bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use edger_bevy_app::prelude::ViewShapeDevPlugin;

pub struct NotationDevPlugins;
impl PluginGroup for NotationDevPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(ViewShapeDevPlugin::default());
        //external plugins
        group.add(LogDiagnosticsPlugin::default());
        group.add(FrameTimeDiagnosticsPlugin::default());
    }
}

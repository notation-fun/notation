use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_utils::dev::plugin::BevyUtilsPlugin;

pub struct NotationDevPlugins;
impl PluginGroup for NotationDevPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(BevyUtilsPlugin::default());
        //external plugins
        group.add(LogDiagnosticsPlugin::default());
        group.add(FrameTimeDiagnosticsPlugin::default());
    }
}

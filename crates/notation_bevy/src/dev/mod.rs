use bevy::app::{PluginGroup, PluginGroupBuilder};
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub struct NotationDevPlugins;
impl PluginGroup for NotationDevPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        //external plugins
        //group.add(LogDiagnosticsPlugin::default());
        //group.add(FrameTimeDiagnosticsPlugin::default());
    }
}

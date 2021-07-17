use bevy::prelude::*;
use bevy::window::WindowResized;

use super::bevy_config::BevyConfig;
use super::config_events::ConfigChangedEvent;
use super::core_theme::CoreTheme;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ConfigChangedEvent>();
        app.init_resource::<BevyConfig>();
        app.insert_resource(ClearColor(CoreTheme::default().background_color));
        app.add_startup_system(setup_window_size.system());
        app.add_system(on_window_resized.system());
    }
}

impl ConfigPlugin {
    pub fn insert_window_descriptor(app: &mut AppBuilder, title: String) {
        app.insert_resource(WindowDescriptor {
            title,
            ..WindowDescriptor::default()
        });
    }
}

fn setup_window_size(window: Res<WindowDescriptor>, mut config: ResMut<BevyConfig>) {
    config.grid.resize(window.width, window.height);
}

fn on_window_resized(
    mut window: ResMut<WindowDescriptor>,
    mut evts: EventReader<WindowResized>,
    mut config: ResMut<BevyConfig>,
    mut config_evts: EventWriter<ConfigChangedEvent>,
) {
    for evt in evts.iter() {
        if evt.width as usize != window.width as usize
            || evt.height as usize != window.height as usize
        {
            window.width = evt.width;
            window.height = evt.height;
            config.grid.resize(evt.width, evt.height);
            config_evts.send(ConfigChangedEvent());
        }
    }
}

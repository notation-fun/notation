use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<super::grid_config::GridConfig>()
        ;
    }
}

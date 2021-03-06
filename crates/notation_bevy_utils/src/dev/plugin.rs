use crate::prelude::*;
use bevy::prelude::*;

use super::theme::BevyUtilsTheme;

#[derive(Debug, Default)]
pub struct BevyUtilsPlugin;

impl Plugin for BevyUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BevyUtilsTheme>();
        app.add_system(on_add_layout_data);
        app.add_system(on_layout_data_changed);
    }
}

fn on_add_layout_data(
    mut commands: Commands,
    theme: Res<BevyUtilsTheme>,
    layout_query: Query<(Entity, &LayoutData), Added<LayoutData>>,
) {
    for (entity, layout) in layout_query.iter() {
        layout.create(&mut commands, &theme, entity);
    }
}

fn on_layout_data_changed(
    mut commands: Commands,
    theme: Res<BevyUtilsTheme>,
    layout_query: Query<(Entity, &LayoutData), Changed<LayoutData>>,
) {
    for (entity, layout) in layout_query.iter() {
        layout.update(&mut commands, &theme, entity);
    }
}

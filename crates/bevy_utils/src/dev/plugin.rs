use crate::prelude::*;
use bevy::prelude::*;

use super::layout::LayoutShape;
use super::theme::BevyUtilsTheme;

#[derive(Debug, Default)]
pub struct BevyUtilsPlugin;

impl Plugin for BevyUtilsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<BevyUtilsTheme>();
        app.add_system(on_add_layout_data.system());
        app.add_system(on_layout_data_changed.system());
    }
}

fn on_add_layout_data(
    mut commands: Commands,
    theme: Res<BevyUtilsTheme>,
    layout_query: Query<(Entity, &LayoutData), Added<LayoutData>>,
) {
    for (entity, layout) in layout_query.iter() {
        LayoutShape::create(&mut commands, &theme, entity, *layout);
    }
}

fn on_layout_data_changed(
    mut commands: Commands,
    theme: Res<BevyUtilsTheme>,
    layout_query: Query<(Entity, &LayoutData), Changed<LayoutData>>,
) {
    for (entity, layout) in layout_query.iter() {
        LayoutShape::update(&mut commands, &theme, entity, &layout);
    }
}

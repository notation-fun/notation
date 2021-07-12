use std::sync::Arc;

use bevy::prelude::*;
use notation_model::prelude::Tab;

use crate::prelude::{AddTabEvent, BarBundle, BevyConfig, ConfigChangedEvent};

use super::tab_bundle::TabBundle;

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddTabEvent>();
        app.add_system(on_add_tab.system());
        app.add_system(on_config_changed.system());
    }
}

fn on_config_changed(
    mut evts: EventReader<ConfigChangedEvent>,
    config: Res<BevyConfig>,
    mut query: Query<(&Arc<Tab>, &mut Transform)>,
) {
    for _evt in evts.iter() {
        for (tab, mut transform) in query.iter_mut() {
            *transform = config.grid.calc_tab_transform(&tab.meta.signature);
        }
    }
}

fn on_add_tab(mut commands: Commands, config: Res<BevyConfig>, mut evts: EventReader<AddTabEvent>) {
    for evt in evts.iter() {
        let tab = evt.0.clone();
        let tab_entity = commands
            .spawn_bundle(TabBundle::new(&config, tab.clone()))
            .id();
        for bar in tab.bars.iter() {
            let bar_bundle = BarBundle::new(bar.clone(), &config.grid);
            let bar_entity = commands.spawn_bundle(bar_bundle).id();
            commands.entity(tab_entity).push_children(&[bar_entity]);
        }
    }
}

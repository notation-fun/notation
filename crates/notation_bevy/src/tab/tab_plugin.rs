use bevy::prelude::*;

use crate::prelude::{AddTabEvent, BarBundle, BevyConfig};

use super::tab_bundle::TabBundle;

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddTabEvent>()
            .add_system(on_add_tab.system());
    }
}

fn on_add_tab(mut commands: Commands, config: Res<BevyConfig>, mut evts: EventReader<AddTabEvent>) {
    for evt in evts.iter() {
        let tab = evt.0.clone();
        let tab_entity = commands.spawn_bundle(TabBundle::from(tab.clone())).id();
        for bar in tab.bars.iter() {
            let bar_bundle = BarBundle::new(bar.clone(), &config.grid);
            let bar_entity = commands.spawn_bundle(bar_bundle).id();
            commands.entity(tab_entity).push_children(&[bar_entity]);
        }
    }
}

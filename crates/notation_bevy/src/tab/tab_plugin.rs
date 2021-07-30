use std::sync::Arc;

use bevy::prelude::*;
use notation_model::prelude::Tab;

use crate::prelude::{
    AddTabEvent, BarBundle, NotationSettings, NotationTheme, TabAsset, WindowResizedEvent,
};

use super::tab_asset::TabAssetLoader;
use super::tab_bundle::TabBundle;

use super::tab_state_bundle::TabStateBundle;

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddTabEvent>();
        app.add_asset::<TabAsset>();
        app.init_asset_loader::<TabAssetLoader>();
        app.add_system(on_add_tab.system());
        app.add_system(on_config_changed.system());
    }
}

fn on_config_changed(
    mut evts: EventReader<WindowResizedEvent>,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut query: Query<(&Arc<Tab>, &mut Transform)>,
) {
    for _evt in evts.iter() {
        for (_tab, mut transform) in query.iter_mut() {
            *transform = theme.grid.calc_tab_transform(&settings);
        }
    }
}

fn on_add_tab(
    mut commands: Commands,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut evts: EventReader<AddTabEvent>,
) {
    for evt in evts.iter() {
        let tab = evt.0.clone();
        let tab_entity = commands
            .spawn_bundle(TabBundle::new(&settings, &theme, tab.clone()))
            .id();
        let state_entity = commands
            .spawn_bundle(TabStateBundle::new(&settings, &theme, tab.clone()))
            .id();
        commands.entity(tab_entity).push_children(&[state_entity]);
        for bar in tab.bars.iter() {
            let bar_bundle = BarBundle::new(&settings, &theme, bar.clone());
            let bar_entity = commands.spawn_bundle(bar_bundle).id();
            commands.entity(tab_entity).push_children(&[bar_entity]);
        }
    }
}

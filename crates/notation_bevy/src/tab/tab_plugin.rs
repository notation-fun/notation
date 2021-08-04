use std::sync::Arc;

use bevy::prelude::*;
use notation_model::prelude::Tab;

use crate::prelude::{AddEntryEvent, AddTabEvent, BarBundle, BarLayout, BarPlugin, NotationSettings, NotationTheme, TabAsset, WindowResizedEvent};

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
    mut add_entry_evts: EventWriter<AddEntryEvent>,
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
        let bar_bundles: Vec<(BarLayout, BarBundle)> = tab.bars.iter()
            .map(|bar|{
                let bar_layout = settings.layout.calc_bar_layout(&bar);
                let transform = theme.grid.calc_bar_transform(&bar_layout);
                (bar_layout, BarBundle::new(bar.clone(), bar_layout, transform))
            }).collect();
        for (bar_layout, bar_bundle) in bar_bundles.into_iter() {
            let bar = bar_bundle.bar.clone();
            let bar_entity = commands.spawn_bundle(bar_bundle).id();
            commands.entity(tab_entity).push_children(&[bar_entity]);
            BarPlugin::create_lanes(&mut commands, &theme, bar_entity, bar, &bar_layout, &mut add_entry_evts);
        }
    }
}

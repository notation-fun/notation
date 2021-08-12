use std::sync::Arc;

use bevy::prelude::*;
use notation_midi::prelude::SwitchTabEvent;
use notation_model::prelude::Tab;

use crate::prelude::{AddEntryEvent, AddTabEvent, BarBundle, BarLayout, BarPlugin, NotationAppState, NotationSettings, NotationTheme, SingleBundle, TabAsset, WindowResizedEvent};

use super::tab_asset::TabAssetLoader;

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
    app_state: Res<NotationAppState>,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut query: Query<(&Arc<Tab>, &mut Transform)>,
) {
    for _evt in evts.iter() {
        for (_tab, mut transform) in query.iter_mut() {
            *transform = theme.grid.calc_tab_transform(&app_state, &settings);
        }
    }
}

fn new_tab_bundle(app_state: &NotationAppState, settings: &NotationSettings, theme: &NotationTheme, tab: Arc<Tab>) -> SingleBundle<Arc<Tab>> {
    let transform = theme.grid.calc_tab_transform(app_state, settings);
    (tab, transform).into()
}

fn on_add_tab(
    mut commands: Commands,
    app_state: Res<NotationAppState>,
    settings: Res<NotationSettings>,
    theme: Res<NotationTheme>,
    mut evts: EventReader<AddTabEvent>,
    mut add_entry_evts: EventWriter<AddEntryEvent>,
    mut switch_tab_evts: EventWriter<SwitchTabEvent>,
) {
    for evt in evts.iter() {
        let tab = evt.0.clone();
        let bar_layouts = settings.layout.calc_bar_layouts(&app_state, &tab);
        let tab_entity = commands
            .spawn_bundle(new_tab_bundle(&app_state, &settings, &theme, tab.clone()))
            .id();
        let state_entity = commands
            .spawn_bundle(TabStateBundle::new(
                tab.clone(),
                Arc::new(bar_layouts.clone()),
            ))
            .id();
        commands.entity(tab_entity).push_children(&[state_entity]);
        let bar_bundles: Vec<(&BarLayout, BarBundle)> = tab
            .bars
            .iter()
            .enumerate()
            .filter_map(|(index, bar)| {
                bar_layouts.get(index).map(|bar_layout| {
                    let transform = theme.grid.calc_bar_transform(&bar_layout);
                    (
                        bar_layout,
                        BarBundle::new(bar.clone(), bar_layout.clone(), transform),
                    )
                })
            })
            .collect();
        for (bar_layout, bar_bundle) in bar_bundles.into_iter() {
            let bar = bar_bundle.bar.clone();
            let bar_entity = commands.spawn_bundle(bar_bundle).id();
            commands.entity(tab_entity).push_children(&[bar_entity]);
            BarPlugin::create_lanes(
                &mut commands,
                &app_state,
                &settings,
                &theme,
                bar_entity,
                bar,
                &bar_layout,
                &mut add_entry_evts,
            );
        }
        switch_tab_evts.send(SwitchTabEvent::new(evt.0.clone()));
    }
}

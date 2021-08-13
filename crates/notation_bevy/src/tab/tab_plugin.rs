use std::sync::Arc;

use bevy::prelude::*;
use notation_midi::prelude::SwitchTabEvent;
use notation_model::prelude::Tab;

use crate::mini::mini_plugin::MiniPlugin;
use crate::prelude::{
    AddEntryEvent, AddTabEvent, BarBundle, BarLayout, BarPlugin, BevyUtil, NotationAppState,
    NotationSettings, NotationTheme, PlayPlugin, SingleBundle, TabAsset, TabBars,
    WindowResizedEvent,
};

use super::tab_asset::TabAssetLoader;

use super::tab_bundle::TabBundle;

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
        let bar_layouts = Arc::new(settings.layout.calc_bar_layouts(&app_state, &tab));
        let transform = theme.grid.calc_tab_transform(&app_state, &settings);
        let tab_entity = commands
            .spawn_bundle(TabBundle::new(tab.clone(), bar_layouts.clone(), transform))
            .id();
        MiniPlugin::spawn_mini_map(
            &mut commands,
            &app_state,
            &settings,
            &theme,
            tab_entity,
            &tab,
        );
        let bars_entity = BevyUtil::spawn_child_bundle(
            &mut commands,
            tab_entity,
            SingleBundle::<TabBars>::from(TabBars::new(tab.clone())),
        );
        PlayPlugin::spawn_pos_indicator(
            &mut commands,
            &theme,
            bars_entity,
            &tab,
            bar_layouts.get(0),
        );
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
            let bar_entity = BevyUtil::spawn_child_bundle(&mut commands, bars_entity, bar_bundle);
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

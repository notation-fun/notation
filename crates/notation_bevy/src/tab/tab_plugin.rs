use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::{DockView, LayoutAnchor, LayoutConstraint, LayoutQuery, View, ViewBundle, ViewQuery, ViewRootAddedQuery, ViewRootQuery};
use notation_midi::prelude::SwitchTabEvent;

use crate::mini::mini_map::MiniMap;
use crate::mini::mini_plugin::MiniPlugin;
use crate::prelude::{AddEntryEvent, AddTabEvent, BarBundle, BarLayout, BarPlugin, BevyUtil, NotationAppState, NotationLabels, NotationSettings, NotationTheme, PlayPlugin, TabAsset, TabBars, WindowResizedEvent};
use crate::ui::layout::NotationLayout;

use super::tab_asset::TabAssetLoader;

use super::tab_bundle::TabBundle;
use super::tab_view::TabView;

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddTabEvent>();
        app.add_asset::<TabAsset>();
        app.init_asset_loader::<TabAssetLoader>();
        app.add_system(on_add_tab.system());
        app.add_system(on_tab_view_added.system());
        app.add_system(on_config_changed.system().label(NotationLabels::TAB));
    }
}

fn do_layout_tab_view(
    engine: &NotationLayout,
    layout_query: &mut LayoutQuery,
    mini_map_query: &ViewQuery<MiniMap>,
    bars_query: &ViewQuery<TabBars>,
    entity: Entity,
    view: &TabView,
) {
    let constraint = LayoutConstraint::from((engine.state.window_width, engine.state.window_height));
    let layout = view.calc_root_layout(engine, constraint, LayoutAnchor::CENTER);
    view.do_layout(&engine,
        layout_query, mini_map_query, bars_query,
        entity, layout)
}

fn on_tab_view_added(
    theme: Res<NotationTheme>,
    state: Res<NotationAppState>,
    settings: Res<NotationSettings>,
    view_query: ViewRootAddedQuery<TabView>,
    mut transform_query: LayoutQuery,
    mini_map_query: ViewQuery<MiniMap>,
    bars_query: ViewQuery<TabBars>,
) {
    let engine = NotationLayout::new(&theme, &state, &settings);
    for (entity, view) in view_query.iter() {
        do_layout_tab_view(&engine, &mut transform_query, &mini_map_query, &bars_query, entity, view);
    }
}

fn on_config_changed(
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    state: Res<NotationAppState>,
    settings: Res<NotationSettings>,
    view_query: ViewRootQuery<TabView>,
    mut layout_query: LayoutQuery,
    mini_map_query: ViewQuery<MiniMap>,
    bars_query: ViewQuery<TabBars>,
) {
    for _evt in evts.iter() {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for (entity, view) in view_query.iter() {
            do_layout_tab_view(&engine, &mut layout_query, &mini_map_query, &bars_query, entity, view);
        }
    }
}

fn on_add_tab(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    state: Res<NotationAppState>,
    settings: Res<NotationSettings>,
    mut evts: EventReader<AddTabEvent>,
    mut add_entry_evts: EventWriter<AddEntryEvent>,
    mut switch_tab_evts: EventWriter<SwitchTabEvent>,
) {
    for evt in evts.iter() {
        let tab = evt.0.clone();
        let bar_layouts = Arc::new(settings.layout.calc_bar_layouts(&state, &tab));
        //let transform = theme.grid.calc_tab_transform(&state, &settings);
        let transform = Transform::default();
        let tab_entity = commands
            .spawn_bundle(TabBundle::new(tab.clone(), bar_layouts.clone(), transform))
            .id();
        MiniPlugin::spawn_mini_map(
            &mut commands,
            &theme,
            &state,
            &settings,
            tab_entity,
            &tab,
        );
        let bars_entity = BevyUtil::spawn_child_bundle(
            &mut commands,
            tab_entity,
            ViewBundle::from(TabBars::new(tab.clone())),
        );
        PlayPlugin::spawn_indicators(&mut commands, &theme, bars_entity, &tab, bar_layouts.get(0));
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
                &state,
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

use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::{LayoutQuery, ViewBundle, ViewQuery, ViewRootQuery};
use notation_midi::prelude::SwitchTabEvent;

use crate::mini::mini_map::MiniMap;

use crate::prelude::{
    AddTabEvent, BevyUtil, NotationAppState, NotationSettings, NotationTheme, TabAsset, TabBars,
    WindowResizedEvent,
};
use crate::ui::layout::NotationLayout;

use super::tab_asset::TabAssetLoader;

use super::tab_bundle::TabBundle;
use super::tab_chords::TabChords;
use super::tab_content::TabContent;
use super::tab_events::{
    TabBarsDoLayoutEvent, TabChordsDoLayoutEvent, TabContentDoLayoutEvent, TabResizedEvent,
};
use super::tab_view::TabView;

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        TabContentDoLayoutEvent::setup(app);
        TabChordsDoLayoutEvent::setup(app);
        TabBarsDoLayoutEvent::setup(app);
        app.add_event::<AddTabEvent>();
        app.add_event::<TabResizedEvent>();
        app.add_asset::<TabAsset>();
        app.init_asset_loader::<TabAssetLoader>();
        app.add_system(on_add_tab.system());
        app.add_system(on_window_resized.system());
        app.add_system(TabView::on_added.system());
        app.add_system(TabContent::do_layout.system());
        app.add_system(TabChords::on_added.system());
        app.add_system(TabChords::do_layout.system());
        app.add_system(TabBars::on_added.system());
        app.add_system(TabBars::do_layout.system());
    }
}

fn on_window_resized(
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    state: Res<NotationAppState>,
    settings: Res<NotationSettings>,
    view_query: ViewRootQuery<TabView>,
    mut layout_query: LayoutQuery,
    mini_map_query: ViewQuery<MiniMap>,
    content_query: ViewQuery<TabContent>,
) {
    for _evt in evts.iter() {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for (entity, view) in view_query.iter() {
            TabView::do_layout(
                &engine,
                &mut layout_query,
                &mini_map_query,
                &content_query,
                entity,
                view,
            );
        }
    }
}

fn on_add_tab(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    state: Res<NotationAppState>,
    settings: Res<NotationSettings>,
    mut evts: EventReader<AddTabEvent>,
    mut switch_tab_evts: EventWriter<SwitchTabEvent>,
    mut layout_query: LayoutQuery,
    mini_map_query: ViewQuery<MiniMap>,
    content_query: ViewQuery<TabContent>,
) {
    for evt in evts.iter() {
        let tab = evt.0.clone();
        let tab_bundle = TabBundle::new(tab.clone());
        let tab_view = tab_bundle.view.clone();
        let tab_entity = commands.spawn_bundle(tab_bundle).id();
        MiniMap::spawn(&mut commands, &theme, tab_entity, &tab);
        let content_entity = BevyUtil::spawn_child_bundle(
            &mut commands,
            tab_entity,
            ViewBundle::from(TabContent::new(tab.clone())),
        );
        BevyUtil::spawn_child_bundle(
            &mut commands,
            content_entity,
            ViewBundle::from(TabChords::new(tab.clone())),
        );
        let bar_layouts = TabBars::calc_bar_layouts(&theme, &settings, &tab);
        BevyUtil::spawn_child_bundle(
            &mut commands,
            content_entity,
            ViewBundle::from(TabBars::new(tab.clone(), Arc::new(bar_layouts))),
        );
        switch_tab_evts.send(SwitchTabEvent::new(evt.0.clone()));
        let engine = NotationLayout::new(&theme, &state, &settings);
        TabView::do_layout(
            &engine,
            &mut layout_query,
            &mini_map_query,
            &content_query,
            tab_entity,
            &tab_view,
        );
    }
}

use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::{LayoutData, LayoutQuery, ViewBundle, ViewQuery, ViewRootQuery};
use notation_midi::prelude::{JumpToBarEvent, SwitchTabEvent};

use crate::mini::mini_bar::MiniBar;
use crate::mini::mini_map::MiniMap;

use crate::prelude::{AddTabEvent, BevyUtil, MouseClickedEvent, MouseDraggedEvent, NotationAppState, NotationAssetsStates, NotationSettings, NotationTheme, TabAsset, TabBars, WindowResizedEvent};
use crate::ui::layout::NotationLayout;

use super::tab_asset::TabAssetLoader;

use super::tab_bundle::TabBundle;
use super::tab_chords::TabChords;
use super::tab_content::TabContent;
use super::tab_events::{TabBarsDoLayoutEvent, TabBarsResizedEvent, TabChordsDoLayoutEvent, TabContentDoLayoutEvent};
use super::tab_view::TabView;

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut AppBuilder) {
        TabContentDoLayoutEvent::setup(app);
        TabChordsDoLayoutEvent::setup(app);
        TabBarsDoLayoutEvent::setup(app);
        app.add_event::<AddTabEvent>();
        app.add_event::<TabBarsResizedEvent>();
        app.add_asset::<TabAsset>();
        app.init_asset_loader::<TabAssetLoader>();
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(on_add_tab.system())
                .with_system(on_window_resized.system())
                .with_system(on_mouse_clicked.system())
                .with_system(on_mouse_dragged.system())
                .with_system(TabView::on_added.system())
                .with_system(TabContent::do_layout.system())
                .with_system(TabChords::on_added.system())
                .with_system(TabChords::do_layout.system())
                .with_system(TabBars::on_added.system())
                .with_system(TabBars::do_layout.system()),
        );
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

fn on_mouse_clicked(
    mut evts: EventReader<MouseClickedEvent>,
    _theme: Res<NotationTheme>,
    state: Res<NotationAppState>,
    _settings: Res<NotationSettings>,
    mini_bar_query: Query<(&Arc<MiniBar>, &LayoutData, &GlobalTransform)>,
    mut jump_to_bar_evts: EventWriter<JumpToBarEvent>,
) {
    for evt in evts.iter() {
        let pos = TabView::convert_pos(&state,evt.cursor_position);
        println!("tab_plugin::on_mouse_clicked() -> {:?} -> {:?}", evt, pos);
        for (mini_bar, layout, global_transform) in mini_bar_query.iter() {
            let offset = pos - Vec2::new(global_transform.translation.x, global_transform.translation.y);
            if layout.is_inside(offset) {
                jump_to_bar_evts.send(JumpToBarEvent::new(mini_bar.bar_props));
            }
        }
    }
}

fn on_mouse_dragged(
    mut evts: EventReader<MouseDraggedEvent>,
    settings: Res<NotationSettings>,
    mut tab_bars_query: Query<(Entity, &mut Transform, &Arc<TabBars>)>,
) {
    for evt in evts.iter() {
        if settings.mouse_dragged_panning {
            settings
                .layout
                .pan_tab_bars(&mut tab_bars_query, -evt.delta.x, -evt.delta.y);
        }
    }
}

fn on_add_tab(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    settings: Res<NotationSettings>,
    mut evts: EventReader<AddTabEvent>,
    mut switch_tab_evts: EventWriter<SwitchTabEvent>,
) {
    for evt in evts.iter() {
        let tab = evt.0.clone();
        let tab_bundle = TabBundle::new(tab.clone());
        //let tab_view = tab_bundle.view.clone();
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
    }
}

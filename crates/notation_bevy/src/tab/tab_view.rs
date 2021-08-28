use bevy::prelude::*;
use std::fmt::Display;
use std::sync::Arc;

use bevy_utils::prelude::{BevyUtil, DockView, LayoutQuery, View, ViewBundle, ViewQuery};
use notation_model::prelude::Tab;

use crate::mini::mini_map::MiniMap;
use crate::prelude::{NotationAppState, NotationAssets, NotationSettings, NotationTheme, TabBars, TabBundle};
use crate::ui::layout::NotationLayout;

use super::tab_chords::TabChords;
use super::tab_content::TabContent;
use super::tab_events::TabViewDoLayoutEvent;

pub struct TabView {
    pub tab: Arc<Tab>,
}
impl Display for TabView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabView>({})", self.tab.bars.len())
    }
}
impl TabView {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}
impl<'a> View<NotationLayout<'a>> for TabView {}
impl<'a> DockView<NotationLayout<'a>, MiniMap, TabContent> for TabView {}

impl TabView {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let tab_bundle = TabBundle::new(tab.clone());
        //let tab_view = tab_bundle.view.clone();
        let tab_entity = BevyUtil::spawn_child_bundle(
            commands,
            entity,
            tab_bundle,
        );
        MiniMap::spawn(commands, theme, tab_entity, &tab);
        let content_entity = BevyUtil::spawn_child_bundle(
            commands,
            tab_entity,
            ViewBundle::from(TabContent::new(tab.clone())),
        );
        TabChords::spawn(commands, theme, content_entity, &tab);
        TabBars::spawn(commands, assets, theme, &settings, content_entity, &tab);
        tab_entity
    }
    pub fn do_layout(
        mut evts: EventReader<TabViewDoLayoutEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<MiniMap>,
        content_query: ViewQuery<TabContent>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
                &engine,
                &mut layout_query,
                &panel_query,
                &content_query,
                evt.entity,
                evt.layout,
            )
        }
    }
}

use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_utils::prelude::{DockView, LayoutChangedQuery, LayoutQuery, View, ViewQuery};
use notation_model::prelude::Tab;

use crate::prelude::{NotationAppState, NotationSettings, NotationTheme, TabBars};
use crate::ui::layout::NotationLayout;

use super::tab_chords::TabChords;
use super::tab_events::TabContentDoLayoutEvent;

pub struct TabContent {
    pub tab: Arc<Tab>,
}
impl Display for TabContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabContent>({})", self.tab.bars.len())
    }
}
impl TabContent {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}
impl<'a> View<NotationLayout<'a>> for TabContent {
}
impl<'a> DockView<NotationLayout<'a>, TabChords, TabBars> for TabContent {}

impl TabContent {
    pub fn do_layout(
        mut evts: EventReader<TabContentDoLayoutEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<TabChords>,
        content_query: ViewQuery<TabBars>,
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
    pub fn on_layout_changed(
        query: LayoutChangedQuery<TabContent>,
        mut evts: EventWriter<TabContentDoLayoutEvent>,
    ) {
        for (entity, view, layout) in query.iter() {
            println!("TabContent::on_layout_changed({})", layout);
            evts.send(TabContentDoLayoutEvent::new(entity, view, layout))
        }
    }
}

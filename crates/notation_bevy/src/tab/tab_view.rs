use bevy::prelude::*;
use std::fmt::Display;
use std::sync::Arc;

use bevy_utils::prelude::{
    DockView, LayoutAnchor, LayoutConstraint, LayoutQuery, View, ViewQuery, ViewRootAddedQuery,
};
use notation_model::prelude::Tab;

use crate::mini::mini_map::MiniMap;
use crate::prelude::{NotationAppState, NotationSettings, NotationTheme};
use crate::ui::layout::NotationLayout;

use super::tab_content::TabContent;

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
impl<'a> View<NotationLayout<'a>> for TabView {
    fn is_root(&self) -> bool {
        true
    }
}
impl<'a> DockView<NotationLayout<'a>, MiniMap, TabContent> for TabView {}

impl TabView {
    pub fn do_layout(
        engine: &NotationLayout,
        layout_query: &mut LayoutQuery,
        panel_query: &ViewQuery<MiniMap>,
        content_query: &ViewQuery<TabContent>,
        entity: Entity,
        view: &TabView,
    ) {
        let constraint =
            LayoutConstraint::from((engine.state.window_width, engine.state.window_height));
        let layout = view.calc_root_layout(engine, constraint, LayoutAnchor::CENTER);
        view.do_layout(
            &engine,
            layout_query,
            panel_query,
            content_query,
            entity,
            layout,
        )
    }
    pub fn on_added(
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        view_query: ViewRootAddedQuery<TabView>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<MiniMap>,
        content_query: ViewQuery<TabContent>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for (entity, view) in view_query.iter() {
            TabView::do_layout(
                &engine,
                &mut layout_query,
                &panel_query,
                &content_query,
                entity,
                view,
            );
        }
    }
}

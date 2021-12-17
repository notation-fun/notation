use bevy::prelude::*;
use notation_model::prelude::Tab;
use std::fmt::Display;
use std::sync::Arc;

use notation_bevy_utils::prelude::{
    BevyUtil, ColorBackground, DoLayoutEvent, DockView, LayoutQuery, View, ViewBundle, ViewQuery,
};

use crate::mini::mini_map::MiniMap;
use crate::prelude::{
    GuitarView, NotationAppState, NotationAssets, NotationAssetsStates, NotationSettings,
    NotationTheme,
};
use crate::tab::tab_view::TabView;
use crate::ui::layout::NotationLayout;

pub struct TabViewer {
    pub tab: Arc<Tab>,
}
impl Display for TabViewer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabViewer>({})", self.tab.bars.len())
    }
}
impl TabViewer {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}
impl<'a> View<NotationLayout<'a>> for TabViewer {}
impl<'a> DockView<NotationLayout<'a>, MiniMap, TabView> for TabViewer {}

pub type TabViewerDoLayoutEvent = DoLayoutEvent<NotationLayout<'static>, TabViewer>;

pub struct TabViewerPlugin;

impl Plugin for TabViewerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        ColorBackground::setup(app);
        TabViewerDoLayoutEvent::setup(app);
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(GuitarView::on_layout_changed.system())
                .with_system(GuitarView::update_hand_shape6.system().label("GuitarView::update_hand_shape6"))
                .with_system(GuitarView::update_string_state.system().label("GuitarView::update_string_state").after("GuitarView::update_hand_shape6"))
                .with_system(GuitarView::adjust_y_by_capo.system())
                .with_system(TabViewer::do_layout.system()),
        );
    }
}

impl TabViewer {
    pub fn spawn(
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let viewer_bundle = ViewBundle::from(TabViewer::new(tab.clone()));
        let viewer_entity = BevyUtil::spawn_child_bundle(commands, entity, viewer_bundle);
        MiniMap::spawn(commands, assets, theme, viewer_entity, &tab);
        TabView::spawn(commands, materials, assets, theme, settings, viewer_entity, tab);
        viewer_entity
    }
    pub fn do_layout(
        mut evts: EventReader<TabViewerDoLayoutEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<MiniMap>,
        content_query: ViewQuery<TabView>,
    ) {
        if theme._bypass_systems { return; }
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
                &engine,
                &mut layout_query,
                &panel_query,
                &content_query,
                evt.entity,
                evt.layout,
            );
        }
    }
}

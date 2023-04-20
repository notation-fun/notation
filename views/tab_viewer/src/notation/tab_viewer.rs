use bevy::prelude::*;
use notation_model::prelude::{Tab, SwitchTabEvent};
use std::fmt::Display;
use std::sync::Arc;

use notation_bevy_utils::prelude::{
    ColorBackground, DockView, LayoutQuery, View, ViewBundle, ViewQuery, LayoutConstraint, ViewRootQuery, ViewRootAddedQuery,
};

use crate::mini::mini_map::MiniMap;
use crate::prelude::{
    GuitarView, NotationState, NotationAssets, NotationAssetsStates, NotationSettings,
    NotationTheme, AddTabEvent,
};
use crate::tab::tab_view::TabView;
use crate::prelude::NotationLayout;

use super::events::WindowResizedEvent;

#[derive(Clone, Debug, Component)]
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

pub struct TabViewerPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum GuitarViewLabel {
    UpdateHandShapes,
    UpdateStringStates,
}

impl Plugin for TabViewerPlugin {
    fn build(&self, app: &mut App) {
        ColorBackground::setup(app);
        app.add_systems((
            GuitarView::on_layout_changed,
            GuitarView::update_hand_shape6
                    .in_set(GuitarViewLabel::UpdateHandShapes),
            GuitarView::update_string_state
                    .in_set(GuitarViewLabel::UpdateStringStates)
                    .after(GuitarViewLabel::UpdateHandShapes),
            GuitarView::adjust_y_by_barre,
        ).in_set(OnUpdate(NotationAssetsStates::Loaded)));
    }
}

impl TabViewer {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        tab: &Arc<Tab>,
    ) -> Entity {
        let viewer_bundle = ViewBundle::from(TabViewer::new(tab.clone()));
        let viewer_entity = commands.spawn(viewer_bundle).id();
        MiniMap::spawn(commands, assets, theme, settings, viewer_entity, &tab);
        TabView::spawn(
            commands,
            assets,
            theme,
            settings,
            viewer_entity,
            tab,
        );
        viewer_entity
    }
    pub fn do_root_layout(
        engine: &NotationLayout,
        layout_query: &mut LayoutQuery,
        panel_query: &ViewQuery<MiniMap>,
        content_query: &ViewQuery<TabView>,
        entity: Entity,
        view: &TabViewer,
    ) {
        if engine.theme._bypass_systems {
            return;
        }
        let constraint =
            LayoutConstraint::from((engine.state.window_width, engine.state.window_height));
        let layout = view.calc_root_layout(&engine, constraint);
        view.do_layout(
            engine,
            layout_query,
            panel_query,
            content_query,
            entity,
            layout,
        );
    }
}

impl TabViewer {
    pub fn on_add_tab(
        mut evts: EventReader<AddTabEvent>,
        mut commands: Commands,
        assets: Res<NotationAssets>,
        mut theme: ResMut<NotationTheme>,
        mut settings: ResMut<NotationSettings>,
        mut switch_tab_evts: EventWriter<SwitchTabEvent>,
    ) {
        let mut tab = None;
        for evt in evts.iter() {
            tab = Some(evt.0.clone());
        }
        if let Some(tab) = tab {
            theme.sizes.melody.update_with_tab_vocal(&tab);
            theme.sizes.harmony.update_with_tab_guitar(&tab, None);
            TabViewer::spawn(
                &mut commands,
                &assets,
                &theme,
                &settings,
                &tab,
            );
            settings.layout.focusing_bar_ordinal = usize::MAX;
            switch_tab_evts.send(SwitchTabEvent::new(tab));
        }
    }
    pub fn on_window_resized(
        mut evts: EventReader<WindowResizedEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationState>,
        mut settings: ResMut<NotationSettings>,
        view_query: ViewRootQuery<TabViewer>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<MiniMap>,
        content_query: ViewQuery<TabView>,
    ) {
        let mut resized = false;
        for _evt in evts.iter() {
            resized = true;
        }
        if resized {
            settings.layout.focusing_bar_ordinal = usize::MAX;
            let engine = NotationLayout::new(&theme, &state, &settings);
            for (entity, view) in view_query.iter() {
                Self::do_root_layout(
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
    pub fn on_added(
        theme: Res<NotationTheme>,
        state: Res<NotationState>,
        settings: Res<NotationSettings>,
        view_query: ViewRootAddedQuery<TabViewer>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<MiniMap>,
        content_query: ViewQuery<TabView>,
    ) {
        for (entity, view) in view_query.iter() {
            let engine = NotationLayout::new(&theme, &state, &settings);
            Self::do_root_layout(
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

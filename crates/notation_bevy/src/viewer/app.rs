use bevy::prelude::*;
use bevy::utils::Uuid;
use notation_midi::prelude::SwitchTabEvent;
use notation_model::prelude::Tab;
use std::fmt::Display;
use std::sync::Arc;

use notation_bevy_utils::prelude::{
    DockView, LayoutConstraint, LayoutQuery, View, ViewBundle, ViewQuery, ViewRootAddedQuery,
    ViewRootQuery,
};

use crate::prelude::{
    AddTabEvent, NotationApp, NotationAppState, NotationAssets, NotationAssetsStates,
    NotationSettings, NotationTheme, WindowResizedEvent,
};
use crate::ui::layout::NotationLayout;
use crate::ui::viewer::TabViewer;

use super::control::ControlView;

pub struct NotationViewer {
    pub uuid: Uuid,
    pub tab: Arc<Tab>,
}

impl Display for NotationViewer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NotationViewer>({})", self.tab.bars.len())
    }
}
impl NotationViewer {
    pub fn new(tab: Arc<Tab>) -> Self {
        let uuid = Uuid::new_v4();
        Self { tab, uuid }
    }
}
impl<'a> View<NotationLayout<'a>> for NotationViewer {
    fn is_root(&self) -> bool {
        true
    }
}
impl<'a> DockView<NotationLayout<'a>, ControlView, TabViewer> for NotationViewer {}

impl NotationViewer {
    pub fn on_add_tab(
        mut evts: EventReader<AddTabEvent>,
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        assets: Res<NotationAssets>,
        theme: Res<NotationTheme>,
        settings: Res<NotationSettings>,
        mut switch_tab_evts: EventWriter<SwitchTabEvent>,
    ) {
        let mut tab = None;
        for evt in evts.iter() {
            tab = Some(evt.0.clone());
        }
        if let Some(tab) = tab {
            let viewer_bundle = ViewBundle::from(NotationViewer::new(tab.clone()));
            let entity = commands.spawn_bundle(viewer_bundle).id();
            ControlView::spawn(
                &mut commands,
                &mut materials,
                &assets,
                &theme,
                &settings,
                entity,
                &tab,
            );
            TabViewer::spawn(
                &mut commands,
                &mut materials,
                &assets,
                &theme,
                &settings,
                entity,
                &tab,
            );
            switch_tab_evts.send(SwitchTabEvent::new(tab));
        }
    }
    pub fn do_root_layout(
        theme: &NotationTheme,
        state: &NotationAppState,
        settings: &NotationSettings,
        layout_query: &mut LayoutQuery,
        panel_query: &ViewQuery<ControlView>,
        content_query: &ViewQuery<TabViewer>,
        entity: Entity,
        view: &Arc<NotationViewer>,
    ) {
        if !theme.loaded { return; }
        let engine = NotationLayout::new(&theme, &state, &settings);
        let constraint =
            LayoutConstraint::from((engine.state.window_width, engine.state.window_height));
        let layout = view.calc_root_layout(&engine, constraint);
        view.do_layout(
            &engine,
            layout_query,
            panel_query,
            content_query,
            entity,
            layout,
        );
    }
    pub fn on_window_resized(
        mut evts: EventReader<WindowResizedEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        mut settings: ResMut<NotationSettings>,
        view_query: ViewRootQuery<NotationViewer>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<ControlView>,
        content_query: ViewQuery<TabViewer>,
    ) {
        let mut resized = false;
        for _evt in evts.iter() {
            resized = true;
        }
        if resized {
            settings.layout.focusing_bar_ordinal = usize::MAX;
            for (entity, view) in view_query.iter() {
                Self::do_root_layout(
                    &theme,
                    &state,
                    &settings,
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
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        view_query: ViewRootAddedQuery<NotationViewer>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<ControlView>,
        content_query: ViewQuery<TabViewer>,
    ) {
        for (entity, view) in view_query.iter() {
            Self::do_root_layout(
                &theme,
                &state,
                &settings,
                &mut layout_query,
                &panel_query,
                &content_query,
                entity,
                view,
            );
        }
    }
    pub fn run(tab_pathes: Vec<String>) {
        NotationApp::run("Notation Viewer", tab_pathes, |app| {
            app.add_system_set(
                SystemSet::on_update(NotationAssetsStates::Loaded)
                    .with_system(ControlView::control_ui.system())
                    .with_system(Self::on_add_tab.system())
                    .with_system(Self::on_window_resized.system())
                    .with_system(Self::on_added.system()),
            );
        })
    }
}

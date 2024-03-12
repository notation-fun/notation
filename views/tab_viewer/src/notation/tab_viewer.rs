use edger_bevy::bevy_prelude::*;
use notation_model::prelude::Tab;
use notation_midi::prelude::SwitchTabEvent;
use std::fmt::Display;
use std::sync::Arc;

use edger_bevy::prelude::*;

use crate::mini::mini_map::MiniMap;
use crate::prelude::{
    GuitarView, NotationState, NotationAssets, NotationSettings,
    NotationTheme, AddTabEvent,
};
use crate::tab::tab_view::TabView;
use crate::prelude::NotationLayout;

use edger_bevy::prelude::WindowResizedEvent;

pub type TabViewerDoRootLayoutEvent = DoRootLayoutEvent<NotationLayout<'static>, TabViewer>;

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
        TabViewerDoRootLayoutEvent::setup(app);

        app.add_systems(Update, (
            TabViewer::do_root_layout,
            TabViewer::on_add_tab,
            GuitarView::on_layout_changed,
            GuitarView::update_hand_shape6
                    .in_set(GuitarViewLabel::UpdateHandShapes),
            GuitarView::update_string_state
                    .in_set(GuitarViewLabel::UpdateStringStates)
                    .after(GuitarViewLabel::UpdateHandShapes),
            GuitarView::adjust_y_by_barre,
        ).run_if(in_state(AssetsStates::Loaded)));
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
        mut evts: EventReader<TabViewerDoRootLayoutEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<MiniMap>,
        content_query: ViewQuery<TabView>,
    ) {
        if theme._bypass_systems {
            return;
        }
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.read() {
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
        for evt in evts.read() {
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
}

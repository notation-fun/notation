use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use notation_bevy_utils::prelude::{
    BevyUtil, ColorBackground, DockPanel, DockSide, DockView, LayoutChangedQuery, LayoutConstraint,
    LayoutQuery, LayoutSize, View, ViewBundle, ViewQuery,
};
use notation_model::prelude::{Tab, TabChord, TrackKind};

use crate::prelude::{NotationState, NotationAssets, NotationSettings, NotationTheme};
use crate::rhythm::rhythm_view::RhythmView;
use crate::prelude::NotationLayout;

use super::tab_chords::TabChords;
use super::tab_events::TabHeaderDoLayoutEvent;

pub struct TabHeader {
    pub tab: Arc<Tab>,
    pub chords: Vec<TabChord>,
}
impl Display for TabHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabHeader>({})", self.tab.bars.len())
    }
}
impl TabHeader {
    pub fn new(tab: Arc<Tab>) -> Self {
        let chords = tab
            .get_track_of_kind(TrackKind::Chord)
            .map(|x| x.get_tab_chords())
            .unwrap_or_default();
        Self { tab, chords }
    }
}
impl<'a> View<NotationLayout<'a>> for TabHeader {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        if engine.settings.hide_chords_view {
            return LayoutSize::new(constraint.max.width, 0.0);
        }
        let grid_data = TabChords::calc_grid_data(engine, constraint.max, self.chords.len() + 1);
        let height = grid_data.content_size().height;
        LayoutSize::new(constraint.max.width, height)
    }
}
impl<'a> DockPanel<NotationLayout<'a>> for TabHeader {
    fn dock_side(&self, _engine: &NotationLayout<'a>, _size: LayoutSize) -> DockSide {
        DockSide::Top
    }
}
impl<'a> DockView<NotationLayout<'a>, RhythmView, TabChords> for TabHeader {}

impl TabHeader {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let view_bundle = ViewBundle::from(TabHeader::new(tab.clone()));
        let view = view_bundle.view.clone();
        let header_entity = BevyUtil::spawn_child_bundle(commands, entity, view_bundle);
        if settings.hide_chords_view {
            return header_entity;
        }
        ColorBackground::spawn(
            commands,
            header_entity,
            theme.z.tab_header,
            theme.colors.chord.background,
        );
        RhythmView::spawn(commands, assets, theme, header_entity, tab);
        TabChords::spawn(commands, assets, theme, header_entity, &tab, &view.chords);
        header_entity
    }
    pub fn do_layout(
        mut evts: EventReader<TabHeaderDoLayoutEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        panel_query: ViewQuery<RhythmView>,
        content_query: ViewQuery<TabChords>,
    ) {
        if theme._bypass_systems {
            return;
        }
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
        query: LayoutChangedQuery<TabHeader>,
        mut evts: EventWriter<TabHeaderDoLayoutEvent>,
    ) {
        for (entity, view, layout) in query.iter() {
            println!("TabContent::on_layout_changed({})", layout);
            evts.send(TabHeaderDoLayoutEvent::new(entity, view, layout))
        }
    }
}

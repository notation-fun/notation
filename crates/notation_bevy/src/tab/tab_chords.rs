use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;

use bevy_utils::prelude::{
    BevyUtil, DockPanel, DockSide, GridData, GridView, LayoutAnchor, LayoutChangedQuery,
    LayoutConstraint, LayoutQuery, LayoutSize, View, ViewAddedQuery, ViewBundle, ViewQuery,
};
use notation_model::prelude::{Chord, ModelEntry, Tab, TrackKind};

use crate::chord::chord_view::ChordView;
use crate::prelude::{NotationAppState, NotationSettings, NotationTheme};
use crate::ui::layout::NotationLayout;

use super::tab_events::TabChordsDoLayoutEvent;

pub struct TabChords {
    pub tab: Arc<Tab>,
    pub chords: Vec<(Chord, Arc<ModelEntry>)>,
}
impl Display for TabChords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabChords>({})", self.tab.bars.len())
    }
}
impl TabChords {
    pub fn new(tab: Arc<Tab>) -> Self {
        let chords = tab
            .get_track_of_kind(TrackKind::Chord)
            .map(|x| x.get_unique_chords())
            .unwrap_or_default();
        Self { tab, chords }
    }
}
impl<'a> GridView<NotationLayout<'a>, ChordView> for TabChords {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        let sizes = engine.theme.sizes.chord;
        let (rows, cols, cell_width) = GridData::cals_fixed_rows_cols_by_height(
            grid_size.height,
            sizes.chord_size_range,
            0.0,
            self.chords.len(),
        );
        let size = LayoutSize::new(cell_width, cell_width);
        GridData::new_fixed(
            rows,
            cols,
            size,
            LayoutSize::ZERO,
            LayoutAnchor::TOP_LEFT,
            grid_size,
        )
    }
}
impl<'a> DockPanel<NotationLayout<'a>> for TabChords {
    fn dock_side(&self) -> DockSide {
        DockSide::Left
    }
}
impl<'a> View<NotationLayout<'a>> for TabChords {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let grid_data = self.calc_grid_data(engine, constraint.max);
        let width = grid_data.content_size().width;
        LayoutSize::new(width, constraint.max.height)
    }
}
impl TabChords {
    pub fn on_added(
        mut commands: Commands,
        _theme: Res<NotationTheme>,
        query: ViewAddedQuery<TabChords>,
    ) {
        for (_parent, entity, view) in query.iter() {
            for (chord, entry) in view.chords.iter() {
                BevyUtil::spawn_child_bundle(
                    &mut commands,
                    entity,
                    ViewBundle::from(ChordView::new(entry, *chord)),
                );
            }
        }
    }
    pub fn do_layout(
        mut evts: EventReader<TabChordsDoLayoutEvent>,
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<ChordView>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
                &mut commands,
                &engine,
                &mut layout_query,
                &cell_query,
                evt.entity,
                evt.layout,
            )
        }
    }
    pub fn on_layout_changed(
        query: LayoutChangedQuery<TabChords>,
        mut evts: EventWriter<TabChordsDoLayoutEvent>,
    ) {
        for (entity, view, layout) in query.iter() {
            println!("TabContent::on_layout_changed({})", layout);
            evts.send(TabChordsDoLayoutEvent::new(entity, view, layout))
        }
    }
}
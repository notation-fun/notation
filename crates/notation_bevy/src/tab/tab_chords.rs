use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;

use bevy_utils::prelude::{BevyUtil, DockPanel, DockSide, GridCellSize, GridData, GridView, LayoutChangedQuery, LayoutConstraint, LayoutData, LayoutQuery, LayoutSize, View, ViewAddedQuery, ViewBundle, ViewQuery};
use notation_model::prelude::Tab;

use crate::chord::chord_view::ChordView;
use crate::prelude::{NotationAppState, NotationSettings, NotationTheme};
use crate::ui::layout::NotationLayout;

use super::tab_events::TabChordsDoLayoutEvent;

pub struct TabChords {
    pub tab: Arc<Tab>,
}
impl Display for TabChords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabChords>({})", self.tab.bars.len())
    }
}
impl TabChords {
    pub fn new(tab: Arc<Tab>) -> Self {
        Self { tab }
    }
}
impl<'a> GridView<NotationLayout<'a>, ChordView> for TabChords {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, data: LayoutData) -> GridData {
        let cols = engine.settings.layout.bars_in_window as usize;
        let rows = GridData::calc_rows(self.tab.bars.len(), cols);
        let width = data.size.width / cols as f32;
        let cell_size = LayoutSize::new(width, width);
        let offset_x = width * (0.5 - engine.theme.sizes.chords.diagram_factor);
        GridData {
            rows,
            cols,
            size: GridCellSize::Fixed(cell_size),
            offset: Vec2::new(offset_x, -engine.theme.grid.header_height - engine.theme.grid.margin),
        }
    }
}
impl<'a> DockPanel<NotationLayout<'a>> for TabChords {
    fn dock_side(&self) -> DockSide {
        DockSide::Left
    }
}
impl<'a> View<NotationLayout<'a>> for TabChords {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let width = constraint.max.width * engine.theme.sizes.chords.chords_panel_factor;
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
            for bar in view.tab.bars.iter() {
                if let Some(chord) = bar.get_chord(None) {
                    BevyUtil::spawn_child_bundle(
                        &mut commands,
                        entity,
                        ViewBundle::from(ChordView::new(bar, chord)),
                    );
                }
            }
        }
    }
    pub fn do_layout(
        mut evts: EventReader<TabChordsDoLayoutEvent>,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<ChordView>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
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

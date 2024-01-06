use std::fmt::Display;
use std::sync::Arc;

use edger_bevy_app::bevy_prelude::*;

use edger_bevy_app::prelude::{
    entity, GridData, GridView, LayoutAnchor, LayoutChangedQuery, LayoutConstraint, LayoutQuery,
    LayoutSize, View, ViewBundle, ViewQuery,
};
use notation_model::prelude::{Tab, TabChord};

use crate::chord::chord_view::ChordView;
use crate::prelude::{NotationState, NotationAssets, NotationSettings, NotationTheme};
use crate::prelude::NotationLayout;

use super::tab_events::TabChordsDoLayoutEvent;

#[derive(Clone, Debug, Component)]
pub struct TabChords {
    pub tab: Arc<Tab>,
    pub chords: Vec<TabChord>,
}
impl Display for TabChords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<TabChords>(B:{}, C:{})",
            self.tab.bars.len(),
            self.chords.len()
        )
    }
}
impl TabChords {
    pub fn new(tab: Arc<Tab>, chords: Vec<TabChord>) -> Self {
        Self { tab, chords }
    }
    pub fn calc_grid_data<'a>(
        engine: &NotationLayout<'a>,
        grid_size: LayoutSize,
        chords: usize,
    ) -> GridData {
        let chord_size_range = match engine.settings.override_chord_size {
            Some(size) => (size, size),
            None => engine.theme.sizes.chord.chord_size_range,
        };
        let (mut rows, mut cols, cell_width) =
            GridData::calc_fixed_rows_cols_by_width(grid_size.width, chord_size_range, 0.0, chords);
        if rows == 1 && cols > chords {
            cols = chords;
        }
        if rows > engine.theme.sizes.chord.max_chord_rows {
            rows = engine.theme.sizes.chord.max_chord_rows;
            //cols = (chords + 1) / rows;
        }
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
impl<'a> GridView<NotationLayout<'a>, ChordView> for TabChords {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        Self::calc_grid_data(engine, grid_size, self.chords.len())
    }
}

impl<'a> View<NotationLayout<'a>> for TabChords {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let grid_data = self.calc_grid_data(engine, constraint.max);
        let height = grid_data.content_size().height;
        LayoutSize::new(constraint.max.width, height)
    }
}
impl TabChords {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Arc<Tab>,
        chords: &Vec<TabChord>,
    ) -> Entity {
        let view_bundle = ViewBundle::from(TabChords::new(tab.clone(), chords.clone()));
        let view = view_bundle.view.clone();
        let chords_entity = entity::spawn_child_bundle(commands, entity, view_bundle);
        for chord_view in view.chords.iter() {
            ChordView::spawn(commands, assets, theme, chords_entity, chord_view);
        }
        chords_entity
    }
    pub fn do_layout(
        mut evts: EventReader<TabChordsDoLayoutEvent>,
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<ChordView>,
    ) {
        if theme._bypass_systems {
            return;
        }
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.read() {
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

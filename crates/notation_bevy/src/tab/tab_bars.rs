use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;

use bevy_utils::prelude::{BevyUtil, GridData, GridView, LayoutAnchor, LayoutChangedQuery, LayoutQuery, LayoutSize, View, ViewAddedQuery, ViewQuery};
use notation_model::prelude::{BarLane, LaneKind, Tab, TabBar};

use crate::bar::bar_layout::BarLayoutData;
use crate::bar::bar_view::BarView;
use crate::lane::lane_layout::LaneLayoutData;
use crate::prelude::{
    AddEntryEvent, BarBundle, BarPlugin, NotationAppState, NotationSettings,
    NotationTheme, PlayPlugin,
};
use crate::theme::theme_sizes::LayoutSizes;
use crate::ui::layout::NotationLayout;

use super::tab_events::TabBarsDoLayoutEvent;

pub struct TabBars {
    pub tab: Arc<Tab>,
    pub bar_layouts: Arc<Vec<BarLayoutData>>,
}
impl Display for TabBars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabBars>({})", self.tab.bars.len())
    }
}
impl TabBars {
    pub fn new(tab: Arc<Tab>, bar_layouts: Arc<Vec<BarLayoutData>>) -> Self {
        Self { tab, bar_layouts }
    }
}
impl<'a> View<NotationLayout<'a>> for TabBars {}

impl<'a> GridView<NotationLayout<'a>, BarView> for TabBars {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        let sizes = engine.theme.sizes.bar;
        let bar_beats = self.tab.bar_beats() as f32;
        let bar_width_range = (sizes.beat_size_range.0 * bar_beats, sizes.beat_size_range.1 * bar_beats);
        let (rows, cols, cell_width) = GridData::cals_fixed_rows_cols_by_width(
            grid_size.width - sizes.row_margin * 2.0, bar_width_range, 0.0, self.tab.bars.len());
        let size = LayoutSize::new(cell_width, 240.0); //TODO
        GridData::new_fixed(grid_size, rows, cols, size, LayoutSize::ZERO, LayoutAnchor::TOP_LEFT)
    }
}

impl TabBars {
    fn calc_lane_layout_data(
        sizes: &LayoutSizes,
        lane: &BarLane,
    ) -> Option<LaneLayoutData> {
        let height = sizes.calc_lane_height(lane.kind);
        if height > 0.0 {
            let order = sizes.calc_lane_order(lane.kind);
            Some(LaneLayoutData::new(order, 0.0, height, sizes.lane_margin))
        } else {
            None
        }
    }
    fn calc_lane_layouts_data(
        sizes: &LayoutSizes,
        bar: &TabBar,
    ) -> HashMap<String, LaneLayoutData> {
        bar.lanes
            .iter()
            .map(|lane| (lane, Self::calc_lane_layout_data(sizes, lane)))
            .filter_map(|(lane, layout)| layout.map(|layout| (lane.id(), layout)))
            .collect()
    }
    fn calc_bar_layout_data(
        sizes: &LayoutSizes,
        bar: &TabBar
    ) -> BarLayoutData {
        BarLayoutData::new(
            0.0,
            200.0,
            sizes.bar_margin,
            Arc::new(Self::calc_lane_layouts_data(sizes, bar)),
        )
    }
    pub fn calc_bar_layouts(
        sizes: &LayoutSizes,
        tab: &Tab,
    ) -> Vec<BarLayoutData> {
        tab
            .bars
            .iter()
            .map(|bar| Self::calc_bar_layout_data(sizes, bar))
            .collect()
    }
}

impl TabBars {
    pub fn on_added(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: ViewAddedQuery<TabBars>,
    ) {
        for (_parent, entity, view) in query.iter() {
            PlayPlugin::spawn_indicators(
                &mut commands,
                &theme,
                entity,
                &view.tab,
            );
            let bar_bundles: Vec<(&BarLayoutData, BarBundle)> = view
                .tab
                .bars
                .iter()
                .enumerate()
                .filter_map(|(index, bar)| {
                    view.bar_layouts.get(index).map(|bar_layout| {
                        //let transform = theme.grid.calc_bar_transform(&bar_layout);
                        (
                            bar_layout,
                            BarBundle::new(bar.clone(), bar_layout.clone()),
                        )
                    })
                })
                .collect();
            for (_bar_layout, bar_bundle) in bar_bundles.into_iter() {
                BevyUtil::spawn_child_bundle(&mut commands, entity, bar_bundle);
            }
        }
    }
    pub fn do_layout(
        mut evts: EventReader<TabBarsDoLayoutEvent>,
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<BarView>,
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
        query: LayoutChangedQuery<TabBars>,
        mut evts: EventWriter<TabBarsDoLayoutEvent>,
    ) {
        for (entity, view, layout) in query.iter() {
            evts.send(TabBarsDoLayoutEvent::new(entity, &view, layout))
        }
    }}

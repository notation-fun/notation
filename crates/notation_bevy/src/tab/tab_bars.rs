use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::sync::{Arc, RwLock};

use bevy::prelude::*;

use notation_bevy_utils::prelude::{
    BevyUtil, GridData, GridView, LayoutAnchor, LayoutChangedQuery, LayoutData, LayoutQuery,
    LayoutSize, View, ViewBundle, ViewQuery,
};
use notation_model::prelude::{Tab, TabBar};

use crate::bar::bar_layout::BarLayoutData;
use crate::bar::bar_view::BarView;
use crate::lane::lane_layout::LaneLayoutData;
use crate::prelude::{
    NotationAppState, NotationAssets, NotationSettings, NotationTheme, PlayPlugin,
};
use crate::settings::layout_settings::LayoutMode;
use crate::ui::layout::NotationLayout;

use super::tab_events::{TabBarsDoLayoutEvent, TabBarsResizedEvent, TabBarsResizedPreEvent};

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
impl<'a> View<NotationLayout<'a>> for TabBars {
    fn log_layout_changed(&self) -> bool {
        false
    }
}

impl<'a> GridView<NotationLayout<'a>, BarView> for TabBars {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        if self.tab.bars.len() == 0 {
            return GridData::ZERO;
        }
        let bar_margin = engine.theme.sizes.layout.bar_margin;
        let beat_size_range =
            match engine.settings.override_beat_size {
                Some(size) => (size, size * 2.0),
                None => engine.theme.sizes.bar.beat_size_range,
            };
        let bar_beats = self.tab.bar_beats() as f32;
        let bar_width_range = (
            beat_size_range.0 * bar_beats,
            beat_size_range.1 * bar_beats,
        );
        let (rows, cols, cell_width) = GridData::cals_fixed_rows_cols_by_width(
            grid_size.width - bar_margin * 2.0,
            bar_width_range,
            0.0,
            self.tab.bars.len(),
        );
        let margin = engine.theme.sizes.bar_margin(&engine.settings);
        if engine.settings.layout.mode == LayoutMode::Line {
            let height = self
                .bar_layouts
                .get(0)
                .map(|x| x.height())
                .unwrap_or(grid_size.height);
            let size = if cols >= 2 {
                LayoutSize::new(cell_width, height)
            } else {
                LayoutSize::new(cell_width * 2.0 / 3.0, height)
            };
            let grid_data = GridData::new_fixed(
                1,
                self.tab.bars.len(),
                size,
                margin,
                LayoutAnchor::TOP_LEFT,
                grid_size,
            );
            GridData {
                offset: grid_data.offset + Vec2::new(bar_margin, 0.0),
                ..grid_data
            }
        } else {
            let mut row_sizes = Vec::new();
            for row in 0..rows {
                let mut non_ghost_lanes: HashSet<String> = HashSet::new();
                for col in 0..cols {
                    if let Some(bar_layout) = self.bar_layouts.get(row * cols + col) {
                        for lane_layout in bar_layout.lane_layouts.iter() {
                            if engine.settings.layout.video_recording_mode || !lane_layout.is_ghost() {
                                non_ghost_lanes.insert(lane_layout.id());
                            }
                        }
                    }
                }
                for col in 0..cols {
                    if let Some(bar_layout) = self.bar_layouts.get(row * cols + col) {
                        for lane_layout in bar_layout.lane_layouts.iter() {
                            if lane_layout.is_ghost() {
                                let visible = non_ghost_lanes.contains(&lane_layout.id());
                                lane_layout.set_visible(visible);
                            }
                        }
                    }
                }
                let bar_layout = self.bar_layouts.get(row * cols).unwrap();
                row_sizes.push(LayoutSize::new(cell_width, bar_layout.height()));
            }
            GridData::new_rows(
                rows,
                cols,
                row_sizes,
                margin,
                LayoutAnchor::TOP_LEFT,
                grid_size,
            )
        }
    }
}

impl TabBars {
    fn calc_all_lane_layouts(
        theme: &NotationTheme,
        settings: &NotationSettings,
        tab: &Tab,
    ) -> Vec<LaneLayoutData> {
        let mut lane_layouts: HashMap<String, LaneLayoutData> = HashMap::new();
        for bar in tab.bars.iter() {
            for lane in bar.lanes.iter() {
                let lane_id = lane.id();
                if !lane_layouts.contains_key(&lane_id) {
                    let height = theme.sizes.calc_lane_height(lane.kind);
                    let margin = theme.sizes.layout.lane_margin;
                    lane_layouts.insert(lane_id, LaneLayoutData::new(&lane, height, margin));
                }
            }
        }
        let result = lane_layouts
            .into_iter()
            .map(|(_, lane_layout)| lane_layout)
            .collect::<Vec<LaneLayoutData>>();
        settings.layout.sort_lane_layouts(&result)
    }
    fn calc_bar_layout_data(all_lane_layouts: &Vec<LaneLayoutData>, bar: &TabBar) -> BarLayoutData {
        let mut lane_layouts = Vec::new();
        for lane_layout in all_lane_layouts.iter() {
            let lane =
                bar.get_lane_of_kind(lane_layout.lane_kind, Some(lane_layout.track_props.index));
            lane_layouts.push(Arc::new(LaneLayoutData {
                lane,
                visible: Arc::new(RwLock::new(lane_layout.height > 0.0)),
                ..lane_layout.clone()
            }));
        }
        BarLayoutData::new(bar.props, lane_layouts)
    }
    pub fn calc_bar_layouts(
        theme: &NotationTheme,
        settings: &NotationSettings,
        tab: &Tab,
    ) -> Vec<BarLayoutData> {
        let all_lane_layouts = Self::calc_all_lane_layouts(theme, settings, tab);
        tab.bars
            .iter()
            .map(|bar| Self::calc_bar_layout_data(&all_lane_layouts, bar))
            .collect()
    }
}

impl TabBars {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let bar_layouts = TabBars::calc_bar_layouts(&theme, &settings, &tab);
        let view_bundle = ViewBundle::from(TabBars::new(tab.clone(), Arc::new(bar_layouts)));
        let view = view_bundle.view.clone();
        let bars_entity = BevyUtil::spawn_child_bundle(commands, entity, view_bundle);
        PlayPlugin::spawn_indicators(commands, theme, bars_entity, tab);
        let bar_bundles: Vec<(&Arc<TabBar>, &BarLayoutData)> = view
            .tab
            .bars
            .iter()
            .enumerate()
            .filter_map(|(index, bar)| {
                view.bar_layouts.get(index).map(|bar_layout| {
                    //let transform = theme.sizes.bar.calc_bar_transform(&bar_layout);
                    (bar, bar_layout)
                })
            })
            .collect();
        for (bar, bar_layout) in bar_bundles.into_iter() {
            BarView::spawn(
                commands,
                assets,
                theme,
                settings,
                bars_entity,
                &bar,
                bar_layout,
            );
        }
        bars_entity
    }
    pub fn on_resized_pre(
        mut evts: EventReader<TabBarsResizedPreEvent>,
        theme: Res<NotationTheme>,
        cell_query: Query<(&Parent, &Arc<BarView>, &LayoutData)>,
        mut tab_resized_evts: EventWriter<TabBarsResizedEvent>,
    ) {
        if theme._bypass_systems { return; }
        for evt in evts.iter() {
            let mut bars = Vec::new();
            for (parent, bar_view, layout) in cell_query.iter() {
                if parent.0 == evt.0 {
                    bars.push((bar_view.clone(), layout.clone()));
                }
            }
            tab_resized_evts.send(TabBarsResizedEvent(Arc::new(bars)));
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
        mut tab_resized_evts: EventWriter<TabBarsResizedPreEvent>,
    ) {
        if theme._bypass_systems { return; }
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
                &mut commands,
                &engine,
                &mut layout_query,
                &cell_query,
                evt.entity,
                evt.layout,
            );
            tab_resized_evts.send(TabBarsResizedPreEvent(evt.entity));
        }
    }
    pub fn on_layout_changed(
        query: LayoutChangedQuery<TabBars>,
        mut evts: EventWriter<TabBarsDoLayoutEvent>,
    ) {
        for (entity, view, layout) in query.iter() {
            evts.send(TabBarsDoLayoutEvent::new(entity, &view, layout))
        }
    }
}

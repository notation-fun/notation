use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::sync::{Arc, RwLock};

use edger_bevy_app::bevy_prelude::*;

use edger_bevy_app::prelude::{
    entity, GridData, GridView, LayoutAnchor, LayoutChangedQuery, LayoutData, LayoutQuery,
    LayoutSize, View, ViewBundle, ViewQuery,
};
use notation_model::lane_kind::LaneKind;
use notation_model::prelude::{Tab, TabBar};

use crate::bar::bar_layout::BarLayoutData;
use crate::bar::bar_view::BarView;
use crate::lane::lane_layout::LaneLayoutData;
use crate::prelude::{
    NotationState, NotationAssets, NotationSettings, NotationTheme, PlayPlugin,
};
use crate::settings::layout_settings::LayoutMode;
use crate::prelude::NotationLayout;

use super::tab_events::{TabBarsDoLayoutEvent, TabBarsResizedEvent, TabBarsResizedPreEvent};

#[derive(Clone, Debug, Component)]
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

impl TabBars {
    fn calc_grid_data_line_mode<'a>(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize, _rows: usize, cols: usize, cell_width: f32, cell_margin: LayoutSize) -> GridData {
        let bar_margin = engine.theme.sizes.layout.bar_margin;
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
            cell_margin,
            LayoutAnchor::TOP_LEFT,
            grid_size,
        );
        GridData {
            offset: grid_data.offset + Vec2::new(bar_margin, 0.0),
            ..grid_data
        }
    }
    fn sync_bar_layouts<'a>(&self, engine: &NotationLayout<'a>, begin: usize, count: usize) -> f32 {
        let mut non_ghost_lanes: HashSet<String> = HashSet::new();
        for offset in 0..count {
            if let Some(bar_layout) = self.bar_layouts.get(begin + offset) {
                for lane_layout in bar_layout.lane_layouts.iter() {
                    if engine.settings.layout.video_recording_mode
                        || !lane_layout.is_ghost()
                    {
                        non_ghost_lanes.insert(lane_layout.id());
                    }
                }
            }
        }
        for offset in 0..count {
            if let Some(bar_layout) = self.bar_layouts.get(begin + offset) {
                for lane_layout in bar_layout.lane_layouts.iter() {
                    if lane_layout.is_ghost() {
                        let visible = non_ghost_lanes.contains(&lane_layout.id());
                        lane_layout.set_visible(visible);
                    }
                }
            }
        }
        let bar_layout = self.bar_layouts.get(begin).unwrap();
        bar_layout.height()
    }
    fn calc_grid_data_normal_grid<'a>(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize, rows: usize, cols: usize, cell_width: f32, cell_margin: LayoutSize) -> GridData {
        let mut row_sizes = Vec::new();
        for row in 0..rows {
            let height = self.sync_bar_layouts(engine, row * cols, cols);
            row_sizes.push(LayoutSize::new(cell_width, height));
        }
        GridData::new_rows(
            rows,
            cols,
            row_sizes,
            cell_margin,
            LayoutAnchor::TOP_LEFT,
            grid_size,
        )
    }
    fn calc_grid_data_sparse_grid<'a>(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize, _rows: usize, cols: usize, cell_width: f32, cell_margin: LayoutSize) -> GridData {
        let mut sparse_rows = 0;
        let mut sparse_row_cols = Vec::new();
        let mut sparse_row_sizes = Vec::new();
        let mut begin = 0;
        let mut count = 0;
        for bar in self.tab.bars.iter() {
            if count >= cols || count > 0 && bar.props.bar_index == 0 {
                sparse_rows += 1;
                let height = self.sync_bar_layouts(engine, begin, count);
                sparse_row_sizes.push(LayoutSize::new(cell_width, height));
                sparse_row_cols.push(count);
                begin = begin + count - 1;
                count = 0;
            }
            count += 1;
        }
        if count > 0 {
            sparse_rows += 1;
            let height = self.sync_bar_layouts(engine, begin, count);
            sparse_row_sizes.push(LayoutSize::new(cell_width, height));
            sparse_row_cols.push(count);
        }
        GridData::new_sparse_rows(
            sparse_rows,
            cols,
            sparse_row_sizes,
            sparse_row_cols,
            cell_margin,
            LayoutAnchor::TOP_LEFT,
            grid_size,
        )
    }
}

impl<'a> GridView<NotationLayout<'a>, BarView> for TabBars {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        if self.tab.bars.len() == 0 {
            return GridData::ZERO;
        }
        let bar_margin = engine.theme.sizes.layout.bar_margin;
        let beat_size_range = match engine.settings.override_beat_size {
            Some(size) => (size, size * 2.0),
            None => engine.theme.sizes.bar.beat_size_range,
        };
        let bar_beats = self.tab.bar_beats() as f32;
        let bar_width_range = (beat_size_range.0 * bar_beats, beat_size_range.1 * bar_beats);
        let tab_width = match engine.settings.layout.override_tab_width {
            Some(width) => width,
            None => grid_size.width,
        };
        let (rows, cols, cell_width) = GridData::calc_fixed_rows_cols_by_width(
            tab_width - bar_margin * 2.0,
            bar_width_range,
            0.0,
            self.tab.bars.len(),
        );
        let cell_margin = engine.theme.sizes.cell_margin(&engine.settings);
        if engine.settings.layout.mode == LayoutMode::Line {
            self.calc_grid_data_line_mode(engine, grid_size, rows, cols, cell_width, cell_margin)
        } else {
            let grid_data = if engine.settings.new_row_for_section {
                self.calc_grid_data_sparse_grid(engine, grid_size, rows, cols, cell_width, cell_margin)
            } else {
                self.calc_grid_data_normal_grid(engine, grid_size, rows, cols, cell_width, cell_margin)
            };
            match engine.settings.layout.override_tab_width {
                None => grid_data,
                Some(_) => {
                    let offset = grid_data
                        .content_size
                        .calc_offset(LayoutAnchor::TOP_LEFT, LayoutAnchor::TOP_LEFT)
                        + Vec2::new(bar_margin, 0.0);
                    GridData {
                        offset,
                        ..grid_data
                    }
                }
            }
        }
    }
}

impl TabBars {
    fn calc_all_lane_layouts(
        theme: &NotationTheme,
        settings: &NotationSettings,
        tab: &Tab,
    ) -> Vec<(LaneKind, LaneLayoutData)> {
        let mut lane_layouts: HashMap<String, (LaneKind, LaneLayoutData)> = HashMap::new();
        for bar in tab.bars.iter() {
            for ((_k, _i), lane) in bar.lanes.iter() {
                let lane_id = lane.id();
                if !lane_layouts.contains_key(&lane_id) {
                    let height = theme.sizes.calc_lane_height(settings, lane.kind);
                    let margin = theme.sizes.layout.lane_margin;
                    lane_layouts.insert(lane_id, (lane.kind, LaneLayoutData::new(&lane, height, margin)));
                    if lane.kind == LaneKind::Strings && !settings.hide_harmony_lane {
                        let lane_id = lane.kind_id(LaneKind::Harmony);
                        if !lane_layouts.contains_key(&lane_id) {
                            let height = theme.sizes.calc_lane_height(settings, LaneKind::Harmony);
                            let margin = theme.sizes.layout.lane_margin;
                            lane_layouts.insert(lane_id, (lane.kind, LaneLayoutData::new_virtual(&lane, LaneKind::Harmony, height, margin)));
                        }
                    }
                }
            }
        }
        let result = lane_layouts
            .into_iter()
            .map(|(_, (lane_kind, lane_layout))| (lane_kind, lane_layout))
            .collect::<Vec<(LaneKind, LaneLayoutData)>>();
        settings.layout.sort_lane_layouts(&result)
    }
    fn calc_bar_layout_data(
        theme: &NotationTheme,
        all_lane_layouts: &Vec<(LaneKind, LaneLayoutData)>,
        bar: &TabBar,
    ) -> BarLayoutData {
        let mut lane_layouts = Vec::new();
        for (lane_kind, lane_layout) in all_lane_layouts.iter() {
            let lane = bar.get_lane_of_kind(*lane_kind, Some(lane_layout.track_props.index));
            lane_layouts.push(Arc::new(LaneLayoutData {
                lane,
                visible: Arc::new(RwLock::new(lane_layout.height > 0.0)),
                ..lane_layout.clone()
            }));
        }
        BarLayoutData::new(theme.sizes.layout.bar_min_height, bar.props, lane_layouts)
    }
    pub fn calc_bar_layouts(
        theme: &NotationTheme,
        settings: &NotationSettings,
        tab: &Tab,
    ) -> Vec<BarLayoutData> {
        let all_lane_layouts = Self::calc_all_lane_layouts(theme, settings, tab);
        tab.bars
            .iter()
            .map(|bar| Self::calc_bar_layout_data(theme, &all_lane_layouts, bar))
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
        let bars_entity = entity::spawn_child_bundle(commands, entity, view_bundle);
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
        cell_query: Query<(&Parent, &BarView, &LayoutData)>,
        mut tab_resized_evts: EventWriter<TabBarsResizedEvent>,
    ) {
        if theme._bypass_systems {
            return;
        }
        for evt in evts.read() {
            let mut bars = Vec::new();
            for (parent, bar_view, layout) in cell_query.iter() {
                if parent.get() == evt.0 {
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
        state: Res<NotationState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<BarView>,
        mut tab_resized_evts: EventWriter<TabBarsResizedPreEvent>,
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
            );
            tab_resized_evts.send(TabBarsResizedPreEvent(evt.entity));
        }
    }
    pub fn on_layout_changed(
        query: LayoutChangedQuery<TabBars>,
        mut evts: EventWriter<TabBarsDoLayoutEvent>,
    ) {
        for (entity, view, layout) in query.iter() {
            evts.send(TabBarsDoLayoutEvent::new(entity, &view, layout));
        }
    }
}

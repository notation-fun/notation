use std::collections::HashMap;
use std::sync::Arc;

use notation_model::{notation_proto::prelude::BarLayer, prelude::{BarLane, LaneKind, Position, Tab, TabBar, TabPosition}};
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::{bar::bar_layout::BarLayoutData, lane::lane_layout::LaneLayoutData, prelude::{BarLayout, LaneLayout, NotationAppState}};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub enum LayoutMode {
    Grid,
    Line,
}
impl Default for LayoutMode {
    fn default() -> Self {
        Self::Grid
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LayoutSettings {
    pub mode: LayoutMode,
    pub bars_in_window: u8,
    pub bar_margin: f32,
    pub lane_margin: f32,
    pub shapes_height: f32,
    pub strings_height: f32,
    pub lyrics_height: f32,
    pub melody_height: f32,
    pub shapes_lane_order: u8,
    pub strings_lane_order: u8,
    pub lyrics_lane_order: u8,
    pub melody_lane_order: u8,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            mode: LayoutMode::default(),
            bars_in_window: 4,
            bar_margin: 32.0,
            lane_margin: 4.0,
            shapes_height: 46.0,
            strings_height: 72.0,
            lyrics_height: 20.0,
            melody_height: 36.0,
            shapes_lane_order: 1,
            strings_lane_order: 2,
            lyrics_lane_order: 3,
            melody_lane_order: 4,
        }
    }
}

impl LayoutSettings {
    pub fn calc_lane_order(&self, lane: &BarLane) -> u8 {
        match lane.kind {
            LaneKind::Lyrics => self.lyrics_lane_order,
            LaneKind::Melody => self.melody_lane_order,
            LaneKind::Strings => self.strings_lane_order,
            LaneKind::Shapes => self.shapes_lane_order,
            _ => 0,
        }
    }
    fn calc_lane_layout_data(
        &self,
        _app_state: &NotationAppState,
        lane: &BarLane,
    ) -> Option<LaneLayoutData> {
        let height = match lane.kind {
            LaneKind::Lyrics => self.lyrics_height,
            LaneKind::Melody => self.melody_height,
            LaneKind::Strings => self.strings_height,
            LaneKind::Shapes => self.shapes_height,
            _ => 0.0,
        };
        if height > 0.0 {
            let order = self.calc_lane_order(lane);
            Some(LaneLayoutData::new(order, height, self.lane_margin))
        } else {
            None
        }
    }
    fn calc_lane_layouts_data(
        &self,
        app_state: &NotationAppState,
        bar: &TabBar,
    ) -> HashMap<String, LaneLayoutData> {
        bar.bar.lanes
            .iter()
            .filter(|lane| lane.in_round(bar.section_round))
            .map(|lane| (lane, self.calc_lane_layout_data(app_state, lane)))
            .filter_map(|(lane, layout)| {
                layout.map(|layout| (lane.id(), layout))
            })
            .collect()
    }
    fn calc_lane_layouts(
        &self,
        _app_state: &NotationAppState,
        lane_layouts_data: HashMap<String, LaneLayoutData>,
    ) -> HashMap<String, LaneLayout> {
        let mut layouts: Vec<(String, LaneLayoutData)> = lane_layouts_data
            .into_iter()
            .collect();
        layouts.sort_by(|(_, a), (_, b)| {
            a.order.cmp(&b.order)
        });
        let mut y: f32 = 0.0;
        layouts
            .into_iter()
            .map(|(lane_id, data)| {
                let offset = y;
                y -= data.height + data.margin;
                (lane_id.clone(), LaneLayout::new(data, offset))
            })
            .collect()
    }
}

impl LayoutSettings {
    fn _calc_bar_row_col(&self, index: usize) -> (usize, usize) {
        match self.mode {
            LayoutMode::Grid => {
                let row = index / self.bars_in_window as usize;
                let col = index % self.bars_in_window as usize;
                (row, col)
            }
            LayoutMode::Line => (0, index),
        }
    }
    fn calc_bar_layout_data(&self,
            app_state: &NotationAppState,
            bar: &TabBar
        ) -> BarLayoutData {
        let (row, col) = self._calc_bar_row_col(bar.bar_ordinal - 1);
        BarLayoutData::new(self.bar_margin, row, col, Arc::new(self.calc_lane_layouts_data(app_state, bar)))
    }
    pub fn calc_pos_layout(&self, tab: &Tab, pos: TabPosition) -> (usize, usize) {
        let bar_units = tab.bar_units();
        let mut index = (pos.in_tab_pos.0 / bar_units.0) as usize;
        if index >= tab.bars.len() {
            index = tab.bars.len() - 1;
        }
        self._calc_bar_row_col(index)
    }
    fn merge_row_lane_layouts_data(&self,
        row_lane_layouts_data: &mut HashMap<String, LaneLayoutData>,
        bar_layout_data: &BarLayoutData,
    ) {
        for (lane_id, lane_layout_data) in bar_layout_data.lane_layouts_data.iter() {
            if !row_lane_layouts_data.contains_key(lane_id) {
                row_lane_layouts_data.insert(lane_id.clone(), *lane_layout_data);
            }
        }
    }
    fn calc_lane_layouts_height(&self,
        _app_state: &NotationAppState,
        lane_layouts_data: &HashMap<String, LaneLayoutData>,
    ) -> f32 {
        let mut height = 0.0;
        for (index, (_, lane_layout_data)) in lane_layouts_data.iter().enumerate() {
            height += lane_layout_data.height;
            if index != lane_layouts_data.len() {
                height += lane_layout_data.margin;
            }
        }
        height
    }
    pub fn calc_bar_layouts(&self,
        app_state: &NotationAppState,
        tab: &Tab
    ) -> Vec<BarLayout> {
        let with_layouts_data: Vec<(&Arc<TabBar>, BarLayoutData)> = tab.bars.iter()
            .map(|bar| {
                (bar, self.calc_bar_layout_data(app_state, bar))
            }).collect();
        let mut rows_lane_layouts_data: Vec<HashMap<String, LaneLayoutData>> = Vec::new();
        for (_bar, bar_layout_data) in with_layouts_data.iter() {
            while rows_lane_layouts_data.len() < bar_layout_data.row + 1 {
                rows_lane_layouts_data.push(HashMap::new());
            }
            let mut row_lane_layouts_data = rows_lane_layouts_data.get_mut(bar_layout_data.row).unwrap();
            self.merge_row_lane_layouts_data(&mut row_lane_layouts_data, &bar_layout_data);
        }
        let mut y: f32 = 0.0;
        let rows_lane_layouts: Vec<(f32, f32, Arc<HashMap<String, LaneLayout>>)> =
            rows_lane_layouts_data.into_iter().map(|data|{
                let offset = y;
                let height = self.calc_lane_layouts_height(app_state, &data);
                let lane_layouts = self.calc_lane_layouts(app_state, data);
                y -= height + self.bar_margin;
                (offset, height, Arc::new(lane_layouts))
            }).collect();
        with_layouts_data.into_iter()
            .map(|(_bar, bar_layout_data)| {
                let (offset, height, lane_layouts) = rows_lane_layouts.get(bar_layout_data.row).unwrap();
                BarLayout::new(bar_layout_data, *offset, *height, lane_layouts.clone())
            }).collect()
    }
    pub fn bar_layout_of_pos(
        &self,
        bar_layouts: &Arc<Vec<BarLayout>>,
        pos: Position,
    ) -> Option<BarLayout> {
        bar_layouts.get(pos.bar.bar_ordinal - 1)
            .map(|x| x.clone())
    }
}

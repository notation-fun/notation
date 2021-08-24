use std::{collections::HashMap, fmt::Display};
use std::sync::Arc;

use crate::prelude::{LaneLayoutData};

#[derive(Clone, Debug)]
pub struct BarLayoutData {
    pub height: f32,
    pub lane_layouts: Vec<LaneLayoutData>,
}
impl Display for BarLayoutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BarLayoutData>({} [{}])", self.height, self.lane_layouts.len())
    }
}
impl BarLayoutData {
    pub fn new(
        lane_layouts: Vec<LaneLayoutData>,
    ) -> Self {
        Self {
            height: Self::calc_height(&lane_layouts),
            lane_layouts: lane_layouts,
        }
    }
    pub fn calc_height(lane_layouts: &Vec<LaneLayoutData>) -> f32 {
        let mut height = 0.0;
        let len = lane_layouts.len();
        for (index, lane_layout) in lane_layouts.iter().enumerate() {
            if lane_layout.visible {
                height += lane_layout.height;
                if index < len - 1 {
                    height += lane_layout.margin;
                }
            }
        }
        height
    }
}

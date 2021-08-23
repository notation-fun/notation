use std::{collections::HashMap, fmt::Display};
use std::sync::Arc;

use crate::prelude::{LaneLayoutData};

#[derive(Clone, Debug)]
pub struct BarLayoutData {
    pub offset: f32,
    pub height: f32,
    pub margin: f32,
    pub lane_layouts: Arc<HashMap<String, LaneLayoutData>>,
}
impl Display for BarLayoutData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BarLayoutData>(m:{} L:[{}])", self.margin, self.lane_layouts.len())
    }
}
impl BarLayoutData {
    pub fn new(
        offset: f32,
        height: f32,
        margin: f32,
        lane_layouts_data: Arc<HashMap<String, LaneLayoutData>>,
    ) -> Self {
        Self {
            offset,
            height,
            margin,
            lane_layouts: lane_layouts_data,
        }
    }
}

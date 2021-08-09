use std::collections::HashMap;
use std::sync::Arc;

use crate::lane::lane_layout::{LaneLayout, LaneLayoutData};

#[derive(Clone, Debug)]
pub struct BarLayoutData {
    pub margin: f32,
    pub row: usize,
    pub col: usize,
    pub lane_layouts_data: Arc<HashMap<String, LaneLayoutData>>,
}
impl BarLayoutData {
    pub fn new(
        margin: f32,
        row: usize,
        col: usize,
        lane_layouts_data: Arc<HashMap<String, LaneLayoutData>>,
    ) -> Self {
        Self {
            margin,
            row,
            col,
            lane_layouts_data,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BarLayout {
    pub data: BarLayoutData,
    pub offset: f32,
    pub height: f32,
    pub lane_layouts: Arc<HashMap<String, LaneLayout>>,
}
impl BarLayout {
    pub fn new(
        data: BarLayoutData,
        offset: f32,
        height: f32,
        lane_layouts: Arc<HashMap<String, LaneLayout>>,
    ) -> Self {
        Self {
            data,
            offset,
            height,
            lane_layouts,
        }
    }
}

use bevy::prelude::*;
use bevy_utils::prelude::LayoutData;
use std::sync::Arc;

use notation_model::prelude::BarLane;

use crate::prelude::LaneLayoutData;

use super::lane_view::LaneView;

#[derive(Bundle)]
pub struct LaneBundle {
    pub name: Name,
    pub lane: Arc<BarLane>,
    pub view: Arc<LaneView>,
    pub layout: LayoutData,
    pub lane_layout: LaneLayoutData,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl LaneBundle {
    pub fn new(lane: Arc<BarLane>, lane_layout: LaneLayoutData) -> Self {
        let name = format!("{} {}", lane.bar_props().bar_ordinal, lane)
            .as_str()
            .into();
        let view = Arc::new(LaneView::new(&lane, lane.kind));
        Self {
            name,
            lane,
            view,
            layout: LayoutData::ZERO,
            lane_layout,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}

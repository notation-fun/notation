use bevy::prelude::*;
use std::sync::Arc;

use notation_model::prelude::BarLane;

use notation_bevy_utils::prelude::LayoutData;

use crate::prelude::LaneLayoutData;

use super::lane_view::LaneView;

#[derive(Bundle)]
pub struct LaneBundle {
    pub name: Name,
    pub lane: Arc<BarLane>,
    pub view: Arc<LaneView>,
    pub layout: LayoutData,
    pub transform: Transform,
    pub global_cransform: GlobalTransform,
}

impl LaneBundle {
    pub fn new(lane: Arc<BarLane>, lane_layout: LaneLayoutData) -> Self {
        let name = format!("{} {}", lane.bar_props().bar_ordinal, lane)
            .as_str()
            .into();
        let view = Arc::new(lane_layout);
        Self {
            name,
            lane,
            view,
            layout: LayoutData::ZERO,
            transform: Transform::default(),
            global_cransform: GlobalTransform::default(),
        }
    }
}

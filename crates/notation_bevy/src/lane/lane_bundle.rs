use bevy::prelude::*;

use notation_model::prelude::BarLane;

use notation_bevy_utils::prelude::{LayoutData, SingleData};

use crate::prelude::LaneLayoutData;

use super::lane_view::LaneView;

#[derive(Bundle)]
pub struct LaneBundle {
    pub name: Name,
    pub lane: SingleData<BarLane>,
    pub view: LaneView,
    pub layout: LayoutData,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl LaneBundle {
    pub fn new(lane: &BarLane, view: LaneLayoutData) -> Self {
        let name = format!("{} {}", lane.bar_props().bar_ordinal, lane)
            .as_str()
            .into();
        Self {
            name,
            lane: SingleData::<BarLane>(lane.clone()),
            view,
            layout: LayoutData::ZERO,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

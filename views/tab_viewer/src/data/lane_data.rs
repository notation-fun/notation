use std::fmt::Display;
use edger_bevy::bevy_prelude::*;

use notation_model::prelude::{BarLane, BarLaneProps, TabBarProps};

#[derive(Clone, Debug, Component)]
pub struct LaneData<T: Send + Sync + 'static> {
    pub bar_props: TabBarProps,
    pub lane_props: BarLaneProps,
    pub value: T,
}
impl<T: Send + Sync + ToString + 'static> Display for LaneData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LaneData<{}>({}: {}: {})",
            std::any::type_name::<T>(),
            self.bar_props.bar_ordinal,
            self.lane_props.index,
            self.value.to_string()
        )
    }
}

impl<T: Send + Sync + 'static> LaneData<T> {
    pub fn new(lane: &BarLane, value: T) -> Self {
        Self {
            bar_props: lane.bar_props(),
            lane_props: lane.props,
            value,
        }
    }
}

use notation_model::prelude::{BarLane, BarLaneProps, TabBarProps};

#[derive(Clone, Debug)]
pub struct LaneData<T: Send + Sync + 'static> {
    pub bar_props: TabBarProps,
    pub lane_props: BarLaneProps,
    pub value: T,
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

use notation_model::prelude::{BarLaneProps, LaneEntry, LaneEntryProps, TabBarProps};

#[derive(Clone, Debug)]
pub struct EntryData<T: Send + Sync + 'static> {
    pub bar_props: TabBarProps,
    pub lane_props: BarLaneProps,
    pub entry_props: LaneEntryProps,
    pub value: T,
}

impl<T: Send + Sync + 'static> EntryData<T> {
    pub fn new(entry: &LaneEntry, value: T) -> Self {
        Self {
            bar_props: entry.bar_props(),
            lane_props: entry.lane_props(),
            entry_props: entry.props,
            value,
        }
    }
}

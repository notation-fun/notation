use edger_bevy_app::bevy_prelude::*;
use notation_model::prelude::{BarLaneProps, BarPosition, LaneEntry, LaneEntryProps, TabBarProps};

#[derive(Clone, Debug, Component)]
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
            entry_props: entry.props.clone(),
            value,
        }
    }
    pub fn bar_position(&self) -> BarPosition {
        BarPosition::new(
            self.bar_props.bar_units,
            self.bar_props.bar_ordinal,
            self.entry_props.in_bar_pos,
        )
    }
}

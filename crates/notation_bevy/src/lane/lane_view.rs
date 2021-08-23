use std::sync::Arc;

use bevy::prelude::*;

use bevy_utils::prelude::{LayoutConstraint, LayoutSize, VBoxCell, View};
use crate::prelude::{LaneData};
use crate::ui::layout::NotationLayout;
use crate::prelude::{
    AddEntryEvent, BevyUtil, LaneBundle, LaneLayoutData,
};
use notation_model::prelude::{BarLane, BarPosition, LaneKind, TabBar};

pub type LaneView = LaneData<LaneKind>;

impl<'a> View<NotationLayout<'a>> for LaneView {
    fn calc_size(&self, engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        let height = engine.theme.sizes.layout.calc_lane_height(self.value);
        LayoutSize::new(constraint.max.width, height + engine.theme.sizes.layout.lane_margin)
    }
}
impl<'a> VBoxCell<NotationLayout<'a>> for LaneView {}

impl LaneView {
    pub fn create_lane(
        commands: &mut Commands,
        bar_entity: Entity,
        bar: &TabBar,
        add_entry_evts: &mut EventWriter<AddEntryEvent>,
        lane: &Arc<BarLane>,
        lane_layout: &LaneLayoutData,
    ) {
        let lane_bundle = LaneBundle::new(lane.clone(), *lane_layout);
        let lane_entity = BevyUtil::spawn_child_bundle(commands, bar_entity, lane_bundle);
        for entry in lane.entries.iter() {
            add_entry_evts.send(AddEntryEvent(
                lane_entity,
                entry.clone(),
                BarPosition::new(
                    bar.bar_units(),
                    bar.props.bar_ordinal,
                    entry.props.in_bar_pos,
                ),
            ));
        }
    }
}


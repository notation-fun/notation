use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;

use bevy_utils::prelude::{LayoutConstraint, LayoutSize, VBoxCell, View, ViewBundle};
use crate::ui::layout::NotationLayout;
use crate::prelude::{
    AddEntryEvent, BevyUtil, LaneBundle, LaneLayoutData,
};
use notation_model::prelude::{BarLane, BarPosition, TabBar};

pub type LaneView = LaneLayoutData;

impl<'a> View<NotationLayout<'a>> for LaneView {
    fn calc_size(&self, _engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        if self.visible {
            LayoutSize::new(constraint.max.width, self.height + self.margin)
        } else {
            LayoutSize::ZERO
        }
    }
}
impl<'a> VBoxCell<NotationLayout<'a>> for LaneView {
    fn order(&self) -> usize {
        self.index
    }
}

impl LaneView {
    pub fn create_lane(
        commands: &mut Commands,
        bar_entity: Entity,
        bar: &TabBar,
        add_entry_evts: &mut EventWriter<AddEntryEvent>,
        lane_layout: &LaneLayoutData,
    ) {
        if let Some(lane) = &lane_layout.lane {
            let lane_bundle = LaneBundle::new(lane.clone(), lane_layout.clone());
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
        } else {
            let view_bundle = ViewBundle::from(lane_layout.clone());
            BevyUtil::spawn_child_bundle(commands, bar_entity, view_bundle);
        }
    }
}

